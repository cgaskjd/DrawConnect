//! Gaussian Blur filter
//!
//! Uses separable Gaussian blur for efficient O(n) per axis processing.

use crate::filters::Filter;
use super::{box_blur_h, box_blur_v};

/// Gaussian Blur filter
#[derive(Debug, Clone)]
pub struct GaussianBlur {
    /// Blur radius in pixels
    pub radius: f32,
}

impl GaussianBlur {
    /// Create a new Gaussian blur filter
    pub fn new(radius: f32) -> Self {
        Self {
            radius: radius.max(0.1),
        }
    }

    /// Calculate box sizes for Gaussian approximation
    fn boxes_for_gauss(sigma: f32, n: usize) -> Vec<u32> {
        let w_ideal = ((12.0 * sigma * sigma / n as f32) + 1.0).sqrt();
        let mut wl = w_ideal.floor() as u32;
        if wl % 2 == 0 {
            wl -= 1;
        }
        let wu = wl + 2;

        let m_ideal = (12.0 * sigma * sigma
            - (n as f32 * wl as f32 * wl as f32)
            - (4.0 * n as f32 * wl as f32)
            - (3.0 * n as f32))
            / (-4.0 * wl as f32 - 4.0);
        let m = m_ideal.round() as usize;

        let mut sizes = Vec::with_capacity(n);
        for i in 0..n {
            sizes.push(if i < m { wl } else { wu });
        }
        sizes
    }
}

impl Default for GaussianBlur {
    fn default() -> Self {
        Self { radius: 5.0 }
    }
}

impl Filter for GaussianBlur {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.radius < 0.5 {
            return;
        }

        let sigma = self.radius / 2.0;
        let boxes = Self::boxes_for_gauss(sigma, 3);

        let mut temp = pixels.to_vec();

        for box_size in boxes {
            let r = ((box_size - 1) / 2) as i32;
            box_blur_h(pixels, &mut temp, width, height, r);
            box_blur_v(&temp, pixels, width, height, r);
        }
    }

    fn name(&self) -> &'static str {
        "Gaussian Blur"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_doesnt_crash() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = GaussianBlur::new(5.0);
        filter.apply(&mut pixels, 100, 100);
    }

    #[test]
    fn test_small_radius() {
        let mut pixels = vec![128u8; 10 * 10 * 4];
        let filter = GaussianBlur::new(0.1);
        filter.apply(&mut pixels, 10, 10);
    }
}
