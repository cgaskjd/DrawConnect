//! Canvas Core Module
//!
//! Provides tile-based canvas management for handling large canvases
//! efficiently with features like:
//! - 16K resolution support
//! - Tile-based rendering
//! - Memory-efficient sparse storage
//! - Undo/Redo support

mod tile;

pub use tile::{Tile, TileManager};

use crate::color::Color;
use crate::error::{EngineError, EngineResult};
use crate::MAX_CANVAS_SIZE;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Canvas settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSettings {
    /// Canvas width in pixels
    pub width: u32,
    /// Canvas height in pixels
    pub height: u32,
    /// Resolution (DPI)
    pub dpi: u32,
    /// Background color (None = transparent)
    pub background: Option<Color>,
    /// Color profile name
    pub color_profile: String,
}

impl Default for CanvasSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            dpi: 300,
            background: Some(Color::white()),
            color_profile: "sRGB".into(),
        }
    }
}

/// Main canvas structure
pub struct Canvas {
    /// Unique canvas ID
    id: Uuid,
    /// Canvas settings
    settings: CanvasSettings,
    /// Tile manager for efficient storage
    tile_manager: TileManager,
    /// Undo history
    undo_history: Vec<CanvasSnapshot>,
    /// Redo history
    redo_history: Vec<CanvasSnapshot>,
    /// Maximum undo steps
    max_undo: usize,
    /// Canvas is modified
    modified: bool,
}

/// Canvas snapshot for undo/redo
#[derive(Clone)]
struct CanvasSnapshot {
    /// Modified tiles
    tiles: HashMap<(u32, u32), Vec<u8>>,
    /// Description of the change
    description: String,
}

impl Canvas {
    /// Create a new canvas
    pub fn new(tile_size: u32, max_width: u32, max_height: u32) -> EngineResult<Self> {
        if max_width > MAX_CANVAS_SIZE || max_height > MAX_CANVAS_SIZE {
            return Err(EngineError::CanvasTooLarge(
                max_width,
                max_height,
                MAX_CANVAS_SIZE,
            ));
        }

        let settings = CanvasSettings {
            width: max_width.min(1920),
            height: max_height.min(1080),
            ..Default::default()
        };

        let tile_manager = TileManager::new(tile_size, settings.width, settings.height);

        Ok(Self {
            id: Uuid::new_v4(),
            settings,
            tile_manager,
            undo_history: Vec::new(),
            redo_history: Vec::new(),
            max_undo: 100,
            modified: false,
        })
    }

    /// Create canvas with specific dimensions
    pub fn with_size(width: u32, height: u32) -> EngineResult<Self> {
        let mut canvas = Self::new(256, width, height)?;
        canvas.settings.width = width;
        canvas.settings.height = height;
        canvas.tile_manager = TileManager::new(256, width, height);
        Ok(canvas)
    }

    /// Get canvas ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get canvas width
    pub fn width(&self) -> u32 {
        self.settings.width
    }

    /// Get canvas height
    pub fn height(&self) -> u32 {
        self.settings.height
    }

    /// Get canvas settings
    pub fn settings(&self) -> &CanvasSettings {
        &self.settings
    }

    /// Get mutable settings
    pub fn settings_mut(&mut self) -> &mut CanvasSettings {
        &mut self.settings
    }

    /// Check if canvas has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Clear modified flag
    pub fn clear_modified(&mut self) {
        self.modified = false;
    }

    /// Get pixel at position
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.settings.width || y >= self.settings.height {
            return None;
        }

