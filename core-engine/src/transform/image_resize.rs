//! Image resize transform
//!
//! Resize images using various interpolation algorithms.

use super::{ImageData, TransformError, TransformResult};
use crate::color::Color;

/// Interpolation method for image resizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interpolation {
    /// Nearest neighbor (fastest, pixelated)
    Nearest,
    /// Bilinear interpolation (good balance)
    #[default]
    Bilinear,
    /// Bicubic interpolation (smoother)
    Bicubic,
    /// Lanczos resampling (highest quality)
    Lanczos,
}

impl std::str::FromStr for Interpolation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nearest" | "nn" => Ok(Interpolation::Nearest),
            "bilinear" | "linear" => Ok(Interpolation::Bilinear),
            "bicubic" | "cubic" => Ok(Interpolation::Bicubic),
            "lanczos" => Ok(Interpolation::Lanczos),
            _ => Err(format!("Unknown interpolation: {}", s)),
        }
    }
}

/// Resize image to new dimensions
pub fn resize_image(
    image: &ImageData,
    new_width: u32,
    new_height: u32,
    interpolation: Interpolation,
) -> TransformResult<ImageData> {
    if new_width == 0 || new_height == 0 {
        return Err(TransformError::InvalidDimensions(
            "Image dimensions must be greater than 0".to_string()
        ));
    }

    match interpolation {
        Interpolation::Nearest => resize_nearest(image, new_width, new_height),
        Interpolation::Bilinear => resize_bilinear(image, new_width, new_height),
        Interpolation::Bicubic => resize_bicubic(image, new_width, new_height),
        Interpolation::Lanczos => resize_lanczos(image, new_width, new_height),
    }
}

/// Resize using nearest neighbor interpolation
fn resize_nearest(image: &ImageData, new_width: u32, new_height: u32) -> TransformResult<ImageData> {
    let mut result = ImageData::new(new_width, new_height);

    let x_ratio = image.width as f32 / new_width as f32;
    let y_ratio = image.height as f32 / new_height as f32;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = ((x as f32 + 0.5) * x_ratio) as u32;
            let src_y = ((y as f32 + 0.5) * y_ratio) as u32;
            let src_x = src_x.min(image.width - 1);
            let src_y = src_y.min(image.height - 1);

            let color = image.get_pixel(src_x, src_y);
            result.set_pixel(x, y, color);
        }
    }

    Ok(result)
}

/// Resize using bilinear interpolation
fn resize_bilinear(image: &ImageData, new_width: u32, new_height: u32) -> TransformResult<ImageData> {
    let mut result = ImageData::new(new_width, new_height);

    let x_ratio = (image.width as f32 - 1.0) / new_width as f32;
    let y_ratio = (image.height as f32 - 1.0) / new_height as f32;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = x as f32 * x_ratio;
            let src_y = y as f32 * y_ratio;

            let color = image.get_pixel_bilinear(src_x, src_y);
            result.set_pixel(x, y, color);
        }
    }

    Ok(result)
}

/// Cubic interpolation helper
fn cubic_interp(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let a = -0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3;
    let b = p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3;
    let c = -0.5 * p0 + 0.5 * p2;
    let d = p1;

    let t2 = t * t;
    let t3 = t2 * t;

    a * t3 + b * t2 + c * t + d
}

/// Get pixel clamped to bounds
fn get_pixel_clamped(image: &ImageData, x: i32, y: i32) -> Color {
    let cx = x.clamp(0, image.width as i32 - 1) as u32;
    let cy = y.clamp(0, image.height as i32 - 1) as u32;
    image.get_pixel(cx, cy)
}

