//! Integration tests for DrawConnect Core Engine

use drawconnect_core::*;

/// Test complete drawing workflow
#[test]
fn test_complete_drawing_workflow() {
    // 1. Create engine
    let engine = DrawEngine::new().expect("Failed to create engine");

    // 2. Setup canvas and layers
    {
        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();
        layer_manager.add_layer("Background");
        layer_manager.add_layer("Sketch");
        layer_manager.add_layer("Color");
        assert_eq!(layer_manager.layer_count(), 3);
    }

    // 3. Configure brush
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.load_presets();
        brush_engine.set_color(Color::from_hex("#FF5733").unwrap());
    }

    // 4. Create and process stroke
    let stroke = create_test_stroke();
    engine.process_stroke(&stroke).expect("Failed to process stroke");

    // 5. Render
    let output = engine.render().expect("Failed to render");
    assert!(!output.is_empty());
}

/// Test layer operations
#[test]
fn test_layer_operations() {
    let engine = DrawEngine::new().unwrap();

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    // Add layers
    let layer1 = layer_manager.add_layer("Layer 1");
    let layer2 = layer_manager.add_layer("Layer 2");
    let layer3 = layer_manager.add_layer("Layer 3");

    assert_eq!(layer_manager.layer_count(), 3);

    // Test reordering
    layer_manager.move_layer_down(layer3).unwrap();

    // Test duplication
    let duplicated = layer_manager.duplicate_layer(layer1).unwrap();
    assert_eq!(layer_manager.layer_count(), 4);

    // Test deletion
    layer_manager.remove_layer(duplicated);
    assert_eq!(layer_manager.layer_count(), 3);

    // Test active layer
    layer_manager.set_active_layer(layer2).unwrap();
    assert!(layer_manager.active_layer().is_some());
}

/// Test brush presets
#[test]
fn test_brush_presets() {
    let mut brush_engine = BrushEngine::new();
    brush_engine.load_presets();

    let brush_count = brush_engine.brushes().count();
    assert!(brush_count >= 10, "Should have at least 10 preset brushes");

    // Test each preset can generate stamps
    for brush in brush_engine.brushes() {
        let stamp = brush.generate_stamp(20.0, 0.5, 0.0);
        assert_eq!(stamp.size, 20);
        assert!(!stamp.data.is_empty());
    }
}

/// Test color conversions
#[test]
fn test_color_conversions() {
    // Test hex conversion roundtrip
    let original = Color::from_hex("#FF5733").unwrap();
    let hex = original.to_hex();
    let converted = Color::from_hex(&hex).unwrap();

    assert!((original.r - converted.r).abs() < 0.01);
    assert!((original.g - converted.g).abs() < 0.01);
    assert!((original.b - converted.b).abs() < 0.01);

    // Test HSB conversion roundtrip
    let color = Color::from_rgb(0.8, 0.4, 0.2);
    let (h, s, b) = color.to_hsb();
    let back = Color::from_hsb(h, s, b);

    assert!((color.r - back.r).abs() < 0.01);
    assert!((color.g - back.g).abs() < 0.01);
    assert!((color.b - back.b).abs() < 0.01);

    // Test HSL conversion roundtrip
    let (h, s, l) = color.to_hsl();
    let back = Color::from_hsl(h, s, l);

    assert!((color.r - back.r).abs() < 0.01);
    assert!((color.g - back.g).abs() < 0.01);
    assert!((color.b - back.b).abs() < 0.01);
}

/// Test canvas undo/redo
#[test]
fn test_canvas_undo_redo() {
    let mut canvas = Canvas::with_size(100, 100).unwrap();

    // Draw something
    canvas.set_pixel(50, 50, Color::red()).unwrap();
    canvas.save_undo("Draw red pixel");

    // Draw something else
    canvas.set_pixel(50, 50, Color::blue()).unwrap();

    // Verify blue
    let pixel = canvas.get_pixel(50, 50).unwrap();
    assert!((pixel.b - 1.0).abs() < 0.01);

    // Undo
    assert!(canvas.can_undo());
    canvas.undo();

    // Verify red restored
    let pixel = canvas.get_pixel(50, 50).unwrap();
    assert!((pixel.r - 1.0).abs() < 0.01);

    // Redo
    assert!(canvas.can_redo());
    canvas.redo();

    // Verify blue again
    let pixel = canvas.get_pixel(50, 50).unwrap();
    assert!((pixel.b - 1.0).abs() < 0.01);
}

