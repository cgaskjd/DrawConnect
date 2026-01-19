//! DrawConnect Desktop Application - Tauri Backend
//!
//! This module bridges the Rust core engine with the frontend UI.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugin_commands;

use std::sync::Arc;
use std::path::Path;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use image::GenericImageView;

use drawconnect_core::{
    DrawEngine, Color, Stroke, StrokePoint, BrushMode,
    selection::{SelectionMode, SelectionInfo},
    import::{AbrParser, PatParser, SwatchParser},
};

use plugin_commands::PluginManagerState;

/// Application state shared across all commands
pub struct AppState {
    engine: Arc<RwLock<Option<DrawEngine>>>,
    current_file: Arc<RwLock<Option<String>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            engine: Arc::new(RwLock::new(None)),
            current_file: Arc::new(RwLock::new(None)),
        }
    }
}

// ============================================================================
// Data Transfer Objects (DTOs)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasInfo {
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    pub background_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushInfo {
    pub name: String,
    pub category: String,
    pub size: f32,
    pub opacity: f32,
    pub hardness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrokePointData {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorData {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub hex: String,
}

// ============================================================================
// PS Import Data Transfer Objects
// ============================================================================

/// Imported brush info from PS .abr file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedBrushInfo {
    pub name: String,
    pub diameter: u32,
    pub hardness: f32,
    pub spacing: f32,
    pub angle: f32,
    pub roundness: f32,
    pub has_tip_image: bool,
}

/// Imported pattern info from PS .pat file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedPatternInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

/// Imported color swatch info from PS .aco/.ase file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedSwatchInfo {
    pub name: String,
    pub hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ============================================================================
// Canvas Commands
// ============================================================================

/// Create a new canvas
#[tauri::command]
fn create_canvas(
    state: State<AppState>,
    width: u32,
    height: u32,
    dpi: Option<u32>,
    background: Option<String>,
) -> Result<CanvasInfo, String> {
    let mut engine_lock = state.engine.write();

    let engine = DrawEngine::new().map_err(|e| e.to_string())?;

    // Set canvas size
    {
        let canvas_arc = engine.canvas();
        let mut canvas = canvas_arc.write();
        canvas.resize(width, height).map_err(|e| e.to_string())?;
    }

    // Store background color for return
    let bg_color_str = background.clone().unwrap_or_else(|| "#FFFFFF".to_string());
    let bg_color = Color::from_hex(&bg_color_str).unwrap_or(Color::white());

    // Add default background layer with proper dimensions
    {
        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        // Update layer manager's canvas dimensions
        layer_manager.set_canvas_size(width, height);

        // Create background layer and fill with white
        let bg_id = layer_manager.add_layer("Background");
        if let Some(layer) = layer_manager.get_layer(bg_id) {
            layer.write().fill(bg_color);
        }

        // Add default drawing layer
        layer_manager.add_layer("Layer 1");
    }

    // Set selection manager canvas size
    {
        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();
        selection_manager.set_canvas_size(width, height);
    }

    // Load brush presets
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.load_presets();
    }

    *engine_lock = Some(engine);
    *state.current_file.write() = None;

    Ok(CanvasInfo {
        width,
        height,
        dpi: dpi.unwrap_or(300),
        background_color: bg_color_str,
    })
}

/// Open an image file and create a canvas from it
#[tauri::command]
fn open_image_as_canvas(
    state: State<AppState>,
    path: String,
) -> Result<CanvasInfo, String> {
    // Load image first to get dimensions
    let img = image::open(Path::new(&path))
        .map_err(|e| format!("Failed to load image '{}': {}", path, e))?;
    let (img_width, img_height) = img.dimensions();

    if img_width == 0 || img_height == 0 {
        return Err("Image has zero dimensions".to_string());
    }

    let rgba_img = img.to_rgba8();

    let mut engine_lock = state.engine.write();
    let engine = DrawEngine::new().map_err(|e| e.to_string())?;

    // Set canvas size to match image
    {
        let canvas_arc = engine.canvas();
        let mut canvas = canvas_arc.write();
        canvas.resize(img_width, img_height).map_err(|e| e.to_string())?;
    }

    // Create layer and draw image onto it
    {
        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        // Update layer manager's canvas dimensions
        layer_manager.set_canvas_size(img_width, img_height);

        // Create background layer with the image
        let bg_id = layer_manager.add_layer("Background");

        // Set it as active layer
        let _ = layer_manager.set_active_layer(bg_id);

        if let Some(layer_arc) = layer_manager.get_layer(bg_id) {
            let mut layer = layer_arc.write();

            // Copy image pixels to layer
            for py in 0..img_height {
                for px in 0..img_width {
                    let pixel = rgba_img.get_pixel(px, py);
                    let idx = ((py * img_width + px) * 4) as usize;
                    if idx + 4 <= layer.pixels.len() {
                        layer.pixels[idx] = pixel[0];
                        layer.pixels[idx + 1] = pixel[1];
                        layer.pixels[idx + 2] = pixel[2];
                        layer.pixels[idx + 3] = pixel[3];
                    }
                }
            }
        }
    }

    // Set selection manager canvas size
    {
        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();
        selection_manager.set_canvas_size(img_width, img_height);
    }

    // Load brush presets
    {
        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.load_presets();
    }

    *engine_lock = Some(engine);
    *state.current_file.write() = None;

    Ok(CanvasInfo {
        width: img_width,
        height: img_height,
        dpi: 300,
        background_color: "#FFFFFF".to_string(),
    })
}

/// Get canvas info
#[tauri::command]
fn get_canvas_info(state: State<AppState>) -> Result<CanvasInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let canvas_arc = engine.canvas();
    let canvas = canvas_arc.read();

    Ok(CanvasInfo {
        width: canvas.width(),
        height: canvas.height(),
        dpi: 300,
        background_color: "#FFFFFF".to_string(),
    })
}

/// Render canvas to PNG bytes (base64 encoded)
#[tauri::command]
fn render_canvas(state: State<AppState>) -> Result<String, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Get raw RGBA pixels from render pipeline
    let pixels = engine.render().map_err(|e| e.to_string())?;

    // Get canvas dimensions
    let canvas = engine.canvas();
    let canvas_read = canvas.read();
    let width = canvas_read.width();
    let height = canvas_read.height();
    drop(canvas_read);

    // Encode to PNG using image crate
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width, height, pixels)
            .ok_or("Failed to create image buffer")?;

    let mut png_data: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_data);
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    Ok(general_purpose::STANDARD.encode(&png_data))
}

