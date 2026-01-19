//! Add Noise filter
//!
//! Adds random noise to an image.

use crate::filters::Filter;

/// Type of noise to add
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoiseType {
    /// Gaussian (normal) distribution
    Gaussian,
    /// Uniform distribution
    Uniform,
}

impl Default for NoiseType {
    fn default() -> Self {
        Self::Gaussian
    }
}

/// Add Noise filter
#[derive(Debug, Clone)]
pub struct AddNoise {
    /// Noise amount (0.0 to 1.0)
    pub amount: f32,
    /// Noise type
    pub noise_type: NoiseType,
    /// Monochrome noise
    pub monochrome: bool,
    /// Random seed
    seed: u64,
}

impl AddNoise {
    /// Create a new add noise filter
    pub fn new(amount: f32, noise_type: NoiseType, monochrome: bool) -> Self {
        Self {
            amount: amount.clamp(0.0, 1.0),
            noise_type,
            monochrome,
            seed: 12345,
        }
    }

    /// Simple pseudo-random number generator
    fn random(&self, x: u32, y: u32, c: u32) -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(c.wrapping_mul(131));
        let n = n.wrapping_mul(n).wrapping_mul(15731);
        let n = n.wrapping_add(self.seed as u32);
        let n = (n >> 13) ^ n;
        let n = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(60493)).wrapping_add(19990303)).wrapping_add(1376312589);
        ((n & 0x7fffffff) as f32) / (0x7fffffff as f32)
    }

    fn gaussian_random(&self, x: u32, y: u32, c: u32) -> f32 {
        // Box-Muller transform approximation
        let u1 = self.random(x, y, c).max(0.0001);
        let u2 = self.random(x.wrapping_add(1), y.wrapping_add(1), c);
        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * std::f32::consts::PI * u2;
        r * theta.cos() * 0.5
    }
}

impl Default for AddNoise {
    fn default() -> Self {
        Self {
            amount: 0.1,
            noise_type: NoiseType::Gaussian,
            monochrome: false,
            seed: 12345,
        }
    }
}

impl Filter for AddNoise {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let strength = self.amount * 255.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                let noise_r;
                let noise_g;
                let noise_b;

                if self.monochrome {
                    let noise = match self.noise_type {
                        NoiseType::Gaussian => self.gaussian_random(x, y, 0),
                        NoiseType::Uniform => self.random(x, y, 0) * 2.0 - 1.0,
                    };
                    noise_r = noise;
                    noise_g = noise;
                    noise_b = noise;
                } else {
                    noise_r = match self.noise_type {
                        NoiseType::Gaussian => self.gaussian_random(x, y, 0),
                        NoiseType::Uniform => self.random(x, y, 0) * 2.0 - 1.0,
                    };
                    noise_g = match self.noise_type {
                        NoiseType::Gaussian => self.gaussian_random(x, y, 1),
                        NoiseType::Uniform => self.random(x, y, 1) * 2.0 - 1.0,
                    };
                    noise_b = match self.noise_type {
                        NoiseType::Gaussian => self.gaussian_random(x, y, 2),
                        NoiseType::Uniform => self.random(x, y, 2) * 2.0 - 1.0,
                    };
                }

                pixels[idx] = (pixels[idx] as f32 + noise_r * strength).clamp(0.0, 255.0) as u8;
                pixels[idx + 1] = (pixels[idx + 1] as f32 + noise_g * strength).clamp(0.0, 255.0) as u8;
                pixels[idx + 2] = (pixels[idx + 2] as f32 + noise_b * strength).clamp(0.0, 255.0) as u8;
                // Alpha unchanged
            }
        }
    }

    fn name(&self) -> &'static str {
        "Add Noise"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_noise() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = AddNoise::new(0.1, NoiseType::Gaussian, false);
        filter.apply(&mut pixels, 50, 50);
    }
}
