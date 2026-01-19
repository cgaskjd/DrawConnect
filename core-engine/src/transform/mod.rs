//! Transform module
//!
//! Image transformation tools: rotate, flip, crop, resize.

pub mod rotate;
pub mod flip;
pub mod crop;
pub mod canvas_resize;
pub mod image_resize;

pub use rotate::{rotate_90_cw, rotate_90_ccw, rotate_180, rotate_arbitrary};
pub use flip::{flip_horizontal, flip_vertical};
pub use crop::{crop_image, CropRegion};
pub use canvas_resize::{canvas_resize, Anchor};
pub use image_resize::{resize_image, Interpolation};

use crate::color::Color;

/// Result type for transform operations
pub type TransformResult<T> = Result<T, TransformError>;

/// Transform error types
#[derive(Debug, Clone)]
pub enum TransformError {
    /// Invalid dimensions
    InvalidDimensions(String),
    /// Invalid crop region
    InvalidCropRegion(String),
    /// Invalid parameters
    InvalidParameters(String),
}

impl std::fmt::Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformError::InvalidDimensions(msg) => write!(f, "Invalid dimensions: {}", msg),
            TransformError::InvalidCropRegion(msg) => write!(f, "Invalid crop region: {}", msg),
            TransformError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
        }
    }
}

impl std::error::Error for TransformError {}

/// Represents image data for transformation
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Pixel data in RGBA format
    pub pixels: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
}

impl ImageData {
    /// Create new image data
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: vec![0u8; (width * height * 4) as usize],
            width,
            height,
        }
    }

    /// Create from existing pixel data
    pub fn from_pixels(pixels: Vec<u8>, width: u32, height: u32) -> TransformResult<Self> {
        let expected_len = (width * height * 4) as usize;
        if pixels.len() != expected_len {
            return Err(TransformError::InvalidDimensions(
                format!("Expected {} bytes, got {}", expected_len, pixels.len())
            ));
        }
        Ok(Self { pixels, width, height })
    }

    /// Get pixel at position
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return Color::transparent();
        }
        let idx = ((y * self.width + x) * 4) as usize;
        Color::from_rgba8(
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        )
    }

    /// Set pixel at position
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        let (r, g, b, a) = color.to_rgba8();
        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;
    }

    /// Get pixel with bilinear interpolation
    pub fn get_pixel_bilinear(&self, x: f32, y: f32) -> Color {
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let get_safe = |px: i32, py: i32| -> Color {
            if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
                Color::transparent()
            } else {
                self.get_pixel(px as u32, py as u32)
            }
        };

        let c00 = get_safe(x0, y0);
        let c10 = get_safe(x1, y0);
        let c01 = get_safe(x0, y1);
        let c11 = get_safe(x1, y1);

        // Bilinear interpolation
        let r = c00.r * (1.0 - fx) * (1.0 - fy)
            + c10.r * fx * (1.0 - fy)
            + c01.r * (1.0 - fx) * fy
            + c11.r * fx * fy;
        let g = c00.g * (1.0 - fx) * (1.0 - fy)
            + c10.g * fx * (1.0 - fy)
            + c01.g * (1.0 - fx) * fy
            + c11.g * fx * fy;
        let b = c00.b * (1.0 - fx) * (1.0 - fy)
            + c10.b * fx * (1.0 - fy)
            + c01.b * (1.0 - fx) * fy
            + c11.b * fx * fy;
        let a = c00.a * (1.0 - fx) * (1.0 - fy)
            + c10.a * fx * (1.0 - fy)
            + c01.a * (1.0 - fx) * fy
            + c11.a * fx * fy;

        Color::from_rgba(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_data_creation() {
        let img = ImageData::new(100, 100);
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
        assert_eq!(img.pixels.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_pixel_access() {
        let mut img = ImageData::new(10, 10);
        let color = Color::new(1.0, 0.5, 0.25, 1.0);
        img.set_pixel(5, 5, color);
        let retrieved = img.get_pixel(5, 5);
        assert!((retrieved.r - color.r).abs() < 0.01);
        assert!((retrieved.g - color.g).abs() < 0.01);
        assert!((retrieved.b - color.b).abs() < 0.01);
    }
}
