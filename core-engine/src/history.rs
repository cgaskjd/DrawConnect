//! History Management Module
//!
//! Provides undo/redo functionality through incremental layer snapshots.
//!
//! ## Memory Optimization
//!
//! Instead of storing complete pixel buffers for each undo step, we use
//! incremental snapshots that only store the modified region (dirty rect).
//! This can reduce memory usage by 80-95% for typical drawing operations.

use std::collections::VecDeque;
use uuid::Uuid;

/// Maximum number of undo steps to keep
const DEFAULT_MAX_UNDO_STEPS: usize = 50;

/// A dirty rectangle representing a modified region
#[derive(Debug, Clone, Copy)]
pub struct DirtyRect {
    /// X coordinate of top-left corner
    pub x: u32,
    /// Y coordinate of top-left corner
    pub y: u32,
    /// Width of the dirty region
    pub width: u32,
    /// Height of the dirty region
    pub height: u32,
}

impl DirtyRect {
    /// Create a new dirty rect
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Create a dirty rect from bounds (x1, y1, x2, y2)
    pub fn from_bounds(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Create a full-layer dirty rect
    pub fn full(width: u32, height: u32) -> Self {
        Self { x: 0, y: 0, width, height }
    }

    /// Expand the dirty rect to include a point
    pub fn expand_to(&mut self, px: u32, py: u32) {
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;

        if px < self.x {
            self.width += self.x - px;
            self.x = px;
        } else if px >= x2 {
            self.width = px - self.x + 1;
        }

        if py < self.y {
            self.height += self.y - py;
            self.y = py;
        } else if py >= y2 {
            self.height = py - self.y + 1;
        }
    }

    /// Expand the dirty rect to include another rect
    pub fn union(&mut self, other: &DirtyRect) {
        if other.width == 0 || other.height == 0 {
            return;
        }

        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);

        self.x = x1;
        self.y = y1;
        self.width = x2 - x1;
        self.height = y2 - y1;
    }

    /// Check if the dirty rect is empty
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Get the area of the dirty rect
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Add padding around the dirty rect (for brush radius)
    pub fn pad(&mut self, padding: u32, max_width: u32, max_height: u32) {
        let new_x = self.x.saturating_sub(padding);
        let new_y = self.y.saturating_sub(padding);
        let new_x2 = (self.x + self.width + padding).min(max_width);
        let new_y2 = (self.y + self.height + padding).min(max_height);

        self.x = new_x;
        self.y = new_y;
        self.width = new_x2 - new_x;
        self.height = new_y2 - new_y;
    }

    /// Clamp the dirty rect to layer bounds
    pub fn clamp(&mut self, layer_width: u32, layer_height: u32) {
        if self.x >= layer_width || self.y >= layer_height {
            self.width = 0;
            self.height = 0;
            return;
        }

        if self.x + self.width > layer_width {
            self.width = layer_width - self.x;
        }
        if self.y + self.height > layer_height {
            self.height = layer_height - self.y;
        }
    }
}

/// Snapshot type - full or incremental
#[derive(Debug, Clone)]
pub enum SnapshotData {
    /// Full layer snapshot (used for large changes like fills)
    Full {
        /// Complete pixel data (optionally compressed)
        pixels: Vec<u8>,
        /// Whether the data is compressed
        compressed: bool,
    },
    /// Incremental snapshot (only dirty region)
    Incremental {
        /// The dirty region bounds
        dirty_rect: DirtyRect,
        /// Pixel data for the dirty region only
        pixels: Vec<u8>,
        /// Whether the data is compressed
        compressed: bool,
    },
}

impl SnapshotData {
    /// Get the memory size of this snapshot data
    pub fn memory_size(&self) -> usize {
        match self {
            SnapshotData::Full { pixels, .. } => pixels.len(),
            SnapshotData::Incremental { pixels, .. } => pixels.len(),
        }
    }
}

/// A snapshot of a layer's pixel data at a point in time
#[derive(Debug, Clone)]
pub struct LayerSnapshot {
    /// The layer ID this snapshot belongs to
    pub layer_id: Uuid,
    /// The snapshot data
    pub data: SnapshotData,
    /// Layer dimensions (width, height)
    pub dimensions: (u32, u32),
}

