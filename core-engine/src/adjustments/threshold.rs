//! Threshold adjustment
//!
//! Converts image to pure black and white based on a threshold level.

use super::Adjustment;
use crate::color::Color;

/// Threshold adjustment
#[derive(Debug, Clone)]
pub struct Threshold {
    /// Threshold level (0 to 255)
    pub level: u8,
}

impl Threshold {
    /// Create a new threshold adjustment
    pub fn new(level: u8) -> Self {
        Self { level }
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self { level: 128 }
    }
}

impl Adjustment for Threshold {
    fn apply_pixel(&self, color: Color) -> Color {
        // Calculate luminance
        let luminance = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;

        // Convert to 0-255 scale for comparison
        let lum_255 = (luminance * 255.0) as u8;

        // Apply threshold
        let value = if lum_255 >= self.level { 1.0 } else { 0.0 };

        Color::from_rgba(value, value, value, color.a)
    }

    fn name(&self) -> &'static str {
        "Threshold"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_middle() {
        let adj = Threshold::new(128);

        let dark = Color::from_rgba(0.3, 0.3, 0.3, 1.0);
        let dark_result = adj.apply_pixel(dark);
        assert!((dark_result.r - 0.0).abs() < 0.01);

        let light = Color::from_rgba(0.7, 0.7, 0.7, 1.0);
        let light_result = adj.apply_pixel(light);
        assert!((light_result.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_threshold_low() {
        let adj = Threshold::new(50);

        // Most colors should become white
        let mid = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(mid);
        assert!((result.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_threshold_high() {
        let adj = Threshold::new(200);

        // Most colors should become black
        let mid = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(mid);
        assert!((result.r - 0.0).abs() < 0.01);
    }
}
