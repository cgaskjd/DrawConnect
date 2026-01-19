//! Oil Paint filter
//!
//! Creates an oil painting effect.

use crate::filters::Filter;

/// Oil Paint filter
#[derive(Debug, Clone)]
pub struct OilPaint {
    /// Radius for neighborhood sampling
    pub radius: u32,
    /// Number of intensity levels
    pub levels: u32,
}

impl OilPaint {
    /// Create a new oil paint filter
    pub fn new(radius: u32, levels: u32) -> Self {
        Self {
            radius: radius.clamp(1, 10),
            levels: levels.clamp(2, 256),
        }
    }
}

impl Default for OilPaint {
    fn default() -> Self {
        Self {
            radius: 4,
            levels: 20,
        }
    }
}

impl Filter for OilPaint {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();
        let r = self.radius as i32;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Intensity histogram
                let mut intensity_count = vec![0u32; self.levels as usize];
                let mut r_sum = vec![0u32; self.levels as usize];
                let mut g_sum = vec![0u32; self.levels as usize];
                let mut b_sum = vec![0u32; self.levels as usize];

                // Sample neighborhood
                for dy in -r..=r {
                    for dx in -r..=r {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let nidx = ((ny as u32 * width + nx as u32) * 4) as usize;

                            let nr = original[nidx] as u32;
                            let ng = original[nidx + 1] as u32;
                            let nb = original[nidx + 2] as u32;

                            // Calculate intensity and map to level
                            let intensity = (nr + ng + nb) / 3;
                            let level = (intensity * (self.levels - 1) / 255) as usize;
                            let level = level.min(self.levels as usize - 1);

                            intensity_count[level] += 1;
                            r_sum[level] += nr;
                            g_sum[level] += ng;
                            b_sum[level] += nb;
                        }
                    }
                }

                // Find most common intensity level
                let mut max_count = 0;
                let mut max_level = 0;
                for (level, &count) in intensity_count.iter().enumerate() {
                    if count > max_count {
                        max_count = count;
                        max_level = level;
                    }
                }

                // Set pixel to average color of most common intensity
                if max_count > 0 {
                    pixels[idx] = (r_sum[max_level] / max_count) as u8;
                    pixels[idx + 1] = (g_sum[max_level] / max_count) as u8;
                    pixels[idx + 2] = (b_sum[max_level] / max_count) as u8;
                }
                // Alpha unchanged
            }
        }
    }

    fn name(&self) -> &'static str {
        "Oil Paint"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oil_paint() {
        let mut pixels = vec![128u8; 30 * 30 * 4];
        let filter = OilPaint::new(2, 10);
        filter.apply(&mut pixels, 30, 30);
    }
}
