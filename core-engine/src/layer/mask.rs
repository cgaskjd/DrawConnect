//! Layer mask functionality

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mask mode - how the mask affects the layer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaskMode {
    /// Grayscale mask (white = visible, black = hidden)
    Grayscale,
    /// Luminosity-based mask
    Luminosity,
    /// Alpha channel mask
    Alpha,
}

impl Default for MaskMode {
    fn default() -> Self {
        Self::Grayscale
    }
}

/// Layer mask structure
#[derive(Debug, Clone)]
pub struct LayerMask {
    /// Unique mask identifier
    pub id: Uuid,
    /// Mask width
    pub width: u32,
    /// Mask height
    pub height: u32,
    /// Mask data (grayscale, 0.0 = transparent, 1.0 = opaque)
    pub data: Vec<f32>,
    /// Mask mode
    pub mode: MaskMode,
    /// Whether mask is enabled
    pub enabled: bool,
    /// Invert the mask
    pub inverted: bool,
    /// Mask density (overall strength)
    pub density: f32,
    /// Feather amount
    pub feather: f32,
    /// Link mask to layer (move together)
    pub linked: bool,
}

impl LayerMask {
    /// Create a new layer mask (fully opaque)
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            width,
            height,
            data: vec![1.0; (width * height) as usize],
            mode: MaskMode::default(),
            enabled: true,
            inverted: false,
            density: 1.0,
            feather: 0.0,
            linked: true,
        }
    }

    /// Create a mask from grayscale image data
    pub fn from_grayscale(width: u32, height: u32, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Self {
            data,
            ..Self::new(width, height)
        }
    }

    /// Create a mask from u8 grayscale data
    pub fn from_u8(width: u32, height: u32, data: &[u8]) -> Self {
        let float_data: Vec<f32> = data.iter().map(|&v| v as f32 / 255.0).collect();
        Self::from_grayscale(width, height, float_data)
    }

    /// Get mask value at position
    pub fn get(&self, x: u32, y: u32) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }

        let idx = (y * self.width + x) as usize;
        let mut value = self.data[idx];

        if self.inverted {
            value = 1.0 - value;
        }

        value * self.density
    }

    /// Set mask value at position
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (y * self.width + x) as usize;
        self.data[idx] = value.clamp(0.0, 1.0);
    }

    /// Fill the entire mask with a value
    pub fn fill(&mut self, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        self.data.fill(clamped);
    }

    /// Invert the mask
    pub fn invert(&mut self) {
        for value in &mut self.data {
            *value = 1.0 - *value;
        }
    }

    /// Apply Gaussian blur to the mask
    pub fn blur(&mut self, radius: f32) {
        if radius <= 0.0 {
            return;
        }

        let kernel_size = (radius * 3.0).ceil() as usize * 2 + 1;
        let kernel = Self::gaussian_kernel(radius, kernel_size);
        let half = kernel_size / 2;

        // Horizontal pass
        let mut temp = vec![0.0f32; self.data.len()];
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                let mut weight_sum = 0.0;

                for k in 0..kernel_size {
                    let sx = x as i32 + k as i32 - half as i32;
                    if sx >= 0 && sx < self.width as i32 {
                        let idx = (y * self.width + sx as u32) as usize;
                        sum += self.data[idx] * kernel[k];
                        weight_sum += kernel[k];
                    }
                }

                temp[(y * self.width + x) as usize] = sum / weight_sum;
            }
        }

        // Vertical pass
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                let mut weight_sum = 0.0;

                for k in 0..kernel_size {
                    let sy = y as i32 + k as i32 - half as i32;
                    if sy >= 0 && sy < self.height as i32 {
                        let idx = (sy as u32 * self.width + x) as usize;
                        sum += temp[idx] * kernel[k];
                        weight_sum += kernel[k];
                    }
                }

                self.data[(y * self.width + x) as usize] = sum / weight_sum;
            }
        }
    }

    /// Generate Gaussian kernel
    fn gaussian_kernel(sigma: f32, size: usize) -> Vec<f32> {
        let mut kernel = vec![0.0f32; size];
        let half = size / 2;
        let sigma_sq = sigma * sigma;

        for i in 0..size {
            let x = i as f32 - half as f32;
            kernel[i] = (-x * x / (2.0 * sigma_sq)).exp();
        }

        // Normalize
        let sum: f32 = kernel.iter().sum();
        for k in &mut kernel {
            *k /= sum;
        }

        kernel
    }

    /// Apply feathering to the mask edges
    pub fn apply_feather(&mut self) {
        if self.feather > 0.0 {
            self.blur(self.feather);
        }
    }

    /// Create mask from selection (threshold-based)
    pub fn from_selection(
        width: u32,
        height: u32,
        pixels: &[u8],
        threshold: f32,
    ) -> Self {
        let pixel_count = (width * height) as usize;
        let mut data = vec![0.0f32; pixel_count];

        for i in 0..pixel_count {
            let alpha = pixels[i * 4 + 3] as f32 / 255.0;
            data[i] = if alpha >= threshold { 1.0 } else { 0.0 };
        }

        Self::from_grayscale(width, height, data)
    }

    /// Resize the mask
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let mut new_data = vec![0.0f32; (new_width * new_height) as usize];

        let scale_x = self.width as f32 / new_width as f32;
        let scale_y = self.height as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * scale_x) as u32;
                let src_y = (y as f32 * scale_y) as u32;

                let src_idx = (src_y.min(self.height - 1) * self.width
                    + src_x.min(self.width - 1)) as usize;
                let dst_idx = (y * new_width + x) as usize;

                new_data[dst_idx] = self.data[src_idx];
            }
        }

        self.data = new_data;
        self.width = new_width;
        self.height = new_height;
    }

    /// Toggle enabled state
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_creation() {
        let mask = LayerMask::new(100, 100);
        assert_eq!(mask.width, 100);
        assert_eq!(mask.height, 100);
        assert!(mask.enabled);
        assert!(!mask.inverted);
    }

    #[test]
    fn test_mask_get_set() {
        let mut mask = LayerMask::new(10, 10);

        mask.set(5, 5, 0.5);
        assert!((mask.get(5, 5) - 0.5).abs() < 0.01);

        mask.set(5, 5, 0.0);
        assert!((mask.get(5, 5) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_mask_invert() {
        let mut mask = LayerMask::new(10, 10);
        mask.fill(0.3);

        mask.invert();
        assert!((mask.data[0] - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_mask_inverted_get() {
        let mut mask = LayerMask::new(10, 10);
        mask.fill(0.8);
        mask.inverted = true;

        // When inverted, 0.8 becomes 0.2
        assert!((mask.get(0, 0) - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_mask_density() {
        let mut mask = LayerMask::new(10, 10);
        mask.fill(1.0);
        mask.density = 0.5;

        assert!((mask.get(0, 0) - 0.5).abs() < 0.01);
    }
}
