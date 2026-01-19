//! High Pass filter
//!
//! Extracts high-frequency detail for sharpening when overlaid.

use crate::filters::Filter;
use crate::filters::blur::GaussianBlur;

/// High Pass filter
#[derive(Debug, Clone)]
pub struct HighPass {
    /// Filter radius
    pub radius: f32,
}

impl HighPass {
    /// Create a new high pass filter
    pub fn new(radius: f32) -> Self {
        Self {
            radius: radius.max(0.1),
        }
    }
}

impl Default for HighPass {
    fn default() -> Self {
        Self { radius: 3.0 }
    }
}

impl Filter for HighPass {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        // Create blurred copy
        let original = pixels.to_vec();
        let mut blurred = pixels.to_vec();
        let blur = GaussianBlur::new(self.radius);
        blur.apply(&mut blurred, width, height);

        // High pass = original - blur + 128 (neutral gray)
        for i in (0..pixels.len()).step_by(4) {
            for c in 0..3 {
                let orig = original[i + c] as f32;
                let blur_val = blurred[i + c] as f32;
                let high_pass = (orig - blur_val + 128.0).clamp(0.0, 255.0);
                pixels[i + c] = high_pass as u8;
            }
            // Keep alpha unchanged
        }
    }

    fn name(&self) -> &'static str {
        "High Pass"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_pass() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = HighPass::new(3.0);
        filter.apply(&mut pixels, 50, 50);
    }
}
