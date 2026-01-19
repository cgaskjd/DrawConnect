//! Radial Blur filter
//!
//! Creates spin or zoom blur effect from a center point.

use crate::filters::Filter;

/// Radial blur type
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RadialBlurType {
    /// Spinning motion blur
    Spin,
    /// Zooming motion blur
    Zoom,
}

impl Default for RadialBlurType {
    fn default() -> Self {
        Self::Spin
    }
}

/// Radial Blur filter
#[derive(Debug, Clone)]
pub struct RadialBlur {
    /// Blur amount (0.0 to 1.0)
    pub amount: f32,
    /// Center X (0.0 to 1.0, relative to width)
    pub center_x: f32,
    /// Center Y (0.0 to 1.0, relative to height)
    pub center_y: f32,
    /// Blur type (spin or zoom)
    pub blur_type: RadialBlurType,
}

impl RadialBlur {
    /// Create a new radial blur filter
    pub fn new(amount: f32, center_x: f32, center_y: f32, blur_type: RadialBlurType) -> Self {
        Self {
            amount: amount.clamp(0.0, 1.0),
            center_x: center_x.clamp(0.0, 1.0),
            center_y: center_y.clamp(0.0, 1.0),
            blur_type,
        }
    }
}

impl Default for RadialBlur {
    fn default() -> Self {
        Self {
            amount: 0.1,
            center_x: 0.5,
            center_y: 0.5,
            blur_type: RadialBlurType::Spin,
        }
    }
}

impl Filter for RadialBlur {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();

        let cx = self.center_x * width as f32;
        let cy = self.center_y * height as f32;
        let samples = (self.amount * 20.0).max(1.0) as i32;

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                let mut r_acc = 0.0f32;
                let mut g_acc = 0.0f32;
                let mut b_acc = 0.0f32;
                let mut a_acc = 0.0f32;
                let mut count = 0.0f32;

                for i in -samples..=samples {
                    let t = i as f32 / samples as f32 * self.amount;

                    let (sx, sy) = match self.blur_type {
                        RadialBlurType::Spin => {
                            // Rotate around center
                            let angle = t * 0.1;
                            let cos_a = angle.cos();
                            let sin_a = angle.sin();
                            (
                                cx + dx * cos_a - dy * sin_a,
                                cy + dx * sin_a + dy * cos_a,
                            )
                        }
                        RadialBlurType::Zoom => {
                            // Scale from center
                            let scale = 1.0 + t * 0.5;
                            (cx + dx * scale, cy + dy * scale)
                        }
                    };

                    if sx >= 0.0 && sx < width as f32 && sy >= 0.0 && sy < height as f32 {
                        let sx_i = sx as u32;
                        let sy_i = sy as u32;
                        let idx = ((sy_i * width + sx_i) * 4) as usize;

                        r_acc += original[idx] as f32;
                        g_acc += original[idx + 1] as f32;
                        b_acc += original[idx + 2] as f32;
                        a_acc += original[idx + 3] as f32;
                        count += 1.0;
                    }
                }

                if count > 0.0 {
                    let idx = ((y * width + x) * 4) as usize;
                    pixels[idx] = (r_acc / count) as u8;
                    pixels[idx + 1] = (g_acc / count) as u8;
                    pixels[idx + 2] = (b_acc / count) as u8;
                    pixels[idx + 3] = (a_acc / count) as u8;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Radial Blur"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin_blur() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = RadialBlur::new(0.2, 0.5, 0.5, RadialBlurType::Spin);
        filter.apply(&mut pixels, 50, 50);
    }

    #[test]
    fn test_zoom_blur() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = RadialBlur::new(0.2, 0.5, 0.5, RadialBlurType::Zoom);
        filter.apply(&mut pixels, 50, 50);
    }
}
