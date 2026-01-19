//! Find Edges filter
//!
//! Edge detection using Sobel operator.

use crate::filters::Filter;

/// Find Edges filter
#[derive(Debug, Clone, Default)]
pub struct FindEdges;

impl FindEdges {
    /// Create a new find edges filter
    pub fn new() -> Self {
        Self
    }
}

impl Filter for FindEdges {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();

        // Sobel kernels
        // Gx: [-1, 0, 1; -2, 0, 2; -1, 0, 1]
        // Gy: [-1, -2, -1; 0, 0, 0; 1, 2, 1]

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = ((y * width + x) * 4) as usize;

                for c in 0..3 {
                    // Get 3x3 neighborhood
                    let p00 = original[(((y - 1) * width + x - 1) * 4) as usize + c] as f32;
                    let p01 = original[(((y - 1) * width + x) * 4) as usize + c] as f32;
                    let p02 = original[(((y - 1) * width + x + 1) * 4) as usize + c] as f32;
                    let p10 = original[((y * width + x - 1) * 4) as usize + c] as f32;
                    let p12 = original[((y * width + x + 1) * 4) as usize + c] as f32;
                    let p20 = original[(((y + 1) * width + x - 1) * 4) as usize + c] as f32;
                    let p21 = original[(((y + 1) * width + x) * 4) as usize + c] as f32;
                    let p22 = original[(((y + 1) * width + x + 1) * 4) as usize + c] as f32;

                    // Apply Sobel
                    let gx = -p00 + p02 - 2.0 * p10 + 2.0 * p12 - p20 + p22;
                    let gy = -p00 - 2.0 * p01 - p02 + p20 + 2.0 * p21 + p22;

                    let magnitude = (gx * gx + gy * gy).sqrt();
                    pixels[idx + c] = magnitude.clamp(0.0, 255.0) as u8;
                }
                // Alpha unchanged
            }
        }

        // Handle edges (set to black)
        for x in 0..width {
            let top_idx = (x * 4) as usize;
            let bottom_idx = (((height - 1) * width + x) * 4) as usize;
            pixels[top_idx] = 0;
            pixels[top_idx + 1] = 0;
            pixels[top_idx + 2] = 0;
            pixels[bottom_idx] = 0;
            pixels[bottom_idx + 1] = 0;
            pixels[bottom_idx + 2] = 0;
        }
        for y in 0..height {
            let left_idx = ((y * width) * 4) as usize;
            let right_idx = ((y * width + width - 1) * 4) as usize;
            pixels[left_idx] = 0;
            pixels[left_idx + 1] = 0;
            pixels[left_idx + 2] = 0;
            pixels[right_idx] = 0;
            pixels[right_idx + 1] = 0;
            pixels[right_idx + 2] = 0;
        }
    }

    fn name(&self) -> &'static str {
        "Find Edges"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_edges() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = FindEdges::new();
        filter.apply(&mut pixels, 50, 50);
    }
}