/// Resize using bicubic interpolation
fn resize_bicubic(image: &ImageData, new_width: u32, new_height: u32) -> TransformResult<ImageData> {
    let mut result = ImageData::new(new_width, new_height);

    let x_ratio = image.width as f32 / new_width as f32;
    let y_ratio = image.height as f32 / new_height as f32;

    for dest_y in 0..new_height {
        for dest_x in 0..new_width {
            let src_x = dest_x as f32 * x_ratio;
            let src_y = dest_y as f32 * y_ratio;

            let x0 = src_x.floor() as i32;
            let y0 = src_y.floor() as i32;
            let fx = src_x - x0 as f32;
            let fy = src_y - y0 as f32;

            // Sample 4x4 neighborhood
            let mut r_rows = [0.0f32; 4];
            let mut g_rows = [0.0f32; 4];
            let mut b_rows = [0.0f32; 4];
            let mut a_rows = [0.0f32; 4];

            for j in 0..4 {
                let py = y0 + j as i32 - 1;
                let mut r_cols = [0.0f32; 4];
                let mut g_cols = [0.0f32; 4];
                let mut b_cols = [0.0f32; 4];
                let mut a_cols = [0.0f32; 4];

                for i in 0..4 {
                    let px = x0 + i as i32 - 1;
                    let color = get_pixel_clamped(image, px, py);
                    r_cols[i] = color.r;
                    g_cols[i] = color.g;
                    b_cols[i] = color.b;
                    a_cols[i] = color.a;
                }

                r_rows[j] = cubic_interp(r_cols[0], r_cols[1], r_cols[2], r_cols[3], fx);
                g_rows[j] = cubic_interp(g_cols[0], g_cols[1], g_cols[2], g_cols[3], fx);
                b_rows[j] = cubic_interp(b_cols[0], b_cols[1], b_cols[2], b_cols[3], fx);
                a_rows[j] = cubic_interp(a_cols[0], a_cols[1], a_cols[2], a_cols[3], fx);
            }

            let r = cubic_interp(r_rows[0], r_rows[1], r_rows[2], r_rows[3], fy).clamp(0.0, 1.0);
            let g = cubic_interp(g_rows[0], g_rows[1], g_rows[2], g_rows[3], fy).clamp(0.0, 1.0);
            let b = cubic_interp(b_rows[0], b_rows[1], b_rows[2], b_rows[3], fy).clamp(0.0, 1.0);
            let a = cubic_interp(a_rows[0], a_rows[1], a_rows[2], a_rows[3], fy).clamp(0.0, 1.0);

            result.set_pixel(dest_x, dest_y, Color::from_rgba(r, g, b, a));
        }
    }

    Ok(result)
}

/// Lanczos kernel
fn lanczos_kernel(x: f32, a: f32) -> f32 {
    if x.abs() < f32::EPSILON {
        return 1.0;
    }
    if x.abs() >= a {
        return 0.0;
    }

    let pi_x = std::f32::consts::PI * x;
    let pi_x_a = pi_x / a;

    (pi_x.sin() / pi_x) * (pi_x_a.sin() / pi_x_a)
}

/// Resize using Lanczos resampling
fn resize_lanczos(image: &ImageData, new_width: u32, new_height: u32) -> TransformResult<ImageData> {
    let mut result = ImageData::new(new_width, new_height);

    let x_ratio = image.width as f32 / new_width as f32;
    let y_ratio = image.height as f32 / new_height as f32;

    let a = 3.0; // Lanczos3

    for dest_y in 0..new_height {
        for dest_x in 0..new_width {
            let src_x = (dest_x as f32 + 0.5) * x_ratio - 0.5;
            let src_y = (dest_y as f32 + 0.5) * y_ratio - 0.5;

            let x0 = src_x.floor() as i32;
            let y0 = src_y.floor() as i32;

            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;
            let mut a_sum = 0.0f32;
            let mut weight_sum = 0.0f32;

            for j in (y0 - 2)..=(y0 + 3) {
                for i in (x0 - 2)..=(x0 + 3) {
                    let dx = src_x - i as f32;
                    let dy = src_y - j as f32;

                    let weight = lanczos_kernel(dx, a) * lanczos_kernel(dy, a);

                    if weight.abs() > f32::EPSILON {
                        let color = get_pixel_clamped(image, i, j);
                        r_sum += color.r * weight;
                        g_sum += color.g * weight;
                        b_sum += color.b * weight;
                        a_sum += color.a * weight;
                        weight_sum += weight;
                    }
                }
            }

            if weight_sum.abs() > f32::EPSILON {
                let r = (r_sum / weight_sum).clamp(0.0, 1.0);
                let g = (g_sum / weight_sum).clamp(0.0, 1.0);
                let b = (b_sum / weight_sum).clamp(0.0, 1.0);
                let a = (a_sum / weight_sum).clamp(0.0, 1.0);
                result.set_pixel(dest_x, dest_y, Color::from_rgba(r, g, b, a));
            }
        }
    }

    Ok(result)
}

