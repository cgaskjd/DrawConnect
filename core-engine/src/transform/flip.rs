//! Flip transforms
//!
//! Flip images horizontally or vertically.

use super::ImageData;

/// Flip image horizontally (mirror along vertical axis)
pub fn flip_horizontal(image: &ImageData) -> ImageData {
    let mut result = ImageData::new(image.width, image.height);

    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            let new_x = image.width - 1 - x;
            result.set_pixel(new_x, y, color);
        }
    }

    result
}

/// Flip image vertically (mirror along horizontal axis)
pub fn flip_vertical(image: &ImageData) -> ImageData {
    let mut result = ImageData::new(image.width, image.height);

    for y in 0..image.height {
        for x in 0..image.width {
            let color = image.get_pixel(x, y);
            let new_y = image.height - 1 - y;
            result.set_pixel(x, new_y, color);
        }
    }

    result
}

/// Flip image horizontally in place
pub fn flip_horizontal_in_place(image: &mut ImageData) {
    let width = image.width;
    let height = image.height;

    for y in 0..height {
        for x in 0..width / 2 {
            let x2 = width - 1 - x;
            let idx1 = ((y * width + x) * 4) as usize;
            let idx2 = ((y * width + x2) * 4) as usize;

            // Swap pixels
            for c in 0..4 {
                image.pixels.swap(idx1 + c, idx2 + c);
            }
        }
    }
}

/// Flip image vertically in place
pub fn flip_vertical_in_place(image: &mut ImageData) {
    let width = image.width;
    let height = image.height;

    for y in 0..height / 2 {
        let y2 = height - 1 - y;
        for x in 0..width {
            let idx1 = ((y * width + x) * 4) as usize;
            let idx2 = ((y2 * width + x) * 4) as usize;

            // Swap pixels
            for c in 0..4 {
                image.pixels.swap(idx1 + c, idx2 + c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_flip_horizontal() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(0, 5, Color::new(1.0, 0.0, 0.0, 1.0));

        let flipped = flip_horizontal(&img);
        let pixel = flipped.get_pixel(9, 5);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_flip_vertical() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(5, 0, Color::new(0.0, 1.0, 0.0, 1.0));

        let flipped = flip_vertical(&img);
        let pixel = flipped.get_pixel(5, 9);
        assert!(pixel.g > 0.9);
    }

    #[test]
    fn test_flip_horizontal_in_place() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(0, 5, Color::new(1.0, 0.0, 0.0, 1.0));

        flip_horizontal_in_place(&mut img);
        let pixel = img.get_pixel(9, 5);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_flip_vertical_in_place() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(5, 0, Color::new(0.0, 1.0, 0.0, 1.0));

        flip_vertical_in_place(&mut img);
        let pixel = img.get_pixel(5, 9);
        assert!(pixel.g > 0.9);
    }

    #[test]
    fn test_double_flip_identity() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(2, 3, Color::new(1.0, 0.5, 0.25, 1.0));
        let original = img.get_pixel(2, 3);

        flip_horizontal_in_place(&mut img);
        flip_horizontal_in_place(&mut img);

        let pixel = img.get_pixel(2, 3);
        assert!((pixel.r - original.r).abs() < 0.01);
        assert!((pixel.g - original.g).abs() < 0.01);
        assert!((pixel.b - original.b).abs() < 0.01);
    }
}
