//! Twirl filter
//!
//! Rotates pixels around the center point, creating a swirl effect.

use crate::filters::Filter;
use crate::filters::pixel_ops::{bilinear_sample_rgba_clamped, write_pixel};
use std::f32::consts::PI;

/// Twirl filter - rotates pixels around center
#[derive(Debug, Clone)]
pub struct Twirl {
    /// Angle in degrees (-999 to 999)
    pub angle: f32,
    /// Radius as percentage of image size (0 to 100)
    pub radius: f32,
}

impl Twirl {
    /// Create a new twirl filter
    pub fn new(angle: f32, radius: f32) -> Self {
        Self {
            angle: angle.clamp(-999.0, 999.0),
            radius: radius.clamp(0.0, 100.0),
        }
    }
}

impl Default for Twirl {
    fn default() -> Self {
        Self {
            angle: 50.0,
            radius: 100.0,
        }
    }
}

impl Filter for Twirl {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.angle == 0.0 {
            return;
        }

        let original = pixels.to_vec();
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let max_radius = (cx.min(cy)) * (self.radius / 100.0);
        let angle_rad = self.angle * PI / 180.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < max_radius && dist > 0.0 {
                    // Calculate rotation amount based on distance from center
                    let factor = 1.0 - (dist / max_radius);
                    let rotation = angle_rad * factor * factor;

                    // Rotate the point
                    let cos_r = rotation.cos();
                    let sin_r = rotation.sin();
                    let src_x = cx + dx * cos_r - dy * sin_r;
                    let src_y = cy + dx * sin_r + dy * cos_r;

                    // Bilinear interpolation using shared utility
                    let rgba = bilinear_sample_rgba_clamped(&original, width, height, src_x, src_y);
                    write_pixel(pixels, idx, rgba);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Twirl"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twirl() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = Twirl::new(90.0, 100.0);
        filter.apply(&mut pixels, 100, 100);
    }
}
