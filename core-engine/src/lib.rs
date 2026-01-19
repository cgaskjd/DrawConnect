//! # DrawConnect Core Engine
//!
//! A high-performance, cross-platform painting engine built with Rust.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DrawConnect Core                      │
//! ├─────────┬─────────┬─────────┬─────────┬─────────────────┤
//! │  Brush  │  Layer  │  Color  │ Canvas  │     Render      │
//! │ Engine  │ System  │ Manager │  Core   │    Pipeline     │
//! └─────────┴─────────┴─────────┴─────────┴─────────────────┘
//! ```
//!
//! ## Features
//!
//! - **High Performance**: GPU-accelerated rendering with wgpu
//! - **Cross-Platform**: Works on Windows, macOS, Linux, iOS, Android, Web
//! - **Pressure Sensitivity**: Full 8192-level pressure support
//! - **Large Canvas**: Support for 16K resolution with tiled rendering
//! - **Professional Tools**: 300+ brushes, advanced layer system

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod brush;
pub mod canvas;
pub mod color;
pub mod error;
pub mod format;
pub mod geometry;
pub mod history;
pub mod import;
pub mod layer;
pub mod optimize;
pub mod plugin;
pub mod render;
pub mod selection;
pub mod stroke;
pub mod tools;
pub mod utils;

// Image editing modules
pub mod adjustments;
pub mod filters;
pub mod transform;

// Re-exports for convenience
pub use brush::{Brush, BrushEngine, BrushMode, BrushPreset, BrushSettings};
pub use canvas::{Canvas, CanvasSettings, TileManager};
pub use color::{Color, ColorSpace, ColorManager};
pub use error::{EngineError, EngineResult};
pub use history::{HistoryManager, HistoryState, LayerSnapshot, DirtyRect};
pub use layer::{Layer, LayerManager, BlendMode, LayerType};
pub use render::{RenderPipeline, RenderContext};
pub use selection::{Selection, SelectionManager, SelectionMode, SelectionInfo};
pub use stroke::{Stroke, StrokePoint, StrokeBuilder};

use std::sync::Arc;
use parking_lot::RwLock;

/// Engine version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum supported canvas dimension (16K)
pub const MAX_CANVAS_SIZE: u32 = 16384;

/// Default tile size for tiled rendering
pub const DEFAULT_TILE_SIZE: u32 = 256;

/// Maximum pressure level (8192)
pub const MAX_PRESSURE_LEVEL: u32 = 8192;

/// Core engine configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EngineConfig {
    /// Maximum canvas width
    pub max_width: u32,
    /// Maximum canvas height
    pub max_height: u32,
    /// Tile size for tiled rendering
    pub tile_size: u32,
    /// Maximum number of undo steps
    pub max_undo_steps: usize,
    /// Enable GPU acceleration
    pub gpu_enabled: bool,
    /// Memory limit in bytes
    pub memory_limit: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_width: MAX_CANVAS_SIZE,
            max_height: MAX_CANVAS_SIZE,
            tile_size: DEFAULT_TILE_SIZE,
            max_undo_steps: 100,
            gpu_enabled: true,
            memory_limit: 4 * 1024 * 1024 * 1024, // 4GB
        }
    }
}

/// The main drawing engine instance
pub struct DrawEngine {
    config: EngineConfig,
    canvas: Arc<RwLock<Canvas>>,
    layer_manager: Arc<RwLock<LayerManager>>,
    brush_engine: Arc<RwLock<BrushEngine>>,
    color_manager: Arc<ColorManager>,
    render_pipeline: Arc<RwLock<RenderPipeline>>,
    history_manager: Arc<RwLock<HistoryManager>>,
    selection_manager: Arc<RwLock<SelectionManager>>,
    // 增量笔触状态
    current_stroke: Arc<RwLock<Option<Stroke>>>,
    // 笔触开始前的像素备份（用于创建增量快照）
    stroke_before_pixels: Arc<RwLock<Option<Vec<u8>>>>,
    stroke_layer_id: Arc<RwLock<Option<uuid::Uuid>>>,
    stroke_layer_dims: Arc<RwLock<Option<(u32, u32)>>>,
    // 脏区域追踪（用于增量快照）
    stroke_dirty_rect: Arc<RwLock<Option<DirtyRect>>>,
}

impl DrawEngine {
    /// Create a new DrawEngine with default configuration
    pub fn new() -> EngineResult<Self> {
        Self::with_config(EngineConfig::default())
    }

