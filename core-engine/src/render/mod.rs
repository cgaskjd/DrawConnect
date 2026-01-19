//! Render Pipeline Module
//!
//! Provides GPU-accelerated rendering with wgpu, including:
//! - Tile-based rendering for large canvases
//! - Layer compositing
//! - Real-time preview
//! - Export rendering

use crate::canvas::Canvas;
use crate::error::{EngineError, EngineResult};
use crate::layer::LayerManager;

/// Render context for a frame
pub struct RenderContext {
    /// Viewport width
    pub width: u32,
    /// Viewport height
    pub height: u32,
    /// Zoom level
    pub zoom: f32,
    /// Pan offset X
    pub pan_x: f32,
    /// Pan offset Y
    pub pan_y: f32,
    /// Rotation angle (degrees)
    pub rotation: f32,
    /// Show pixel grid (when zoomed in)
    pub show_grid: bool,
    /// Background color (checkerboard if None)
    pub background: Option<[f32; 4]>,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            rotation: 0.0,
            show_grid: false,
            background: None,
        }
    }
}

/// Render quality settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderQuality {
    /// Fast preview (lower quality)
    Preview,
    /// Normal interactive rendering
    Normal,
    /// High quality for export
    High,
    /// Maximum quality
    Maximum,
}

impl Default for RenderQuality {
    fn default() -> Self {
        Self::Normal
    }
}

/// The render pipeline handles all rendering operations
pub struct RenderPipeline {
    /// GPU enabled
    gpu_enabled: bool,
    /// Current render quality
    quality: RenderQuality,
    /// Render context
    context: RenderContext,
    /// Dirty regions that need re-rendering
    dirty_regions: Vec<DirtyRegion>,
    /// Cached render output
    cached_output: Option<Vec<u8>>,
    /// Cache valid flag
    cache_valid: bool,
}

/// A region that needs re-rendering
#[derive(Debug, Clone, Copy)]
pub struct DirtyRegion {
    /// X position
    pub x: u32,
    /// Y position
    pub y: u32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
}

impl DirtyRegion {
    /// Create a new dirty region
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if two regions overlap
    pub fn overlaps(&self, other: &DirtyRegion) -> bool {
        !(self.x + self.width <= other.x
            || other.x + other.width <= self.x
            || self.y + self.height <= other.y
            || other.y + other.height <= self.y)
    }

    /// Merge two overlapping regions
    pub fn merge(&self, other: &DirtyRegion) -> DirtyRegion {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);

        DirtyRegion::new(x, y, right - x, bottom - y)
    }
}

impl RenderPipeline {
    /// Create a new render pipeline
    pub fn new(gpu_enabled: bool) -> EngineResult<Self> {
        Ok(Self {
            gpu_enabled,
            quality: RenderQuality::default(),
            context: RenderContext::default(),
            dirty_regions: Vec::new(),
            cached_output: None,
            cache_valid: false,
        })
    }