impl LayerSnapshot {
    /// Create a new full layer snapshot (legacy compatibility)
    pub fn new(layer_id: Uuid, pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            layer_id,
            data: SnapshotData::Full {
                pixels,
                compressed: false,
            },
            dimensions: (width, height),
        }
    }

    /// Create an incremental snapshot from dirty region
    pub fn incremental(
        layer_id: Uuid,
        layer_pixels: &[u8],
        layer_width: u32,
        layer_height: u32,
        dirty_rect: DirtyRect,
    ) -> Self {
        let mut rect = dirty_rect;
        rect.clamp(layer_width, layer_height);

        if rect.is_empty() {
            // Return empty incremental snapshot
            return Self {
                layer_id,
                data: SnapshotData::Incremental {
                    dirty_rect: rect,
                    pixels: Vec::new(),
                    compressed: false,
                },
                dimensions: (layer_width, layer_height),
            };
        }

        // Extract only the dirty region pixels
        let dirty_pixels = extract_region(
            layer_pixels,
            layer_width,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        );

        Self {
            layer_id,
            data: SnapshotData::Incremental {
                dirty_rect: rect,
                pixels: dirty_pixels,
                compressed: false,
            },
            dimensions: (layer_width, layer_height),
        }
    }

    /// Create a full snapshot with optional compression
    pub fn full_compressed(
        layer_id: Uuid,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
        compress: bool,
    ) -> Self {
        let (data, compressed) = if compress {
            match lz4_flex::compress_prepend_size(&pixels) {
                compressed_data => (compressed_data, true),
            }
        } else {
            (pixels, false)
        };

        Self {
            layer_id,
            data: SnapshotData::Full {
                pixels: data,
                compressed,
            },
            dimensions: (width, height),
        }
    }

    /// Create an incremental snapshot with compression
    pub fn incremental_compressed(
        layer_id: Uuid,
        layer_pixels: &[u8],
        layer_width: u32,
        layer_height: u32,
        dirty_rect: DirtyRect,
    ) -> Self {
        let mut rect = dirty_rect;
        rect.clamp(layer_width, layer_height);

        if rect.is_empty() {
            return Self {
                layer_id,
                data: SnapshotData::Incremental {
                    dirty_rect: rect,
                    pixels: Vec::new(),
                    compressed: false,
                },
                dimensions: (layer_width, layer_height),
            };
        }

        // Extract only the dirty region pixels
        let dirty_pixels = extract_region(
            layer_pixels,
            layer_width,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        );

        // Compress if worthwhile (region > 1KB)
        let (pixels, compressed) = if dirty_pixels.len() > 1024 {
            (lz4_flex::compress_prepend_size(&dirty_pixels), true)
        } else {
            (dirty_pixels, false)
        };

        Self {
            layer_id,
            data: SnapshotData::Incremental {
                dirty_rect: rect,
                pixels,
                compressed,
            },
            dimensions: (layer_width, layer_height),
        }
    }

    /// Restore pixels from snapshot to target buffer
    pub fn restore_to(&self, target: &mut [u8], target_width: u32) {
        match &self.data {
            SnapshotData::Full { pixels, compressed } => {
                let decompressed = if *compressed {
                    lz4_flex::decompress_size_prepended(pixels)
                        .unwrap_or_default()
                } else {
                    pixels.clone()
                };

                // Direct copy for full snapshot
                if target.len() == decompressed.len() {
                    target.copy_from_slice(&decompressed);
                }
            }
            SnapshotData::Incremental { dirty_rect, pixels, compressed } => {
                if dirty_rect.is_empty() || pixels.is_empty() {
                    return;
                }

                let decompressed = if *compressed {
                    lz4_flex::decompress_size_prepended(pixels)
                        .unwrap_or_default()
                } else {
                    pixels.clone()
                };

                // Copy dirty region back
                restore_region(
                    &decompressed,
                    target,
                    target_width,
                    dirty_rect.x,
                    dirty_rect.y,
                    dirty_rect.width,
                    dirty_rect.height,
                );
            }
        }
    }

    /// Get memory size of this snapshot
    pub fn memory_size(&self) -> usize {
        std::mem::size_of::<Self>() + self.data.memory_size()
    }

    /// Get the dirty rect if this is an incremental snapshot
    pub fn dirty_rect(&self) -> Option<DirtyRect> {
        match &self.data {
            SnapshotData::Full { .. } => None,
            SnapshotData::Incremental { dirty_rect, .. } => Some(*dirty_rect),
        }
    }

    /// Legacy accessor for pixels (decompresses if needed)
    pub fn get_pixels(&self) -> Vec<u8> {
        match &self.data {
            SnapshotData::Full { pixels, compressed } => {
                if *compressed {
                    lz4_flex::decompress_size_prepended(pixels)
                        .unwrap_or_default()
                } else {
                    pixels.clone()
                }
            }
            SnapshotData::Incremental { .. } => {
                // For incremental snapshots, we can't return full pixels
                // This is a fallback that returns empty - caller should use restore_to
                Vec::new()
            }
        }
    }

    /// Check if this is a full snapshot
    pub fn is_full(&self) -> bool {
        matches!(self.data, SnapshotData::Full { .. })
    }
}

