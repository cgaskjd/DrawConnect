//! Hue/Saturation/Lightness adjustment
//!
//! Adjusts the hue, saturation, and lightness of an image.

use super::Adjustment;
use crate::color::Color;

/// Hue/Saturation/Lightness adjustment
#[derive(Debug, Clone)]
pub struct HueSaturation {
    /// Hue shift (-180 to 180 degrees)
    pub hue: f32,
    /// Saturation adjustment (-1.0 to 1.0)
    pub saturation: f32,
    /// Lightness adjustment (-1.0 to 1.0)
    pub lightness: f32,
}

impl HueSaturation {
    /// Create a new hue/saturation adjustment
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue: hue.clamp(-180.0, 180.0),
            saturation: saturation.clamp(-1.0, 1.0),
            lightness: lightness.clamp(-1.0, 1.0),
        }
    }
}

impl Default for HueSaturation {
    fn default() -> Self {
        Self {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.0,
        }
    }
}

impl Adjustment for HueSaturation {
    fn apply_pixel(&self, color: Color) -> Color {
        // Convert RGB to HSL
        let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);

        // Apply adjustments
        let new_h = (h + self.hue / 360.0).rem_euclid(1.0);
        let new_s = (s + self.saturation).clamp(0.0, 1.0);
        let new_l = (l + self.lightness).clamp(0.0, 1.0);

        // Convert back to RGB
        let (r, g, b) = hsl_to_rgb(new_h, new_s, new_l);

        Color::from_rgba(r, g, b, color.a)
    }

    fn name(&self) -> &'static str {
        "Hue/Saturation"
    }
}

/// Convert RGB to HSL
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < 0.0001 {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < 0.0001 {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < 0.0001 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    (h, s, l)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s.abs() < 0.0001 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    (r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let adj = HueSaturation::default();
        let color = Color::from_rgba(0.5, 0.3, 0.7, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - color.r).abs() < 0.01);
        assert!((result.g - color.g).abs() < 0.01);
        assert!((result.b - color.b).abs() < 0.01);
    }

    #[test]
    fn test_desaturate() {
        let adj = HueSaturation::new(0.0, -1.0, 0.0);
        let color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
        let result = adj.apply_pixel(color);
        // Should be grayscale
        assert!((result.r - result.g).abs() < 0.01);
        assert!((result.g - result.b).abs() < 0.01);
    }
}
