//! WASM Draw Engine wrapper
//!
//! This module provides JavaScript bindings for the DrawConnect core engine.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use base64::{Engine as _, engine::general_purpose};

use drawconnect_core::{
    DrawEngine, Color, Stroke, StrokePoint, BrushMode,
    selection::SelectionMode,
};

use crate::bridge::{hex_to_color, color_to_hex, layer_to_js, pixels_to_base64_png};
use crate::types::*;

/// The main WASM Draw Engine
#[wasm_bindgen]
pub struct WasmDrawEngine {
    engine: Option<DrawEngine>,
    current_file: Option<String>,
}

#[wasm_bindgen]
impl WasmDrawEngine {
    /// Create a new WasmDrawEngine instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: None,
            current_file: None,
        }
    }

    // ========================================================================
    // Canvas Commands
    // ========================================================================

    /// Create a new canvas
    #[wasm_bindgen(js_name = createCanvas)]
    pub fn create_canvas(
        &mut self,
        width: u32,
        height: u32,
        dpi: Option<u32>,
        background: Option<String>,
    ) -> Result<JsValue, JsError> {
        let engine = DrawEngine::new().map_err(|e| JsError::new(&e.to_string()))?;

        // Set canvas size
        {
            let canvas_arc = engine.canvas();
            let mut canvas = canvas_arc.write();
            canvas.resize(width, height).map_err(|e| JsError::new(&e.to_string()))?;
        }

        let bg_color_str = background.clone().unwrap_or_else(|| "#FFFFFF".to_string());
        let bg_color = Color::from_hex(&bg_color_str).unwrap_or(Color::white());

        // Add default background layer with proper dimensions
        {
            let layer_manager_arc = engine.layer_manager();
            let mut layer_manager = layer_manager_arc.write();

            layer_manager.set_canvas_size(width, height);

            let bg_id = layer_manager.add_layer("Background");
            if let Some(layer) = layer_manager.get_layer(bg_id) {
                layer.write().fill(bg_color);
            }

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

        self.engine = Some(engine);
        self.current_file = None;

        let info = CanvasInfo {
            width,
            height,
            dpi: dpi.unwrap_or(300),
            background_color: bg_color_str,
        };

        serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Load image data from a Uint8Array and create canvas
    #[wasm_bindgen(js_name = openImageFromBytes)]
    pub fn open_image_from_bytes(&mut self, data: &[u8]) -> Result<JsValue, JsError> {
        let img = image::load_from_memory(data)
            .map_err(|e| JsError::new(&format!("Failed to load image: {}", e)))?;

        let (img_width, img_height) = img.dimensions();
        if img_width == 0 || img_height == 0 {
            return Err(JsError::new("Image has zero dimensions"));
        }

        let rgba_img = img.to_rgba8();

        let engine = DrawEngine::new().map_err(|e| JsError::new(&e.to_string()))?;

        // Set canvas size to match image
        {
            let canvas_arc = engine.canvas();
            let mut canvas = canvas_arc.write();
            canvas.resize(img_width, img_height).map_err(|e| JsError::new(&e.to_string()))?;
        }

        // Create layer and draw image onto it
        {
            let layer_manager_arc = engine.layer_manager();
            let mut layer_manager = layer_manager_arc.write();

            layer_manager.set_canvas_size(img_width, img_height);

            let bg_id = layer_manager.add_layer("Background");
            let _ = layer_manager.set_active_layer(bg_id);

            if let Some(layer_arc) = layer_manager.get_layer(bg_id) {
                let mut layer = layer_arc.write();

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

        self.engine = Some(engine);
        self.current_file = None;

        let info = CanvasInfo {
            width: img_width,
            height: img_height,
            dpi: 300,
            background_color: "#FFFFFF".to_string(),
        };

        serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get canvas info
    #[wasm_bindgen(js_name = getCanvasInfo)]
    pub fn get_canvas_info(&self) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let canvas_arc = engine.canvas();
        let canvas = canvas_arc.read();

        let info = CanvasInfo {
            width: canvas.width(),
            height: canvas.height(),
            dpi: 300,
            background_color: "#FFFFFF".to_string(),
        };

        serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Render canvas to base64 PNG
    #[wasm_bindgen(js_name = renderCanvas)]
    pub fn render_canvas(&self) -> Result<String, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let pixels = engine.render().map_err(|e| JsError::new(&e.to_string()))?;

        let canvas = engine.canvas();
        let canvas_read = canvas.read();
        let width = canvas_read.width();
        let height = canvas_read.height();
        drop(canvas_read);

        let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(width, height, pixels)
                .ok_or_else(|| JsError::new("Failed to create image buffer"))?;

        let mut png_data: Vec<u8> = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.encode(img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .map_err(|e| JsError::new(&format!("Failed to encode PNG: {}", e)))?;

        Ok(general_purpose::STANDARD.encode(&png_data))
    }

    // ========================================================================
    // Layer Commands
    // ========================================================================

    /// Get all layers
    #[wasm_bindgen(js_name = getLayers)]
    pub fn get_layers(&self) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

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

        serde_wasm_bindgen::to_value(&layers).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Add a new layer
    #[wasm_bindgen(js_name = addLayer)]
    pub fn add_layer(&self, name: String) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let id = layer_manager.add_layer(&name);

        if let Some(layer_arc) = layer_manager.get_layer(id) {
            let layer = layer_arc.read();
            let info = LayerInfo {
                id: layer.id.to_string(),
                name: layer.name.clone(),
                visible: layer.visible,
                locked: layer.lock.is_locked(),
                opacity: layer.opacity,
                blend_mode: layer.blend_mode.name().to_string(),
            };
            serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
        } else {
            Err(JsError::new("Failed to create layer"))
        }
    }

    /// Delete a layer
    #[wasm_bindgen(js_name = deleteLayer)]
    pub fn delete_layer(&self, layer_id: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        layer_manager.remove_layer(uuid);

        Ok(())
    }

    /// Set active layer
    #[wasm_bindgen(js_name = setActiveLayer)]
    pub fn set_active_layer(&self, layer_id: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        layer_manager.set_active_layer(uuid).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(())
    }

    /// Set layer visibility
    #[wasm_bindgen(js_name = setLayerVisibility)]
    pub fn set_layer_visibility(&self, layer_id: String, visible: bool) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let layer_manager = layer_manager_arc.read();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;

        if let Some(layer_arc) = layer_manager.get_layer(uuid) {
            layer_arc.write().visible = visible;
            Ok(())
        } else {
            Err(JsError::new("Layer not found"))
        }
    }

    /// Set layer opacity
    #[wasm_bindgen(js_name = setLayerOpacity)]
    pub fn set_layer_opacity(&self, layer_id: String, opacity: f32) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let layer_manager = layer_manager_arc.read();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;

        if let Some(layer_arc) = layer_manager.get_layer(uuid) {
            layer_arc.write().opacity = opacity.clamp(0.0, 1.0);
            Ok(())
        } else {
            Err(JsError::new("Layer not found"))
        }
    }

    /// Move layer up
    #[wasm_bindgen(js_name = moveLayerUp)]
    pub fn move_layer_up(&self, layer_id: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        layer_manager.move_layer_up(uuid).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(())
    }

    /// Move layer down
    #[wasm_bindgen(js_name = moveLayerDown)]
    pub fn move_layer_down(&self, layer_id: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        layer_manager.move_layer_down(uuid).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(())
    }

    /// Duplicate a layer
    #[wasm_bindgen(js_name = duplicateLayer)]
    pub fn duplicate_layer(&self, layer_id: String) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        let new_id = layer_manager.duplicate_layer(uuid).map_err(|e| JsError::new(&e.to_string()))?;

        if let Some(layer_arc) = layer_manager.get_layer(new_id) {
            let layer = layer_arc.read();
            let info = LayerInfo {
                id: layer.id.to_string(),
                name: layer.name.clone(),
                visible: layer.visible,
                locked: layer.lock.is_locked(),
                opacity: layer.opacity,
                blend_mode: layer.blend_mode.name().to_string(),
            };
            serde_wasm_bindgen::to_value(&info).map_err(|e| JsError::new(&e.to_string()))
        } else {
            Err(JsError::new("Failed to duplicate layer"))
        }
    }

    /// Merge layer down
    #[wasm_bindgen(js_name = mergeLayerDown)]
    pub fn merge_layer_down(&self, layer_id: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let layer_manager_arc = engine.layer_manager();
        let mut layer_manager = layer_manager_arc.write();

        let uuid = uuid::Uuid::parse_str(&layer_id).map_err(|e| JsError::new(&e.to_string()))?;
        layer_manager.merge_layer_down(uuid).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(())
    }

    // ========================================================================
    // Brush Commands
    // ========================================================================

    /// Get all brushes
    #[wasm_bindgen(js_name = getBrushes)]
    pub fn get_brushes(&self) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let brush_engine_arc = engine.brush_engine();
        let brush_engine = brush_engine_arc.read();

        let brushes: Vec<BrushInfo> = brush_engine
            .presets()
            .iter()
            .map(|preset| BrushInfo {
                name: preset.name.clone(),
                category: preset.category.clone(),
                size: preset.settings.size,
                opacity: preset.settings.opacity,
                hardness: preset.settings.hardness,
            })
            .collect();

        serde_wasm_bindgen::to_value(&brushes).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Set current brush
    #[wasm_bindgen(js_name = setBrush)]
    pub fn set_brush(&self, brush_name: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();

        brush_engine.set_brush_by_name(&brush_name)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Set brush color
    #[wasm_bindgen(js_name = setBrushColor)]
    pub fn set_brush_color(&self, hex: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let color = hex_to_color(&hex).map_err(|e| JsError::new(&e))?;

        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_color(color);

        Ok(())
    }

    /// Set brush size
    #[wasm_bindgen(js_name = setBrushSize)]
    pub fn set_brush_size(&self, size: f32) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_size(size);

        Ok(())
    }

    /// Set brush opacity
    #[wasm_bindgen(js_name = setBrushOpacity)]
    pub fn set_brush_opacity(&self, opacity: f32) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_opacity(opacity);

        Ok(())
    }

    /// Set brush mode (normal/eraser)
    #[wasm_bindgen(js_name = setBrushMode)]
    pub fn set_brush_mode(&self, mode: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let brush_mode = match mode.to_lowercase().as_str() {
            "eraser" => BrushMode::Eraser,
            _ => BrushMode::Normal,
        };

        let brush_engine_arc = engine.brush_engine();
        let mut brush_engine = brush_engine_arc.write();
        brush_engine.set_mode(brush_mode);

        Ok(())
    }

    // ========================================================================
    // Stroke Commands
    // ========================================================================

    /// Begin a new stroke
    #[wasm_bindgen(js_name = beginStroke)]
    pub fn begin_stroke(&self) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;
        engine.begin_stroke().map_err(|e| JsError::new(&e.to_string()))
    }

    /// Add a point to the current stroke
    #[wasm_bindgen(js_name = addStrokePoint)]
    pub fn add_stroke_point(&self, x: f32, y: f32, pressure: f32, tilt_x: f32, tilt_y: f32, timestamp: f64) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let point = StrokePoint {
            position: glam::Vec2::new(x, y),
            pressure,
            tilt: glam::Vec2::new(tilt_x, tilt_y),
            timestamp: timestamp as u64,
        };

        engine.add_stroke_point(point).map_err(|e| JsError::new(&e.to_string()))
    }

    /// End the current stroke
    #[wasm_bindgen(js_name = endStroke)]
    pub fn end_stroke(&self) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;
        engine.end_stroke().map_err(|e| JsError::new(&e.to_string()))
    }

    // ========================================================================
    // Undo/Redo Commands
    // ========================================================================

    /// Undo last action
    #[wasm_bindgen]
    pub fn undo(&self) -> Result<bool, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;
        engine.undo().map_err(|e| JsError::new(&e.to_string()))
    }

    /// Redo last undone action
    #[wasm_bindgen]
    pub fn redo(&self) -> Result<bool, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;
        engine.redo().map_err(|e| JsError::new(&e.to_string()))
    }

    /// Check if undo is available
    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.engine.as_ref().map(|e| e.can_undo()).unwrap_or(false)
    }

    /// Check if redo is available
    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.engine.as_ref().map(|e| e.can_redo()).unwrap_or(false)
    }

    // ========================================================================
    // Color Commands
    // ========================================================================

    /// Pick color at position
    #[wasm_bindgen(js_name = pickColor)]
    pub fn pick_color(&self, x: u32, y: u32) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let color = engine.pick_color(x, y).map_err(|e| JsError::new(&e.to_string()))?;
        let (r, g, b, a) = color.to_rgba8();

        let result = ColorResult {
            hex: color_to_hex(&color),
            r,
            g,
            b,
            a,
        };

        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Flood fill at position
    #[wasm_bindgen(js_name = floodFill)]
    pub fn flood_fill(&self, x: u32, y: u32, hex: String, tolerance: f32) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let color = hex_to_color(&hex).map_err(|e| JsError::new(&e))?;
        engine.flood_fill(x, y, color, tolerance).map_err(|e| JsError::new(&e.to_string()))
    }

    // ========================================================================
    // Selection Commands
    // ========================================================================

    /// Create rectangular selection
    #[wasm_bindgen(js_name = selectRect)]
    pub fn select_rect(&self, x: u32, y: u32, width: u32, height: u32) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();

        selection_manager.select_rectangle(x, y, width, height);
        let info = selection_manager.get_info();

        serde_wasm_bindgen::to_value(&SelectionInfoDto::from(info))
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Create lasso selection
    #[wasm_bindgen(js_name = selectLasso)]
    pub fn select_lasso(&self, points: JsValue) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let points: Vec<(u32, u32)> = serde_wasm_bindgen::from_value(points)
            .map_err(|e| JsError::new(&e.to_string()))?;

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();

        selection_manager.select_lasso(&points);
        let info = selection_manager.get_info();

        serde_wasm_bindgen::to_value(&SelectionInfoDto::from(info))
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Clear selection
    #[wasm_bindgen(js_name = clearSelection)]
    pub fn clear_selection(&self) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();
        selection_manager.clear();

        Ok(())
    }

    /// Select all
    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&self) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();

        selection_manager.select_all();
        let info = selection_manager.get_info();

        serde_wasm_bindgen::to_value(&SelectionInfoDto::from(info))
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Invert selection
    #[wasm_bindgen(js_name = invertSelection)]
    pub fn invert_selection(&self) -> Result<JsValue, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();

        selection_manager.invert();
        let info = selection_manager.get_info();

        serde_wasm_bindgen::to_value(&SelectionInfoDto::from(info))
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Set selection mode
    #[wasm_bindgen(js_name = setSelectionMode)]
    pub fn set_selection_mode(&self, mode: String) -> Result<(), JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let selection_mode = match mode.to_lowercase().as_str() {
            "add" => SelectionMode::Add,
            "subtract" => SelectionMode::Subtract,
            "intersect" => SelectionMode::Intersect,
            _ => SelectionMode::Replace,
        };

        let selection_manager_arc = engine.selection_manager();
        let mut selection_manager = selection_manager_arc.write();
        selection_manager.set_mode(selection_mode);

        Ok(())
    }

    // ========================================================================
    // Export Commands
    // ========================================================================

    /// Export canvas as PNG bytes
    #[wasm_bindgen(js_name = exportPng)]
    pub fn export_png(&self) -> Result<Vec<u8>, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let pixels = engine.render().map_err(|e| JsError::new(&e.to_string()))?;

        let canvas = engine.canvas();
        let canvas_read = canvas.read();
        let width = canvas_read.width();
        let height = canvas_read.height();
        drop(canvas_read);

        let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(width, height, pixels)
                .ok_or_else(|| JsError::new("Failed to create image buffer"))?;

        let mut png_data: Vec<u8> = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.encode(img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .map_err(|e| JsError::new(&format!("Failed to encode PNG: {}", e)))?;

        Ok(png_data)
    }

    /// Export canvas as JPEG bytes
    #[wasm_bindgen(js_name = exportJpeg)]
    pub fn export_jpeg(&self, quality: u8) -> Result<Vec<u8>, JsError> {
        let engine = self.engine.as_ref().ok_or_else(|| JsError::new("No canvas open"))?;

        let pixels = engine.render().map_err(|e| JsError::new(&e.to_string()))?;

        let canvas = engine.canvas();
        let canvas_read = canvas.read();
        let width = canvas_read.width();
        let height = canvas_read.height();
        drop(canvas_read);

        let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(width, height, pixels)
                .ok_or_else(|| JsError::new("Failed to create image buffer"))?;

        // Convert RGBA to RGB for JPEG
        let rgb_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::from_fn(width, height, |x, y| {
                let pixel = img.get_pixel(x, y);
                image::Rgb([pixel[0], pixel[1], pixel[2]])
            });

        let mut jpeg_data: Vec<u8> = Vec::new();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, quality);
        encoder.encode(rgb_img.as_raw(), width, height, image::ExtendedColorType::Rgb8)
            .map_err(|e| JsError::new(&format!("Failed to encode JPEG: {}", e)))?;

        Ok(jpeg_data)
    }
}

// ============================================================================
// Helper DTOs
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CanvasInfo {
    width: u32,
    height: u32,
    dpi: u32,
    background_color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LayerInfo {
    id: String,
    name: String,
    visible: bool,
    locked: bool,
    opacity: f32,
    blend_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BrushInfo {
    name: String,
    category: String,
    size: f32,
    opacity: f32,
    hardness: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ColorResult {
    hex: String,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SelectionInfoDto {
    is_active: bool,
    bounds: Option<(u32, u32, u32, u32)>,
    mode: String,
    feather: f32,
    shape_type: String,
}

impl From<drawconnect_core::SelectionInfo> for SelectionInfoDto {
    fn from(info: drawconnect_core::SelectionInfo) -> Self {
        Self {
            is_active: info.is_active,
            bounds: info.bounds,
            mode: match info.mode {
                SelectionMode::Replace => "replace".to_string(),
                SelectionMode::Add => "add".to_string(),
                SelectionMode::Subtract => "subtract".to_string(),
                SelectionMode::Intersect => "intersect".to_string(),
            },
            feather: info.feather,
            shape_type: format!("{:?}", info.shape_type).to_lowercase(),
        }
    }
}

use image::GenericImageView;
