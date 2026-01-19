//! Invert adjustment
//!
//! Inverts the colors of an image (negative effect).

use super::Adjustment;
use crate::color::Color;

/// Invert colors adjustment
#[derive(Debug, Clone, Default)]
pub struct Invert;

impl Invert {
    /// Create a new invert adjustment
    pub fn new() -> Self {
        Self
    }
}

impl Adjustment for Invert {
    fn apply_pixel(&self, color: Color) -> Color {
        Color::from_rgba(
            1.0 - color.r,
            1.0 - color.g,
            1.0 - color.b,
            color.a, // Keep alpha unchanged
        )
    }

    fn name(&self) -> &'static str {
        "Invert"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_black_to_white() {
        let adj = Invert::new();
        let black = Color::from_rgba(0.0, 0.0, 0.0, 1.0);
        let result = adj.apply_pixel(black);
        assert!((result.r - 1.0).abs() < 0.001);
        assert!((result.g - 1.0).abs() < 0.001);
        assert!((result.b - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_invert_white_to_black() {
        let adj = Invert::new();
        let white = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
        let result = adj.apply_pixel(white);
        assert!((result.r - 0.0).abs() < 0.001);
        assert!((result.g - 0.0).abs() < 0.001);
        assert!((result.b - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_double_invert() {
        let adj = Invert::new();
        let original = Color::from_rgba(0.3, 0.5, 0.7, 1.0);
        let inverted = adj.apply_pixel(original);
        let double_inverted = adj.apply_pixel(inverted);

        assert!((double_inverted.r - original.r).abs() < 0.001);
        assert!((double_inverted.g - original.g).abs() < 0.001);
        assert!((double_inverted.b - original.b).abs() < 0.001);
    }

    #[test]
    fn test_alpha_preserved() {
        let adj = Invert::new();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 0.5);
        let result = adj.apply_pixel(color);
        assert!((result.a - 0.5).abs() < 0.001);
    }
}
