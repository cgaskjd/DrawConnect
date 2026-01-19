//! Basic drawing example
//!
//! Demonstrates the core drawing workflow

use drawconnect_core::*;

fn main() {
    println!("=== DrawConnect Core Engine - Basic Drawing Example ===\n");

    // 1. Create the engine
    println!("1. Creating drawing engine...");
    let engine = DrawEngine::new().expect("Failed to create engine");
    println!("   Engine created with config: {:?}\n", engine.config());

    // 2. Setup layers
    println!("2. Setting up layers...");
    {
        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        // Create a background layer
        let bg_id = layer_manager.add_layer("Background");
        if let Some(layer) = layer_manager.get_layer(bg_id) {
            layer.write().fill(Color::white());
        }
        println!("   Created 'Background' layer");

        // Create a drawing layer
        layer_manager.add_layer("Drawing");
        println!("   Created 'Drawing' layer");

        println!("   Total layers: {}\n", layer_manager.layer_count());
    }

    // 3. Configure brush
    println!("3. Configuring brush...");
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.load_presets();

        // Set color
        brush_engine.set_color(Color::from_hex("#2563EB").unwrap());
        println!("   Color set to: {}", brush_engine.current_color().to_hex());

        // Show available brushes
        println!("   Available brushes:");
        for brush in brush_engine.brushes().take(5) {
            println!("     - {} ({})", brush.name, brush.category);
        }
        println!("   ... and more\n");
    }

    // 4. Draw a stroke
    println!("4. Drawing a stroke...");
    let stroke = create_sample_stroke();
    println!("   Stroke has {} points", stroke.point_count());
    println!("   Stroke length: {:.2} pixels", stroke.length());

    if let Some((min_x, min_y, max_x, max_y)) = stroke.bounds() {
        println!("   Bounds: ({:.0}, {:.0}) to ({:.0}, {:.0})", min_x, min_y, max_x, max_y);
    }

    engine.process_stroke(&stroke).expect("Failed to process stroke");
    println!("   Stroke rendered!\n");

    // 5. Render the result
    println!("5. Rendering canvas...");
    let output = engine.render().expect("Failed to render");
    println!("   Output size: {} bytes", output.len());

    let canvas_arc = engine.canvas();
    let canvas = canvas_arc.read();
    println!("   Canvas size: {}x{}", canvas.width(), canvas.height());
    println!("   Canvas modified: {}\n", canvas.is_modified());

    // 6. Test undo/redo
    println!("6. Testing undo/redo...");
    drop(canvas);
    {
        let canvas_arc = engine.canvas();
        let mut canvas = canvas_arc.write();
        canvas.save_undo("Draw stroke");
        println!("   Saved undo state");
        println!("   Can undo: {}", canvas.can_undo());
        println!("   Undo count: {}\n", canvas.undo_count());
    }

    // 7. Color operations
    println!("7. Color operations demo...");
    let color = Color::from_hex("#FF5733").unwrap();
    println!("   Original: {}", color.to_hex());

    let (h, s, b) = color.to_hsb();
    println!("   HSB: H={:.1}, S={:.2}, B={:.2}", h, s, b);

    let (h, s, l) = color.to_hsl();
    println!("   HSL: H={:.1}, S={:.2}, L={:.2}", h, s, l);

    let complement = color.complement();
    println!("   Complement: {}", complement.to_hex());

    let lighter = color.lighten(0.2);
    println!("   Lightened: {}", lighter.to_hex());

    let darker = color.darken(0.2);
    println!("   Darkened: {}\n", darker.to_hex());

    // 8. Blend mode demo
    println!("8. Blend mode demo...");
    let base = Color::from_rgb(0.8, 0.2, 0.2);
    let blend = Color::from_rgb(0.2, 0.2, 0.8);

    println!("   Base: {} (red)", base.to_hex());
    println!("   Blend: {} (blue)", blend.to_hex());

    let modes = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
    ];

    for mode in modes {
        let result = mode.blend(base, blend);
        println!("   {}: {}", mode.name(), result.to_hex());
    }

    println!("\n=== Example completed successfully! ===");
}

fn create_sample_stroke() -> Stroke {
    let mut stroke = Stroke::new();

    // Create a curved stroke
    for i in 0..50 {
        let t = i as f32 / 49.0;
        let x = 100.0 + t * 300.0;
        let y = 200.0 + (t * std::f32::consts::PI * 2.0).sin() * 50.0;
        let pressure = 0.3 + t * 0.7;

        stroke.add_point(StrokePoint::full(
            x, y, pressure,
            0.0, 0.0, 0.0,
            (i * 10) as u64,
        ));
    }

    stroke
}
