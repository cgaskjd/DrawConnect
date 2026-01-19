//! Clouds filter
//!
//! Generates procedural cloud texture using Perlin noise.

use crate::filters::Filter;

/// Clouds filter
#[derive(Debug, Clone)]
pub struct Clouds {
    /// Foreground color RGB
    pub foreground: (u8, u8, u8),
    /// Background color RGB
    pub background: (u8, u8, u8),
    /// Seed for random generation
    pub seed: u32,
    /// Scale of cloud pattern
    pub scale: f32,
}

impl Clouds {
    /// Create a new clouds filter with default black/white colors
    pub fn new(seed: u32) -> Self {
        Self {
            foreground: (0, 0, 0),
            background: (255, 255, 255),
            seed,
            scale: 4.0,
        }
    }

    // Simple pseudo-random number generator
    fn hash(&self, x: i32, y: i32) -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(self.seed as i32);
        let n = (n << 13) ^ n;
        let nn = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589));
        1.0 - (nn & 0x7fffffff) as f32 / 1073741824.0
    }

    // Smoothed noise
    fn smooth_noise(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let fx = x - x.floor();
        let fy = y - y.floor();

        // Smoothstep interpolation
        let fx = fx * fx * (3.0 - 2.0 * fx);
        let fy = fy * fy * (3.0 - 2.0 * fy);

        let n00 = self.hash(x0, y0);
        let n10 = self.hash(x0 + 1, y0);
        let n01 = self.hash(x0, y0 + 1);
        let n11 = self.hash(x0 + 1, y0 + 1);

        let nx0 = n00 * (1.0 - fx) + n10 * fx;
        let nx1 = n01 * (1.0 - fx) + n11 * fx;

        nx0 * (1.0 - fy) + nx1 * fy
    }

    // Fractal Brownian Motion noise
    fn fbm(&self, x: f32, y: f32, octaves: u32) -> f32 {
        let mut value = 0.0;
        let mut amplitude = 0.5;
        let mut frequency = 1.0;

        for _ in 0..octaves {
            value += amplitude * self.smooth_noise(x * frequency, y * frequency);
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value
    }
}

impl Default for Clouds {
    fn default() -> Self {
        Self {
            foreground: (0, 0, 0),
            background: (255, 255, 255),
            seed: 42,
            scale: 4.0,
        }
    }
}

impl Filter for Clouds {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let scale_x = self.scale / width as f32;
        let scale_y = self.scale / height as f32;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Generate cloud noise value
                let nx = x as f32 * scale_x * 10.0;
                let ny = y as f32 * scale_y * 10.0;
                let noise = self.fbm(nx, ny, 6);

                // Map noise to 0-1 range
                let t = (noise + 1.0) / 2.0;

                // Interpolate between foreground and background colors
                pixels[idx] = (self.background.0 as f32 * t + self.foreground.0 as f32 * (1.0 - t)) as u8;
                pixels[idx + 1] = (self.background.1 as f32 * t + self.foreground.1 as f32 * (1.0 - t)) as u8;
                pixels[idx + 2] = (self.background.2 as f32 * t + self.foreground.2 as f32 * (1.0 - t)) as u8;
                pixels[idx + 3] = 255;
            }
        }
    }

    fn name(&self) -> &'static str {
        "Clouds"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clouds() {
        let mut pixels = vec![0u8; 100 * 100 * 4];
        let filter = Clouds::default();
        filter.apply(&mut pixels, 100, 100);
    }
}
