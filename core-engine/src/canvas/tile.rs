//! Tile-based canvas storage

use crate::color::Color;
use std::collections::HashMap;

/// A single tile in the canvas
#[derive(Clone)]
pub struct Tile {
    /// Tile size (width and height)
    pub size: u32,
    /// Pixel data (RGBA, row-major)
    pub data: Vec<u8>,
    /// Is tile dirty (needs redraw)
    pub dirty: bool,
}

impl Tile {
    /// Create a new empty tile
    pub fn new(size: u32) -> Self {
        Self {
            size,
            data: vec![0; (size * size * 4) as usize],
            dirty: true,
        }
    }

    /// Create a tile filled with color
    pub fn filled(size: u32, color: Color) -> Self {
        let (r, g, b, a) = color.to_rgba8();
        let mut data = vec![0; (size * size * 4) as usize];

        for chunk in data.chunks_exact_mut(4) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }

        Self {
            size,
            data,
            dirty: true,
        }
    }

    /// Get pixel at position within tile
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= self.size || y >= self.size {
            return Color::transparent();
        }

        let idx = ((y * self.size + x) * 4) as usize;
        Color::from_rgba8(
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        )
    }

    /// Set pixel at position within tile
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.size || y >= self.size {
            return;
        }

        let idx = ((y * self.size + x) * 4) as usize;
        let (r, g, b, a) = color.to_rgba8();
        self.data[idx] = r;
        self.data[idx + 1] = g;
        self.data[idx + 2] = b;
        self.data[idx + 3] = a;
        self.dirty = true;
    }

    /// Clear tile (transparent)
    pub fn clear(&mut self) {
        self.data.fill(0);
        self.dirty = true;
    }

    /// Fill tile with color
    pub fn fill(&mut self, color: Color) {
        let (r, g, b, a) = color.to_rgba8();
        for chunk in self.data.chunks_exact_mut(4) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
        self.dirty = true;
    }

    /// Check if tile is empty (all transparent)
    pub fn is_empty(&self) -> bool {
        self.data.chunks_exact(4).all(|c| c[3] == 0)
    }

    /// Get raw data
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Manages tiles for efficient large canvas handling
pub struct TileManager {
    /// Tile size
    tile_size: u32,
    /// Canvas width
    canvas_width: u32,
    /// Canvas height
    canvas_height: u32,
    /// Number of tiles horizontally
    tiles_x: u32,
    /// Number of tiles vertically
    tiles_y: u32,
    /// Tile storage (sparse - only allocated when needed)
    tiles: HashMap<(u32, u32), Tile>,
}

impl TileManager {
    /// Create a new tile manager
    pub fn new(tile_size: u32, canvas_width: u32, canvas_height: u32) -> Self {
        let tiles_x = (canvas_width + tile_size - 1) / tile_size;
        let tiles_y = (canvas_height + tile_size - 1) / tile_size;

        Self {
            tile_size,
            canvas_width,
            canvas_height,
            tiles_x,
            tiles_y,
            tiles: HashMap::new(),
        }
    }