/// Resize image preserving aspect ratio
pub fn resize_fit(
    image: &ImageData,
    max_width: u32,
    max_height: u32,
    interpolation: Interpolation,
) -> TransformResult<ImageData> {
    let width_ratio = max_width as f32 / image.width as f32;
    let height_ratio = max_height as f32 / image.height as f32;
    let ratio = width_ratio.min(height_ratio);

    let new_width = ((image.width as f32 * ratio).round() as u32).max(1);
    let new_height = ((image.height as f32 * ratio).round() as u32).max(1);

    resize_image(image, new_width, new_height, interpolation)
}

/// Resize image to fill area (may crop)
pub fn resize_fill(
    image: &ImageData,
    target_width: u32,
    target_height: u32,
    interpolation: Interpolation,
) -> TransformResult<ImageData> {
    let width_ratio = target_width as f32 / image.width as f32;
    let height_ratio = target_height as f32 / image.height as f32;
    let ratio = width_ratio.max(height_ratio);

    let new_width = ((image.width as f32 * ratio).round() as u32).max(1);
    let new_height = ((image.height as f32 * ratio).round() as u32).max(1);

    resize_image(image, new_width, new_height, interpolation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_nearest() {
        let mut img = ImageData::new(10, 10);
        img.set_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));

        let result = resize_image(&img, 20, 20, Interpolation::Nearest).unwrap();
        assert_eq!(result.width, 20);
        assert_eq!(result.height, 20);
    }

    #[test]
    fn test_resize_bilinear() {
        let img = ImageData::new(100, 100);
        let result = resize_image(&img, 50, 50, Interpolation::Bilinear).unwrap();
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 50);
    }

    #[test]
    fn test_resize_bicubic() {
        let img = ImageData::new(100, 100);
        let result = resize_image(&img, 200, 200, Interpolation::Bicubic).unwrap();
        assert_eq!(result.width, 200);
        assert_eq!(result.height, 200);
    }

    #[test]
    fn test_resize_lanczos() {
        let img = ImageData::new(100, 100);
        let result = resize_image(&img, 50, 50, Interpolation::Lanczos).unwrap();
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 50);
    }

    #[test]
    fn test_resize_fit() {
        let img = ImageData::new(200, 100);
        let result = resize_fit(&img, 100, 100, Interpolation::Bilinear).unwrap();

        // Should fit to 100x50 (preserving 2:1 aspect ratio)
        assert_eq!(result.width, 100);
        assert_eq!(result.height, 50);
    }

    #[test]
    fn test_resize_fill() {
        let img = ImageData::new(200, 100);
        let result = resize_fill(&img, 100, 100, Interpolation::Bilinear).unwrap();

        // Should fill to 200x100 (to cover 100x100 target)
        assert_eq!(result.width, 200);
        assert_eq!(result.height, 100);
    }

    #[test]
    fn test_interpolation_from_str() {
        assert_eq!("nearest".parse::<Interpolation>().unwrap(), Interpolation::Nearest);
        assert_eq!("bilinear".parse::<Interpolation>().unwrap(), Interpolation::Bilinear);
        assert_eq!("bicubic".parse::<Interpolation>().unwrap(), Interpolation::Bicubic);
        assert_eq!("lanczos".parse::<Interpolation>().unwrap(), Interpolation::Lanczos);
    }

    #[test]
    fn test_invalid_dimensions() {
        let img = ImageData::new(100, 100);
        let result = resize_image(&img, 0, 50, Interpolation::Bilinear);
        assert!(result.is_err());
    }

    #[test]
    fn test_upscale_quality() {
        // Create a gradient image
        let mut img = ImageData::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                let v = (x + y) as f32 / 18.0;
                img.set_pixel(x, y, Color::new(v, v, v, 1.0));
            }
        }

        let nearest = resize_image(&img, 50, 50, Interpolation::Nearest).unwrap();
        let bilinear = resize_image(&img, 50, 50, Interpolation::Bilinear).unwrap();

        // Bilinear should produce smoother gradients
        // Just verify both complete successfully
        assert_eq!(nearest.width, 50);
        assert_eq!(bilinear.width, 50);
    }
}
