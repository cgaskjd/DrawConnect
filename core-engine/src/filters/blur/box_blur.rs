//! Box Blur filter
//!
//! Fast blur using running sum optimization.

use crate::filters::Filter;
use super::{box_blur_h, box_blur_v};

/// Box Blur filter
#[derive(Debug, Clone)]
pub struct BoxBlur {
    /// Blur radius in pixels
    pub radius: u32,
}

impl BoxBlur {
    /// Create a new box blur filter
    pub fn new(radius: u32) -> Self {
        Self {
            radius: radius.max(1),
        }
    }
}

impl Default for BoxBlur {
    fn default() -> Self {
        Self { radius: 5 }
    }
}

impl Filter for BoxBlur {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.radius == 0 {
            return;
        }

        let mut temp = pixels.to_vec();
        let r = self.radius as i32;

        // Horizontal pass
        box_blur_h(pixels, &mut temp, width, height, r);
        // Vertical pass
        box_blur_v(&temp, pixels, width, height, r);
    }

    fn name(&self) -> &'static str {
        "Box Blur"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_blur() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = BoxBlur::new(3);
        filter.apply(&mut pixels, 50, 50);
    }
}