// ============================================================================
// Layer Commands
// ============================================================================

/// Get all layers
#[tauri::command]
fn get_layers(state: State<AppState>) -> Result<Vec<LayerInfo>, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let layers: Vec<LayerInfo> = layer_manager
        .layers()
        .iter()
        .map(|l| {
            let layer = l.read();
            LayerInfo {
                id: layer.id.to_string(),
                name: layer.name.clone(),
                visible: layer.visible,
                locked: layer.lock.is_locked(),
                opacity: layer.opacity,
                blend_mode: layer.blend_mode.name().to_string(),
            }
        })
        .collect();

    Ok(layers)
}

/// Add a new layer
#[tauri::command]
fn add_layer(state: State<AppState>, name: String) -> Result<LayerInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let id = layer_manager.add_layer(&name);

    if let Some(layer_arc) = layer_manager.get_layer(id) {
        let layer = layer_arc.read();
        Ok(LayerInfo {
            id: layer.id.to_string(),
            name: layer.name.clone(),
            visible: layer.visible,
            locked: layer.lock.is_locked(),
            opacity: layer.opacity,
            blend_mode: layer.blend_mode.name().to_string(),
        })
    } else {
        Err("Failed to create layer".to_string())
    }
}

/// Delete a layer
#[tauri::command]
fn delete_layer(state: State<AppState>, layer_id: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    layer_manager.remove_layer(uuid);

    Ok(())
}

/// Set active layer
#[tauri::command]
fn set_active_layer(state: State<AppState>, layer_id: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    layer_manager.set_active_layer(uuid).map_err(|e| e.to_string())?;

    Ok(())
}

/// Set layer visibility
#[tauri::command]
fn set_layer_visibility(state: State<AppState>, layer_id: String, visible: bool) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;

    if let Some(layer_arc) = layer_manager.get_layer(uuid) {
        layer_arc.write().visible = visible;
        Ok(())
    } else {
        Err("Layer not found".to_string())
    }
}

/// Set layer opacity
#[tauri::command]
fn set_layer_opacity(state: State<AppState>, layer_id: String, opacity: f32) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;

    if let Some(layer_arc) = layer_manager.get_layer(uuid) {
        layer_arc.write().opacity = opacity.clamp(0.0, 1.0);
        Ok(())
    } else {
        Err("Layer not found".to_string())
    }
}

/// Move layer up (towards top)
#[tauri::command]
fn move_layer_up(state: State<AppState>, layer_id: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    layer_manager.move_layer_up(uuid).map_err(|e| e.to_string())?;

    Ok(())
}

/// Move layer down (towards bottom)
#[tauri::command]
fn move_layer_down(state: State<AppState>, layer_id: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    layer_manager.move_layer_down(uuid).map_err(|e| e.to_string())?;

    Ok(())
}

/// Duplicate a layer
#[tauri::command]
fn duplicate_layer(state: State<AppState>, layer_id: String) -> Result<LayerInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    let new_id = layer_manager.duplicate_layer(uuid).map_err(|e| e.to_string())?;

    // Get the new layer info
    if let Some(layer_arc) = layer_manager.get_layer(new_id) {
        let layer = layer_arc.read();
        Ok(LayerInfo {
            id: layer.id.to_string(),
            name: layer.name.clone(),
            visible: layer.visible,
            locked: layer.lock.is_locked(),
            opacity: layer.opacity,
            blend_mode: layer.blend_mode.name().to_string(),
        })
    } else {
        Err("Failed to get duplicated layer".to_string())
    }
}

/// Merge layer down
#[tauri::command]
fn merge_layer_down(state: State<AppState>, layer_id: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    layer_manager.merge_down(uuid).map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Brush Commands
// ============================================================================

/// Get available brushes
#[tauri::command]
fn get_brushes(state: State<AppState>) -> Result<Vec<BrushInfo>, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let brush_engine = brush_engine_arc.read();

    let brushes: Vec<BrushInfo> = brush_engine
        .brushes()
        .map(|b| BrushInfo {
            name: b.name.clone(),
            category: b.category.clone(),
            size: b.settings.size,
            opacity: b.settings.opacity,
            hardness: b.settings.hardness,
        })
        .collect();

    Ok(brushes)
}

/// Set current brush by name
#[tauri::command]
fn set_brush(state: State<AppState>, brush_name: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    if brush_engine.set_current_brush_by_name(&brush_name) {
        Ok(())
    } else {
        Err(format!("Brush '{}' not found", brush_name))
    }
}

/// Set brush color
#[tauri::command]
fn set_brush_color(state: State<AppState>, hex: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    let color = Color::from_hex(&hex).ok_or_else(|| format!("Invalid color: {}", hex))?;
    brush_engine.set_color(color);

    Ok(())
}

/// Get current brush color
#[tauri::command]
fn get_brush_color(state: State<AppState>) -> Result<ColorData, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let brush_engine = brush_engine_arc.read();

    let color = brush_engine.current_color();

    Ok(ColorData {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
        hex: color.to_hex(),
    })
}

/// Set brush size
#[tauri::command]
fn set_brush_size(state: State<AppState>, size: f32) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    brush_engine.current_brush_mut().settings.size = size.clamp(1.0, 1000.0);

    Ok(())
}

/// Set brush opacity
#[tauri::command]
fn set_brush_opacity(state: State<AppState>, opacity: f32) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    brush_engine.current_brush_mut().settings.opacity = opacity.clamp(0.0, 1.0);

    Ok(())
}

/// Set brush mode (normal or eraser)
#[tauri::command]
fn set_brush_mode(state: State<AppState>, mode: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    let brush_mode = match mode.to_lowercase().as_str() {
        "eraser" => BrushMode::Eraser,
        _ => BrushMode::Normal,
    };

    brush_engine.set_mode(brush_mode);

    Ok(())
}

/// Import a brush from a .dcbrush file
#[tauri::command]
fn import_brush(state: State<AppState>, path: String) -> Result<BrushInfo, String> {
    use std::fs;

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Read the brush file
    let json_content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read brush file: {}", e))?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    // Import the brush
    let brush_id = brush_engine.import_brush_from_json(&json_content)
        .map_err(|e| format!("Failed to import brush: {}", e))?;

    // Get the imported brush info
    let brush = brush_engine.brushes()
        .find(|b| b.id == brush_id)
        .ok_or("Failed to find imported brush")?;

    Ok(BrushInfo {
        name: brush.name.clone(),
        category: brush.category.clone(),
        size: brush.settings.size,
        opacity: brush.settings.opacity,
        hardness: brush.settings.hardness,
    })
}

