//! Brush Engine Module
//!
//! Provides professional brush system with 300+ presets, custom brush creation,
//! and full pressure sensitivity support (8192 levels).

mod dynamics;
mod preset;
mod settings;
mod texture;

pub use dynamics::{BrushDynamics, DynamicsCurve};
pub use preset::BrushPreset;
pub use settings::BrushSettings;
pub use texture::BrushTexture;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::error::{EngineError, EngineResult};
use crate::layer::Layer;
use crate::optimize::{StampCache, PressureSmoother, catmull_rom_interpolate};
use crate::stroke::{Stroke, StrokePoint};

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Brush shape type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrushShape {
    /// Round brush
    Round,
    /// Square brush
    Square,
    /// Custom texture-based shape
    Texture,
    /// Elliptical brush
    Ellipse,
    /// Flat brush
    Flat,
}

impl Default for BrushShape {
    fn default() -> Self {
        Self::Round
    }
}

/// Brush tip type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrushTip {
    /// Solid tip
    Solid,
    /// Soft (feathered) tip
    Soft,
    /// Textured tip
    Textured,
    /// Bristle simulation
    Bristle,
    /// Airbrush
    Airbrush,
}

impl Default for BrushTip {
    fn default() -> Self {
        Self::Soft
    }
}

/// Main brush structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brush {
    /// Unique brush identifier
    pub id: Uuid,
    /// Brush name
    pub name: String,
    /// Brush shape
    pub shape: BrushShape,
    /// Brush tip type
    pub tip: BrushTip,
    /// Brush settings
    pub settings: BrushSettings,
    /// Brush dynamics (pressure, tilt, velocity responses)
    pub dynamics: BrushDynamics,
    /// Optional texture
    #[serde(skip)]
    pub texture: Option<BrushTexture>,
    /// Brush category
    pub category: String,
    /// Is this a custom user brush
    pub is_custom: bool,
}

