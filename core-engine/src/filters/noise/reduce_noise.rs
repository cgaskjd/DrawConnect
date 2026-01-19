//! Reduce Noise filter
//!
//! Basic noise reduction using bilateral-like filtering.

use crate::filters::Filter;

/// Reduce Noise filter
#[derive(Debug, Clone)]
pub struct ReduceNoise {
    /// Strength (0.0 to 1.0)
    pub strength: f32,
    /// Preserve details (0.0 to 1.0)
    pub preserve_details: f32,
}

impl ReduceNoise {
    /// Create a new reduce noise filter
    pub fn new(strength: f32, preserve_details: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            preserve_details: preserve_details.clamp(0.0, 1.0),
        }
    }
}

impl Default for ReduceNoise {
    fn default() -> Self {
        Self {
            strength: 0.5,
            preserve_details: 0.5,
        }
    }
}

impl Filter for ReduceNoise {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();
        let radius = ((self.strength * 3.0) as i32).max(1);
        let color_sigma = 30.0 + (1.0 - self.preserve_details) * 70.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                for c in 0..3 {
                    let center_val = original[idx + c] as f32;
                    let mut sum = 0.0f32;
                    let mut weight_sum = 0.0f32;

                    for dy in -radius..=radius {
                        for dx in -radius..=radius {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nidx = ((ny as u32 * width + nx as u32) * 4) as usize;
                                let neighbor_val = original[nidx + c] as f32;

                                // Spatial weight (simplified)
                                let spatial_dist = (dx * dx + dy * dy) as f32;
                                let spatial_weight = (-spatial_dist / (2.0 * radius as f32 * radius as f32)).exp();

                                // Color/range weight
                                let color_dist = (center_val - neighbor_val).abs();
                                let color_weight = (-color_dist * color_dist / (2.0 * color_sigma * color_sigma)).exp();

                                let weight = spatial_weight * color_weight;
                                sum += neighbor_val * weight;
                                weight_sum += weight;
                            }
                        }
                    }

                    if weight_sum > 0.0 {
                        pixels[idx + c] = (sum / weight_sum) as u8;
                    }
                }
                // Alpha unchanged
            }
        }
    }

    fn name(&self) -> &'static str {
        "Reduce Noise"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_noise() {
        let mut pixels = vec![128u8; 30 * 30 * 4];
        let filter = ReduceNoise::new(0.5, 0.5);
        filter.apply(&mut pixels, 30, 30);
    }
}
