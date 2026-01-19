//! Crop transform
//!
//! Crop images to a specified region.

use super::{ImageData, TransformError, TransformResult};

/// Crop region
#[derive(Debug, Clone, Copy)]
pub struct CropRegion {
    /// X position of top-left corner
    pub x: u32,
    /// Y position of top-left corner
    pub y: u32,
    /// Width of crop region
    pub width: u32,
    /// Height of crop region
    pub height: u32,
}

impl CropRegion {
    /// Create a new crop region
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if region is valid for given image dimensions
    pub fn is_valid(&self, img_width: u32, img_height: u32) -> bool {
        self.width > 0
            && self.height > 0
            && self.x < img_width
            && self.y < img_height
            && self.x + self.width <= img_width
            && self.y + self.height <= img_height
    }

    /// Clamp region to fit within image bounds
    pub fn clamp_to_bounds(self, img_width: u32, img_height: u32) -> Self {
        let x = self.x.min(img_width.saturating_sub(1));
        let y = self.y.min(img_height.saturating_sub(1));
        let max_width = img_width.saturating_sub(x);
        let max_height = img_height.saturating_sub(y);
        let width = self.width.min(max_width).max(1);
        let height = self.height.min(max_height).max(1);
        Self { x, y, width, height }
    }
}

/// Crop image to specified region
pub fn crop_image(image: &ImageData, region: CropRegion) -> TransformResult<ImageData> {
    // Validate region
    if !region.is_valid(image.width, image.height) {
        return Err(TransformError::InvalidCropRegion(format!(
            "Region ({}, {}, {}x{}) is out of bounds for image {}x{}",
            region.x, region.y, region.width, region.height, image.width, image.height
        )));
    }

    let mut result = ImageData::new(region.width, region.height);

    for y in 0..region.height {
        for x in 0..region.width {
            let src_x = region.x + x;
            let src_y = region.y + y;
            let color = image.get_pixel(src_x, src_y);
            result.set_pixel(x, y, color);
        }
    }

    Ok(result)
}

/// Crop image with automatic bounds clamping
pub fn crop_image_safe(image: &ImageData, mut region: CropRegion) -> ImageData {
    region = region.clamp_to_bounds(image.width, image.height);

    let mut result = ImageData::new(region.width, region.height);

    for y in 0..region.height {
        for x in 0..region.width {
            let src_x = region.x + x;
            let src_y = region.y + y;
            let color = image.get_pixel(src_x, src_y);
            result.set_pixel(x, y, color);
        }
    }

    result
}

/// Crop to content (remove transparent borders)
pub fn crop_to_content(image: &ImageData, threshold: f32) -> TransformResult<ImageData> {
    let mut min_x = image.width;
    let mut max_x = 0u32;
    let mut min_y = image.height;
    let mut max_y = 0u32;

    // Find bounds of non-transparent content
    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            if color.a > threshold {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    // Check if any content was found
    if min_x > max_x || min_y > max_y {
        // No content found, return 1x1 transparent image
        return Ok(ImageData::new(1, 1));
    }

    let region = CropRegion::new(
        min_x,
        min_y,
        max_x - min_x + 1,
        max_y - min_y + 1,
    );

    crop_image(image, region)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_crop_basic() {
        let mut img = ImageData::new(100, 100);
        img.set_pixel(50, 50, Color::new(1.0, 0.0, 0.0, 1.0));

        let region = CropRegion::new(40, 40, 20, 20);
        let cropped = crop_image(&img, region).unwrap();

        assert_eq!(cropped.width, 20);
        assert_eq!(cropped.height, 20);

        let pixel = cropped.get_pixel(10, 10);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_crop_invalid_region() {
        let img = ImageData::new(100, 100);
        let region = CropRegion::new(90, 90, 20, 20); // Out of bounds
        let result = crop_image(&img, region);
        assert!(result.is_err());
    }

    #[test]
    fn test_crop_safe() {
        let img = ImageData::new(100, 100);
        let region = CropRegion::new(90, 90, 50, 50); // Would be out of bounds
        let cropped = crop_image_safe(&img, region);

        // Should be clamped to valid region
        assert_eq!(cropped.width, 10);
        assert_eq!(cropped.height, 10);
    }

    #[test]
    fn test_crop_to_content() {
        let mut img = ImageData::new(100, 100);
        // Draw a small rectangle in the center
        for y in 40..60 {
            for x in 30..70 {
                img.set_pixel(x, y, Color::new(1.0, 1.0, 1.0, 1.0));
            }
        }

        let cropped = crop_to_content(&img, 0.5).unwrap();
        assert_eq!(cropped.width, 40); // 70 - 30
        assert_eq!(cropped.height, 20); // 60 - 40
    }

    #[test]
    fn test_crop_empty_content() {
        let img = ImageData::new(100, 100); // All transparent
        let cropped = crop_to_content(&img, 0.5).unwrap();
        assert_eq!(cropped.width, 1);
        assert_eq!(cropped.height, 1);
    }

    #[test]
    fn test_region_validity() {
        let valid = CropRegion::new(10, 10, 50, 50);
        assert!(valid.is_valid(100, 100));

        let invalid = CropRegion::new(90, 90, 20, 20);
        assert!(!invalid.is_valid(100, 100));

        let zero_size = CropRegion::new(0, 0, 0, 50);
        assert!(!zero_size.is_valid(100, 100));
    }
}
