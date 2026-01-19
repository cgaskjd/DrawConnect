//! Canvas resize transform
//!
//! Resize canvas while keeping image content.

use super::{ImageData, TransformError, TransformResult};
use crate::color::Color;

/// Anchor position for canvas resize
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    /// Get offset for placing image on new canvas
    pub fn get_offset(&self, old_width: u32, old_height: u32, new_width: u32, new_height: u32) -> (i32, i32) {
        let dx = new_width as i32 - old_width as i32;
        let dy = new_height as i32 - old_height as i32;

        match self {
            Anchor::TopLeft => (0, 0),
            Anchor::TopCenter => (dx / 2, 0),
            Anchor::TopRight => (dx, 0),
            Anchor::MiddleLeft => (0, dy / 2),
            Anchor::MiddleCenter => (dx / 2, dy / 2),
            Anchor::MiddleRight => (dx, dy / 2),
            Anchor::BottomLeft => (0, dy),
            Anchor::BottomCenter => (dx / 2, dy),
            Anchor::BottomRight => (dx, dy),
        }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::MiddleCenter
    }
}

impl std::str::FromStr for Anchor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top-left" | "topleft" | "tl" => Ok(Anchor::TopLeft),
            "top-center" | "topcenter" | "top" | "tc" => Ok(Anchor::TopCenter),
            "top-right" | "topright" | "tr" => Ok(Anchor::TopRight),
            "middle-left" | "middleleft" | "left" | "ml" => Ok(Anchor::MiddleLeft),
            "middle-center" | "middlecenter" | "center" | "mc" => Ok(Anchor::MiddleCenter),
            "middle-right" | "middleright" | "right" | "mr" => Ok(Anchor::MiddleRight),
            "bottom-left" | "bottomleft" | "bl" => Ok(Anchor::BottomLeft),
            "bottom-center" | "bottomcenter" | "bottom" | "bc" => Ok(Anchor::BottomCenter),
            "bottom-right" | "bottomright" | "br" => Ok(Anchor::BottomRight),
            _ => Err(format!("Unknown anchor: {}", s)),
        }
    }
}

/// Resize canvas
///
/// Creates a new canvas of specified size and places the original image
/// according to the anchor position.
pub fn canvas_resize(
    image: &ImageData,
    new_width: u32,
    new_height: u32,
    anchor: Anchor,
    fill_color: Color,
) -> TransformResult<ImageData> {
    if new_width == 0 || new_height == 0 {
        return Err(TransformError::InvalidDimensions(
            "Canvas dimensions must be greater than 0".to_string()
        ));
    }

    let mut result = ImageData::new(new_width, new_height);

    // Fill with background color
    let (fr, fg, fb, fa) = fill_color.to_rgba8();
    for y in 0..new_height {
        for x in 0..new_width {
            let idx = ((y * new_width + x) * 4) as usize;
            result.pixels[idx] = fr;
            result.pixels[idx + 1] = fg;
            result.pixels[idx + 2] = fb;
            result.pixels[idx + 3] = fa;
        }
    }

    // Calculate offset
    let (offset_x, offset_y) = anchor.get_offset(image.width, image.height, new_width, new_height);

    // Copy original image to new canvas
    for src_y in 0..image.height {
        for src_x in 0..image.width {
            let dest_x = src_x as i32 + offset_x;
            let dest_y = src_y as i32 + offset_y;

            if dest_x >= 0 && dest_x < new_width as i32
               && dest_y >= 0 && dest_y < new_height as i32 {
                let color = image.get_pixel(src_x, src_y);
                result.set_pixel(dest_x as u32, dest_y as u32, color);
            }
        }
    }

    Ok(result)
}