        self.tile_manager.get_pixel(x, y)
    }

    /// Set pixel at position
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> EngineResult<()> {
        if x >= self.settings.width || y >= self.settings.height {
            return Err(EngineError::InvalidOperation(format!(
                "Pixel position ({}, {}) out of bounds",
                x, y
            )));
        }

        self.tile_manager.set_pixel(x, y, color);
        self.modified = true;
        Ok(())
    }

    /// Blend pixel at position
    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color) -> EngineResult<()> {
        if x >= self.settings.width || y >= self.settings.height {
            return Ok(()); // Silently ignore out-of-bounds
        }

        if let Some(existing) = self.get_pixel(x, y) {
            let blended = Self::alpha_blend(existing, color);
            self.tile_manager.set_pixel(x, y, blended);
            self.modified = true;
        }

        Ok(())
    }

    /// Alpha blend two colors
    fn alpha_blend(dst: Color, src: Color) -> Color {
        if src.a == 0.0 {
            return dst;
        }
        if src.a == 1.0 {
            return src;
        }

        let out_a = src.a + dst.a * (1.0 - src.a);
        if out_a == 0.0 {
            return Color::transparent();
        }

        Color::from_rgba(
            (src.r * src.a + dst.r * dst.a * (1.0 - src.a)) / out_a,
            (src.g * src.a + dst.g * dst.a * (1.0 - src.a)) / out_a,
            (src.b * src.a + dst.b * dst.a * (1.0 - src.a)) / out_a,
            out_a,
        )
    }

    /// Fill canvas with color
    pub fn fill(&mut self, color: Color) {
        self.tile_manager.fill(color);
        self.modified = true;
    }

    /// Clear canvas (transparent)
    pub fn clear(&mut self) {
        self.tile_manager.clear();
        self.modified = true;
    }

    /// Resize canvas
    pub fn resize(&mut self, new_width: u32, new_height: u32) -> EngineResult<()> {
        if new_width > MAX_CANVAS_SIZE || new_height > MAX_CANVAS_SIZE {
            return Err(EngineError::CanvasTooLarge(
                new_width,
                new_height,
                MAX_CANVAS_SIZE,
            ));
        }

        if new_width == 0 || new_height == 0 {
            return Err(EngineError::InvalidCanvasSize(new_width, new_height));
        }

        self.settings.width = new_width;
        self.settings.height = new_height;
        self.tile_manager.resize(new_width, new_height);
        self.modified = true;
        Ok(())
    }

    /// Save current state for undo
    pub fn save_undo(&mut self, description: impl Into<String>) {
        let snapshot = CanvasSnapshot {
            tiles: self.tile_manager.snapshot(),
            description: description.into(),
        };

        self.undo_history.push(snapshot);
        self.redo_history.clear();

        // Limit undo history
        while self.undo_history.len() > self.max_undo {
            self.undo_history.remove(0);
        }
    }

    /// Undo last change
    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.undo_history.pop() {
            // Save current state for redo
            let current = CanvasSnapshot {
                tiles: self.tile_manager.snapshot(),
                description: snapshot.description.clone(),
            };
            self.redo_history.push(current);

            // Restore snapshot
            self.tile_manager.restore(&snapshot.tiles);
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Redo last undone change
    pub fn redo(&mut self) -> bool {
        if let Some(snapshot) = self.redo_history.pop() {
            // Save current state for undo
            let current = CanvasSnapshot {
                tiles: self.tile_manager.snapshot(),
                description: snapshot.description.clone(),
            };
            self.undo_history.push(current);

            // Restore snapshot
            self.tile_manager.restore(&snapshot.tiles);
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_history.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_history.is_empty()
    }

    /// Get number of undo steps available
    pub fn undo_count(&self) -> usize {
        self.undo_history.len()
    }

    /// Get number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.redo_history.len()
    }

    /// Get raw pixel data (RGBA)
    pub fn get_pixels(&self) -> Vec<u8> {
        let width = self.settings.width;
        let height = self.settings.height;
        let mut data = vec![0u8; (width * height * 4) as usize];

        for y in 0..height {
            for x in 0..width {
                if let Some(color) = self.get_pixel(x, y) {
                    let idx = ((y * width + x) * 4) as usize;
                    let (r, g, b, a) = color.to_rgba8();
                    data[idx] = r;
                    data[idx + 1] = g;
                    data[idx + 2] = b;
                    data[idx + 3] = a;
                }
            }
        }

        data
    }

    /// Get tile manager reference
    pub fn tile_manager(&self) -> &TileManager {
        &self.tile_manager
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::with_size(1920, 1080).expect("Failed to create default canvas")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::with_size(100, 100);
        assert!(canvas.is_ok());

        let canvas = canvas.unwrap();
        assert_eq!(canvas.width(), 100);
        assert_eq!(canvas.height(), 100);
    }

    #[test]
    fn test_canvas_too_large() {
        let canvas = Canvas::with_size(MAX_CANVAS_SIZE + 1, 100);
        assert!(canvas.is_err());
    }

    #[test]
    fn test_pixel_operations() {
        let mut canvas = Canvas::with_size(100, 100).unwrap();
        let color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);

        canvas.set_pixel(50, 50, color).unwrap();
        let retrieved = canvas.get_pixel(50, 50).unwrap();

        assert!((retrieved.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_undo_redo() {
        let mut canvas = Canvas::with_size(100, 100).unwrap();

        canvas.set_pixel(0, 0, Color::red()).unwrap();
        canvas.save_undo("Set red pixel");

        canvas.set_pixel(0, 0, Color::blue()).unwrap();

        assert!(canvas.can_undo());
        canvas.undo();

        let color = canvas.get_pixel(0, 0).unwrap();
        assert!((color.r - 1.0).abs() < 0.01);
    }
}
