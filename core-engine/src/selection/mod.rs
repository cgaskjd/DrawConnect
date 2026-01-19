//! Selection system for DrawConnect
//!
//! Provides selection tools including rectangle, lasso, and magic wand selection.

use serde::{Deserialize, Serialize};
use crate::error::{EngineError, EngineResult};

/// Selection mode for combining selections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    /// Replace existing selection
    Replace,
    /// Add to existing selection
    Add,
    /// Subtract from existing selection
    Subtract,
    /// Intersect with existing selection
    Intersect,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::Replace
    }
}

/// Selection shape type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionShape {
    /// No selection
    None,
    /// Rectangle selection with bounds (x, y, width, height)
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Lasso selection with polygon points
    Lasso {
        points: Vec<(f32, f32)>,
    },
    /// Bitmap mask selection (for magic wand results)
    Mask {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}

/// Selection data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    /// The shape of the selection
    pub shape: SelectionShape,
    /// Feather radius for soft edges
    pub feather: f32,
    /// Whether the selection is active
    pub is_active: bool,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            shape: SelectionShape::None,
            feather: 0.0,
            is_active: false,
        }
    }
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a rectangle selection
    pub fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        // Normalize negative dimensions
        let (x, width) = if width < 0.0 {
            (x + width, -width)
        } else {
            (x, width)
        };
        let (y, height) = if height < 0.0 {
            (y + height, -height)
        } else {
            (y, height)
        };

        Self {
            shape: SelectionShape::Rectangle { x, y, width, height },
            feather: 0.0,
            is_active: true,
        }
    }

    /// Create a lasso selection from points
    pub fn lasso(points: Vec<(f32, f32)>) -> Self {
        if points.len() < 3 {
            return Self::default();
        }
        Self {
            shape: SelectionShape::Lasso { points },
            feather: 0.0,
            is_active: true,
        }
    }

    /// Clear the selection
    pub fn clear(&mut self) {
        self.shape = SelectionShape::None;
        self.is_active = false;
    }

    /// Check if a point is inside the selection
    pub fn contains(&self, px: f32, py: f32) -> bool {
        match &self.shape {
            SelectionShape::None => false,
            SelectionShape::Rectangle { x, y, width, height } => {
                px >= *x && px < x + width && py >= *y && py < y + height
            }
            SelectionShape::Lasso { points } => {
                Self::point_in_polygon(px, py, points)
            }
            SelectionShape::Mask { x, y, width, height, data } => {
                let ix = px as i32 - *x as i32;
                let iy = py as i32 - *y as i32;
                if ix < 0 || iy < 0 || ix >= *width as i32 || iy >= *height as i32 {
                    return false;
                }
                let idx = (iy as usize * *width as usize + ix as usize);
                idx < data.len() && data[idx] > 127
            }
        }
    }

    /// Get the bounding box of the selection (x, y, width, height)
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        match &self.shape {
            SelectionShape::None => None,
            SelectionShape::Rectangle { x, y, width, height } => {
                Some((*x, *y, *width, *height))
            }
            SelectionShape::Lasso { points } => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;
                for (px, py) in points {
                    min_x = min_x.min(*px);
                    min_y = min_y.min(*py);
                    max_x = max_x.max(*px);
                    max_y = max_y.max(*py);
                }
                Some((min_x, min_y, max_x - min_x, max_y - min_y))
            }
            SelectionShape::Mask { x, y, width, height, data } => {
                // Calculate actual bounding box of selected pixels
                let mut min_x = *width;
                let mut min_y = *height;
                let mut max_x = 0u32;
                let mut max_y = 0u32;
                let mut has_selection = false;

                for py in 0..*height {
                    for px in 0..*width {
                        let idx = (py * *width + px) as usize;
                        if idx < data.len() && data[idx] > 127 {
                            has_selection = true;
                            min_x = min_x.min(px);
                            min_y = min_y.min(py);
                            max_x = max_x.max(px);
                            max_y = max_y.max(py);
                        }
                    }
                }

                if has_selection {
                    Some((
                        (*x + min_x) as f32,
                        (*y + min_y) as f32,
                        (max_x - min_x + 1) as f32,
                        (max_y - min_y + 1) as f32,
                    ))
                } else {
                    None
                }
            }
        }
    }

    /// Invert the selection within canvas bounds
    pub fn invert(&mut self, canvas_width: u32, canvas_height: u32) {
        match &self.shape {
            SelectionShape::None => {
                // Select all
                self.shape = SelectionShape::Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: canvas_width as f32,
                    height: canvas_height as f32,
                };
                self.is_active = true;
            }
            SelectionShape::Rectangle { x, y, width, height } => {
                // Convert to mask and invert
                let mut mask = vec![255u8; (canvas_width * canvas_height) as usize];
                let x1 = (*x as u32).min(canvas_width);
                let y1 = (*y as u32).min(canvas_height);
                let x2 = ((x + width) as u32).min(canvas_width);
                let y2 = ((y + height) as u32).min(canvas_height);

                for py in y1..y2 {
                    for px in x1..x2 {
                        mask[(py * canvas_width + px) as usize] = 0;
                    }
                }

                self.shape = SelectionShape::Mask {
                    x: 0,
                    y: 0,
                    width: canvas_width,
                    height: canvas_height,
                    data: mask,
                };
            }
            SelectionShape::Lasso { points } => {
                // Convert to mask and invert
                let mut mask = vec![255u8; (canvas_width * canvas_height) as usize];
                for py in 0..canvas_height {
                    for px in 0..canvas_width {
                        if Self::point_in_polygon(px as f32, py as f32, points) {
                            mask[(py * canvas_width + px) as usize] = 0;
                        }
                    }
                }
                self.shape = SelectionShape::Mask {
                    x: 0,
                    y: 0,
                    width: canvas_width,
                    height: canvas_height,
                    data: mask,
                };
            }
            SelectionShape::Mask { width, height, data, .. } => {
                let inverted: Vec<u8> = data.iter().map(|v| 255 - v).collect();
                self.shape = SelectionShape::Mask {
                    x: 0,
                    y: 0,
                    width: *width,
                    height: *height,
                    data: inverted,
                };
            }
        }
    }

    /// Expand the selection by given pixels
    pub fn expand(&mut self, pixels: f32) {
        if let SelectionShape::Rectangle { x, y, width, height } = &mut self.shape {
            *x -= pixels;
            *y -= pixels;
            *width += pixels * 2.0;
            *height += pixels * 2.0;
        }
    }

    /// Contract the selection by given pixels
    pub fn contract(&mut self, pixels: f32) {
        if let SelectionShape::Rectangle { x, y, width, height } = &mut self.shape {
            *x += pixels;
            *y += pixels;
            *width = (*width - pixels * 2.0).max(0.0);
            *height = (*height - pixels * 2.0).max(0.0);
        }
    }

    /// Set feather radius
    pub fn set_feather(&mut self, radius: f32) {
        self.feather = radius.max(0.0);
    }

    /// Check if point is in polygon using ray casting algorithm
    fn point_in_polygon(px: f32, py: f32, polygon: &[(f32, f32)]) -> bool {
        let n = polygon.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;

        for i in 0..n {
            let (xi, yi) = polygon[i];
            let (xj, yj) = polygon[j];

            if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }

        inside
    }
}