    /// Set render quality
    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.quality = quality;
        self.invalidate_cache();
    }

    /// Get current quality setting
    pub fn quality(&self) -> RenderQuality {
        self.quality
    }

    /// Update render context
    pub fn set_context(&mut self, context: RenderContext) {
        self.context = context;
        self.invalidate_cache();
    }

    /// Get render context
    pub fn context(&self) -> &RenderContext {
        &self.context
    }

    /// Mark a region as dirty
    pub fn mark_dirty(&mut self, region: DirtyRegion) {
        // Try to merge with existing regions
        let mut merged = false;
        for existing in &mut self.dirty_regions {
            if existing.overlaps(&region) {
                *existing = existing.merge(&region);
                merged = true;
                break;
            }
        }

        if !merged {
            self.dirty_regions.push(region);
        }

        self.cache_valid = false;
    }

    /// Mark entire canvas as dirty
    pub fn mark_all_dirty(&mut self, width: u32, height: u32) {
        self.dirty_regions.clear();
        self.dirty_regions.push(DirtyRegion::new(0, 0, width, height));
        self.cache_valid = false;
    }

    /// Clear dirty regions
    pub fn clear_dirty(&mut self) {
        self.dirty_regions.clear();
    }

    /// Invalidate render cache
    pub fn invalidate_cache(&mut self) {
        self.cache_valid = false;
    }

    /// Check if GPU is enabled
    pub fn is_gpu_enabled(&self) -> bool {
        self.gpu_enabled
    }

    /// Render the canvas
    pub fn render(&self, canvas: &Canvas, layer_manager: &LayerManager) -> EngineResult<Vec<u8>> {
        if self.gpu_enabled {
            self.render_gpu(canvas, layer_manager)
        } else {
            self.render_cpu(canvas, layer_manager)
        }
    }

    /// CPU-based rendering fallback
    fn render_cpu(&self, canvas: &Canvas, layer_manager: &LayerManager) -> EngineResult<Vec<u8>> {
        let width = canvas.width();
        let height = canvas.height();
        let mut output = vec![255u8; (width * height * 4) as usize];

        // Render checkerboard background
        self.render_background(&mut output, width, height);

        // Composite all layers from bottom to top
        for layer_arc in layer_manager.layers() {
            let layer = layer_arc.read();
            if !layer.visible {
                continue;
            }

            let layer_width = layer.width();
            let layer_height = layer.height();
            let layer_opacity = layer.opacity;

            for y in 0..layer_height.min(height) {
                for x in 0..layer_width.min(width) {
                    let src_idx = ((y * layer_width + x) * 4) as usize;
                    let dst_idx = ((y * width + x) * 4) as usize;

                    // Use <= for clarity: we need indices src_idx through src_idx+3
                    if src_idx + 4 <= layer.pixels.len() && dst_idx + 4 <= output.len() {
                        let src_a = layer.pixels[src_idx + 3] as f32 / 255.0 * layer_opacity;

                        if src_a > 0.001 {
                            // Get source and destination colors
                            let src_r = layer.pixels[src_idx] as f32 / 255.0;
                            let src_g = layer.pixels[src_idx + 1] as f32 / 255.0;
                            let src_b = layer.pixels[src_idx + 2] as f32 / 255.0;

                            let dst_r = output[dst_idx] as f32 / 255.0;
                            let dst_g = output[dst_idx + 1] as f32 / 255.0;
                            let dst_b = output[dst_idx + 2] as f32 / 255.0;
                            let dst_a = output[dst_idx + 3] as f32 / 255.0;

                            // Alpha compositing (Porter-Duff over)
                            let out_a = src_a + dst_a * (1.0 - src_a);

                            if out_a > 0.001 {
                                // Blend colors
                                let out_r = (src_r * src_a + dst_r * dst_a * (1.0 - src_a)) / out_a;
                                let out_g = (src_g * src_a + dst_g * dst_a * (1.0 - src_a)) / out_a;
                                let out_b = (src_b * src_a + dst_b * dst_a * (1.0 - src_a)) / out_a;

                                output[dst_idx] = (out_r * 255.0).clamp(0.0, 255.0) as u8;
                                output[dst_idx + 1] = (out_g * 255.0).clamp(0.0, 255.0) as u8;
                                output[dst_idx + 2] = (out_b * 255.0).clamp(0.0, 255.0) as u8;
                                output[dst_idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;
                            }
                        }
                    }
                }
            }
        }

        Ok(output)
    }

    /// GPU-accelerated rendering
    fn render_gpu(&self, canvas: &Canvas, layer_manager: &LayerManager) -> EngineResult<Vec<u8>> {
        // For now, fall back to CPU rendering
        // Full GPU implementation would use wgpu
        self.render_cpu(canvas, layer_manager)
    }

    /// Render checkerboard background
    fn render_background(&self, output: &mut [u8], width: u32, height: u32) {
        if let Some(bg) = self.context.background {
            // Solid color background
            let r = (bg[0] * 255.0) as u8;
            let g = (bg[1] * 255.0) as u8;
            let b = (bg[2] * 255.0) as u8;
            let a = (bg[3] * 255.0) as u8;

            for chunk in output.chunks_exact_mut(4) {
                chunk[0] = r;
                chunk[1] = g;
                chunk[2] = b;
                chunk[3] = a;
            }
        } else {
            // Checkerboard pattern
            let check_size = 8;
            let light = 255u8;
            let dark = 204u8;

            for y in 0..height {
                for x in 0..width {
                    let idx = ((y * width + x) * 4) as usize;
                    let is_light = ((x / check_size) + (y / check_size)) % 2 == 0;
                    let value = if is_light { light } else { dark };

                    output[idx] = value;
                    output[idx + 1] = value;
                    output[idx + 2] = value;
                    output[idx + 3] = 255;
                }
            }
        }
    }

    /// Render to image for export
    pub fn render_export(
        &self,
        canvas: &Canvas,
        layer_manager: &LayerManager,
        _quality: RenderQuality,
    ) -> EngineResult<Vec<u8>> {
        self.render_cpu(canvas, layer_manager)
    }

    /// Generate thumbnail
    pub fn render_thumbnail(
        &self,
        canvas: &Canvas,
        layer_manager: &LayerManager,
        max_size: u32,
    ) -> EngineResult<(Vec<u8>, u32, u32)> {
        let full = self.render_cpu(canvas, layer_manager)?;

        let width = canvas.width();
        let height = canvas.height();
        let scale = (max_size as f32 / width.max(height) as f32).min(1.0);

        let thumb_w = (width as f32 * scale) as u32;
        let thumb_h = (height as f32 * scale) as u32;

        let mut thumbnail = vec![0u8; (thumb_w * thumb_h * 4) as usize];

        // Simple nearest-neighbor downscaling
        for ty in 0..thumb_h {
            for tx in 0..thumb_w {
                let sx = (tx as f32 / scale) as u32;
                let sy = (ty as f32 / scale) as u32;

                let src_idx = ((sy.min(height - 1) * width + sx.min(width - 1)) * 4) as usize;
                let dst_idx = ((ty * thumb_w + tx) * 4) as usize;

                thumbnail[dst_idx..dst_idx + 4].copy_from_slice(&full[src_idx..src_idx + 4]);
            }
        }

        Ok((thumbnail, thumb_w, thumb_h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = RenderPipeline::new(false);
        assert!(pipeline.is_ok());
    }

    #[test]
    fn test_dirty_region_overlap() {
        let r1 = DirtyRegion::new(0, 0, 100, 100);
        let r2 = DirtyRegion::new(50, 50, 100, 100);
        let r3 = DirtyRegion::new(200, 200, 50, 50);

        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn test_dirty_region_merge() {
        let r1 = DirtyRegion::new(0, 0, 100, 100);
        let r2 = DirtyRegion::new(50, 50, 100, 100);

        let merged = r1.merge(&r2);
        assert_eq!(merged.x, 0);
        assert_eq!(merged.y, 0);
        assert_eq!(merged.width, 150);
        assert_eq!(merged.height, 150);
    }
}