    /// Create a new DrawEngine with custom configuration
    pub fn with_config(config: EngineConfig) -> EngineResult<Self> {
        let canvas = Arc::new(RwLock::new(Canvas::new(
            config.tile_size,
            config.max_width,
            config.max_height,
        )?));

        let layer_manager = Arc::new(RwLock::new(LayerManager::new()));
        let brush_engine = Arc::new(RwLock::new(BrushEngine::new()));
        let color_manager = Arc::new(ColorManager::new());
        let render_pipeline = Arc::new(RwLock::new(RenderPipeline::new(config.gpu_enabled)?));
        let history_manager = Arc::new(RwLock::new(HistoryManager::with_max_steps(config.max_undo_steps)));
        let selection_manager = Arc::new(RwLock::new(SelectionManager::new()));

        Ok(Self {
            config,
            canvas,
            layer_manager,
            brush_engine,
            color_manager,
            render_pipeline,
            history_manager,
            selection_manager,
            current_stroke: Arc::new(RwLock::new(None)),
            stroke_before_pixels: Arc::new(RwLock::new(None)),
            stroke_layer_id: Arc::new(RwLock::new(None)),
            stroke_layer_dims: Arc::new(RwLock::new(None)),
            stroke_dirty_rect: Arc::new(RwLock::new(None)),
        })
    }

    /// Get the engine configuration
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Get access to the canvas
    pub fn canvas(&self) -> Arc<RwLock<Canvas>> {
        Arc::clone(&self.canvas)
    }

    /// Get access to the layer manager
    pub fn layer_manager(&self) -> Arc<RwLock<LayerManager>> {
        Arc::clone(&self.layer_manager)
    }

    /// Get access to the brush engine
    pub fn brush_engine(&self) -> Arc<RwLock<BrushEngine>> {
        Arc::clone(&self.brush_engine)
    }

    /// Get access to the color manager
    pub fn color_manager(&self) -> Arc<ColorManager> {
        Arc::clone(&self.color_manager)
    }

    /// Get access to the render pipeline
    pub fn render_pipeline(&self) -> Arc<RwLock<RenderPipeline>> {
        Arc::clone(&self.render_pipeline)
    }

    /// Get access to the history manager
    pub fn history_manager(&self) -> Arc<RwLock<HistoryManager>> {
        Arc::clone(&self.history_manager)
    }

    /// Get access to the selection manager
    pub fn selection_manager(&self) -> Arc<RwLock<SelectionManager>> {
        Arc::clone(&self.selection_manager)
    }

    /// Begin a new stroke for incremental drawing
    pub fn begin_stroke(&self) -> EngineResult<()> {
        // Initialize dirty rect tracking
        *self.stroke_dirty_rect.write() = None;

        // Save BEFORE state - copy current layer pixels for undo
        let layer_manager = self.layer_manager.read();
        if let Some(active_layer) = layer_manager.active_layer() {
            let layer = active_layer.read();
            // Store the original pixels before any modification
            *self.stroke_before_pixels.write() = Some(layer.pixels.clone());
            *self.stroke_layer_id.write() = Some(layer.id);
            *self.stroke_layer_dims.write() = Some((layer.width(), layer.height()));
        }
        drop(layer_manager);

        // Start new stroke
        *self.current_stroke.write() = Some(Stroke::new());
        Ok(())
    }

