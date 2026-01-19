//! Utility functions and helpers

use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in milliseconds
pub fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Smoothstep interpolation
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smooth interpolation (Ken Perlin's version)
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Map value from one range to another
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

/// Round to nearest multiple
pub fn round_to_multiple(value: f32, multiple: f32) -> f32 {
    (value / multiple).round() * multiple
}

/// Calculate distance between two points
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate angle between two points (in radians)
pub fn angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (y2 - y1).atan2(x2 - x1)
}

/// Degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Radians to degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Format bytes as human-readable string
pub fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Simple hash function for seeds
pub fn simple_hash(x: u32, y: u32, seed: u32) -> u32 {
    let mut h = seed;
    h ^= x;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= y;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

/// Generate pseudo-random float from hash (0.0 - 1.0)
pub fn hash_to_float(hash: u32) -> f32 {
    (hash as f32) / (u32::MAX as f32)
}

/// Image resize mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    /// Nearest neighbor (pixelated)
    Nearest,
    /// Bilinear interpolation
    Bilinear,
    /// Bicubic interpolation
    Bicubic,
    /// Lanczos resampling
    Lanczos,
}

/// Bilinear sample from image data
pub fn bilinear_sample(
    data: &[u8],
    width: u32,
    height: u32,
    x: f32,
    y: f32,
) -> [u8; 4] {
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);

    let fx = x.fract();
    let fy = y.fract();

    let get_pixel = |px: u32, py: u32| -> [f32; 4] {
        let idx = ((py * width + px) * 4) as usize;
        [
            data[idx] as f32,
            data[idx + 1] as f32,
            data[idx + 2] as f32,
            data[idx + 3] as f32,
        ]
    };

    let p00 = get_pixel(x0, y0);
    let p10 = get_pixel(x1, y0);
    let p01 = get_pixel(x0, y1);
    let p11 = get_pixel(x1, y1);

    let mut result = [0u8; 4];
    for i in 0..4 {
        let top = p00[i] * (1.0 - fx) + p10[i] * fx;
        let bottom = p01[i] * (1.0 - fx) + p11[i] * fx;
        result[i] = (top * (1.0 - fy) + bottom * fy) as u8;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 100.0, 0.5) - 50.0).abs() < 0.01);
        assert!((lerp(0.0, 100.0, 0.0) - 0.0).abs() < 0.01);
        assert!((lerp(0.0, 100.0, 1.0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_map_range() {
        assert!((map_range(0.5, 0.0, 1.0, 0.0, 100.0) - 50.0).abs() < 0.01);
        assert!((map_range(50.0, 0.0, 100.0, 0.0, 1.0) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_distance() {
        assert!((distance(0.0, 0.0, 3.0, 4.0) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_angle_conversion() {
        assert!((rad_to_deg(std::f32::consts::PI) - 180.0).abs() < 0.01);
        assert!((deg_to_rad(180.0) - std::f32::consts::PI).abs() < 0.01);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert!(format_bytes(1500).contains("KB"));
        assert!(format_bytes(1500000).contains("MB"));
    }

    #[test]
    fn test_smoothstep() {
        let mid = smoothstep(0.0, 1.0, 0.5);
        assert!(mid > 0.4 && mid < 0.6);

        assert!((smoothstep(0.0, 1.0, 0.0) - 0.0).abs() < 0.01);
        assert!((smoothstep(0.0, 1.0, 1.0) - 1.0).abs() < 0.01);
    }
}