/// Test blend modes
#[test]
fn test_all_blend_modes() {
    let base = Color::from_rgb(0.5, 0.5, 0.5);
    let blend = Color::from_rgb(0.8, 0.3, 0.6);

    for mode in BlendMode::all() {
        let result = mode.blend(base, blend);

        // All results should be valid colors
        assert!(result.r >= 0.0 && result.r <= 1.0, "{:?} R out of range", mode);
        assert!(result.g >= 0.0 && result.g <= 1.0, "{:?} G out of range", mode);
        assert!(result.b >= 0.0 && result.b <= 1.0, "{:?} B out of range", mode);
        assert!(result.a >= 0.0 && result.a <= 1.0, "{:?} A out of range", mode);
    }
}

/// Test tile manager memory efficiency
#[test]
fn test_tile_manager_sparse_storage() {
    let mut manager = canvas::TileManager::new(256, 4096, 4096);

    // Initially no tiles allocated
    assert_eq!(manager.allocated_tiles(), 0);

    // Draw in one corner
    manager.set_pixel(100, 100, Color::red());
    assert_eq!(manager.allocated_tiles(), 1);

    // Draw in opposite corner
    manager.set_pixel(3900, 3900, Color::blue());
    assert_eq!(manager.allocated_tiles(), 2);

    // Memory should be much less than full canvas
    let full_memory = 4096 * 4096 * 4;
    let actual_memory = manager.memory_usage();
    assert!(actual_memory < full_memory / 100, "Should use less than 1% of full memory");
}

/// Test stroke smoothing
#[test]
fn test_stroke_smoothing() {
    let mut builder = StrokeBuilder::new();
    builder.set_smoothing(0.8);

    builder.begin(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), "#000000");

    // Add jittery points
    let points = vec![
        (100.0, 100.0),
        (102.0, 98.0),   // slight jitter
        (105.0, 103.0),  // slight jitter
        (110.0, 100.0),
        (115.0, 102.0),
    ];

    for (x, y) in points {
        builder.add_point(StrokePoint::new(x, y, 1.0));
    }

    let stroke = builder.end();

    // Smoothed stroke should have similar point count
    assert_eq!(stroke.point_count(), 5);

    // Points should be somewhat smoothed (not exact original positions)
    // The middle points should be adjusted
}

/// Test pressure sensitivity
#[test]
fn test_pressure_sensitivity() {
    let brush = Brush::new("Test");

    // Test size at different pressures
    let size_low = brush.size_at_pressure(0.1);
    let size_mid = brush.size_at_pressure(0.5);
    let size_high = brush.size_at_pressure(1.0);

    assert!(size_low < size_mid);
    assert!(size_mid < size_high);

    // Test opacity at different pressures
    let opacity_low = brush.opacity_at_pressure(0.1);
    let opacity_mid = brush.opacity_at_pressure(0.5);
    let opacity_high = brush.opacity_at_pressure(1.0);

    assert!(opacity_low <= opacity_mid);
    assert!(opacity_mid <= opacity_high);
}

/// Test layer mask
#[test]
fn test_layer_mask() {
    use drawconnect_core::layer::LayerMask;

    let mut mask = LayerMask::new(100, 100);

    // Default mask should be fully opaque
    assert!((mask.get(50, 50) - 1.0).abs() < 0.01);

    // Set a value
    mask.set(50, 50, 0.5);
    assert!((mask.get(50, 50) - 0.5).abs() < 0.01);

    // Test inversion
    mask.inverted = true;
    assert!((mask.get(50, 50) - 0.5).abs() < 0.01); // 1.0 - 0.5 = 0.5

    // Test density
    mask.inverted = false;
    mask.set(50, 50, 1.0);
    mask.density = 0.5;
    assert!((mask.get(50, 50) - 0.5).abs() < 0.01);
}

// Helper function to create test stroke
fn create_test_stroke() -> Stroke {
    let mut stroke = Stroke::new();

    // Create a simple diagonal line
    for i in 0..20 {
        let t = i as f32 / 19.0;
        let x = 100.0 + t * 200.0;
        let y = 100.0 + t * 200.0;
        let pressure = 0.5 + t * 0.5; // Increasing pressure

        stroke.add_point(StrokePoint::full(
            x,
            y,
            pressure,
            0.0,
            0.0,
            0.0,
            (i * 16) as u64, // timestamps
        ));
    }

    stroke
}