    /// Add a point to the current stroke and render incrementally
    pub fn add_stroke_point(&self, point: StrokePoint) -> EngineResult<()> {
        let mut stroke_lock = self.current_stroke.write();
        if let Some(ref mut stroke) = *stroke_lock {
            stroke.add_point(point.clone());

            // Update dirty rect with brush coverage
            let brush_engine = self.brush_engine.read();
            let brush_radius = (brush_engine.current_brush().settings.size / 2.0).ceil() as u32 + 2;
            drop(brush_engine);

            let px = point.position.x as u32;
            let py = point.position.y as u32;

            // Expand dirty rect to include this point with brush radius
            let mut dirty_lock = self.stroke_dirty_rect.write();
            if let Some(ref mut dirty) = *dirty_lock {
                // Expand existing rect
                dirty.expand_to(px.saturating_sub(brush_radius), py.saturating_sub(brush_radius));
                dirty.expand_to(px + brush_radius, py + brush_radius);
            } else {
                // Initialize dirty rect centered on this point
                *dirty_lock = Some(DirtyRect::new(
                    px.saturating_sub(brush_radius),
                    py.saturating_sub(brush_radius),
                    brush_radius * 2 + 1,
                    brush_radius * 2 + 1,
                ));
            }
            drop(dirty_lock);

            // 只有有足够的点时才渲染
            if stroke.points.len() >= 2 {
                drop(stroke_lock);

                // 创建只包含最后几个点的临时笔触进行增量渲染
                let stroke_read = self.current_stroke.read();
                if let Some(ref full_stroke) = *stroke_read {
                    let points = &full_stroke.points;
                    let start_idx = if points.len() > 3 { points.len() - 3 } else { 0 };
                    let mut partial_stroke = Stroke::new();
                    for i in start_idx..points.len() {
                        partial_stroke.add_point(points[i].clone());
                    }
                    drop(stroke_read);

                    // 渲染部分笔触
                    let mut brush = self.brush_engine.write();
                    let layer_manager = self.layer_manager.read();
                    if let Some(active_layer) = layer_manager.active_layer() {
                        let mut layer = active_layer.write();
                        brush.render_stroke_to_layer(&partial_stroke, &mut layer)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// End the current stroke and commit to history
    pub fn end_stroke(&self) -> EngineResult<()> {
        // Get dirty rect and stored BEFORE state
        let dirty_rect = self.stroke_dirty_rect.write().take();
        let before_pixels = self.stroke_before_pixels.write().take();
        let layer_id = self.stroke_layer_id.write().take();
        let layer_dims = self.stroke_layer_dims.write().take();

        // Create incremental snapshot from BEFORE pixels if we have everything
        if let (Some(mut dirty), Some(before), Some(id), Some((width, height))) =
            (dirty_rect, before_pixels, layer_id, layer_dims)
        {
            // Add padding for brush softness and clamp to layer bounds
            dirty.pad(4, width, height);
            dirty.clamp(width, height);

            if !dirty.is_empty() {
                // Create incremental snapshot from BEFORE pixels (for undo)
                let snapshot = LayerSnapshot::incremental_compressed(
                    id,
                    &before,  // Use BEFORE state, not current pixels
                    width,
                    height,
                    dirty,
                );

                let mut state = HistoryState::new("Stroke");
                state.add_snapshot(snapshot);
                self.history_manager.write().push_state(state);
            }
        }

        // Clear current stroke
        *self.current_stroke.write() = None;
        Ok(())
    }

    /// Process a stroke on the current layer (with undo support)
    pub fn process_stroke(&self, stroke: &Stroke) -> EngineResult<()> {
        let mut brush = self.brush_engine.write();
        let layer_manager = self.layer_manager.read();

        if let Some(active_layer) = layer_manager.active_layer() {
            let mut layer = active_layer.write();

            // Save current state for undo before modifying
            let snapshot = LayerSnapshot::new(
                layer.id,
                layer.pixels.clone(),
                layer.width(),
                layer.height(),
            );
            let mut state = HistoryState::new("Stroke");
            state.add_snapshot(snapshot);
            self.history_manager.write().push_state(state);

            // Apply the stroke
            brush.render_stroke_to_layer(stroke, &mut layer)?;
        }

        Ok(())
    }

    /// Undo the last action
    pub fn undo(&self) -> EngineResult<bool> {
        let layer_manager = self.layer_manager.read();

        // Create current state to save for redo
        let mut current_state = HistoryState::new("Current");
        for layer_arc in layer_manager.layers() {
            let layer = layer_arc.read();
            let snapshot = LayerSnapshot::new(
                layer.id,
                layer.pixels.clone(),
                layer.width(),
                layer.height(),
            );
            current_state.add_snapshot(snapshot);
        }
        drop(layer_manager);

        // Get state to restore
        let mut history = self.history_manager.write();
        if let Some(state_to_restore) = history.undo(current_state) {
            drop(history);

            // Restore layer states using restore_to (handles both full and incremental)
            let layer_manager = self.layer_manager.read();
            for snapshot in state_to_restore.layer_snapshots {
                if let Some(layer_arc) = layer_manager.get_layer(snapshot.layer_id) {
                    let mut layer = layer_arc.write();
                    // Get width before mutable borrow of pixels
                    let width = layer.width();
                    // Use restore_to which handles incremental snapshots properly
                    snapshot.restore_to(&mut layer.pixels, width);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Redo the last undone action
    pub fn redo(&self) -> EngineResult<bool> {
        let layer_manager = self.layer_manager.read();

        // Create current state to save for undo
        let mut current_state = HistoryState::new("Current");
        for layer_arc in layer_manager.layers() {
            let layer = layer_arc.read();
            let snapshot = LayerSnapshot::new(
                layer.id,
                layer.pixels.clone(),
                layer.width(),
                layer.height(),
            );
            current_state.add_snapshot(snapshot);
        }
        drop(layer_manager);

        // Get state to restore
        let mut history = self.history_manager.write();
        if let Some(state_to_restore) = history.redo(current_state) {
            drop(history);

            // Restore layer states using restore_to (handles both full and incremental)
            let layer_manager = self.layer_manager.read();
            for snapshot in state_to_restore.layer_snapshots {
                if let Some(layer_arc) = layer_manager.get_layer(snapshot.layer_id) {
                    let mut layer = layer_arc.write();
                    // Get width before mutable borrow of pixels
                    let width = layer.width();
                    // Use restore_to which handles incremental snapshots properly
                    snapshot.restore_to(&mut layer.pixels, width);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history_manager.read().can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history_manager.read().can_redo()
    }

    /// Pick color at position from the rendered canvas
    pub fn pick_color(&self, x: u32, y: u32) -> EngineResult<Color> {
        let layer_manager = self.layer_manager.read();

        // Get the composite color from all visible layers
        let mut result_color = Color::transparent();

        for layer_arc in layer_manager.layers() {
            let layer = layer_arc.read();
            if !layer.visible {
                continue;
            }

            if let Some(color) = layer.get_pixel(x, y) {
                // Blend with result
                result_color = result_color.blend_over(color);
            }
        }

        Ok(result_color)
    }

    /// Flood fill at position with color
    pub fn flood_fill(&self, x: u32, y: u32, fill_color: Color, tolerance: f32) -> EngineResult<()> {
        let layer_manager = self.layer_manager.read();

        if let Some(active_layer) = layer_manager.active_layer() {
            let mut layer = active_layer.write();
            let (_, _, width, height) = layer.bounds;

            // Get the target color at the click position
            let target_color = match layer.get_pixel(x, y) {
                Some(c) => c,
                None => return Err(EngineError::InvalidOperation("Position out of bounds".into())),
            };

            // Save current state for undo before modifying
            let snapshot = LayerSnapshot::new(
                layer.id,
                layer.pixels.clone(),
                layer.width(),
                layer.height(),
            );
            let mut state = HistoryState::new("Fill");
            state.add_snapshot(snapshot);
            self.history_manager.write().push_state(state);

            // Use a simple flood fill algorithm (scanline fill would be more efficient for large areas)
            let mut visited = vec![false; (width * height) as usize];
            let mut stack = vec![(x, y)];

            while let Some((cx, cy)) = stack.pop() {
                if cx >= width || cy >= height {
                    continue;
                }

                let idx = (cy * width + cx) as usize;
                if visited[idx] {
                    continue;
                }

                if let Some(current_color) = layer.get_pixel(cx, cy) {
                    // Check if colors are similar within tolerance
                    if colors_similar(&current_color, &target_color, tolerance) {
                        visited[idx] = true;
                        layer.set_pixel(cx, cy, fill_color);

                        // Add neighbors
                        if cx > 0 {
                            stack.push((cx - 1, cy));
                        }
                        if cx + 1 < width {
                            stack.push((cx + 1, cy));
                        }
                        if cy > 0 {
                            stack.push((cx, cy - 1));
                        }
                        if cy + 1 < height {
                            stack.push((cx, cy + 1));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Render the current canvas state
    pub fn render(&self) -> EngineResult<Vec<u8>> {
        let canvas = self.canvas.read();
        let layer_manager = self.layer_manager.read();
        let render_pipeline = self.render_pipeline.read();

        render_pipeline.render(&canvas, &layer_manager)
    }
}

impl Default for DrawEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default DrawEngine")
    }
}

/// Helper function to check if two colors are similar within tolerance
fn colors_similar(a: &Color, b: &Color, tolerance: f32) -> bool {
    let dr = (a.r - b.r).abs();
    let dg = (a.g - b.g).abs();
    let db = (a.b - b.b).abs();
    let da = (a.a - b.a).abs();

    // Use Euclidean distance in RGBA space
    let distance = (dr * dr + dg * dg + db * db + da * da).sqrt();
    distance <= tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = DrawEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_engine_config() {
        let config = EngineConfig::default();
        assert_eq!(config.max_width, MAX_CANVAS_SIZE);
        assert_eq!(config.tile_size, DEFAULT_TILE_SIZE);
    }
}