/// Selection manager for handling selection operations
#[derive(Debug)]
pub struct SelectionManager {
    /// Current selection
    current: Selection,
    /// Selection mode for next operation
    mode: SelectionMode,
    /// Magic wand tolerance
    tolerance: f32,
    /// Contiguous selection for magic wand
    contiguous: bool,
    /// Canvas width for mask operations
    canvas_width: u32,
    /// Canvas height for mask operations
    canvas_height: u32,
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionManager {
    /// Create a new selection manager
    pub fn new() -> Self {
        Self {
            current: Selection::new(),
            mode: SelectionMode::Replace,
            tolerance: 32.0,
            contiguous: true,
            canvas_width: 1920,
            canvas_height: 1080,
        }
    }

    /// Set canvas dimensions for mask operations
    pub fn set_canvas_size(&mut self, width: u32, height: u32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }

    /// Get the current selection
    pub fn selection(&self) -> &Selection {
        &self.current
    }

    /// Get mutable reference to current selection
    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.current
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        self.current.is_active
    }

    /// Set selection mode
    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
    }

    /// Get current selection mode
    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Set magic wand tolerance
    pub fn set_tolerance(&mut self, tolerance: f32) {
        self.tolerance = tolerance.clamp(0.0, 255.0);
    }

    /// Get magic wand tolerance
    pub fn tolerance(&self) -> f32 {
        self.tolerance
    }

    /// Set contiguous mode for magic wand
    pub fn set_contiguous(&mut self, contiguous: bool) {
        self.contiguous = contiguous;
    }

    /// Convert a selection to a bitmap mask
    fn selection_to_mask(&self, selection: &Selection, width: u32, height: u32) -> Vec<u8> {
        let mut mask = vec![0u8; (width * height) as usize];

        match &selection.shape {
            SelectionShape::None => {},
            SelectionShape::Rectangle { x, y, width: w, height: h } => {
                let x1 = (*x as u32).min(width);
                let y1 = (*y as u32).min(height);
                let x2 = ((x + w) as u32).min(width);
                let y2 = ((y + h) as u32).min(height);

                for py in y1..y2 {
                    for px in x1..x2 {
                        mask[(py * width + px) as usize] = 255;
                    }
                }
            },
            SelectionShape::Lasso { points } => {
                for py in 0..height {
                    for px in 0..width {
                        if Selection::point_in_polygon(px as f32, py as f32, points) {
                            mask[(py * width + px) as usize] = 255;
                        }
                    }
                }
            },
            SelectionShape::Mask { x, y, width: mw, height: mh, data } => {
                for my in 0..*mh {
                    for mx in 0..*mw {
                        let tx = *x + mx;
                        let ty = *y + my;
                        if tx < width && ty < height {
                            let src_idx = (my * mw + mx) as usize;
                            let dst_idx = (ty * width + tx) as usize;
                            if src_idx < data.len() {
                                mask[dst_idx] = data[src_idx];
                            }
                        }
                    }
                }
            }
        }

        mask
    }

    /// Create a selection from a mask
    fn mask_to_selection(&self, mask: Vec<u8>, width: u32, height: u32) -> Selection {
        // Check if mask has any selection
        let has_selection = mask.iter().any(|&v| v > 127);
        if !has_selection {
            return Selection::new();
        }

        Selection {
            shape: SelectionShape::Mask {
                x: 0,
                y: 0,
                width,
                height,
                data: mask,
            },
            feather: 0.0,
            is_active: true,
        }
    }

    /// Union two masks (OR operation)
    fn union_masks(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(&va, &vb)| va.max(vb)).collect()
    }

    /// Subtract mask b from a (AND NOT operation)
    fn subtract_masks(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(&va, &vb)| {
            if vb > 127 { 0 } else { va }
        }).collect()
    }

    /// Intersect two masks (AND operation)
    fn intersect_masks(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(&va, &vb)| va.min(vb)).collect()
    }

    /// Create a rectangle selection
    pub fn select_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let new_selection = Selection::rectangle(x, y, width, height);
        self.apply_selection(new_selection);
    }

    /// Create a lasso selection
    pub fn select_lasso(&mut self, points: Vec<(f32, f32)>) {
        let new_selection = Selection::lasso(points);
        self.apply_selection(new_selection);
    }

    /// Select all
    pub fn select_all(&mut self, canvas_width: u32, canvas_height: u32) {
        self.current = Selection::rectangle(0.0, 0.0, canvas_width as f32, canvas_height as f32);
    }

    /// Clear selection
    pub fn clear(&mut self) {
        self.current.clear();
    }

    /// Invert selection
    pub fn invert(&mut self, canvas_width: u32, canvas_height: u32) {
        self.current.invert(canvas_width, canvas_height);
    }

    /// Expand selection
    pub fn expand(&mut self, pixels: f32) {
        self.current.expand(pixels);
    }

    /// Contract selection
    pub fn contract(&mut self, pixels: f32) {
        self.current.contract(pixels);
    }

    /// Set feather
    pub fn set_feather(&mut self, radius: f32) {
        self.current.set_feather(radius);
    }

    /// Magic wand selection
    pub fn select_magic_wand(
        &mut self,
        x: u32,
        y: u32,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> EngineResult<()> {
        if x >= width || y >= height {
            return Err(EngineError::InvalidOperation("Point outside canvas".to_string()));
        }

        let idx = ((y * width + x) * 4) as usize;
        if idx + 3 >= pixels.len() {
            return Err(EngineError::InvalidOperation("Invalid pixel data".to_string()));
        }

        let target_r = pixels[idx] as i32;
        let target_g = pixels[idx + 1] as i32;
        let target_b = pixels[idx + 2] as i32;
        let tolerance = self.tolerance as i32;

        let mut mask = vec![0u8; (width * height) as usize];

        if self.contiguous {
            // Flood fill algorithm for contiguous selection
            let mut stack = vec![(x, y)];
            let mut visited = vec![false; (width * height) as usize];

            while let Some((px, py)) = stack.pop() {
                let pidx = (py * width + px) as usize;
                if visited[pidx] {
                    continue;
                }
                visited[pidx] = true;

                let pixel_idx = pidx * 4;
                if pixel_idx + 3 >= pixels.len() {
                    continue;
                }

                let r = pixels[pixel_idx] as i32;
                let g = pixels[pixel_idx + 1] as i32;
                let b = pixels[pixel_idx + 2] as i32;

                let diff = (r - target_r).abs() + (g - target_g).abs() + (b - target_b).abs();
                if diff <= tolerance * 3 {
                    mask[pidx] = 255;

                    // Add neighbors
                    if px > 0 {
                        stack.push((px - 1, py));
                    }
                    if px < width - 1 {
                        stack.push((px + 1, py));
                    }
                    if py > 0 {
                        stack.push((px, py - 1));
                    }
                    if py < height - 1 {
                        stack.push((px, py + 1));
                    }
                }
            }
        } else {
            // Non-contiguous: select all matching pixels
            for py in 0..height {
                for px in 0..width {
                    let pidx = ((py * width + px) * 4) as usize;
                    if pidx + 3 >= pixels.len() {
                        continue;
                    }

                    let r = pixels[pidx] as i32;
                    let g = pixels[pidx + 1] as i32;
                    let b = pixels[pidx + 2] as i32;

                    let diff = (r - target_r).abs() + (g - target_g).abs() + (b - target_b).abs();
                    if diff <= tolerance * 3 {
                        mask[(py * width + px) as usize] = 255;
                    }
                }
            }
        }

        let new_selection = Selection {
            shape: SelectionShape::Mask {
                x: 0,
                y: 0,
                width,
                height,
                data: mask,
            },
            feather: 0.0,
            is_active: true,
        };

        self.apply_selection(new_selection);
        Ok(())
    }

    /// Apply a new selection based on current mode
    fn apply_selection(&mut self, new_selection: Selection) {
        match self.mode {
            SelectionMode::Replace => {
                self.current = new_selection;
            }
            SelectionMode::Add => {
                if !self.current.is_active {
                    self.current = new_selection;
                } else {
                    // Union: convert both to masks and combine
                    let mask_a = self.selection_to_mask(&self.current, self.canvas_width, self.canvas_height);
                    let mask_b = self.selection_to_mask(&new_selection, self.canvas_width, self.canvas_height);
                    let combined = self.union_masks(&mask_a, &mask_b);
                    self.current = self.mask_to_selection(combined, self.canvas_width, self.canvas_height);
                }
            }
            SelectionMode::Subtract => {
                if self.current.is_active {
                    // Subtract: remove new selection from current
                    let mask_a = self.selection_to_mask(&self.current, self.canvas_width, self.canvas_height);
                    let mask_b = self.selection_to_mask(&new_selection, self.canvas_width, self.canvas_height);
                    let combined = self.subtract_masks(&mask_a, &mask_b);
                    self.current = self.mask_to_selection(combined, self.canvas_width, self.canvas_height);
                }
            }
            SelectionMode::Intersect => {
                if !self.current.is_active {
                    // No current selection, nothing to intersect
                    self.current = Selection::new();
                } else {
                    // Intersect: keep only overlapping area
                    let mask_a = self.selection_to_mask(&self.current, self.canvas_width, self.canvas_height);
                    let mask_b = self.selection_to_mask(&new_selection, self.canvas_width, self.canvas_height);
                    let combined = self.intersect_masks(&mask_a, &mask_b);
                    self.current = self.mask_to_selection(combined, self.canvas_width, self.canvas_height);
                }
            }
        }
    }

    /// Get selection info for serialization
    pub fn get_info(&self) -> SelectionInfo {
        let (shape_type, points) = match &self.current.shape {
            SelectionShape::None => ("none".to_string(), None),
            SelectionShape::Rectangle { .. } => ("rectangle".to_string(), None),
            SelectionShape::Lasso { points } => ("lasso".to_string(), Some(points.clone())),
            SelectionShape::Mask { .. } => ("mask".to_string(), None),
        };

        SelectionInfo {
            is_active: self.current.is_active,
            bounds: self.current.bounds(),
            mode: self.mode,
            feather: self.current.feather,
            shape_type,
            points,
        }
    }
}

/// Selection info for frontend communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionInfo {
    /// Whether selection is active
    pub is_active: bool,
    /// Bounding box (x, y, width, height)
    pub bounds: Option<(f32, f32, f32, f32)>,
    /// Current selection mode
    pub mode: SelectionMode,
    /// Feather radius
    pub feather: f32,
    /// Shape type for visualization
    pub shape_type: String,
    /// Points for lasso visualization
    pub points: Option<Vec<(f32, f32)>>,
}
