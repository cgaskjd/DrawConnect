//! Brush texture module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Brush texture for custom brush tips
#[derive(Debug, Clone)]
pub struct BrushTexture {
    /// Unique texture ID
    pub id: Uuid,
    /// Texture name
    pub name: String,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Grayscale texture data (0.0 = transparent, 1.0 = opaque)
    pub data: Vec<f32>,
    /// Texture mode
    pub mode: TextureMode,
}

/// How the texture is applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureMode {
    /// Multiply with brush shape
    Multiply,
    /// Subtract from brush shape
    Subtract,
    /// Replace brush shape
    Replace,
    /// Use as height map
    HeightMap,
}

impl Default for TextureMode {
    fn default() -> Self {
        Self::Multiply
    }
}

impl BrushTexture {
    /// Create a new brush texture from grayscale data
    pub fn new(name: impl Into<String>, width: u32, height: u32, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            width,
            height,
            data,
            mode: TextureMode::default(),
        }
    }

    /// Create from RGBA image data (converts to grayscale)
    pub fn from_rgba(name: impl Into<String>, width: u32, height: u32, rgba: &[u8]) -> Self {
        let pixel_count = (width * height) as usize;
        assert_eq!(rgba.len(), pixel_count * 4);

        let data: Vec<f32> = (0..pixel_count)
            .map(|i| {
                let r = rgba[i * 4] as f32 / 255.0;
                let g = rgba[i * 4 + 1] as f32 / 255.0;
                let b = rgba[i * 4 + 2] as f32 / 255.0;
                let a = rgba[i * 4 + 3] as f32 / 255.0;
                // Convert to grayscale using luminance
                (0.299 * r + 0.587 * g + 0.114 * b) * a
            })
            .collect();

        Self::new(name, width, height, data)
    }

    /// Sample the texture at normalized coordinates (0.0 - 1.0)
    pub fn sample(&self, u: f32, v: f32) -> f32 {
        let u = u.fract();
        let v = v.fract();

        let x = (u * (self.width - 1) as f32) as u32;
        let y = (v * (self.height - 1) as f32) as u32;

        self.data[(y * self.width + x) as usize]
    }

    /// Sample with bilinear interpolation
    pub fn sample_bilinear(&self, u: f32, v: f32) -> f32 {
        let u = u.fract() * (self.width - 1) as f32;
        let v = v.fract() * (self.height - 1) as f32;

        let x0 = u.floor() as u32;
        let y0 = v.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = u.fract();
        let fy = v.fract();

        let v00 = self.data[(y0 * self.width + x0) as usize];
        let v10 = self.data[(y0 * self.width + x1) as usize];
        let v01 = self.data[(y1 * self.width + x0) as usize];
        let v11 = self.data[(y1 * self.width + x1) as usize];

        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;

        v0 * (1.0 - fy) + v1 * fy
    }

    /// Generate a noise texture
    pub fn noise(name: impl Into<String>, size: u32, seed: u64) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut data = Vec::with_capacity((size * size) as usize);

        for y in 0..size {
            for x in 0..size {
                let mut hasher = DefaultHasher::new();
                (x, y, seed).hash(&mut hasher);
                let hash = hasher.finish();
                let value = (hash as f32) / (u64::MAX as f32);
                data.push(value);
            }
        }

        Self::new(name, size, size, data)
    }

    /// Generate a paper texture
    pub fn paper(name: impl Into<String>, size: u32) -> Self {
        let mut data = Vec::with_capacity((size * size) as usize);

        for y in 0..size {
            for x in 0..size {
                // Simple procedural paper texture
                let nx = x as f32 / size as f32 * 10.0;
                let ny = y as f32 / size as f32 * 10.0;
                let value = 0.8 + 0.2 * ((nx * 7.0).sin() * (ny * 11.0).cos()).abs();
                data.push(value.clamp(0.0, 1.0));
            }
        }

        Self::new(name, size, size, data)
    }

    /// Apply texture to a brush stamp
    pub fn apply_to_stamp(&self, stamp: &mut [f32], stamp_size: u32) {
        let scale_x = self.width as f32 / stamp_size as f32;
        let scale_y = self.height as f32 / stamp_size as f32;

        for y in 0..stamp_size {
            for x in 0..stamp_size {
                let u = x as f32 * scale_x / self.width as f32;
                let v = y as f32 * scale_y / self.height as f32;
                let tex_value = self.sample_bilinear(u, v);
                let idx = (y * stamp_size + x) as usize;

                match self.mode {
                    TextureMode::Multiply => {
                        stamp[idx] *= tex_value;
                    }
                    TextureMode::Subtract => {
                        stamp[idx] = (stamp[idx] - tex_value).max(0.0);
                    }
                    TextureMode::Replace => {
                        stamp[idx] = tex_value * stamp[idx].signum();
                    }
                    TextureMode::HeightMap => {
                        // Height map mode - affects edge behavior
                        stamp[idx] *= tex_value.powf(0.5);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_creation() {
        let data = vec![0.5; 16];
        let texture = BrushTexture::new("Test", 4, 4, data);
        assert_eq!(texture.width, 4);
        assert_eq!(texture.height, 4);
    }

    #[test]
    fn test_texture_sampling() {
        let mut data = vec![0.0; 4];
        data[0] = 1.0; // Top-left
        data[3] = 1.0; // Bottom-right

        let texture = BrushTexture::new("Test", 2, 2, data);

        assert!((texture.sample(0.0, 0.0) - 1.0).abs() < 0.01);
        assert!((texture.sample(1.0, 1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_noise_texture() {
        let texture = BrushTexture::noise("Noise", 32, 12345);
        assert_eq!(texture.width, 32);
        assert_eq!(texture.data.len(), 32 * 32);
    }
}
