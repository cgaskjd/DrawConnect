//! Layer System Module
//!
//! Provides comprehensive layer management including:
//! - Layer creation, deletion, reordering
//! - Blend modes (Normal, Multiply, Screen, Overlay, etc.)
//! - Layer masks and clipping masks
//! - Layer groups and folders
//! - Lock options (transparency, pixels, position)

mod blend;
mod group;
mod mask;

pub use blend::BlendMode;
pub use group::LayerGroup;
pub use mask::{LayerMask, MaskMode};

use crate::color::Color;
use crate::error::{EngineError, EngineResult};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Layer type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerType {
    /// Standard raster layer
    Raster,
    /// Vector layer
    Vector,
    /// Text layer
    Text,
    /// Adjustment layer
    Adjustment,
    /// Fill layer (solid color, gradient, pattern)
    Fill,
    /// Smart object layer
    SmartObject,
    /// Layer group/folder
    Group,
}

impl Default for LayerType {
    fn default() -> Self {
        Self::Raster
    }
}

/// Layer lock options
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct LayerLock {
    /// Lock transparent pixels
    pub transparency: bool,
    /// Lock all pixels
    pub pixels: bool,
    /// Lock position
    pub position: bool,
    /// Lock all (fully locked)
    pub all: bool,
}

impl LayerLock {
    /// Check if any lock is active
    pub fn is_locked(&self) -> bool {
        self.transparency || self.pixels || self.position || self.all
    }

    /// Check if drawing is allowed
    pub fn can_draw(&self) -> bool {
        !self.pixels && !self.all
    }

    /// Check if moving is allowed
    pub fn can_move(&self) -> bool {
        !self.position && !self.all
    }
}

/// Layer structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Unique layer identifier
    pub id: Uuid,
    /// Layer name
    pub name: String,
    /// Layer type
    pub layer_type: LayerType,
    /// Layer visibility
    pub visible: bool,
    /// Layer opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Blend mode
    pub blend_mode: BlendMode,
    /// Lock options
    pub lock: LayerLock,
    /// Layer bounds (x, y, width, height)
    pub bounds: (i32, i32, u32, u32),
    /// Parent group ID (if in a group)
    pub parent_id: Option<Uuid>,
    /// Clipping mask (clips to layer below)
    pub clipping: bool,
    /// Layer mask
    #[serde(skip)]
    pub mask: Option<LayerMask>,
    /// Layer pixel data (RGBA)
    #[serde(skip)]
    pub pixels: Vec<u8>,
    /// Thumbnail data
    #[serde(skip)]
    pub thumbnail: Option<Vec<u8>>,
}