impl Brush {
    /// Create a new brush with default settings
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            shape: BrushShape::default(),
            tip: BrushTip::default(),
            settings: BrushSettings::default(),
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "General".into(),
            is_custom: false,
        }
    }

    /// Create a brush with specific settings
    pub fn with_settings(name: impl Into<String>, settings: BrushSettings) -> Self {
        Self {
            settings,
            ..Self::new(name)
        }
    }

    /// Calculate brush size at a given pressure level
    pub fn size_at_pressure(&self, pressure: f32) -> f32 {
        let base_size = self.settings.size;
        let min_size = self.settings.min_size_ratio * base_size;
        let pressure_effect = self.dynamics.size_pressure_curve.evaluate(pressure);
        min_size + (base_size - min_size) * pressure_effect
    }

    /// Calculate brush opacity at a given pressure level
    pub fn opacity_at_pressure(&self, pressure: f32) -> f32 {
        let base_opacity = self.settings.opacity;
        let min_opacity = self.settings.min_opacity_ratio * base_opacity;
        let pressure_effect = self.dynamics.opacity_pressure_curve.evaluate(pressure);
        min_opacity + (base_opacity - min_opacity) * pressure_effect
    }

    /// Calculate brush hardness at a given pressure level
    pub fn hardness_at_pressure(&self, pressure: f32) -> f32 {
        let base_hardness = self.settings.hardness;
        let pressure_effect = self.dynamics.hardness_pressure_curve.evaluate(pressure);
        base_hardness * pressure_effect
    }

    /// Generate a brush stamp at given parameters
    pub fn generate_stamp(&self, size: f32, hardness: f32, angle: f32) -> BrushStamp {
        // Ensure minimum size of 1.0 for rendering at least one pixel
        let effective_size = size.max(1.0);
        let stamp_size = (effective_size.ceil() as u32).max(1);
        let center = stamp_size as f32 / 2.0;
        let mut data = vec![0.0f32; (stamp_size * stamp_size) as usize];

        match self.tip {
            BrushTip::Solid => {
                self.generate_solid_stamp(&mut data, stamp_size, center, hardness, angle);
            }
            BrushTip::Soft => {
                self.generate_soft_stamp(&mut data, stamp_size, center, hardness, angle);
            }
            BrushTip::Airbrush => {
                self.generate_airbrush_stamp(&mut data, stamp_size, center, angle);
            }
            _ => {
                self.generate_soft_stamp(&mut data, stamp_size, center, hardness, angle);
            }
        }

        BrushStamp {
            data,
            size: stamp_size,
        }
    }

    fn generate_solid_stamp(
        &self,
        data: &mut [f32],
        size: u32,
        center: f32,
        hardness: f32,
        _angle: f32,
    ) {
        let radius = center;
        let edge_width = radius * (1.0 - hardness);

        for y in 0..size {
            for x in 0..size {
                // Sample from pixel center (+0.5) for correct small brush rendering
                let dx = x as f32 + 0.5 - center;
                let dy = y as f32 + 0.5 - center;
                let dist = (dx * dx + dy * dy).sqrt();

                let alpha = if dist <= radius - edge_width {
                    1.0
                } else if dist <= radius {
                    let t = (radius - dist) / edge_width;
                    t * t * (3.0 - 2.0 * t) // Smoothstep
                } else {
                    0.0
                };

                data[(y * size + x) as usize] = alpha;
            }
        }
    }

    fn generate_soft_stamp(
        &self,
        data: &mut [f32],
        size: u32,
        center: f32,
        hardness: f32,
        _angle: f32,
    ) {
        let radius = center;
        let hard_radius = radius * hardness;

        for y in 0..size {
            for x in 0..size {
                // Sample from pixel center (+0.5) for correct small brush rendering
                let dx = x as f32 + 0.5 - center;
                let dy = y as f32 + 0.5 - center;
                let dist = (dx * dx + dy * dy).sqrt();

                let alpha = if dist <= hard_radius {
                    1.0
                } else if dist <= radius {
                    let t = 1.0 - (dist - hard_radius) / (radius - hard_radius);
                    t * t // Quadratic falloff
                } else {
                    0.0
                };

                data[(y * size + x) as usize] = alpha;
            }
        }
    }

    fn generate_airbrush_stamp(&self, data: &mut [f32], size: u32, center: f32, _angle: f32) {
        let radius = center;

        for y in 0..size {
            for x in 0..size {
                // Sample from pixel center (+0.5) for correct small brush rendering
                let dx = x as f32 + 0.5 - center;
                let dy = y as f32 + 0.5 - center;
                let dist = (dx * dx + dy * dy).sqrt();

                let alpha = if dist <= radius {
                    let t = 1.0 - dist / radius;
                    t * t * t // Cubic falloff for soft airbrush effect
                } else {
                    0.0
                };

                data[(y * size + x) as usize] = alpha * 0.3; // Lower base opacity
            }
        }
    }
}

/// Brush stamp data
#[derive(Debug, Clone)]
pub struct BrushStamp {
    /// Alpha values for the stamp
    pub data: Vec<f32>,
    /// Stamp size (square)
    pub size: u32,
}

/// Brush mode (normal painting or eraser)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BrushMode {
    /// Normal painting mode
    #[default]
    Normal,
    /// Eraser mode - removes pixels
    Eraser,
}

/// The brush engine manages brushes and renders strokes
pub struct BrushEngine {
    /// Active brush
    current_brush: Brush,
    /// Brush library
    brushes: HashMap<Uuid, Brush>,
    /// Current drawing color
    current_color: Color,
    /// Current brush mode (normal or eraser)
    current_mode: BrushMode,
    /// Stroke accumulator for smooth rendering
    stroke_accumulator: f32,
    /// Last rendered point
    last_point: Option<StrokePoint>,
    /// Optimized stamp cache for performance
    stamp_cache: StampCache,
    /// Pressure smoother for stable input
    pressure_smoother: PressureSmoother,
    /// Point history for spline interpolation (last 4 points)
    point_history: Vec<StrokePoint>,
}

