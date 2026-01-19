//! Black & White adjustment
//!
//! Converts image to black and white with channel mixing control.

use super::Adjustment;
use crate::color::Color;

/// Black & White adjustment with channel mixing
#[derive(Debug, Clone)]
pub struct BlackWhite {
    /// Red channel contribution (-2.0 to 3.0)
    pub red: f32,
    /// Yellow contribution (-2.0 to 3.0)
    pub yellow: f32,
    /// Green channel contribution (-2.0 to 3.0)
    pub green: f32,
    /// Cyan contribution (-2.0 to 3.0)
    pub cyan: f32,
    /// Blue channel contribution (-2.0 to 3.0)
    pub blue: f32,
    /// Magenta contribution (-2.0 to 3.0)
    pub magenta: f32,
}

impl BlackWhite {
    /// Create a new black & white adjustment
    pub fn new(red: f32, yellow: f32, green: f32, cyan: f32, blue: f32, magenta: f32) -> Self {
        Self {
            red: red.clamp(-2.0, 3.0),
            yellow: yellow.clamp(-2.0, 3.0),
            green: green.clamp(-2.0, 3.0),
            cyan: cyan.clamp(-2.0, 3.0),
            blue: blue.clamp(-2.0, 3.0),
            magenta: magenta.clamp(-2.0, 3.0),
        }
    }

    /// Create a default B&W conversion (standard luminosity)
    pub fn standard() -> Self {
        Self {
            red: 0.40,
            yellow: 0.60,
            green: 0.40,
            cyan: 0.60,
            blue: 0.20,
            magenta: 0.80,
        }
    }
}

impl Default for BlackWhite {
    fn default() -> Self {
        Self::standard()
    }
}

impl Adjustment for BlackWhite {
    fn apply_pixel(&self, color: Color) -> Color {
        // Calculate the contribution of each color channel
        // This is based on the dominant hue of the pixel

        let max = color.r.max(color.g).max(color.b);
        let min = color.r.min(color.g).min(color.b);

        // Calculate weights based on color position
        let red_weight = (color.r - color.g.max(color.b)).max(0.0);
        let green_weight = (color.g - color.r.max(color.b)).max(0.0);
        let blue_weight = (color.b - color.r.max(color.g)).max(0.0);

        // Yellow is red + green
        let yellow_weight = (color.r.min(color.g) - color.b).max(0.0);
        // Cyan is green + blue
        let cyan_weight = (color.g.min(color.b) - color.r).max(0.0);
        // Magenta is red + blue
        let magenta_weight = (color.r.min(color.b) - color.g).max(0.0);

        // Base luminosity
        let base_lum = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;

        // Calculate adjustment based on channel contributions
        let total_weight = red_weight + green_weight + blue_weight
            + yellow_weight + cyan_weight + magenta_weight + 0.001;

        let adjustment = (red_weight * self.red
            + green_weight * self.green
            + blue_weight * self.blue
            + yellow_weight * self.yellow
            + cyan_weight * self.cyan
            + magenta_weight * self.magenta)
            / total_weight;

        // Apply adjustment to luminosity
        let gray = (base_lum * (1.0 + adjustment * 0.5)).clamp(0.0, 1.0);

        Color::from_rgba(gray, gray, gray, color.a)
    }

    fn name(&self) -> &'static str {
        "Black & White"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_is_grayscale() {
        let adj = BlackWhite::default();
        let color = Color::from_rgba(0.8, 0.3, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - result.g).abs() < 0.001);
        assert!((result.g - result.b).abs() < 0.001);
    }

    #[test]
    fn test_alpha_preserved() {
        let adj = BlackWhite::default();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 0.7);
        let result = adj.apply_pixel(color);
        assert!((result.a - 0.7).abs() < 0.001);
    }
}