/// Extract a region from a pixel buffer
fn extract_region(
    source: &[u8],
    source_width: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let mut result = vec![0u8; (width * height * 4) as usize];
    let src_stride = source_width as usize * 4;
    let dst_stride = width as usize * 4;

    for row in 0..height as usize {
        let src_offset = ((y as usize + row) * source_width as usize + x as usize) * 4;
        let dst_offset = row * dst_stride;

        if src_offset + dst_stride <= source.len() && dst_offset + dst_stride <= result.len() {
            result[dst_offset..dst_offset + dst_stride]
                .copy_from_slice(&source[src_offset..src_offset + dst_stride]);
        }
    }

    result
}

/// Restore a region to a pixel buffer
fn restore_region(
    source: &[u8],
    target: &mut [u8],
    target_width: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let src_stride = width as usize * 4;
    let dst_stride = target_width as usize * 4;

    for row in 0..height as usize {
        let src_offset = row * src_stride;
        let dst_offset = ((y as usize + row) * target_width as usize + x as usize) * 4;

        if src_offset + src_stride <= source.len() && dst_offset + src_stride <= target.len() {
            target[dst_offset..dst_offset + src_stride]
                .copy_from_slice(&source[src_offset..src_offset + src_stride]);
        }
    }
}

/// A history state containing snapshots of all affected layers
#[derive(Debug, Clone)]
pub struct HistoryState {
    /// Description of the action
    pub description: String,
    /// Snapshots of layers before the action
    pub layer_snapshots: Vec<LayerSnapshot>,
}

impl HistoryState {
    /// Create a new history state
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            layer_snapshots: Vec::new(),
        }
    }

    /// Add a layer snapshot
    pub fn add_snapshot(&mut self, snapshot: LayerSnapshot) {
        self.layer_snapshots.push(snapshot);
    }

    /// Get total memory size of all snapshots
    pub fn memory_size(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.description.len()
            + self.layer_snapshots.iter().map(|s| s.memory_size()).sum::<usize>()
    }
}

/// History manager for undo/redo operations
pub struct HistoryManager {
    /// Undo stack (past states)
    undo_stack: VecDeque<HistoryState>,
    /// Redo stack (undone states)
    redo_stack: VecDeque<HistoryState>,
    /// Maximum number of undo steps
    max_steps: usize,
    /// Memory limit for history (soft limit)
    memory_limit: usize,
    /// Current estimated memory usage
    current_memory: usize,
}