impl BrushEngine {
    /// Create a new brush engine
    pub fn new() -> Self {
        let default_brush = Brush::new("Default Round");
        let brush_id = default_brush.id;
        let mut brushes = HashMap::new();
        brushes.insert(brush_id, default_brush.clone());

        Self {
            current_brush: default_brush,
            brushes,
            current_color: Color::black(),
            current_mode: BrushMode::Normal,
            stroke_accumulator: 0.0,
            last_point: None,
            stamp_cache: StampCache::new(256), // Cache up to 256 stamps
            pressure_smoother: PressureSmoother::new(0.3), // Smooth pressure with 30% factor
            point_history: Vec::with_capacity(4),
        }
    }

    /// Get the current brush
    pub fn current_brush(&self) -> &Brush {
        &self.current_brush
    }

    /// Get the current brush mutably
    pub fn current_brush_mut(&mut self) -> &mut Brush {
        &mut self.current_brush
    }

    /// Set the current brush by index (from brushes iterator)
    pub fn set_current_brush(&mut self, index: usize) {
        let brush_ids: Vec<Uuid> = self.brushes.keys().copied().collect();
        if let Some(&brush_id) = brush_ids.get(index) {
            if let Some(brush) = self.brushes.get(&brush_id) {
                self.current_brush = brush.clone();
                self.stamp_cache.clear(); // Clear cache when brush changes
            }
        }
    }

    /// Set the current brush by name
    pub fn set_current_brush_by_name(&mut self, name: &str) -> bool {
        for brush in self.brushes.values() {
            if brush.name == name {
                self.current_brush = brush.clone();
                self.stamp_cache.clear(); // Clear cache when brush changes
                return true;
            }
        }
        false
    }

    /// Set the current brush by ID
    pub fn set_brush(&mut self, brush_id: Uuid) -> EngineResult<()> {
        let brush = self
            .brushes
            .get(&brush_id)
            .ok_or_else(|| EngineError::BrushNotFound(brush_id.to_string()))?
            .clone();

        self.current_brush = brush;
        self.stamp_cache.clear(); // Clear cache when brush changes
        Ok(())
    }

    /// Add a brush to the library
    pub fn add_brush(&mut self, brush: Brush) {
        self.brushes.insert(brush.id, brush);
    }

    /// Remove a brush from the library
    pub fn remove_brush(&mut self, brush_id: Uuid) -> Option<Brush> {
        self.brushes.remove(&brush_id)
    }

    /// Add a custom brush to the library
    pub fn add_custom_brush(&mut self, mut brush: Brush) -> Uuid {
        brush.is_custom = true;
        brush.id = Uuid::new_v4(); // Ensure unique ID
        let id = brush.id;
        self.brushes.insert(id, brush);
        id
    }

    /// Import a brush from JSON string
    pub fn import_brush_from_json(&mut self, json: &str) -> EngineResult<Uuid> {
        let mut brush: Brush = serde_json::from_str(json)
            .map_err(|e| EngineError::SerializationError(format!("Invalid brush JSON: {}", e)))?;

        brush.settings.validate();
        let id = self.add_custom_brush(brush);
        Ok(id)
    }

    /// Export a brush to JSON string
    pub fn export_brush_to_json(&self, brush_id: Uuid) -> EngineResult<String> {
        let brush = self.brushes.get(&brush_id)
            .ok_or_else(|| EngineError::BrushNotFound(brush_id.to_string()))?;

        serde_json::to_string_pretty(brush)
            .map_err(|e| EngineError::SerializationError(format!("Failed to serialize brush: {}", e)))
    }

    /// Export a brush by name to JSON string
    pub fn export_brush_by_name(&self, name: &str) -> EngineResult<String> {
        let brush = self.brushes.values()
            .find(|b| b.name == name)
            .ok_or_else(|| EngineError::BrushNotFound(name.to_string()))?;

        serde_json::to_string_pretty(brush)
            .map_err(|e| EngineError::SerializationError(format!("Failed to serialize brush: {}", e)))
    }

