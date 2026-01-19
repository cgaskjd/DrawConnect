//! Vignette filter
//!
//! Darkens or lightens the edges of an image, creating a vignette effect.

use crate::filters::Filter;

/// Vignette filter
#[derive(Debug, Clone)]
pub struct Vignette {
    /// Amount of vignette effect (-100 to 100)
    /// Negative values brighten edges, positive values darken
    pub amount: f32,
    /// Midpoint - where the vignette starts (0 to 100)
    pub midpoint: f32,
    /// Roundness of the vignette (-100 to 100)
    pub roundness: f32,
    /// Feather amount (0 to 100)
    pub feather: f32,
}

impl Vignette {
    /// Create a new vignette filter
    pub fn new(amount: f32) -> Self {
        Self {
            amount: amount.clamp(-100.0, 100.0),
            midpoint: 50.0,
            roundness: 0.0,
            feather: 50.0,
        }
    }
}

impl Default for Vignette {
    fn default() -> Self {
        Self {
            amount: -50.0,
            midpoint: 50.0,
            roundness: 0.0,
            feather: 50.0,
        }
    }
}

impl Filter for Vignette {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        if self.amount == 0.0 {
            return;
        }

        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();

        // Calculate vignette parameters
        let midpoint = self.midpoint / 100.0;
        let feather = (self.feather / 100.0).max(0.01);
        let amount = self.amount / 100.0;

        // Roundness affects the aspect ratio of the vignette
        let aspect = if self.roundness >= 0.0 {
            let r = self.roundness / 100.0;
            (1.0 - r, 1.0)
        } else {
            let r = -self.roundness / 100.0;
            (1.0, 1.0 - r)
        };

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Calculate normalized distance from center
                let dx = (x as f32 - cx) / cx * aspect.0;
                let dy = (y as f32 - cy) / cy * aspect.1;
                let dist = (dx * dx + dy * dy).sqrt();

                // Calculate vignette factor
                let start = midpoint;
                let end = midpoint + feather;

                let factor = if dist <= start {
                    0.0
                } else if dist >= end {
                    1.0
                } else {
                    let t = (dist - start) / (end - start);
                    // Smooth interpolation
                    t * t * (3.0 - 2.0 * t)
                };

                // Apply vignette
                let multiplier = if amount < 0.0 {
                    // Darken
                    1.0 + amount * factor
                } else {
                    // Brighten
                    1.0 + amount * factor
                };

                for c in 0..3 {
                    let v = pixels[idx + c] as f32 * multiplier;
                    pixels[idx + c] = v.round().clamp(0.0, 255.0) as u8;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Vignette"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vignette() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = Vignette::default();
        filter.apply(&mut pixels, 100, 100);
    }
}