impl Layer {
    /// Create a new raster layer
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        let pixel_count = (width * height) as usize;
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            layer_type: LayerType::Raster,
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            lock: LayerLock::default(),
            bounds: (0, 0, width, height),
            parent_id: None,
            clipping: false,
            mask: None,
            pixels: vec![0; pixel_count * 4],
            thumbnail: None,
        }
    }

    /// Create a new layer with specific type
    pub fn with_type(name: impl Into<String>, layer_type: LayerType, width: u32, height: u32) -> Self {
        let mut layer = Self::new(name, width, height);
        layer.layer_type = layer_type;
        layer
    }

    /// Get pixel at position
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        let (_, _, width, height) = self.bounds;
        if x >= width || y >= height {
            return None;
        }

        let idx = ((y * width + x) * 4) as usize;
        if idx + 3 >= self.pixels.len() {
            return None;
        }

        Some(Color::from_rgba8(
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ))
    }

    /// Set pixel at position
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        if !self.lock.can_draw() {
            return false;
        }

        let (_, _, width, height) = self.bounds;
        if x >= width || y >= height {
            return false;
        }

        let idx = ((y * width + x) * 4) as usize;
        if idx + 3 >= self.pixels.len() {
            return false;
        }

        // Check transparency lock
        if self.lock.transparency && self.pixels[idx + 3] == 0 {
            return false;
        }

        let (r, g, b, a) = color.to_rgba8();
        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;

        true
    }

    /// Blend a color at position
    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        if let Some(existing) = self.get_pixel(x, y) {
            let blended = self.blend_mode.blend(existing, color);
            self.set_pixel(x, y, blended)
        } else {
            false
        }
    }

    /// Fill the layer with a color
    pub fn fill(&mut self, color: Color) {
        if !self.lock.can_draw() {
            return;
        }

        let (r, g, b, a) = color.to_rgba8();
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
    }

    /// Clear the layer (make fully transparent)
    pub fn clear(&mut self) {
        if !self.lock.can_draw() {
            return;
        }
        self.pixels.fill(0);
    }

    /// Resize the layer
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let (_, _, old_width, old_height) = self.bounds;
        let new_pixels = vec![0u8; (new_width * new_height * 4) as usize];

        // Copy existing pixels
        let mut result_pixels = new_pixels;
        let copy_width = old_width.min(new_width);
        let copy_height = old_height.min(new_height);

        for y in 0..copy_height {
            for x in 0..copy_width {
                let old_idx = ((y * old_width + x) * 4) as usize;
                let new_idx = ((y * new_width + x) * 4) as usize;

                if old_idx + 3 < self.pixels.len() && new_idx + 3 < result_pixels.len() {
                    result_pixels[new_idx..new_idx + 4]
                        .copy_from_slice(&self.pixels[old_idx..old_idx + 4]);
                }
            }
        }

        self.pixels = result_pixels;
        self.bounds.2 = new_width;
        self.bounds.3 = new_height;
    }

    /// Add a mask to the layer
    pub fn add_mask(&mut self) {
        let (_, _, width, height) = self.bounds;
        self.mask = Some(LayerMask::new(width, height));
    }

    /// Remove the layer mask
    pub fn remove_mask(&mut self) -> Option<LayerMask> {
        self.mask.take()
    }

    /// Generate thumbnail
    pub fn generate_thumbnail(&mut self, thumb_size: u32) {
        let (_, _, width, height) = self.bounds;
        let scale = (thumb_size as f32 / width.max(height) as f32).min(1.0);
        let thumb_w = (width as f32 * scale) as u32;
        let thumb_h = (height as f32 * scale) as u32;

        let mut thumbnail = vec![0u8; (thumb_w * thumb_h * 4) as usize];

        // Simple nearest-neighbor downscaling
        for ty in 0..thumb_h {
            for tx in 0..thumb_w {
                let sx = (tx as f32 / scale) as u32;
                let sy = (ty as f32 / scale) as u32;

                let src_idx = ((sy * width + sx) * 4) as usize;
                let dst_idx = ((ty * thumb_w + tx) * 4) as usize;

                if src_idx + 3 < self.pixels.len() && dst_idx + 3 < thumbnail.len() {
                    thumbnail[dst_idx..dst_idx + 4]
                        .copy_from_slice(&self.pixels[src_idx..src_idx + 4]);
                }
            }
        }

        self.thumbnail = Some(thumbnail);
    }

    /// Get layer width
    pub fn width(&self) -> u32 {
        self.bounds.2
    }

    /// Get layer height
    pub fn height(&self) -> u32 {
        self.bounds.3
    }
}

/// Layer manager handles multiple layers
pub struct LayerManager {
    /// All layers
    layers: Vec<Arc<RwLock<Layer>>>,
    /// Layer groups
    groups: Vec<LayerGroup>,
    /// Active layer ID
    active_layer_id: Option<Uuid>,
    /// Selection (multiple selected layers)
    selection: Vec<Uuid>,
    /// Canvas width
    canvas_width: u32,
    /// Canvas height
    canvas_height: u32,
}