/// Resize canvas with relative offset
///
/// Extends or shrinks the canvas by the specified amounts on each side.
pub fn canvas_resize_relative(
    image: &ImageData,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    fill_color: Color,
) -> TransformResult<ImageData> {
    let new_width = (image.width as i32 + left + right).max(1) as u32;
    let new_height = (image.height as i32 + top + bottom).max(1) as u32;

    let mut result = ImageData::new(new_width, new_height);

    // Fill with background color
    let (fr, fg, fb, fa) = fill_color.to_rgba8();
    for i in (0..result.pixels.len()).step_by(4) {
        result.pixels[i] = fr;
        result.pixels[i + 1] = fg;
        result.pixels[i + 2] = fb;
        result.pixels[i + 3] = fa;
    }

    // Copy original image with offset
    for src_y in 0..image.height {
        for src_x in 0..image.width {
            let dest_x = src_x as i32 + left;
            let dest_y = src_y as i32 + top;

            if dest_x >= 0 && dest_x < new_width as i32
               && dest_y >= 0 && dest_y < new_height as i32 {
                let color = image.get_pixel(src_x, src_y);
                result.set_pixel(dest_x as u32, dest_y as u32, color);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_resize_expand() {
        let mut img = ImageData::new(50, 50);
        img.set_pixel(25, 25, Color::new(1.0, 0.0, 0.0, 1.0));

        let result = canvas_resize(
            &img,
            100,
            100,
            Anchor::MiddleCenter,
            Color::transparent(),
        ).unwrap();

        assert_eq!(result.width, 100);
        assert_eq!(result.height, 100);

        // Original pixel should be at center (50, 50)
        let pixel = result.get_pixel(50, 50);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_canvas_resize_shrink() {
        let mut img = ImageData::new(100, 100);
        img.set_pixel(50, 50, Color::new(1.0, 0.0, 0.0, 1.0));

        let result = canvas_resize(
            &img,
            50,
            50,
            Anchor::MiddleCenter,
            Color::transparent(),
        ).unwrap();

        assert_eq!(result.width, 50);
        assert_eq!(result.height, 50);

        // Original pixel at (50, 50) should now be at (25, 25)
        let pixel = result.get_pixel(25, 25);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_canvas_resize_anchor_top_left() {
        let mut img = ImageData::new(50, 50);
        img.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));

        let result = canvas_resize(
            &img,
            100,
            100,
            Anchor::TopLeft,
            Color::transparent(),
        ).unwrap();

        // Pixel should stay at (0, 0)
        let pixel = result.get_pixel(0, 0);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_canvas_resize_anchor_bottom_right() {
        let mut img = ImageData::new(50, 50);
        img.set_pixel(49, 49, Color::new(1.0, 0.0, 0.0, 1.0));

        let result = canvas_resize(
            &img,
            100,
            100,
            Anchor::BottomRight,
            Color::transparent(),
        ).unwrap();

        // Pixel should be at (99, 99)
        let pixel = result.get_pixel(99, 99);
        assert!(pixel.r > 0.9);
    }

    #[test]
    fn test_canvas_resize_relative() {
        let img = ImageData::new(50, 50);

        // Add 10px on each side
        let result = canvas_resize_relative(
            &img,
            10, 10, 10, 10,
            Color::transparent(),
        ).unwrap();

        assert_eq!(result.width, 70);
        assert_eq!(result.height, 70);
    }

    #[test]
    fn test_canvas_resize_relative_negative() {
        let img = ImageData::new(100, 100);

        // Remove 10px from each side
        let result = canvas_resize_relative(
            &img,
            -10, -10, -10, -10,
            Color::transparent(),
        ).unwrap();

        assert_eq!(result.width, 80);
        assert_eq!(result.height, 80);
    }

    #[test]
    fn test_anchor_from_str() {
        assert_eq!("top-left".parse::<Anchor>().unwrap(), Anchor::TopLeft);
        assert_eq!("center".parse::<Anchor>().unwrap(), Anchor::MiddleCenter);
        assert_eq!("br".parse::<Anchor>().unwrap(), Anchor::BottomRight);
    }

    #[test]
    fn test_invalid_dimensions() {
        let img = ImageData::new(50, 50);
        let result = canvas_resize(&img, 0, 50, Anchor::MiddleCenter, Color::transparent());
        assert!(result.is_err());
    }
}
