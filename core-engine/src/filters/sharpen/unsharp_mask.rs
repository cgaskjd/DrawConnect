//! Unsharp Mask filter
//!
//! Classic sharpening technique using blur subtraction.

use crate::filters::Filter;
use crate::filters::blur::GaussianBlur;

/// Unsharp Mask filter
#[derive(Debug, Clone)]
pub struct UnsharpMask {
    /// Sharpening amount (0.0 to 5.0)
    pub amount: f32,
    /// Blur radius for mask
    pub radius: f32,
    /// Threshold to avoid sharpening noise (0 to 255)
    pub threshold: u8,
}

impl UnsharpMask {
    /// Create a new unsharp mask filter
    pub fn new(amount: f32, radius: f32, threshold: u8) -> Self {
        Self {
            amount: amount.clamp(0.0, 5.0),
            radius: radius.max(0.1),
            threshold,
        }
    }
}

impl Default for UnsharpMask {
    fn default() -> Self {
        Self {
            amount: 1.0,
            radius: 1.0,
            threshold: 0,
        }
    }
}

impl Filter for UnsharpMask {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        // Create blurred copy
        let mut blurred = pixels.to_vec();
        let blur = GaussianBlur::new(self.radius);
        blur.apply(&mut blurred, width, height);

        let threshold = self.threshold as f32;

        // Apply unsharp mask
        for i in (0..pixels.len()).step_by(4) {
            for c in 0..3 {
                let original = pixels[i + c] as f32;
                let blur_val = blurred[i + c] as f32;
                let diff = original - blur_val;

                // Only sharpen if difference is above threshold
                if diff.abs() > threshold {
                    let sharpened = original + self.amount * diff;
                    pixels[i + c] = sharpened.clamp(0.0, 255.0) as u8;
                }
            }
            // Keep alpha unchanged
        }
    }

    fn name(&self) -> &'static str {
        "Unsharp Mask"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsharp_mask() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = UnsharpMask::new(1.0, 1.0, 0);
        filter.apply(&mut pixels, 50, 50);
    }
}