/// Export a brush to a .dcbrush file
#[tauri::command]
fn export_brush(state: State<AppState>, brush_name: String, path: String) -> Result<(), String> {
    use std::fs;

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let brush_engine = brush_engine_arc.read();

    // Export the brush to JSON
    let json_content = brush_engine.export_brush_by_name(&brush_name)
        .map_err(|e| format!("Failed to export brush: {}", e))?;

    // Write to file
    fs::write(&path, json_content)
        .map_err(|e| format!("Failed to write brush file: {}", e))?;

    Ok(())
}

/// Delete a custom brush
#[tauri::command]
fn delete_custom_brush(state: State<AppState>, brush_name: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let brush_engine_arc = engine.brush_engine();
    let mut brush_engine = brush_engine_arc.write();

    // Find the brush ID by name
    let brush_id = brush_engine.brushes()
        .find(|b| b.name == brush_name && b.is_custom)
        .map(|b| b.id);

    if let Some(id) = brush_id {
        brush_engine.remove_brush(id);
        Ok(())
    } else {
        Err(format!("Custom brush '{}' not found", brush_name))
    }
}

// ============================================================================
// Stroke Commands
// ============================================================================

/// Begin a new stroke
#[tauri::command]
fn begin_stroke(state: State<AppState>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;
    engine.begin_stroke().map_err(|e| e.to_string())
}

/// Add a point to the current stroke and render incrementally
#[tauri::command]
fn add_stroke_point(state: State<AppState>, point: StrokePointData) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let stroke_point = StrokePoint::full(
        point.x,
        point.y,
        point.pressure,
        point.tilt_x,
        point.tilt_y,
        0.0,
        point.timestamp,
    );

    engine.add_stroke_point(stroke_point).map_err(|e| e.to_string())
}

/// End the current stroke
#[tauri::command]
fn end_stroke(state: State<AppState>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;
    engine.end_stroke().map_err(|e| e.to_string())
}

/// Process a stroke (array of points) - legacy method
#[tauri::command]
fn process_stroke(state: State<AppState>, points: Vec<StrokePointData>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    if points.is_empty() {
        return Ok(());
    }

    let mut stroke = Stroke::new();

    for p in points {
        stroke.add_point(StrokePoint::full(
            p.x,
            p.y,
            p.pressure,
            p.tilt_x,
            p.tilt_y,
            0.0,
            p.timestamp,
        ));
    }

    engine.process_stroke(&stroke).map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Undo/Redo Commands
// ============================================================================

/// Undo last action
#[tauri::command]
fn undo(state: State<AppState>) -> Result<bool, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    engine.undo().map_err(|e| e.to_string())
}

/// Redo last undone action
#[tauri::command]
fn redo(state: State<AppState>) -> Result<bool, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    engine.redo().map_err(|e| e.to_string())
}

/// Check if can undo
#[tauri::command]
fn can_undo(state: State<AppState>) -> Result<bool, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    Ok(engine.can_undo())
}

/// Check if can redo
#[tauri::command]
fn can_redo(state: State<AppState>) -> Result<bool, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    Ok(engine.can_redo())
}

// ============================================================================
// File Commands
// ============================================================================

/// Save to file
#[tauri::command]
async fn save_file(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Get raw RGBA pixels
    let pixels = engine.render().map_err(|e| e.to_string())?;

    // Get canvas dimensions
    let canvas = engine.canvas();
    let canvas_read = canvas.read();
    let width = canvas_read.width();
    let height = canvas_read.height();
    drop(canvas_read);

    // Create image buffer and save as PNG
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width, height, pixels)
            .ok_or("Failed to create image buffer")?;

    img.save(&path).map_err(|e| format!("Failed to save file: {}", e))?;

    *state.current_file.write() = Some(path);

    Ok(())
}

/// Export as PNG
#[tauri::command]
async fn export_png(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Get raw RGBA pixels
    let pixels = engine.render().map_err(|e| e.to_string())?;

    // Get canvas dimensions
    let canvas = engine.canvas();
    let canvas_read = canvas.read();
    let width = canvas_read.width();
    let height = canvas_read.height();
    drop(canvas_read);

    // Create image buffer and save as PNG
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width, height, pixels)
            .ok_or("Failed to create image buffer")?;

    img.save(&path).map_err(|e| format!("Failed to save PNG: {}", e))?;

    Ok(())
}

/// Import image from file to active layer (centered by default)
#[tauri::command]
async fn import_image(state: State<'_, AppState>, path: String, x: Option<i32>, y: Option<i32>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Load image using image crate
    let img = image::open(Path::new(&path)).map_err(|e| format!("Failed to load image '{}': {}", path, e))?;
    let (img_width, img_height) = img.dimensions();

    if img_width == 0 || img_height == 0 {
        return Err("Image has zero dimensions".to_string());
    }

    let rgba_img = img.to_rgba8();

    // Get active layer and draw the image onto it
    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let active_layer = layer_manager.active_layer()
        .ok_or("No active layer")?;

    let mut layer = active_layer.write();
    let layer_width = layer.width();
    let layer_height = layer.height();

    if layer_width == 0 || layer_height == 0 {
        return Err(format!("Layer has invalid dimensions: {}x{}", layer_width, layer_height));
    }

    // Verify and fix pixel array size if needed
    let expected_size = (layer_width * layer_height * 4) as usize;
    if layer.pixels.len() != expected_size {
        // Resize pixel array to correct size
        layer.pixels.resize(expected_size, 0);
    }

    // Calculate offset - center the image if no position specified
    let offset_x = x.unwrap_or_else(|| {
        ((layer_width as i32) - (img_width as i32)) / 2
    });
    let offset_y = y.unwrap_or_else(|| {
        ((layer_height as i32) - (img_height as i32)) / 2
    });

    let mut pixels_written = 0u32;

    // Draw each pixel from the image onto the layer - write directly to pixels array
    for py in 0..img_height {
        for px in 0..img_width {
            let layer_x = offset_x + px as i32;
            let layer_y = offset_y + py as i32;

            // Skip pixels outside layer bounds
            if layer_x < 0 || layer_y < 0 {
                continue;
            }

            let layer_x = layer_x as u32;
            let layer_y = layer_y as u32;

            if layer_x >= layer_width || layer_y >= layer_height {
                continue;
            }

            let pixel = rgba_img.get_pixel(px, py);

            // Write all non-transparent pixels
            if pixel[3] > 0 {
                let idx = ((layer_y * layer_width + layer_x) * 4) as usize;
                // Double check bounds (should always pass after resize)
                if idx + 4 <= layer.pixels.len() {
                    layer.pixels[idx] = pixel[0];
                    layer.pixels[idx + 1] = pixel[1];
                    layer.pixels[idx + 2] = pixel[2];
                    layer.pixels[idx + 3] = pixel[3];
                    pixels_written += 1;
                }
            }
        }
    }

    if pixels_written == 0 {
        return Err("No pixels were written - image may be fully transparent or outside canvas bounds".to_string());
    }

    Ok(())
}