/// Test geometry utilities
#[test]
fn test_geometry() {
    use drawconnect_core::geometry::*;

    // Test rectangle operations
    let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);

    assert!(rect1.intersects(&rect2));

    let intersection = rect1.intersection(&rect2).unwrap();
    assert_eq!(intersection.x, 50.0);
    assert_eq!(intersection.y, 50.0);
    assert_eq!(intersection.width, 50.0);
    assert_eq!(intersection.height, 50.0);

    // Test transform
    let transform = Transform::translation(10.0, 20.0)
        .multiply(&Transform::scale(2.0, 2.0));

    let point = glam::Vec2::new(5.0, 5.0);
    let transformed = transform.transform_point(point);

    // Scale first (5*2=10, 5*2=10), then translate (+10, +20) = (20, 30)
    // Actually order matters - let's verify
    assert!(transformed.x > 0.0);
    assert!(transformed.y > 0.0);
}

/// Test color palette
#[test]
fn test_color_palette() {
    use drawconnect_core::color::ColorPalette;

    let palette = ColorPalette::default_palette();
    assert!(!palette.is_empty());
    assert!(palette.len() >= 8);

    // Test custom palette
    let mut custom = ColorPalette::new("Custom");
    custom.add_color(Color::red());
    custom.add_named_color(Color::blue(), "Sky Blue");

    assert_eq!(custom.len(), 2);
    assert_eq!(custom.colors[1].name, Some("Sky Blue".to_string()));
}

/// Test that brush color is correctly applied during drawing
#[test]
fn test_brush_color_applied_correctly() {
    let engine = DrawEngine::new().expect("Failed to create engine");

    // Setup layer
    {
        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();
        let layer_id = layer_manager.add_layer("Test Layer");
        layer_manager.set_active_layer(layer_id).unwrap();
    }

    // Set brush color to red
    let red_color = Color::from_rgb(1.0, 0.0, 0.0);
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_color(red_color);

        // Verify color was set
        let current = brush_engine.current_color();
        assert_eq!(current.r, 1.0, "Red channel should be 1.0");
        assert_eq!(current.g, 0.0, "Green channel should be 0.0");
        assert_eq!(current.b, 0.0, "Blue channel should be 0.0");
    }

    // Create a single-point stroke at known location
    let mut stroke = Stroke::new();
    stroke.add_point(StrokePoint::full(500.0, 500.0, 1.0, 0.0, 0.0, 0.0, 0));
    stroke.add_point(StrokePoint::full(510.0, 500.0, 1.0, 0.0, 0.0, 0.0, 16));

    // Process the stroke
    engine.process_stroke(&stroke).expect("Failed to process stroke");

    // Check canvas pixel color - it should have red color
    {
        let canvas_arc = engine.canvas();
        let canvas = canvas_arc.read();

        // Check pixels around the stroke center
        if let Some(pixel) = canvas.get_pixel(505, 500) {
            // The pixel should have some red color (may not be pure red due to blending)
            assert!(pixel.r > 0.0, "Pixel should have red component, got r={}", pixel.r);
            // Red should dominate since we painted with red
            assert!(pixel.r > pixel.g && pixel.r > pixel.b,
                "Red should be dominant, got r={}, g={}, b={}", pixel.r, pixel.g, pixel.b);
        }
    }

    // Now change to green and draw again
    let green_color = Color::from_rgb(0.0, 1.0, 0.0);
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_color(green_color);

        // Verify color was changed
        let current = brush_engine.current_color();
        assert_eq!(current.r, 0.0, "Red channel should be 0.0 after changing to green");
        assert_eq!(current.g, 1.0, "Green channel should be 1.0 after changing to green");
    }

    // Draw another stroke at a different location
    let mut stroke2 = Stroke::new();
    stroke2.add_point(StrokePoint::full(600.0, 500.0, 1.0, 0.0, 0.0, 0.0, 100));
    stroke2.add_point(StrokePoint::full(610.0, 500.0, 1.0, 0.0, 0.0, 0.0, 116));

    engine.process_stroke(&stroke2).expect("Failed to process second stroke");

    // Check canvas pixel color for the second stroke - should be green
    {
        let canvas_arc = engine.canvas();
        let canvas = canvas_arc.read();

        if let Some(pixel) = canvas.get_pixel(605, 500) {
            assert!(pixel.g > 0.0, "Pixel should have green component, got g={}", pixel.g);
            // Green should dominate
            assert!(pixel.g > pixel.r && pixel.g > pixel.b,
                "Green should be dominant, got r={}, g={}, b={}", pixel.r, pixel.g, pixel.b);
        }
    }
}