    /// Get all custom brushes
    pub fn custom_brushes(&self) -> impl Iterator<Item = &Brush> {
        self.brushes.values().filter(|b| b.is_custom)
    }

    /// Get all brushes in the library
    pub fn brushes(&self) -> impl Iterator<Item = &Brush> {
        self.brushes.values()
    }

    /// Set the current drawing color
    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Get the current drawing color
    pub fn current_color(&self) -> &Color {
        &self.current_color
    }

    /// Set the brush mode (normal or eraser)
    pub fn set_mode(&mut self, mode: BrushMode) {
        self.current_mode = mode;
    }

    /// Get the current brush mode
    pub fn current_mode(&self) -> BrushMode {
        self.current_mode
    }

    /// Render a stroke to the canvas with optimized interpolation
    pub fn render_stroke(
        &mut self,
        stroke: &Stroke,
        canvas: &mut Canvas,
        layer: &Layer,
    ) -> EngineResult<()> {
        if stroke.points.is_empty() {
            return Ok(());
        }

        let spacing = self.current_brush.settings.spacing;
        let base_size = self.current_brush.settings.size;

        // Use Catmull-Rom interpolation for 4+ points, linear for fewer
        if stroke.points.len() >= 4 {
            // Catmull-Rom spline interpolation for smooth curves
            for i in 0..(stroke.points.len() - 1) {
                let p0 = if i == 0 { &stroke.points[0] } else { &stroke.points[i - 1] };
                let p1 = &stroke.points[i];
                let p2 = &stroke.points[i + 1];
                let p3 = if i + 2 < stroke.points.len() {
                    &stroke.points[i + 2]
                } else {
                    &stroke.points[stroke.points.len() - 1]
                };

                let distance = p1.position.distance(p2.position);
                let steps = (distance / (base_size * spacing)).ceil() as usize;

                for j in 0..=steps {
                    let t = if steps == 0 { 0.0 } else { j as f32 / steps as f32 };

                    // Interpolate position using Catmull-Rom
                    let (x, y) = catmull_rom_interpolate(
                        (p0.position.x, p0.position.y),
                        (p1.position.x, p1.position.y),
                        (p2.position.x, p2.position.y),
                        (p3.position.x, p3.position.y),
                        t,
                    );

                    // Linear interpolate pressure and other attributes
                    let pressure = self.pressure_smoother.update(
                        p1.pressure + (p2.pressure - p1.pressure) * t
                    );

                    let point = StrokePoint {
                        position: Vec2::new(x, y),
                        pressure,
                        tilt_x: p1.tilt_x + (p2.tilt_x - p1.tilt_x) * t,
                        tilt_y: p1.tilt_y + (p2.tilt_y - p1.tilt_y) * t,
                        rotation: p1.rotation + (p2.rotation - p1.rotation) * t,
                        timestamp: p1.timestamp + ((p2.timestamp - p1.timestamp) as f32 * t) as u64,
                    };

                    self.render_point_cached(&point, canvas, layer)?;
                }
            }
        } else {
            // Handle single point (click without drag)
            if stroke.points.len() == 1 {
                self.render_point_cached(&stroke.points[0], canvas, layer)?;
            } else {
                // Linear interpolation for short strokes (2-3 points)
                for window in stroke.points.windows(2) {
                    let p0 = &window[0];
                    let p1 = &window[1];

                    let distance = p0.position.distance(p1.position);
                    let steps = (distance / (base_size * spacing)).ceil() as usize;

                    for i in 0..=steps {
                        let t = if steps == 0 { 0.0 } else { i as f32 / steps as f32 };
                        let point = StrokePoint::lerp(p0, p1, t);
                        self.render_point_cached(&point, canvas, layer)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Render a stroke directly to a layer's pixels
    pub fn render_stroke_to_layer(
        &mut self,
        stroke: &Stroke,
        layer: &mut Layer,
    ) -> EngineResult<()> {
        if stroke.points.is_empty() {
            return Ok(());
        }

        let spacing = self.current_brush.settings.spacing;
        let base_size = self.current_brush.settings.size;

        // Use Catmull-Rom interpolation for 4+ points, linear for fewer
        if stroke.points.len() >= 4 {
            for i in 0..(stroke.points.len() - 1) {
                let p0 = if i == 0 { &stroke.points[0] } else { &stroke.points[i - 1] };
                let p1 = &stroke.points[i];
                let p2 = &stroke.points[i + 1];
                let p3 = if i + 2 < stroke.points.len() {
                    &stroke.points[i + 2]
                } else {
                    &stroke.points[stroke.points.len() - 1]
                };

                let distance = p1.position.distance(p2.position);
                let steps = (distance / (base_size * spacing)).ceil() as usize;

                for j in 0..=steps {
                    let t = if steps == 0 { 0.0 } else { j as f32 / steps as f32 };

                    let (x, y) = catmull_rom_interpolate(
                        (p0.position.x, p0.position.y),
                        (p1.position.x, p1.position.y),
                        (p2.position.x, p2.position.y),
                        (p3.position.x, p3.position.y),
                        t,
                    );

                    let pressure = self.pressure_smoother.update(
                        p1.pressure + (p2.pressure - p1.pressure) * t
                    );

                    let point = StrokePoint {
                        position: Vec2::new(x, y),
                        pressure,
                        tilt_x: p1.tilt_x + (p2.tilt_x - p1.tilt_x) * t,
                        tilt_y: p1.tilt_y + (p2.tilt_y - p1.tilt_y) * t,
                        rotation: p1.rotation + (p2.rotation - p1.rotation) * t,
                        timestamp: p1.timestamp + ((p2.timestamp - p1.timestamp) as f32 * t) as u64,
                    };

                    self.render_point_to_layer(&point, layer);
                }
            }
        } else {
            // Handle single point (click without drag)
            if stroke.points.len() == 1 {
                self.render_point_to_layer(&stroke.points[0], layer);
            } else {
                // Linear interpolation for short strokes (2-3 points)
                for window in stroke.points.windows(2) {
                    let p0 = &window[0];
                    let p1 = &window[1];

                    let distance = p0.position.distance(p1.position);
                    let steps = (distance / (base_size * spacing)).ceil() as usize;

                    for i in 0..=steps {
                        let t = if steps == 0 { 0.0 } else { i as f32 / steps as f32 };
                        let point = StrokePoint::lerp(p0, p1, t);
                        self.render_point_to_layer(&point, layer);
                    }
                }
            }
        }

        Ok(())
    }

    /// Render a single point directly to layer pixels
    fn render_point_to_layer(&mut self, point: &StrokePoint, layer: &mut Layer) {
        let size = self.current_brush.size_at_pressure(point.pressure);
        let opacity = self.current_brush.opacity_at_pressure(point.pressure);
        let hardness = self.current_brush.hardness_at_pressure(point.pressure);
        let angle = point.rotation;

        // Ensure minimum size of 1.0 for rendering at least one pixel
        let effective_size = size.max(1.0);

        // Get or generate cached stamp
        let brush = &self.current_brush;
        let stamp_data = self.stamp_cache.get_or_generate(effective_size, hardness, angle, |s, h, a| {
            let stamp = brush.generate_stamp(s, h, a);
            (stamp.data, stamp.size)
        });

        let stamp_size = (effective_size.ceil() as u32).max(1);
        let half_size = stamp_size as f32 / 2.0;
        let start_x = (point.position.x - half_size).floor() as i32;
        let start_y = (point.position.y - half_size).floor() as i32;

        let layer_width = layer.width();
        let layer_height = layer.height();
        let is_eraser = self.current_mode == BrushMode::Eraser;

        // Render stamp directly to layer pixels
        for sy in 0..stamp_size {
            for sx in 0..stamp_size {
                let px = start_x + sx as i32;
                let py = start_y + sy as i32;

                if px >= 0 && py >= 0 && (px as u32) < layer_width && (py as u32) < layer_height {
                    let idx = (sy * stamp_size + sx) as usize;
                    if idx < stamp_data.len() {
                        let stamp_alpha = stamp_data[idx];
                        if stamp_alpha > 0.001 {
                            let final_alpha = stamp_alpha * opacity;

                            // Blend pixel directly to layer
                            let pixel_idx = ((py as u32 * layer_width + px as u32) * 4) as usize;
                            if pixel_idx + 4 <= layer.pixels.len() {
                                if is_eraser {
                                    // Eraser mode: reduce alpha (Porter-Duff "destination-out")
                                    let dst_a = layer.pixels[pixel_idx + 3] as f32 / 255.0;
                                    let out_a = dst_a * (1.0 - final_alpha);
                                    layer.pixels[pixel_idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;

                                    // If alpha is very low, clear the color too
                                    if out_a < 0.01 {
                                        layer.pixels[pixel_idx] = 0;
                                        layer.pixels[pixel_idx + 1] = 0;
                                        layer.pixels[pixel_idx + 2] = 0;
                                        layer.pixels[pixel_idx + 3] = 0;
                                    }
                                } else {
                                    // Normal mode: Porter-Duff "over" compositing
                                    let color = self.current_color.with_alpha(final_alpha);

                                    // Get destination color
                                    let dst_r = layer.pixels[pixel_idx] as f32 / 255.0;
                                    let dst_g = layer.pixels[pixel_idx + 1] as f32 / 255.0;
                                    let dst_b = layer.pixels[pixel_idx + 2] as f32 / 255.0;
                                    let dst_a = layer.pixels[pixel_idx + 3] as f32 / 255.0;

                                    let src_a = color.a;
                                    let out_a = src_a + dst_a * (1.0 - src_a);

                                    if out_a > 0.001 {
                                        let out_r = (color.r * src_a + dst_r * dst_a * (1.0 - src_a)) / out_a;
                                        let out_g = (color.g * src_a + dst_g * dst_a * (1.0 - src_a)) / out_a;
                                        let out_b = (color.b * src_a + dst_b * dst_a * (1.0 - src_a)) / out_a;

                                        layer.pixels[pixel_idx] = (out_r * 255.0).clamp(0.0, 255.0) as u8;
                                        layer.pixels[pixel_idx + 1] = (out_g * 255.0).clamp(0.0, 255.0) as u8;
                                        layer.pixels[pixel_idx + 2] = (out_b * 255.0).clamp(0.0, 255.0) as u8;
                                        layer.pixels[pixel_idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Render a single point using cached stamps for performance
    fn render_point_cached(
        &mut self,
        point: &StrokePoint,
        canvas: &mut Canvas,
        _layer: &Layer,
    ) -> EngineResult<()> {
        let size = self.current_brush.size_at_pressure(point.pressure);
        let opacity = self.current_brush.opacity_at_pressure(point.pressure);
        let hardness = self.current_brush.hardness_at_pressure(point.pressure);
        let angle = point.rotation;

        // Ensure minimum size of 1.0 for rendering at least one pixel
        let effective_size = size.max(1.0);

        // Get or generate cached stamp
        let brush = &self.current_brush;
        let stamp_data = self.stamp_cache.get_or_generate(effective_size, hardness, angle, |s, h, a| {
            let stamp = brush.generate_stamp(s, h, a);
            (stamp.data, stamp.size)
        });

        let stamp_size = (effective_size.ceil() as u32).max(1);
        let half_size = stamp_size as f32 / 2.0;
        let start_x = (point.position.x - half_size).floor() as i32;
        let start_y = (point.position.y - half_size).floor() as i32;

        // Render stamp to canvas
        for sy in 0..stamp_size {
            for sx in 0..stamp_size {
                let px = start_x + sx as i32;
                let py = start_y + sy as i32;

                if px >= 0 && py >= 0 {
                    let idx = (sy * stamp_size + sx) as usize;
                    if idx < stamp_data.len() {
                        let stamp_alpha = stamp_data[idx];
                        if stamp_alpha > 0.001 { // Skip nearly transparent pixels
                            let final_alpha = stamp_alpha * opacity;
                            let color = self.current_color.with_alpha(final_alpha);
                            canvas.blend_pixel(px as u32, py as u32, color)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Render a single point (legacy method for compatibility)
    fn render_point(
        &self,
        point: &StrokePoint,
        canvas: &mut Canvas,
        _layer: &Layer,
    ) -> EngineResult<()> {
        let size = self.current_brush.size_at_pressure(point.pressure);
        let opacity = self.current_brush.opacity_at_pressure(point.pressure);
        let hardness = self.current_brush.hardness_at_pressure(point.pressure);

        let stamp = self.current_brush.generate_stamp(size, hardness, point.rotation);

        let half_size = stamp.size as f32 / 2.0;
        let start_x = (point.position.x - half_size).floor() as i32;
        let start_y = (point.position.y - half_size).floor() as i32;

        for sy in 0..stamp.size {
            for sx in 0..stamp.size {
                let px = start_x + sx as i32;
                let py = start_y + sy as i32;

                if px >= 0 && py >= 0 {
                    let stamp_alpha = stamp.data[(sy * stamp.size + sx) as usize];
                    if stamp_alpha > 0.0 {
                        let final_alpha = stamp_alpha * opacity;
                        let color = self.current_color.with_alpha(final_alpha);
                        canvas.blend_pixel(px as u32, py as u32, color)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Begin a new stroke
    pub fn begin_stroke(&mut self) {
        self.stroke_accumulator = 0.0;
        self.last_point = None;
        self.pressure_smoother.reset();
        self.point_history.clear();
    }

    /// End current stroke
    pub fn end_stroke(&mut self) {
        self.last_point = None;
        self.point_history.clear();
    }

    /// Load built-in brush presets
    pub fn load_presets(&mut self) {
        // Load all 50 brush presets
        for brush in BrushPreset::all_presets() {
            self.add_brush(brush);
        }
    }
}

impl Default for BrushEngine {
    fn default() -> Self {
        let mut engine = Self::new();
        engine.load_presets();
        engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_creation() {
        let brush = Brush::new("Test Brush");
        assert_eq!(brush.name, "Test Brush");
        assert!(!brush.is_custom);
    }

    #[test]
    fn test_pressure_calculation() {
        let brush = Brush::new("Test");

        // At full pressure, should return base size
        let size_full = brush.size_at_pressure(1.0);
        assert!((size_full - brush.settings.size).abs() < 0.01);

        // At zero pressure, should return minimum size
        let size_zero = brush.size_at_pressure(0.0);
        let min_size = brush.settings.size * brush.settings.min_size_ratio;
        assert!(size_zero >= min_size);
    }

    #[test]
    fn test_stamp_generation() {
        let brush = Brush::new("Test");
        let stamp = brush.generate_stamp(10.0, 0.5, 0.0);

        assert_eq!(stamp.size, 10);
        assert_eq!(stamp.data.len(), 100);
    }

    #[test]
    fn test_color_setting() {
        let mut engine = BrushEngine::new();

        // Default color should be black
        let default_color = engine.current_color();
        assert_eq!(default_color.r, 0.0);
        assert_eq!(default_color.g, 0.0);
        assert_eq!(default_color.b, 0.0);

        // Set a new color (red)
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        engine.set_color(red);

        // Verify the color was set
        let current = engine.current_color();
        assert_eq!(current.r, 1.0);
        assert_eq!(current.g, 0.0);
        assert_eq!(current.b, 0.0);

        // Set another color (green)
        let green = Color::from_hex("#00FF00").unwrap();
        engine.set_color(green);

        // Verify the color changed
        let current = engine.current_color();
        assert_eq!(current.r, 0.0);
        assert_eq!(current.g, 1.0);
        assert_eq!(current.b, 0.0);
    }
}
