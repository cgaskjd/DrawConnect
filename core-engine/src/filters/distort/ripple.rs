//! Ripple filter
//!
//! Creates concentric ripple effect emanating from center.

use crate::filters::Filter;
use crate::filters::pixel_ops::{bilinear_sample_rgba, write_pixel};
use std::f32::consts::PI;

/// Ripple filter - creates ripple distortion
#[derive(Debug, Clone)]
pub struct Ripple {
    /// Amount/amplitude of ripple (0 to 999)
    pub amount: f32,
    /// Size of ripples (small, medium, large)
    pub size: RippleSize,
}

/// Ripple size preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RippleSize {
    /// Small ripples
    Small,
    /// Medium ripples
    #[default]
    Medium,
    /// Large ripples
    Large,
}

impl RippleSize {
    fn wavelength(&self) -> f32 {
        match self {
            RippleSize::Small => 5.0,
            RippleSize::Medium => 10.0,
            RippleSize::Large => 20.0,
        }
    }
}

impl Ripple {
    /// Create a new ripple filter
    pub fn new(amount: f32, size: RippleSize) -> Self {
        Self {
            amount: amount.clamp(-999.0, 999.0),
            size,
        }
    }
}

impl Default for Ripple {
    fn default() -> Self {
        Self {
            amount: 100.0,
            size: RippleSize::Medium,
        }
    }
}

impl Filter for Ripple {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.amount == 0.0 {
            return;
        }

        let original = pixels.to_vec();
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let wavelength = self.size.wavelength();
        let amplitude = self.amount / 100.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > 0.0 {
                    // Calculate ripple displacement
                    let ripple = (dist / wavelength * 2.0 * PI).sin() * amplitude;

                    // Displace along the radius
                    let factor = (dist + ripple) / dist;
                    let src_x = cx + dx * factor;
                    let src_y = cy + dy * factor;

                    // Bilinear interpolation using shared utility
                    if let Some(rgba) = bilinear_sample_rgba(&original, width, height, src_x, src_y) {
                        write_pixel(pixels, idx, rgba);
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Ripple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ripple() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = Ripple::default();
        filter.apply(&mut pixels, 100, 100);
    }
}