    /// Get tile size
    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    /// Get canvas dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.canvas_width, self.canvas_height)
    }

    /// Get number of tiles
    pub fn tile_count(&self) -> (u32, u32) {
        (self.tiles_x, self.tiles_y)
    }

    /// Get pixel at canvas position
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        let (tile_x, tile_y) = self.pixel_to_tile(x, y);
        let (local_x, local_y) = self.pixel_to_local(x, y);

        self.tiles
            .get(&(tile_x, tile_y))
            .map(|tile| tile.get_pixel(local_x, local_y))
            .or(Some(Color::transparent()))
    }

    /// Set pixel at canvas position
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let (tile_x, tile_y) = self.pixel_to_tile(x, y);
        let (local_x, local_y) = self.pixel_to_local(x, y);

        let tile = self
            .tiles
            .entry((tile_x, tile_y))
            .or_insert_with(|| Tile::new(self.tile_size));

        tile.set_pixel(local_x, local_y, color);
    }

    /// Convert canvas pixel to tile coordinates
    fn pixel_to_tile(&self, x: u32, y: u32) -> (u32, u32) {
        (x / self.tile_size, y / self.tile_size)
    }

    /// Convert canvas pixel to local tile coordinates
    fn pixel_to_local(&self, x: u32, y: u32) -> (u32, u32) {
        (x % self.tile_size, y % self.tile_size)
    }

    /// Get tile at position
    pub fn get_tile(&self, tile_x: u32, tile_y: u32) -> Option<&Tile> {
        self.tiles.get(&(tile_x, tile_y))
    }

    /// Get mutable tile at position
    pub fn get_tile_mut(&mut self, tile_x: u32, tile_y: u32) -> Option<&mut Tile> {
        self.tiles.get_mut(&(tile_x, tile_y))
    }

    /// Ensure tile exists at position
    pub fn ensure_tile(&mut self, tile_x: u32, tile_y: u32) -> &mut Tile {
        self.tiles
            .entry((tile_x, tile_y))
            .or_insert_with(|| Tile::new(self.tile_size))
    }

    /// Clear all tiles
    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    /// Fill all tiles with color
    pub fn fill(&mut self, color: Color) {
        // Allocate all tiles and fill them
        for ty in 0..self.tiles_y {
            for tx in 0..self.tiles_x {
                let tile = self.ensure_tile(tx, ty);
                tile.fill(color);
            }
        }
    }

    /// Resize canvas
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.canvas_width = new_width;
        self.canvas_height = new_height;
        self.tiles_x = (new_width + self.tile_size - 1) / self.tile_size;
        self.tiles_y = (new_height + self.tile_size - 1) / self.tile_size;

        // Remove tiles outside new bounds
        self.tiles.retain(|&(tx, ty), _| tx < self.tiles_x && ty < self.tiles_y);
    }

    /// Create snapshot of current state
    pub fn snapshot(&self) -> HashMap<(u32, u32), Vec<u8>> {
        self.tiles
            .iter()
            .map(|(&pos, tile)| (pos, tile.data.clone()))
            .collect()
    }

    /// Restore from snapshot
    pub fn restore(&mut self, snapshot: &HashMap<(u32, u32), Vec<u8>>) {
        self.tiles.clear();

        for (&pos, data) in snapshot {
            let mut tile = Tile::new(self.tile_size);
            tile.data = data.clone();
            tile.dirty = true;
            self.tiles.insert(pos, tile);
        }
    }

    /// Get dirty tiles
    pub fn dirty_tiles(&self) -> impl Iterator<Item = ((u32, u32), &Tile)> {
        self.tiles.iter().filter(|(_, tile)| tile.dirty).map(|(&pos, tile)| (pos, tile))
    }

    /// Clear dirty flags
    pub fn clear_dirty(&mut self) {
        for tile in self.tiles.values_mut() {
            tile.dirty = false;
        }
    }

    /// Get allocated tile count
    pub fn allocated_tiles(&self) -> usize {
        self.tiles.len()
    }

    /// Estimate memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        self.tiles.len() * (self.tile_size * self.tile_size * 4) as usize
    }

    /// Compact by removing empty tiles
    pub fn compact(&mut self) {
        self.tiles.retain(|_, tile| !tile.is_empty());
    }

    /// Get all tiles
    pub fn tiles(&self) -> impl Iterator<Item = (&(u32, u32), &Tile)> {
        self.tiles.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_creation() {
        let tile = Tile::new(64);
        assert_eq!(tile.size, 64);
        assert_eq!(tile.data.len(), 64 * 64 * 4);
        assert!(tile.is_empty());
    }

    #[test]
    fn test_tile_pixel_operations() {
        let mut tile = Tile::new(64);
        let color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);

        tile.set_pixel(10, 10, color);
        let retrieved = tile.get_pixel(10, 10);

        assert!((retrieved.r - 1.0).abs() < 0.01);
        assert!(!tile.is_empty());
    }

    #[test]
    fn test_tile_manager() {
        let mut manager = TileManager::new(64, 256, 256);

        assert_eq!(manager.tile_count(), (4, 4));
        assert_eq!(manager.allocated_tiles(), 0);

        manager.set_pixel(100, 100, Color::red());
        assert_eq!(manager.allocated_tiles(), 1);

        let color = manager.get_pixel(100, 100).unwrap();
        assert!((color.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_tile_manager_snapshot() {
        let mut manager = TileManager::new(64, 128, 128);
        manager.set_pixel(50, 50, Color::blue());

        let snapshot = manager.snapshot();
        manager.clear();

        assert!(manager.get_pixel(50, 50).unwrap().b < 0.1);

        manager.restore(&snapshot);
        assert!((manager.get_pixel(50, 50).unwrap().b - 1.0).abs() < 0.01);
    }
}
