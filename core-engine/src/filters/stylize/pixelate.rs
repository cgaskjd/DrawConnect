//! Pixelate filter
//!
//! Creates a pixelated/mosaic effect.

use crate::filters::Filter;

/// Pixelate filter
#[derive(Debug, Clone)]
pub struct Pixelate {
    /// Cell size in pixels
    pub cell_size: u32,
}

impl Pixelate {
    /// Create a new pixelate filter
    pub fn new(cell_size: u32) -> Self {
        Self {
            cell_size: cell_size.max(1),
        }
    }
}

impl Default for Pixelate {
    fn default() -> Self {
        Self { cell_size: 10 }
    }
}

impl Filter for Pixelate {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let cell = self.cell_size;

        // Process each cell
        for cy in (0..height).step_by(cell as usize) {
            for cx in (0..width).step_by(cell as usize) {
                // Calculate average color in cell
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;

                for y in cy..(cy + cell).min(height) {
                    for x in cx..(cx + cell).min(width) {
                        let idx = ((y * width + x) * 4) as usize;
                        r_sum += pixels[idx] as u32;
                        g_sum += pixels[idx + 1] as u32;
                        b_sum += pixels[idx + 2] as u32;
                        a_sum += pixels[idx + 3] as u32;
                        count += 1;
                    }
                }

                if count > 0 {
                    let r = (r_sum / count) as u8;
                    let g = (g_sum / count) as u8;
                    let b = (b_sum / count) as u8;
                    let a = (a_sum / count) as u8;

                    // Fill cell with average color
                    for y in cy..(cy + cell).min(height) {
                        for x in cx..(cx + cell).min(width) {
                            let idx = ((y * width + x) * 4) as usize;
                            pixels[idx] = r;
                            pixels[idx + 1] = g;
                            pixels[idx + 2] = b;
                            pixels[idx + 3] = a;
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Pixelate"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixelate() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = Pixelate::new(5);
        filter.apply(&mut pixels, 50, 50);
    }
}
