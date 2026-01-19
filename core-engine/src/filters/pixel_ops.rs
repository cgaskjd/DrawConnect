//! Pixel operation utilities
//!
//! Common pixel manipulation functions used across multiple filters.

/// Calculate pixel index for RGBA buffer
#[inline]
pub fn pixel_index(x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * 4) as usize
}

/// Bilinear sample a single channel from pixel buffer
#[inline]
fn bilinear_sample_channel(
    data: &[u8],
    width: u32,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    fx: f32,
    fy: f32,
    channel: usize,
) -> f32 {
    let i00 = pixel_index(x0, y0, width) + channel;
    let i10 = pixel_index(x1, y0, width) + channel;
    let i01 = pixel_index(x0, y1, width) + channel;
    let i11 = pixel_index(x1, y1, width) + channel;

    let v00 = data[i00] as f32;
    let v10 = data[i10] as f32;
    let v01 = data[i01] as f32;
    let v11 = data[i11] as f32;

    v00 * (1.0 - fx) * (1.0 - fy)
        + v10 * fx * (1.0 - fy)
        + v01 * (1.0 - fx) * fy
        + v11 * fx * fy
}

/// Bilinear sample RGBA pixel from source buffer
///
/// # Arguments
/// * `data` - Source pixel buffer (RGBA format)
/// * `width` - Image width
/// * `height` - Image height
/// * `x` - X coordinate (can be fractional)
/// * `y` - Y coordinate (can be fractional)
///
/// # Returns
/// RGBA values as [u8; 4], or None if out of bounds
#[inline]
pub fn bilinear_sample_rgba(
    data: &[u8],
    width: u32,
    height: u32,
    x: f32,
    y: f32,
) -> Option<[u8; 4]> {
    if x < 0.0 || x >= width as f32 - 1.0 || y < 0.0 || y >= height as f32 - 1.0 {
        return None;
    }

    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let fx = x - x.floor();
    let fy = y - y.floor();

    let mut result = [0u8; 4];
    for c in 0..4 {
        let v = bilinear_sample_channel(data, width, x0, y0, x1, y1, fx, fy, c);
        result[c] = v.round().clamp(0.0, 255.0) as u8;
    }

    Some(result)
}

/// Bilinear sample RGBA pixel with clamping (no bounds check)
///
/// Clamps coordinates to valid range instead of returning None.
/// Use this when you need a value even for edge pixels.
#[inline]
pub fn bilinear_sample_rgba_clamped(
    data: &[u8],
    width: u32,
    height: u32,
    x: f32,
    y: f32,
) -> [u8; 4] {
    let sx = x.clamp(0.0, width as f32 - 1.0);
    let sy = y.clamp(0.0, height as f32 - 1.0);

    let x0 = sx.floor() as u32;
    let y0 = sy.floor() as u32;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);

    let fx = sx - sx.floor();
    let fy = sy - sy.floor();

    let mut result = [0u8; 4];
    for c in 0..4 {
        let v = bilinear_sample_channel(data, width, x0, y0, x1, y1, fx, fy, c);
        result[c] = v.round().clamp(0.0, 255.0) as u8;
    }

    result
}

/// Write RGBA pixel to destination buffer
#[inline]
pub fn write_pixel(pixels: &mut [u8], idx: usize, rgba: [u8; 4]) {
    pixels[idx] = rgba[0];
    pixels[idx + 1] = rgba[1];
    pixels[idx + 2] = rgba[2];
    pixels[idx + 3] = rgba[3];
}

/// Write transparent pixel to destination buffer
#[inline]
pub fn write_transparent(pixels: &mut [u8], idx: usize) {
    pixels[idx] = 0;
    pixels[idx + 1] = 0;
    pixels[idx + 2] = 0;
    pixels[idx + 3] = 0;
}

/// Calculate luminance from RGB values
#[inline]
pub fn luminance(r: u8, g: u8, b: u8) -> f32 {
    0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_index() {
        assert_eq!(pixel_index(0, 0, 100), 0);
        assert_eq!(pixel_index(1, 0, 100), 4);
        assert_eq!(pixel_index(0, 1, 100), 400);
    }

    #[test]
    fn test_bilinear_sample() {
        // Create a 2x2 test image
        let data = vec![
            255, 0, 0, 255,   // (0,0) red
            0, 255, 0, 255,   // (1,0) green
            0, 0, 255, 255,   // (0,1) blue
            255, 255, 0, 255, // (1,1) yellow
        ];

        // Sample at center should blend all four
        let result = bilinear_sample_rgba(&data, 2, 2, 0.5, 0.5);
        assert!(result.is_some());
    }

    #[test]
    fn test_luminance() {
        assert_eq!(luminance(255, 255, 255), 255.0);
        assert_eq!(luminance(0, 0, 0), 0.0);
    }
}
