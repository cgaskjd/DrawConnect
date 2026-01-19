//! Posterize adjustment
//!
//! Reduces the number of color levels in an image.

use super::Adjustment;
use crate::color::Color;

/// Posterize adjustment
#[derive(Debug, Clone)]
pub struct Posterize {
    /// Number of levels per channel (2 to 255)
    pub levels: u8,
}

impl Posterize {
    /// Create a new posterize adjustment
    pub fn new(levels: u8) -> Self {
        Self {
            levels: levels.clamp(2, 255),
        }
    }
}

impl Default for Posterize {
    fn default() -> Self {
        Self { levels: 4 }
    }
}

impl Adjustment for Posterize {
    fn apply_pixel(&self, color: Color) -> Color {
        let levels = self.levels as f32;
        let step = 1.0 / (levels - 1.0);

        // Quantize each channel to the specified number of levels
        let r = (color.r * (levels - 1.0)).round() * step;
        let g = (color.g * (levels - 1.0)).round() * step;
        let b = (color.b * (levels - 1.0)).round() * step;

        Color::from_rgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            color.a,
        )
    }

    fn name(&self) -> &'static str {
        "Posterize"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_levels() {
        let adj = Posterize::new(2);

        let dark = Color::from_rgba(0.3, 0.3, 0.3, 1.0);
        let dark_result = adj.apply_pixel(dark);
        assert!((dark_result.r - 0.0).abs() < 0.01);

        let light = Color::from_rgba(0.7, 0.7, 0.7, 1.0);
        let light_result = adj.apply_pixel(light);
        assert!((light_result.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_four_levels() {
        let adj = Posterize::new(4);
        // Levels should be: 0, 0.333, 0.667, 1.0

        let color = Color::from_rgba(0.4, 0.4, 0.4, 1.0);
        let result = adj.apply_pixel(color);
        // Should round to 0.333
        assert!((result.r - 0.333).abs() < 0.05);
    }
}