/// Import image as new layer (centered)
#[tauri::command]
async fn import_image_as_layer(state: State<'_, AppState>, path: String, layer_name: Option<String>) -> Result<LayerInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Load image to get dimensions
    let img = image::open(Path::new(&path)).map_err(|e| format!("Failed to load image '{}': {}", path, e))?;
    let (img_width, img_height) = img.dimensions();

    if img_width == 0 || img_height == 0 {
        return Err("Image has zero dimensions".to_string());
    }

    let rgba_img = img.to_rgba8();

    // Create new layer
    let layer_manager_arc = engine.layer_manager();
    let mut layer_manager = layer_manager_arc.write();

    let name = layer_name.unwrap_or_else(|| {
        Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Imported Image")
            .to_string()
    });

    let layer_id = layer_manager.add_layer(&name);

    // Get the layer and draw image onto it (centered)
    if let Some(layer_arc) = layer_manager.get_layer(layer_id) {
        let mut layer = layer_arc.write();
        let layer_width = layer.width();
        let layer_height = layer.height();

        if layer_width == 0 || layer_height == 0 {
            return Err(format!("Layer has invalid dimensions: {}x{}", layer_width, layer_height));
        }

        // Verify and fix pixel array size if needed
        let expected_size = (layer_width * layer_height * 4) as usize;
        if layer.pixels.len() != expected_size {
            layer.pixels.resize(expected_size, 0);
        }

        // Calculate offset to center the image
        let offset_x = ((layer_width as i32) - (img_width as i32)) / 2;
        let offset_y = ((layer_height as i32) - (img_height as i32)) / 2;

        let mut pixels_written = 0u32;

        // Draw image onto layer - write directly to pixels array
        for py in 0..img_height {
            for px in 0..img_width {
                let layer_x = offset_x + px as i32;
                let layer_y = offset_y + py as i32;

                // Skip pixels outside layer bounds
                if layer_x < 0 || layer_y < 0 {
                    continue;
                }

                let layer_x = layer_x as u32;
                let layer_y = layer_y as u32;

                if layer_x >= layer_width || layer_y >= layer_height {
                    continue;
                }

                let pixel = rgba_img.get_pixel(px, py);

                // Write all non-transparent pixels
                if pixel[3] > 0 {
                    let idx = ((layer_y * layer_width + layer_x) * 4) as usize;
                    if idx + 4 <= layer.pixels.len() {
                        layer.pixels[idx] = pixel[0];
                        layer.pixels[idx + 1] = pixel[1];
                        layer.pixels[idx + 2] = pixel[2];
                        layer.pixels[idx + 3] = pixel[3];
                        pixels_written += 1;
                    }
                }
            }
        }

        if pixels_written == 0 {
            return Err("No pixels were written - image may be fully transparent or outside canvas bounds".to_string());
        }

        Ok(LayerInfo {
            id: layer.id.to_string(),
            name: layer.name.clone(),
            visible: layer.visible,
            locked: layer.lock.is_locked(),
            opacity: layer.opacity,
            blend_mode: layer.blend_mode.name().to_string(),
        })
    } else {
        Err("Failed to create layer".to_string())
    }
}

// ============================================================================
// Debug Commands
// ============================================================================

/// Get debug info about layers (for troubleshooting)
#[tauri::command]
fn debug_layer_info(state: State<AppState>) -> Result<String, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let canvas = engine.canvas();
    let canvas_read = canvas.read();
    let canvas_width = canvas_read.width();
    let canvas_height = canvas_read.height();
    drop(canvas_read);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let mut info = format!("Canvas: {}x{}\n", canvas_width, canvas_height);
    info.push_str(&format!("Layer count: {}\n\n", layer_manager.layer_count()));

    for (i, layer_arc) in layer_manager.layers().iter().enumerate() {
        let layer = layer_arc.read();
        let non_transparent_pixels: u32 = (0..layer.width() * layer.height())
            .filter(|&idx| {
                let pixel_idx = (idx as usize) * 4;
                pixel_idx + 3 < layer.pixels.len() && layer.pixels[pixel_idx + 3] > 0
            })
            .count() as u32;

        info.push_str(&format!(
            "Layer {}: '{}'\n  - Dimensions: {}x{}\n  - Pixels array size: {}\n  - Expected size: {}\n  - Visible: {}\n  - Opacity: {}\n  - Non-transparent pixels: {}\n\n",
            i,
            layer.name,
            layer.width(),
            layer.height(),
            layer.pixels.len(),
            layer.width() * layer.height() * 4,
            layer.visible,
            layer.opacity,
            non_transparent_pixels
        ));
    }

    Ok(info)
}

// ============================================================================
// Color Utilities
// ============================================================================

/// Convert color between formats
#[tauri::command]
fn convert_color(hex: String) -> Result<ColorData, String> {
    let color = Color::from_hex(&hex).ok_or_else(|| format!("Invalid color: {}", hex))?;

    Ok(ColorData {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
        hex: color.to_hex(),
    })
}

/// Get color HSB values
#[tauri::command]
fn color_to_hsb(hex: String) -> Result<(f32, f32, f32), String> {
    let color = Color::from_hex(&hex).ok_or_else(|| format!("Invalid color: {}", hex))?;
    Ok(color.to_hsb())
}

/// Create color from HSB
#[tauri::command]
fn color_from_hsb(h: f32, s: f32, b: f32) -> Result<String, String> {
    let color = Color::from_hsb(h, s, b);
    Ok(color.to_hex())
}

// ============================================================================
// Selection Commands
// ============================================================================

/// Create a rectangle selection
#[tauri::command]
fn select_rect(state: State<AppState>, x: f32, y: f32, width: f32, height: f32) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.select_rectangle(x, y, width, height);

    Ok(selection_manager.get_info())
}

/// Create a lasso selection from points
#[tauri::command]
fn select_lasso(state: State<AppState>, points: Vec<(f32, f32)>) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.select_lasso(points);

    Ok(selection_manager.get_info())
}

