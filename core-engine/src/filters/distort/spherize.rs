//! Spherize filter
//!
//! Creates a 3D sphere effect by wrapping the image around a sphere.

use crate::filters::Filter;
use crate::filters::pixel_ops::{bilinear_sample_rgba_clamped, write_pixel};

/// Spherize distortion mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpherizeMode {
    /// Normal spherize (both horizontal and vertical)
    #[default]
    Normal,
    /// Horizontal only
    Horizontal,
    /// Vertical only
    Vertical,
}

/// Spherize filter - creates a 3D sphere effect
#[derive(Debug, Clone)]
pub struct Spherize {
    /// Amount of spherize effect (-100 to 100)
    /// Positive values create a bulge, negative values create a pinch
    pub amount: i32,
    /// Spherize mode
    pub mode: SpherizeMode,
}

impl Spherize {
    /// Create a new spherize filter
    pub fn new(amount: i32, mode: SpherizeMode) -> Self {
        Self {
            amount: amount.clamp(-100, 100),
            mode,
        }
    }
}

impl Default for Spherize {
    fn default() -> Self {
        Self {
            amount: 50,
            mode: SpherizeMode::Normal,
        }
    }
}

impl Filter for Spherize {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.amount == 0 {
            return;
        }

        let original = pixels.to_vec();
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let radius = cx.min(cy);
        let amount = self.amount as f32 / 100.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Calculate normalized distance from center
                let dx = (x as f32 - cx) / radius;
                let dy = (y as f32 - cy) / radius;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 1.0 {
                    // Apply spherize distortion
                    let (src_x, src_y) = match self.mode {
                        SpherizeMode::Normal => {
                            let factor = if amount > 0.0 {
                                (1.0 - dist.powi(2)).sqrt() * amount + (1.0 - amount)
                            } else {
                                1.0 / ((1.0 - dist.powi(2)).sqrt() * (-amount) + (1.0 + amount))
                            };
                            (
                                cx + dx * radius * factor,
                                cy + dy * radius * factor,
                            )
                        }
                        SpherizeMode::Horizontal => {
                            let factor = if amount > 0.0 {
                                (1.0 - dx.powi(2)).sqrt() * amount + (1.0 - amount)
                            } else {
                                1.0 / ((1.0 - dx.powi(2)).sqrt() * (-amount) + (1.0 + amount))
                            };
                            (cx + dx * radius * factor, y as f32)
                        }
                        SpherizeMode::Vertical => {
                            let factor = if amount > 0.0 {
                                (1.0 - dy.powi(2)).sqrt() * amount + (1.0 - amount)
                            } else {
                                1.0 / ((1.0 - dy.powi(2)).sqrt() * (-amount) + (1.0 + amount))
                            };
                            (x as f32, cy + dy * radius * factor)
                        }
                    };

                    // Bilinear interpolation using shared utility
                    let rgba = bilinear_sample_rgba_clamped(&original, width, height, src_x, src_y);
                    write_pixel(pixels, idx, rgba);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Spherize"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spherize() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = Spherize::new(50, SpherizeMode::Normal);
        filter.apply(&mut pixels, 100, 100);
    }
}
