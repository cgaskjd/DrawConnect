//! Rotation transforms
//!
//! Rotate images by 90°, 180°, or arbitrary angles.

use super::{ImageData, TransformResult};

/// Rotate image 90 degrees clockwise
pub fn rotate_90_cw(image: &ImageData) -> ImageData {
    let new_width = image.height;
    let new_height = image.width;
    let mut result = ImageData::new(new_width, new_height);

    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            // New position: (new_width - 1 - y, x)
            let new_x = new_width - 1 - y;
            let new_y = x;
            result.set_pixel(new_x, new_y, color);
        }
    }

    result
}

/// Rotate image 90 degrees counter-clockwise
pub fn rotate_90_ccw(image: &ImageData) -> ImageData {
    let new_width = image.height;
    let new_height = image.width;
    let mut result = ImageData::new(new_width, new_height);

    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            // New position: (y, new_height - 1 - x)
            let new_x = y;
            let new_y = new_height - 1 - x;
            result.set_pixel(new_x, new_y, color);
        }
    }

    result
}

/// Rotate image 180 degrees
pub fn rotate_180(image: &ImageData) -> ImageData {
    let mut result = ImageData::new(image.width, image.height);

    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            let new_x = image.width - 1 - x;
            let new_y = image.height - 1 - y;
            result.set_pixel(new_x, new_y, color);
        }
    }

    result
}

/// Rotate image by arbitrary angle (in degrees)
///
/// Uses bilinear interpolation for smooth results.
/// The resulting image may be larger to accommodate the rotated content.
pub fn rotate_arbitrary(image: &ImageData, angle: f32) -> TransformResult<ImageData> {
    let angle_rad = angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    // Calculate new dimensions to fit rotated image
    let corners = [
        (0.0, 0.0),
        (image.width as f32, 0.0),
        (0.0, image.height as f32),
        (image.width as f32, image.height as f32),
    ];

    let center_x = image.width as f32 / 2.0;
    let center_y = image.height as f32 / 2.0;

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for (x, y) in corners {
        let dx = x - center_x;
        let dy = y - center_y;
        let new_x = dx * cos_a - dy * sin_a + center_x;
        let new_y = dx * sin_a + dy * cos_a + center_y;
        min_x = min_x.min(new_x);
        max_x = max_x.max(new_x);
        min_y = min_y.min(new_y);
        max_y = max_y.max(new_y);
    }

    let new_width = (max_x - min_x).ceil() as u32;
    let new_height = (max_y - min_y).ceil() as u32;

    if new_width == 0 || new_height == 0 {
        return Ok(ImageData::new(1, 1));
    }

    let mut result = ImageData::new(new_width, new_height);

    let new_center_x = new_width as f32 / 2.0;
    let new_center_y = new_height as f32 / 2.0;

    // Inverse transform for each destination pixel
    for dest_y in 0..new_height {
        for dest_x in 0..new_width {
            // Transform from destination to source
            let dx = dest_x as f32 - new_center_x;
            let dy = dest_y as f32 - new_center_y;

            // Inverse rotation
            let src_x = dx * cos_a + dy * sin_a + center_x;
            let src_y = -dx * sin_a + dy * cos_a + center_y;

            // Check bounds and sample
            if src_x >= 0.0 && src_x < image.width as f32
               && src_y >= 0.0 && src_y < image.height as f32 {
                let color = image.get_pixel_bilinear(src_x, src_y);
                result.set_pixel(dest_x, dest_y, color);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_rotate_90_cw() {
        let mut img = ImageData::new(2, 3);
        img.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));
        img.set_pixel(1, 2, Color::new(0.0, 1.0, 0.0, 1.0));

        let rotated = rotate_90_cw(&img);
        assert_eq!(rotated.width, 3);
        assert_eq!(rotated.height, 2);
    }

    #[test]
    fn test_rotate_90_ccw() {
        let mut img = ImageData::new(2, 3);
        img.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));

        let rotated = rotate_90_ccw(&img);
        assert_eq!(rotated.width, 3);
        assert_eq!(rotated.height, 2);
    }

    #[test]
    fn test_rotate_180() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));

        let rotated = rotate_180(&img);
        let pixel = rotated.get_pixel(9, 9);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_rotate_arbitrary() {
        let img = ImageData::new(100, 100);
        let rotated = rotate_arbitrary(&img, 45.0).unwrap();
        // Rotated 45° should be larger
        assert!(rotated.width > 100 || rotated.height > 100);
    }
}