/// Magic wand selection at a point
#[tauri::command]
fn select_magic_wand(state: State<AppState>, x: u32, y: u32, tolerance: Option<f32>) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Get the current layer pixels
    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    let active_layer = layer_manager.active_layer()
        .ok_or("No active layer")?;

    let layer = active_layer.read();
    let width = layer.width();
    let height = layer.height();
    let pixels = layer.pixels.clone();
    drop(layer);
    drop(layer_manager);

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    if let Some(tol) = tolerance {
        selection_manager.set_tolerance(tol);
    }

    selection_manager.select_magic_wand(x, y, &pixels, width, height)
        .map_err(|e| e.to_string())?;

    Ok(selection_manager.get_info())
}

/// Get current selection info
#[tauri::command]
fn get_selection(state: State<AppState>) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let selection_manager = selection_manager_arc.read();

    Ok(selection_manager.get_info())
}

/// Clear current selection
#[tauri::command]
fn clear_selection(state: State<AppState>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.clear();

    Ok(())
}

/// Select all
#[tauri::command]
fn select_all(state: State<AppState>) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let canvas_arc = engine.canvas();
    let canvas = canvas_arc.read();
    let width = canvas.width();
    let height = canvas.height();
    drop(canvas);

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.select_all(width, height);

    Ok(selection_manager.get_info())
}

/// Invert selection
#[tauri::command]
fn invert_selection(state: State<AppState>) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let canvas_arc = engine.canvas();
    let canvas = canvas_arc.read();
    let width = canvas.width();
    let height = canvas.height();
    drop(canvas);

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.invert(width, height);

    Ok(selection_manager.get_info())
}

/// Expand selection by pixels
#[tauri::command]
fn expand_selection(state: State<AppState>, pixels: f32) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.expand(pixels);

    Ok(selection_manager.get_info())
}

/// Shrink selection by pixels
#[tauri::command]
fn shrink_selection(state: State<AppState>, pixels: f32) -> Result<SelectionInfo, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    selection_manager.contract(pixels);

    Ok(selection_manager.get_info())
}

/// Set selection mode
#[tauri::command]
fn set_selection_mode(state: State<AppState>, mode: String) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let selection_manager_arc = engine.selection_manager();
    let mut selection_manager = selection_manager_arc.write();

    let selection_mode = match mode.to_lowercase().as_str() {
        "add" => SelectionMode::Add,
        "subtract" => SelectionMode::Subtract,
        "intersect" => SelectionMode::Intersect,
        _ => SelectionMode::Replace,
    };

    selection_manager.set_mode(selection_mode);

    Ok(())
}

// ============================================================================
// Eyedropper and Fill Commands
// ============================================================================

/// Pick color at position (eyedropper tool)
#[tauri::command]
fn pick_color(state: State<AppState>, x: u32, y: u32) -> Result<ColorData, String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let color = engine.pick_color(x, y).map_err(|e| e.to_string())?;

    Ok(ColorData {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
        hex: color.to_hex(),
    })
}

