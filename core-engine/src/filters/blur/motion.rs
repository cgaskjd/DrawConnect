//! Motion Blur filter
//!
//! Creates blur effect in a specific direction.

use crate::filters::Filter;

/// Motion Blur filter
#[derive(Debug, Clone)]
pub struct MotionBlur {
    /// Angle in degrees (0-360)
    pub angle: f32,
    /// Distance in pixels
    pub distance: u32,
}

impl MotionBlur {
    /// Create a new motion blur filter
    pub fn new(angle: f32, distance: u32) -> Self {
        Self {
            angle: angle % 360.0,
            distance: distance.max(1),
        }
    }
}

impl Default for MotionBlur {
    fn default() -> Self {
        Self {
            angle: 0.0,
            distance: 10,
        }
    }
}

impl Filter for MotionBlur {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();

        let angle_rad = self.angle.to_radians();
        let dx = angle_rad.cos();
        let dy = angle_rad.sin();

        let samples = self.distance as i32;

        for y in 0..height {
            for x in 0..width {
                let mut r_acc = 0.0f32;
                let mut g_acc = 0.0f32;
                let mut b_acc = 0.0f32;
                let mut a_acc = 0.0f32;
                let mut count = 0.0f32;

                for i in -samples..=samples {
                    let sx = x as f32 + dx * i as f32;
                    let sy = y as f32 + dy * i as f32;

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
        "Motion Blur"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_blur() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = MotionBlur::new(45.0, 5);
        filter.apply(&mut pixels, 50, 50);
    }
}
