//! Performance optimizations for the DrawConnect core engine
//!
//! This module provides optimized implementations of critical rendering paths.

use std::collections::HashMap;

/// Stamp cache for reusing brush stamps
pub struct StampCache {
    cache: HashMap<StampKey, CachedStamp>,
    max_entries: usize,
    access_counter: u64,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct StampKey {
    size_quantized: u32,
    hardness_quantized: u8,
    angle_quantized: u8,
}

struct CachedStamp {
    data: Vec<f32>,
    #[allow(dead_code)]
    size: u32,
    last_access: u64,
}

impl StampCache {
    /// Create a new stamp cache with specified maximum entries
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_entries),
            max_entries,
            access_counter: 0,
        }
    }

    /// Get or generate a stamp
    pub fn get_or_generate<F>(&mut self, size: f32, hardness: f32, angle: f32, generator: F) -> &[f32]
    where
        F: FnOnce(f32, f32, f32) -> (Vec<f32>, u32),
    {
        // Quantize parameters for cache key
        let key = StampKey {
            size_quantized: (size * 2.0).round() as u32, // 0.5px precision
            hardness_quantized: (hardness * 100.0) as u8,
            angle_quantized: ((angle % 360.0) / 2.0) as u8, // 2 degree precision
        };

        self.access_counter += 1;

        if !self.cache.contains_key(&key) {
            // Generate new stamp
            let (data, stamp_size) = generator(size, hardness, angle);

            // Evict if cache is full
            if self.cache.len() >= self.max_entries {
                self.evict_oldest();
            }

            self.cache.insert(
                key.clone(),
                CachedStamp {
                    data,
                    size: stamp_size,
                    last_access: self.access_counter,
                },
            );
        }

        // Update access time and return
        if let Some(entry) = self.cache.get_mut(&key) {
            entry.last_access = self.access_counter;
            &entry.data
        } else {
            &[]
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .cache
            .iter()
            .min_by_key(|(_, v)| v.last_access)
            .map(|(k, _)| k.clone())
        {
            self.cache.remove(&oldest_key);
        }
    }

    /// Clear all cached stamps
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// Optimized pixel blending using batch operations
pub struct BatchBlender {
    /// Accumulated pixels to blend
    buffer: Vec<BlendOp>,
    capacity: usize,
}

struct BlendOp {
    x: u32,
    y: u32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl BatchBlender {
    /// Create a new batch blender with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Add a pixel blend operation to the batch
    #[inline]
    pub fn add(&mut self, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(BlendOp { x, y, r, g, b, a });
        }
    }

    /// Flush all pending operations to pixel buffer
    pub fn flush(&mut self, pixels: &mut [u8], width: u32, height: u32) {
        for op in self.buffer.drain(..) {
            if op.x < width && op.y < height {
                let idx = ((op.y * width + op.x) * 4) as usize;
                if idx + 3 < pixels.len() {
                    // Alpha blending
                    let src_a = op.a;
                    let dst_a = pixels[idx + 3] as f32 / 255.0;
                    let out_a = src_a + dst_a * (1.0 - src_a);

                    if out_a > 0.0 {
                        let inv_out_a = 1.0 / out_a;
                        pixels[idx] = ((op.r * src_a + (pixels[idx] as f32 / 255.0) * dst_a * (1.0 - src_a)) * inv_out_a * 255.0) as u8;
                        pixels[idx + 1] = ((op.g * src_a + (pixels[idx + 1] as f32 / 255.0) * dst_a * (1.0 - src_a)) * inv_out_a * 255.0) as u8;
                        pixels[idx + 2] = ((op.b * src_a + (pixels[idx + 2] as f32 / 255.0) * dst_a * (1.0 - src_a)) * inv_out_a * 255.0) as u8;
                        pixels[idx + 3] = (out_a * 255.0) as u8;
                    }
                }
            }
        }
    }

    /// Clear all pending operations
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

/// Dirty region tracking for incremental rendering
#[derive(Debug, Clone)]
pub struct DirtyRegionTracker {
    regions: Vec<DirtyRect>,
    canvas_width: u32,
    canvas_height: u32,
    tile_size: u32,
    dirty_tiles: Vec<bool>,
    tiles_x: u32,
    tiles_y: u32,
}

/// A rectangular dirty region
#[derive(Debug, Clone, Copy)]
pub struct DirtyRect {
    /// X coordinate of the region
    pub x: u32,
    /// Y coordinate of the region
    pub y: u32,
    /// Width of the region
    pub width: u32,
    /// Height of the region
    pub height: u32,
}

impl DirtyRegionTracker {
    /// Create a new dirty region tracker
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles_x = (width + tile_size - 1) / tile_size;
        let tiles_y = (height + tile_size - 1) / tile_size;

        Self {
            regions: Vec::new(),
            canvas_width: width,
            canvas_height: height,
            tile_size,
            dirty_tiles: vec![false; (tiles_x * tiles_y) as usize],
            tiles_x,
            tiles_y,
        }
    }

    /// Mark a region as dirty
    pub fn mark_dirty(&mut self, x: i32, y: i32, width: u32, height: u32) {
        // Clamp to canvas bounds
        let x = x.max(0) as u32;
        let y = y.max(0) as u32;
        let x2 = (x + width).min(self.canvas_width);
        let y2 = (y + height).min(self.canvas_height);

        if x2 <= x || y2 <= y {
            return;
        }

        // Mark affected tiles
        let tile_x1 = x / self.tile_size;
        let tile_y1 = y / self.tile_size;
        let tile_x2 = (x2 - 1) / self.tile_size;
        let tile_y2 = (y2 - 1) / self.tile_size;

        for ty in tile_y1..=tile_y2 {
            for tx in tile_x1..=tile_x2 {
                if tx < self.tiles_x && ty < self.tiles_y {
                    self.dirty_tiles[(ty * self.tiles_x + tx) as usize] = true;
                }
            }
        }

        self.regions.push(DirtyRect {
            x,
            y,
            width: x2 - x,
            height: y2 - y,
        });
    }

    /// Get merged dirty regions for rendering
    pub fn get_dirty_regions(&self) -> Vec<DirtyRect> {
        let mut merged_regions = Vec::new();

        for ty in 0..self.tiles_y {
            for tx in 0..self.tiles_x {
                if self.dirty_tiles[(ty * self.tiles_x + tx) as usize] {
                    merged_regions.push(DirtyRect {
                        x: tx * self.tile_size,
                        y: ty * self.tile_size,
                        width: self.tile_size.min(self.canvas_width - tx * self.tile_size),
                        height: self.tile_size.min(self.canvas_height - ty * self.tile_size),
                    });
                }
            }
        }

        merged_regions
    }

    /// Clear all dirty regions
    pub fn clear(&mut self) {
        self.regions.clear();
        self.dirty_tiles.fill(false);
    }

    /// Check if any region is dirty
    pub fn is_dirty(&self) -> bool {
        self.dirty_tiles.iter().any(|&d| d)
    }
}

/// SIMD-optimized color operations (when available)
#[cfg(target_arch = "x86_64")]
pub mod simd {
    use std::arch::x86_64::*;

    /// Blend 4 pixels at once using SSE
    #[target_feature(enable = "sse2")]
    pub unsafe fn blend_pixels_sse(
        dst: &mut [u8],
        src_r: f32,
        src_g: f32,
        src_b: f32,
        src_a: f32,
    ) {
        if dst.len() < 16 {
            return;
        }

        // Load source color
        let src = _mm_set_ps(src_a, src_b, src_g, src_r);
        let src_alpha = _mm_set1_ps(src_a);
        let one = _mm_set1_ps(1.0);
        let inv_src_alpha = _mm_sub_ps(one, src_alpha);
        let scale = _mm_set1_ps(255.0);
        let inv_scale = _mm_set1_ps(1.0 / 255.0);

        // Process 4 pixels
        for i in 0..4 {
            let idx = i * 4;

            // Load destination
            let dst_bytes = _mm_set_ps(
                dst[idx + 3] as f32,
                dst[idx + 2] as f32,
                dst[idx + 1] as f32,
                dst[idx] as f32,
            );
            let dst_normalized = _mm_mul_ps(dst_bytes, inv_scale);

            // Blend: out = src * src_a + dst * (1 - src_a)
            let blended = _mm_add_ps(
                _mm_mul_ps(src, src_alpha),
                _mm_mul_ps(dst_normalized, inv_src_alpha),
            );

            // Convert back to bytes
            let result = _mm_mul_ps(blended, scale);

            // Store (simplified - actual implementation would use proper stores)
            let mut temp = [0.0f32; 4];
            _mm_storeu_ps(temp.as_mut_ptr(), result);

            dst[idx] = temp[0].clamp(0.0, 255.0) as u8;
            dst[idx + 1] = temp[1].clamp(0.0, 255.0) as u8;
            dst[idx + 2] = temp[2].clamp(0.0, 255.0) as u8;
            dst[idx + 3] = temp[3].clamp(0.0, 255.0) as u8;
        }
    }
}

/// Fast integer square root approximation
#[inline]
pub fn fast_sqrt(x: f32) -> f32 {
    // Newton-Raphson approximation
    if x <= 0.0 {
        return 0.0;
    }

    let mut y = x;
    let mut i = y.to_bits();
    i = 0x5f3759df - (i >> 1); // Initial guess
    y = f32::from_bits(i);
    y = y * (1.5 - (x * 0.5 * y * y)); // One iteration
    1.0 / y
}

/// Fast distance calculation without full sqrt
#[inline]
pub fn fast_distance_squared(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}

/// Stroke point interpolation with Catmull-Rom splines for smoother curves
pub fn catmull_rom_interpolate(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;

    let x = 0.5 * ((2.0 * p1.0) +
        (-p0.0 + p2.0) * t +
        (2.0 * p0.0 - 5.0 * p1.0 + 4.0 * p2.0 - p3.0) * t2 +
        (-p0.0 + 3.0 * p1.0 - 3.0 * p2.0 + p3.0) * t3);

    let y = 0.5 * ((2.0 * p1.1) +
        (-p0.1 + p2.1) * t +
        (2.0 * p0.1 - 5.0 * p1.1 + 4.0 * p2.1 - p3.1) * t2 +
        (-p0.1 + 3.0 * p1.1 - 3.0 * p2.1 + p3.1) * t3);

    (x, y)
}

/// Pressure smoothing using exponential moving average
pub struct PressureSmoother {
    smoothed_pressure: f32,
    smoothing_factor: f32,
}

impl PressureSmoother {
    /// Create a new pressure smoother with the given smoothing factor (0.0-1.0)
    pub fn new(smoothing_factor: f32) -> Self {
        Self {
            smoothed_pressure: 0.5,
            smoothing_factor: smoothing_factor.clamp(0.0, 1.0),
        }
    }

    /// Update the smoothed pressure with a new raw value
    pub fn update(&mut self, raw_pressure: f32) -> f32 {
        self.smoothed_pressure = self.smoothed_pressure * self.smoothing_factor
            + raw_pressure * (1.0 - self.smoothing_factor);
        self.smoothed_pressure
    }

    /// Reset the smoother to the default pressure value
    pub fn reset(&mut self) {
        self.smoothed_pressure = 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stamp_cache() {
        let mut cache = StampCache::new(10);

        let stamp = cache.get_or_generate(10.0, 0.5, 0.0, |size, _, _| {
            let s = size.ceil() as u32;
            (vec![1.0; (s * s) as usize], s)
        });

        assert!(!stamp.is_empty());
    }

    #[test]
    fn test_dirty_region_tracker() {
        let mut tracker = DirtyRegionTracker::new(1024, 1024, 64);

        tracker.mark_dirty(100, 100, 50, 50);
        assert!(tracker.is_dirty());

        let regions = tracker.get_dirty_regions();
        assert!(!regions.is_empty());

        tracker.clear();
        assert!(!tracker.is_dirty());
    }

    #[test]
    fn test_fast_sqrt() {
        let x = 16.0;
        let result = fast_sqrt(x);
        assert!((result - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_catmull_rom() {
        let p0 = (0.0, 0.0);
        let p1 = (1.0, 1.0);
        let p2 = (2.0, 1.0);
        let p3 = (3.0, 0.0);

        let mid = catmull_rom_interpolate(p0, p1, p2, p3, 0.5);
        assert!(mid.0 > 1.0 && mid.0 < 2.0);
    }
}