/// Flood fill at position (paint bucket tool)
#[tauri::command]
fn flood_fill(state: State<AppState>, x: u32, y: u32, hex: String, tolerance: Option<f32>) -> Result<(), String> {
    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let fill_color = Color::from_hex(&hex).ok_or("Invalid color hex")?;
    let tolerance = tolerance.unwrap_or(0.1);

    engine.flood_fill(x, y, fill_color, tolerance).map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Image Adjustments Commands
// ============================================================================

/// Adjust brightness and contrast
#[tauri::command]
fn adjust_brightness_contrast(state: State<AppState>, brightness: f32, contrast: f32) -> Result<(), String> {
    use drawconnect_core::adjustments::{BrightnessContrast, Adjustment};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // Convert from -100..100 range to -1.0..1.0 range
    let adjustment = BrightnessContrast::new(brightness / 100.0, contrast / 100.0);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Brightness/Contrast");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        // Apply the adjustment
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust levels
#[tauri::command]
fn adjust_levels(
    state: State<AppState>,
    input_black: f32,
    input_white: f32,
    gamma: f32,
    output_black: f32,
    output_white: f32,
    channel: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{Levels, CurveChannel, Adjustment};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let curve_channel = match channel.as_deref() {
        Some("red") | Some("r") => CurveChannel::Red,
        Some("green") | Some("g") => CurveChannel::Green,
        Some("blue") | Some("b") => CurveChannel::Blue,
        _ => CurveChannel::RGB,
    };

    let adjustment = Levels::new(input_black, input_white, gamma, output_black, output_white, curve_channel);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Levels");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust curves
#[tauri::command]
fn adjust_curves(
    state: State<AppState>,
    points: Vec<(f32, f32)>,
    channel: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{Curves, CurvePoint, CurveChannel, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let curve_points: Vec<CurvePoint> = points
        .iter()
        .map(|(x, y)| CurvePoint::new(*x, *y))
        .collect();

    let curve_channel = match channel.as_deref() {
        Some("red") | Some("r") => CurveChannel::Red,
        Some("green") | Some("g") => CurveChannel::Green,
        Some("blue") | Some("b") => CurveChannel::Blue,
        _ => CurveChannel::RGB,
    };

    let adjustment = Curves::new(curve_points, curve_channel);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust hue, saturation, and lightness
#[tauri::command]
fn adjust_hue_saturation(
    state: State<AppState>,
    hue: f32,
    saturation: f32,
    lightness: f32,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{HueSaturation, Adjustment};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    // hue is already in -180..180 range
    // saturation and lightness need conversion from -100..100 to -1.0..1.0
    let adjustment = HueSaturation::new(hue, saturation / 100.0, lightness / 100.0);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Hue/Saturation");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust color balance
#[tauri::command]
fn adjust_color_balance(
    state: State<AppState>,
    shadows: (f32, f32, f32),
    midtones: (f32, f32, f32),
    highlights: (f32, f32, f32),
) -> Result<(), String> {
    use drawconnect_core::adjustments::{ColorBalance, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = ColorBalance::new(
        [shadows.0, shadows.1, shadows.2],
        [midtones.0, midtones.1, midtones.2],
        [highlights.0, highlights.1, highlights.2],
    );

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust vibrance
#[tauri::command]
fn adjust_vibrance(
    state: State<AppState>,
    vibrance: f32,
    saturation: f32,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{Vibrance, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = Vibrance::new(vibrance, saturation);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Adjust exposure
#[tauri::command]
fn adjust_exposure(
    state: State<AppState>,
    exposure: f32,
    offset: f32,
    gamma: f32,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{Exposure, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = Exposure::new(exposure, offset, gamma);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Convert to black and white with channel mix
#[tauri::command]
fn adjust_black_white(
    state: State<AppState>,
    red: f32,
    yellow: f32,
    green: f32,
    cyan: f32,
    blue: f32,
    magenta: f32,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{BlackWhite, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = BlackWhite::new(red, yellow, green, cyan, blue, magenta);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply photo filter
#[tauri::command]
fn adjust_photo_filter(
    state: State<AppState>,
    color: String,
    density: f32,
    preserve_luminosity: bool,
) -> Result<(), String> {
    use drawconnect_core::adjustments::{PhotoFilter, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter_color = Color::from_hex(&color).ok_or("Invalid color")?;
    let adjustment = PhotoFilter::new(filter_color, density, preserve_luminosity);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Invert colors
#[tauri::command]
fn adjust_invert(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::adjustments::{Invert, Adjustment};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = Invert::new();

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Posterize to limited levels
#[tauri::command]
fn adjust_posterize(state: State<AppState>, levels: u8) -> Result<(), String> {
    use drawconnect_core::adjustments::{Posterize, Adjustment};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = Posterize::new(levels);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Posterize");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply threshold
#[tauri::command]
fn adjust_threshold(state: State<AppState>, level: u8) -> Result<(), String> {
    use drawconnect_core::adjustments::{Threshold, Adjustment};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let adjustment = Threshold::new(level);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Threshold");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        adjustment.apply_to_layer(&mut layer);
    }

    Ok(())
}

// ============================================================================
// Filter Commands
// ============================================================================

/// Apply Gaussian blur
#[tauri::command]
fn filter_gaussian_blur(state: State<AppState>, radius: f32) -> Result<(), String> {
    use drawconnect_core::filters::{GaussianBlur, Filter};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = GaussianBlur::new(radius);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Gaussian Blur");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply box blur
#[tauri::command]
fn filter_box_blur(state: State<AppState>, radius: u32) -> Result<(), String> {
    use drawconnect_core::filters::{BoxBlur, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = BoxBlur::new(radius);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply motion blur
#[tauri::command]
fn filter_motion_blur(state: State<AppState>, angle: f32, distance: u32) -> Result<(), String> {
    use drawconnect_core::filters::{MotionBlur, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = MotionBlur::new(angle, distance);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply radial blur
#[tauri::command]
fn filter_radial_blur(
    state: State<AppState>,
    amount: f32,
    center_x: f32,
    center_y: f32,
    blur_type: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::filters::{RadialBlur, RadialBlurType, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let blur_type = match blur_type.as_deref() {
        Some("zoom") => RadialBlurType::Zoom,
        _ => RadialBlurType::Spin,
    };

    let filter = RadialBlur::new(amount, center_x, center_y, blur_type);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply unsharp mask
#[tauri::command]
fn filter_unsharp_mask(
    state: State<AppState>,
    amount: f32,
    radius: f32,
    threshold: u8,
) -> Result<(), String> {
    use drawconnect_core::filters::{UnsharpMask, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = UnsharpMask::new(amount, radius, threshold);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply high pass filter
#[tauri::command]
fn filter_high_pass(state: State<AppState>, radius: f32) -> Result<(), String> {
    use drawconnect_core::filters::{HighPass, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = HighPass::new(radius);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Add noise to image
#[tauri::command]
fn filter_add_noise(
    state: State<AppState>,
    amount: f32,
    noise_type: Option<String>,
    monochrome: bool,
) -> Result<(), String> {
    use drawconnect_core::filters::{AddNoise, NoiseType, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let noise_type = match noise_type.as_deref() {
        Some("gaussian") => NoiseType::Gaussian,
        _ => NoiseType::Uniform,
    };

    let filter = AddNoise::new(amount, noise_type, monochrome);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Reduce noise
#[tauri::command]
fn filter_reduce_noise(
    state: State<AppState>,
    strength: f32,
    preserve_details: f32,
) -> Result<(), String> {
    use drawconnect_core::filters::{ReduceNoise, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = ReduceNoise::new(strength, preserve_details);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Find edges (Sobel edge detection)
#[tauri::command]
fn filter_find_edges(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::filters::{FindEdges, Filter};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = FindEdges::new();

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply emboss effect
#[tauri::command]
fn filter_emboss(
    state: State<AppState>,
    angle: f32,
    height: f32,
    amount: f32,
) -> Result<(), String> {
    use drawconnect_core::filters::{Emboss, Filter};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = Emboss::new(angle, height, amount);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Emboss");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply pixelate effect
#[tauri::command]
fn filter_pixelate(state: State<AppState>, cell_size: u32) -> Result<(), String> {
    use drawconnect_core::filters::{Pixelate, Filter};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = Pixelate::new(cell_size);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Pixelate");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply oil paint effect
#[tauri::command]
fn filter_oil_paint(state: State<AppState>, radius: u32, levels: u32) -> Result<(), String> {
    use drawconnect_core::filters::{OilPaint, Filter};
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = OilPaint::new(radius, levels);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        // Save current state for undo before modifying
        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Oil Paint");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

// ============================================================================
// Distort Filter Commands
// ============================================================================

/// Apply spherize distortion
#[tauri::command]
fn filter_spherize(state: State<AppState>, amount: i32, mode: Option<String>) -> Result<(), String> {
    use drawconnect_core::filters::distort::{Spherize, SpherizeMode};
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let spherize_mode = match mode.as_deref() {
        Some("horizontal") => SpherizeMode::Horizontal,
        Some("vertical") => SpherizeMode::Vertical,
        _ => SpherizeMode::Normal,
    };

    let filter = Spherize::new(amount, spherize_mode);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Spherize");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply twirl distortion
#[tauri::command]
fn filter_twirl(state: State<AppState>, angle: f32, radius: Option<f32>) -> Result<(), String> {
    use drawconnect_core::filters::distort::Twirl;
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let filter = Twirl::new(angle, radius.unwrap_or(100.0));

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Twirl");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply wave distortion
#[tauri::command]
fn filter_wave(
    state: State<AppState>,
    wavelength: f32,
    amplitude: f32,
    wave_type: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::filters::distort::{Wave, WaveType};
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let wt = match wave_type.as_deref() {
        Some("triangle") => WaveType::Triangle,
        Some("square") => WaveType::Square,
        _ => WaveType::Sine,
    };

    let filter = Wave::new(wt, wavelength, amplitude);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Wave");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply ripple distortion
#[tauri::command]
fn filter_ripple(state: State<AppState>, amount: f32, size: Option<String>) -> Result<(), String> {
    use drawconnect_core::filters::distort::{Ripple, RippleSize};
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let ripple_size = match size.as_deref() {
        Some("small") => RippleSize::Small,
        Some("large") => RippleSize::Large,
        _ => RippleSize::Medium,
    };

    let filter = Ripple::new(amount, ripple_size);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Ripple");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

// ============================================================================
// Render Filter Commands
// ============================================================================

/// Apply vignette effect
#[tauri::command]
fn filter_vignette(
    state: State<AppState>,
    amount: f32,
    midpoint: Option<f32>,
    feather: Option<f32>,
) -> Result<(), String> {
    use drawconnect_core::filters::render::Vignette;
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let mut filter = Vignette::new(amount);
    if let Some(m) = midpoint {
        filter.midpoint = m;
    }
    if let Some(f) = feather {
        filter.feather = f;
    }

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Vignette");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Apply lens flare effect
#[tauri::command]
fn filter_lens_flare(
    state: State<AppState>,
    center_x: f32,
    center_y: f32,
    brightness: Option<f32>,
    style: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::filters::render::{LensFlare, FlareStyle};
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let mut filter = LensFlare::new(center_x, center_y, brightness.unwrap_or(100.0));
    filter.style = match style.as_deref() {
        Some("prime35") => FlareStyle::Prime35,
        Some("lens105") => FlareStyle::Lens105,
        Some("movie") => FlareStyle::MoviePrime,
        _ => FlareStyle::Zoom50_300,
    };

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Lens Flare");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

/// Generate clouds
#[tauri::command]
fn filter_clouds(
    state: State<AppState>,
    foreground: Option<String>,
    background: Option<String>,
    seed: Option<u32>,
) -> Result<(), String> {
    use drawconnect_core::filters::render::Clouds;
    use drawconnect_core::filters::Filter;
    use drawconnect_core::{LayerSnapshot, HistoryState};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let mut filter = Clouds::new(seed.unwrap_or(42));

    if let Some(fg) = foreground {
        if let Some(color) = Color::from_hex(&fg) {
            let (r, g, b, _) = color.to_rgba8();
            filter.foreground = (r, g, b);
        }
    }
    if let Some(bg) = background {
        if let Some(color) = Color::from_hex(&bg) {
            let (r, g, b, _) = color.to_rgba8();
            filter.background = (r, g, b);
        }
    }

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();

        let snapshot = LayerSnapshot::new(
            layer.id,
            layer.pixels.clone(),
            layer.width(),
            layer.height(),
        );
        let mut history_state = HistoryState::new("Clouds");
        history_state.add_snapshot(snapshot);
        engine.history_manager().write().push_state(history_state);

        filter.apply_to_layer(&mut layer);
    }

    Ok(())
}

// ============================================================================
// Transform Commands
// ============================================================================

/// Rotate 90 degrees clockwise
#[tauri::command]
fn transform_rotate_90_cw(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, rotate_90_cw};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let rotated = rotate_90_cw(&img);
        layer.pixels = rotated.pixels;
        layer.bounds = (0, 0, rotated.width, rotated.height);
    }

    Ok(())
}

/// Rotate 90 degrees counter-clockwise
#[tauri::command]
fn transform_rotate_90_ccw(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, rotate_90_ccw};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let rotated = rotate_90_ccw(&img);
        layer.pixels = rotated.pixels;
        layer.bounds = (0, 0, rotated.width, rotated.height);
    }

    Ok(())
}

/// Rotate 180 degrees
#[tauri::command]
fn transform_rotate_180(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, rotate_180};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let rotated = rotate_180(&img);
        layer.pixels = rotated.pixels;
    }

    Ok(())
}

/// Rotate by arbitrary angle
#[tauri::command]
fn transform_rotate(state: State<AppState>, angle: f32) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, rotate_arbitrary};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let rotated = rotate_arbitrary(&img, angle).map_err(|e| e.to_string())?;
        layer.pixels = rotated.pixels;
        layer.bounds = (0, 0, rotated.width, rotated.height);
    }

    Ok(())
}

/// Flip horizontally
#[tauri::command]
fn transform_flip_horizontal(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, flip_horizontal};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let flipped = flip_horizontal(&img);
        layer.pixels = flipped.pixels;
    }

    Ok(())
}

/// Flip vertically
#[tauri::command]
fn transform_flip_vertical(state: State<AppState>) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, flip_vertical};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let width = layer.width();
        let height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), width, height)
            .map_err(|e| e.to_string())?;

        let flipped = flip_vertical(&img);
        layer.pixels = flipped.pixels;
    }

    Ok(())
}

/// Crop image
#[tauri::command]
fn transform_crop(
    state: State<AppState>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, crop_image, CropRegion};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let layer_width = layer.width();
        let layer_height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), layer_width, layer_height)
            .map_err(|e| e.to_string())?;

        let region = CropRegion::new(x, y, width, height);
        let cropped = crop_image(&img, region).map_err(|e| e.to_string())?;

        layer.pixels = cropped.pixels;
        layer.bounds = (0, 0, cropped.width, cropped.height);
    }

    Ok(())
}

/// Resize canvas
#[tauri::command]
fn transform_canvas_resize(
    state: State<AppState>,
    width: u32,
    height: u32,
    anchor: Option<String>,
    fill_color: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, canvas_resize, Anchor};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let anchor = anchor
        .and_then(|s| s.parse::<Anchor>().ok())
        .unwrap_or(Anchor::MiddleCenter);

    let fill = fill_color
        .and_then(|s| Color::from_hex(&s))
        .unwrap_or(Color::transparent());

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let layer_width = layer.width();
        let layer_height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), layer_width, layer_height)
            .map_err(|e| e.to_string())?;

        let resized = canvas_resize(&img, width, height, anchor, fill)
            .map_err(|e| e.to_string())?;

        layer.pixels = resized.pixels;
        layer.bounds = (0, 0, resized.width, resized.height);
    }

    Ok(())
}

/// Resize image
#[tauri::command]
fn transform_image_resize(
    state: State<AppState>,
    width: u32,
    height: u32,
    interpolation: Option<String>,
) -> Result<(), String> {
    use drawconnect_core::transform::{ImageData, resize_image, Interpolation};

    let engine_lock = state.engine.read();
    let engine = engine_lock.as_ref().ok_or("No canvas open")?;

    let interpolation = interpolation
        .and_then(|s| s.parse::<Interpolation>().ok())
        .unwrap_or(Interpolation::Bilinear);

    let layer_manager_arc = engine.layer_manager();
    let layer_manager = layer_manager_arc.read();

    if let Some(active_layer) = layer_manager.active_layer() {
        let mut layer = active_layer.write();
        let layer_width = layer.width();
        let layer_height = layer.height();

        let img = ImageData::from_pixels(layer.pixels.clone(), layer_width, layer_height)
            .map_err(|e| e.to_string())?;

        let resized = resize_image(&img, width, height, interpolation)
            .map_err(|e| e.to_string())?;

        layer.pixels = resized.pixels;
        layer.bounds = (0, 0, resized.width, resized.height);
    }

    Ok(())
}

// ============================================================================
// PS Resource Import Commands
// ============================================================================

/// Import PS brushes from .abr file
#[tauri::command]
fn import_abr_brushes(path: String) -> Result<Vec<ImportedBrushInfo>, String> {
    use std::fs;

    let data = fs::read(&path)
        .map_err(|e| format!("Failed to read ABR file '{}': {}", path, e))?;

    let brushes = AbrParser::parse(&data)
        .map_err(|e| format!("Failed to parse ABR file: {}", e))?;

    let result: Vec<ImportedBrushInfo> = brushes
        .into_iter()
        .map(|b| ImportedBrushInfo {
            name: b.name,
            diameter: b.diameter,
            hardness: b.hardness,
            spacing: b.spacing,
            angle: b.angle,
            roundness: b.roundness,
            has_tip_image: b.tip_image.is_some(),
        })
        .collect();

    Ok(result)
}

/// Import PS patterns from .pat file
#[tauri::command]
fn import_pat_patterns(path: String) -> Result<Vec<ImportedPatternInfo>, String> {
    use std::fs;

    let data = fs::read(&path)
        .map_err(|e| format!("Failed to read PAT file '{}': {}", path, e))?;

    let patterns = PatParser::parse(&data)
        .map_err(|e| format!("Failed to parse PAT file: {}", e))?;

    let result: Vec<ImportedPatternInfo> = patterns
        .into_iter()
        .map(|p| ImportedPatternInfo {
            name: p.name,
            width: p.width,
            height: p.height,
        })
        .collect();

    Ok(result)
}

/// Import color swatches from .aco or .ase file
#[tauri::command]
fn import_color_swatches(path: String) -> Result<Vec<ImportedSwatchInfo>, String> {
    use std::fs;

    let data = fs::read(&path)
        .map_err(|e| format!("Failed to read swatch file '{}': {}", path, e))?;

    // Auto-detect format based on file content
    let swatches = SwatchParser::parse_auto(&data)
        .map_err(|e| format!("Failed to parse swatch file: {}", e))?;

    let result: Vec<ImportedSwatchInfo> = swatches
        .into_iter()
        .map(|s| {
            let hex = format!("#{:02X}{:02X}{:02X}", s.color[0], s.color[1], s.color[2]);
            ImportedSwatchInfo {
                name: s.name,
                hex,
                r: s.color[0],
                g: s.color[1],
                b: s.color[2],
                a: s.color[3],
            }
        })
        .collect();

    Ok(result)
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .manage(PluginManagerState::default())
        .invoke_handler(tauri::generate_handler![
            // Canvas
            create_canvas,
            open_image_as_canvas,
            get_canvas_info,
            render_canvas,
            // Layers
            get_layers,
            add_layer,
            delete_layer,
            set_active_layer,
            set_layer_visibility,
            set_layer_opacity,
            move_layer_up,
            move_layer_down,
            duplicate_layer,
            merge_layer_down,
            // Brushes
            get_brushes,
            set_brush,
            set_brush_color,
            get_brush_color,
            set_brush_size,
            set_brush_opacity,
            set_brush_mode,
            import_brush,
            export_brush,
            delete_custom_brush,
            // Strokes
            process_stroke,
            begin_stroke,
            add_stroke_point,
            end_stroke,
            // Undo/Redo
            undo,
            redo,
            can_undo,
            can_redo,
            // Files
            save_file,
            export_png,
            import_image,
            import_image_as_layer,
            // Debug
            debug_layer_info,
            // Colors
            convert_color,
            color_to_hsb,
            color_from_hsb,
            // Selection
            select_rect,
            select_lasso,
            select_magic_wand,
            get_selection,
            clear_selection,
            select_all,
            invert_selection,
            expand_selection,
            shrink_selection,
            set_selection_mode,
            // Eyedropper and Fill
            pick_color,
            flood_fill,
            // Image Adjustments
            adjust_brightness_contrast,
            adjust_levels,
            adjust_curves,
            adjust_hue_saturation,
            adjust_color_balance,
            adjust_vibrance,
            adjust_exposure,
            adjust_black_white,
            adjust_photo_filter,
            adjust_invert,
            adjust_posterize,
            adjust_threshold,
            // Filters
            filter_gaussian_blur,
            filter_box_blur,
            filter_motion_blur,
            filter_radial_blur,
            filter_unsharp_mask,
            filter_high_pass,
            filter_add_noise,
            filter_reduce_noise,
            filter_find_edges,
            filter_emboss,
            filter_pixelate,
            filter_oil_paint,
            // Distort Filters
            filter_spherize,
            filter_twirl,
            filter_wave,
            filter_ripple,
            // Render Filters
            filter_vignette,
            filter_lens_flare,
            filter_clouds,
            // Transforms
            transform_rotate_90_cw,
            transform_rotate_90_ccw,
            transform_rotate_180,
            transform_rotate,
            transform_flip_horizontal,
            transform_flip_vertical,
            transform_crop,
            transform_canvas_resize,
            transform_image_resize,
            // PS Resource Import
            import_abr_brushes,
            import_pat_patterns,
            import_color_swatches,
            // Plugins
            plugin_commands::init_plugin_system,
            plugin_commands::get_plugins,
            plugin_commands::get_plugin_detail,
            plugin_commands::install_plugin,
            plugin_commands::uninstall_plugin,
            plugin_commands::enable_plugin,
            plugin_commands::disable_plugin,
            plugin_commands::get_plugin_settings,
            plugin_commands::set_plugin_setting,
            plugin_commands::get_plugin_contributions,
            plugin_commands::open_plugins_folder,
            plugin_commands::refresh_plugins,
            plugin_commands::search_store_plugins,
            plugin_commands::check_plugin_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