impl HistoryManager {
    /// Create a new history manager
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_steps: DEFAULT_MAX_UNDO_STEPS,
            memory_limit: 512 * 1024 * 1024, // 512 MB default
            current_memory: 0,
        }
    }

    /// Create with custom max steps
    pub fn with_max_steps(max_steps: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_steps,
            memory_limit: 512 * 1024 * 1024,
            current_memory: 0,
        }
    }

    /// Set memory limit for history
    pub fn set_memory_limit(&mut self, limit: usize) {
        self.memory_limit = limit;
    }

    /// Get current memory usage
    pub fn memory_usage(&self) -> usize {
        self.current_memory
    }

    /// Push a new state to the undo stack
    /// This clears the redo stack since we're branching from this point
    pub fn push_state(&mut self, state: HistoryState) {
        // Clear redo stack when new action is performed
        for s in self.redo_stack.drain(..) {
            self.current_memory = self.current_memory.saturating_sub(s.memory_size());
        }

        // Add memory usage
        self.current_memory += state.memory_size();

        // Add state to undo stack
        self.undo_stack.push_back(state);

        // Trim if exceeding max steps or memory limit
        while self.undo_stack.len() > self.max_steps ||
              (self.current_memory > self.memory_limit && self.undo_stack.len() > 1) {
            if let Some(old) = self.undo_stack.pop_front() {
                self.current_memory = self.current_memory.saturating_sub(old.memory_size());
            } else {
                break;
            }
        }
    }

    /// Pop the most recent state from undo stack
    /// Returns the state to restore and pushes current state to redo stack
    pub fn undo(&mut self, current_state: HistoryState) -> Option<HistoryState> {
        if let Some(state) = self.undo_stack.pop_back() {
            // Update memory tracking
            self.current_memory = self.current_memory.saturating_sub(state.memory_size());
            self.current_memory += current_state.memory_size();

            // Push current state to redo stack
            self.redo_stack.push_back(current_state);
            Some(state)
        } else {
            None
        }
    }

    /// Pop the most recent state from redo stack
    /// Returns the state to restore and pushes current state to undo stack
    pub fn redo(&mut self, current_state: HistoryState) -> Option<HistoryState> {
        if let Some(state) = self.redo_stack.pop_back() {
            // Update memory tracking
            self.current_memory = self.current_memory.saturating_sub(state.memory_size());
            self.current_memory += current_state.memory_size();

            // Push current state to undo stack
            self.undo_stack.push_back(current_state);
            Some(state)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undo steps available
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_memory = 0;
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_rect_expand() {
        let mut rect = DirtyRect::new(10, 10, 5, 5);
        rect.expand_to(20, 20);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 10);
        assert_eq!(rect.width, 11);
        assert_eq!(rect.height, 11);
    }

    #[test]
    fn test_dirty_rect_union() {
        let mut rect1 = DirtyRect::new(10, 10, 10, 10);
        let rect2 = DirtyRect::new(5, 15, 10, 10);
        rect1.union(&rect2);
        assert_eq!(rect1.x, 5);
        assert_eq!(rect1.y, 10);
        assert_eq!(rect1.width, 15);
        assert_eq!(rect1.height, 15);
    }

    #[test]
    fn test_incremental_snapshot() {
        // Create a 100x100 layer with some pixels
        let layer_width = 100u32;
        let layer_height = 100u32;
        let mut pixels = vec![0u8; (layer_width * layer_height * 4) as usize];

        // Fill a 10x10 region with red
        for y in 20..30 {
            for x in 20..30 {
                let idx = ((y * layer_width + x) * 4) as usize;
                pixels[idx] = 255;     // R
                pixels[idx + 1] = 0;   // G
                pixels[idx + 2] = 0;   // B
                pixels[idx + 3] = 255; // A
            }
        }

        // Create incremental snapshot of just the red region
        let dirty = DirtyRect::new(20, 20, 10, 10);
        let snapshot = LayerSnapshot::incremental(
            Uuid::new_v4(),
            &pixels,
            layer_width,
            layer_height,
            dirty,
        );

        // Snapshot should be much smaller than full (400 bytes vs 40000 bytes)
        assert!(snapshot.memory_size() < 1000);

        // Restore to a fresh buffer
        let mut restored = vec![0u8; (layer_width * layer_height * 4) as usize];
        snapshot.restore_to(&mut restored, layer_width);

        // Check restored region has red pixels
        for y in 20..30 {
            for x in 20..30 {
                let idx = ((y * layer_width + x) * 4) as usize;
                assert_eq!(restored[idx], 255, "Red channel should be 255");
                assert_eq!(restored[idx + 3], 255, "Alpha channel should be 255");
            }
        }
    }

    #[test]
    fn test_push_and_undo() {
        let mut manager = HistoryManager::new();

        // Create a state
        let state1 = HistoryState::new("Draw stroke 1");
        manager.push_state(state1);

        assert!(manager.can_undo());
        assert!(!manager.can_redo());

        // Undo with current state
        let current = HistoryState::new("Current");
        let restored = manager.undo(current);
        assert!(restored.is_some());
        assert_eq!(restored.unwrap().description, "Draw stroke 1");

        assert!(!manager.can_undo());
        assert!(manager.can_redo());
    }

    #[test]
    fn test_undo_redo_cycle() {
        let mut manager = HistoryManager::new();

        manager.push_state(HistoryState::new("Action 1"));
        manager.push_state(HistoryState::new("Action 2"));

        assert_eq!(manager.undo_count(), 2);
        assert_eq!(manager.redo_count(), 0);

        // Undo
        let current = HistoryState::new("Current after 2");
        manager.undo(current);

        assert_eq!(manager.undo_count(), 1);
        assert_eq!(manager.redo_count(), 1);

        // Redo
        let current = HistoryState::new("Current after undo");
        manager.redo(current);

        assert_eq!(manager.undo_count(), 2);
        assert_eq!(manager.redo_count(), 0);
    }

    #[test]
    fn test_max_steps() {
        let mut manager = HistoryManager::with_max_steps(3);

        for i in 0..5 {
            manager.push_state(HistoryState::new(format!("Action {}", i)));
        }

        // Should only keep last 3
        assert_eq!(manager.undo_count(), 3);
    }
}