impl LayerManager {
    /// Create a new layer manager
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            groups: Vec::new(),
            active_layer_id: None,
            selection: Vec::new(),
            canvas_width: 1920,
            canvas_height: 1080,
        }
    }

    /// Create with canvas dimensions
    pub fn with_canvas_size(width: u32, height: u32) -> Self {
        let mut manager = Self::new();
        manager.canvas_width = width;
        manager.canvas_height = height;
        manager
    }

    /// Set canvas dimensions (for layers created after this call)
    pub fn set_canvas_size(&mut self, width: u32, height: u32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }

    /// Add a new layer
    pub fn add_layer(&mut self, name: impl Into<String>) -> Uuid {
        let layer = Layer::new(name, self.canvas_width, self.canvas_height);
        let id = layer.id;
        self.layers.push(Arc::new(RwLock::new(layer)));
        self.active_layer_id = Some(id);
        id
    }

    /// Add an existing layer
    pub fn add_existing_layer(&mut self, layer: Layer) -> Uuid {
        let id = layer.id;
        self.layers.push(Arc::new(RwLock::new(layer)));
        self.active_layer_id = Some(id);
        id
    }

    /// Remove a layer by ID
    pub fn remove_layer(&mut self, id: Uuid) -> Option<Layer> {
        if let Some(pos) = self.layers.iter().position(|l| l.read().id == id) {
            let layer_arc = self.layers.remove(pos);
            let layer = Arc::try_unwrap(layer_arc)
                .ok()
                .map(|l| l.into_inner());

            // Update active layer
            if self.active_layer_id == Some(id) {
                self.active_layer_id = self.layers.last().map(|l| l.read().id);
            }

            return layer;
        }
        None
    }

    /// Get layer by ID
    pub fn get_layer(&self, id: Uuid) -> Option<Arc<RwLock<Layer>>> {
        self.layers.iter().find(|l| l.read().id == id).cloned()
    }

    /// Get active layer
    pub fn active_layer(&self) -> Option<&Arc<RwLock<Layer>>> {
        self.active_layer_id
            .and_then(|id| self.layers.iter().find(|l| l.read().id == id))
    }

    /// Set active layer
    pub fn set_active_layer(&mut self, id: Uuid) -> EngineResult<()> {
        if self.layers.iter().any(|l| l.read().id == id) {
            self.active_layer_id = Some(id);
            Ok(())
        } else {
            Err(EngineError::LayerNotFound(id))
        }
    }

    /// Get all layers (bottom to top order)
    pub fn layers(&self) -> &[Arc<RwLock<Layer>>] {
        &self.layers
    }

    /// Get layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Move layer to new position
    pub fn move_layer(&mut self, id: Uuid, new_index: usize) -> EngineResult<()> {
        let current_pos = self
            .layers
            .iter()
            .position(|l| l.read().id == id)
            .ok_or(EngineError::LayerNotFound(id))?;

        if new_index >= self.layers.len() {
            return Err(EngineError::LayerIndexOutOfBounds(
                new_index,
                self.layers.len(),
            ));
        }

        let layer = self.layers.remove(current_pos);
        self.layers.insert(new_index, layer);
        Ok(())
    }

    /// Move layer up (towards top)
    pub fn move_layer_up(&mut self, id: Uuid) -> EngineResult<()> {
        let current_pos = self
            .layers
            .iter()
            .position(|l| l.read().id == id)
            .ok_or(EngineError::LayerNotFound(id))?;

        if current_pos < self.layers.len() - 1 {
            self.layers.swap(current_pos, current_pos + 1);
        }
        Ok(())
    }

    /// Move layer down (towards bottom)
    pub fn move_layer_down(&mut self, id: Uuid) -> EngineResult<()> {
        let current_pos = self
            .layers
            .iter()
            .position(|l| l.read().id == id)
            .ok_or(EngineError::LayerNotFound(id))?;

        if current_pos > 0 {
            self.layers.swap(current_pos, current_pos - 1);
        }
        Ok(())
    }

    /// Duplicate a layer
    pub fn duplicate_layer(&mut self, id: Uuid) -> EngineResult<Uuid> {
        let layer = self
            .get_layer(id)
            .ok_or(EngineError::LayerNotFound(id))?;

        let original = layer.read();
        let mut duplicated = original.clone();
        duplicated.id = Uuid::new_v4();
        duplicated.name = format!("{} Copy", original.name);
        drop(original);

        let new_id = duplicated.id;
        self.layers.push(Arc::new(RwLock::new(duplicated)));
        Ok(new_id)
    }

    /// Merge layer down
    pub fn merge_down(&mut self, id: Uuid) -> EngineResult<()> {
        let pos = self
            .layers
            .iter()
            .position(|l| l.read().id == id)
            .ok_or(EngineError::LayerNotFound(id))?;

        if pos == 0 {
            return Err(EngineError::InvalidOperation(
                "Cannot merge bottom layer".into(),
            ));
        }

        let upper_layer = self.layers[pos].read();
        let mut lower_layer = self.layers[pos - 1].write();

        // Merge pixels
        let width = lower_layer.width();
        let height = lower_layer.height();

        for y in 0..height {
            for x in 0..width {
                if let Some(upper_color) = upper_layer.get_pixel(x, y) {
                    if upper_color.a > 0.0 {
                        if let Some(lower_color) = lower_layer.get_pixel(x, y) {
                            let blended = upper_layer.blend_mode.blend(lower_color, upper_color);
                            let blended = blended.with_alpha(
                                blended.a * upper_layer.opacity
                            );
                            lower_layer.set_pixel(x, y, blended);
                        }
                    }
                }
            }
        }

        drop(upper_layer);
        drop(lower_layer);

        // Remove upper layer
        self.layers.remove(pos);
        Ok(())
    }

    /// Flatten all visible layers
    pub fn flatten(&mut self) -> Layer {
        let mut result = Layer::new("Flattened", self.canvas_width, self.canvas_height);

        // Fill with white background
        result.fill(Color::white());

        // Composite all visible layers
        for layer_arc in &self.layers {
            let layer = layer_arc.read();
            if !layer.visible {
                continue;
            }

            let width = layer.width().min(result.width());
            let height = layer.height().min(result.height());

            for y in 0..height {
                for x in 0..width {
                    if let Some(src_color) = layer.get_pixel(x, y) {
                        if src_color.a > 0.0 {
                            if let Some(dst_color) = result.get_pixel(x, y) {
                                let blended = layer.blend_mode.blend(dst_color, src_color);
                                let blended = blended.with_alpha(
                                    dst_color.a + src_color.a * layer.opacity * (1.0 - dst_color.a)
                                );
                                result.set_pixel(x, y, blended);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Create a new group
    pub fn create_group(&mut self, name: impl Into<String>) -> Uuid {
        let group = LayerGroup::new(name);
        let id = group.id;
        self.groups.push(group);
        id
    }

    /// Add layer to group
    pub fn add_to_group(&mut self, layer_id: Uuid, group_id: Uuid) -> EngineResult<()> {
        // Verify layer exists
        let layer = self
            .get_layer(layer_id)
            .ok_or(EngineError::LayerNotFound(layer_id))?;

        // Find group and add layer
        let group = self
            .groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or(EngineError::LayerNotFound(group_id))?;

        group.add_layer(layer_id);
        layer.write().parent_id = Some(group_id);

        Ok(())
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new("Test Layer", 100, 100);
        assert_eq!(layer.name, "Test Layer");
        assert_eq!(layer.width(), 100);
        assert_eq!(layer.height(), 100);
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
    }

    #[test]
    fn test_layer_pixel_operations() {
        let mut layer = Layer::new("Test", 10, 10);
        let color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);

        assert!(layer.set_pixel(5, 5, color));

        let retrieved = layer.get_pixel(5, 5).unwrap();
        assert!((retrieved.r - 1.0).abs() < 0.01);
        assert!((retrieved.g - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_layer_manager() {
        let mut manager = LayerManager::with_canvas_size(100, 100);

        let id1 = manager.add_layer("Layer 1");
        let id2 = manager.add_layer("Layer 2");

        assert_eq!(manager.layer_count(), 2);
        assert_eq!(manager.active_layer_id, Some(id2));

        manager.set_active_layer(id1).unwrap();
        assert_eq!(manager.active_layer_id, Some(id1));
    }

    #[test]
    fn test_layer_reordering() {
        let mut manager = LayerManager::with_canvas_size(100, 100);

        let id1 = manager.add_layer("Layer 1");
        let id2 = manager.add_layer("Layer 2");
        let _id3 = manager.add_layer("Layer 3");

        // Initial: [Layer1, Layer2, Layer3] at [0, 1, 2]
        // After move_layer(id1, 2): [Layer2, Layer3, Layer1]
        manager.move_layer(id1, 2).unwrap();

        let layers = manager.layers();
        assert_eq!(layers[2].read().id, id1);
        assert_eq!(layers[0].read().id, id2);
    }
}
