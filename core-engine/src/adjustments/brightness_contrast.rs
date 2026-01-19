//! Brightness and Contrast adjustment
//!
//! Adjusts the overall brightness and contrast of an image.

use super::Adjustment;
use crate::color::Color;

/// Brightness and Contrast adjustment
#[derive(Debug, Clone)]
pub struct BrightnessContrast {
    /// Brightness adjustment (-1.0 to 1.0)
    pub brightness: f32,
    /// Contrast adjustment (-1.0 to 1.0)
    pub contrast: f32,
}

impl BrightnessContrast {
    /// Create a new brightness/contrast adjustment
    pub fn new(brightness: f32, contrast: f32) -> Self {
        Self {
            brightness: brightness.clamp(-1.0, 1.0),
            contrast: contrast.clamp(-1.0, 1.0),
        }
    }
}

impl Default for BrightnessContrast {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
        }
    }
}

impl Adjustment for BrightnessContrast {
    fn apply_pixel(&self, color: Color) -> Color {
        // Calculate contrast factor
        // Using the standard contrast formula
        let contrast_scaled = self.contrast * 255.0;
        let factor = (259.0 * (contrast_scaled + 255.0)) / (255.0 * (259.0 - contrast_scaled));

        // Apply contrast and brightness
        // Colors are in 0.0-1.0 range
        let r = ((color.r - 0.5) * factor + 0.5 + self.brightness).clamp(0.0, 1.0);
        let g = ((color.g - 0.5) * factor + 0.5 + self.brightness).clamp(0.0, 1.0);
        let b = ((color.b - 0.5) * factor + 0.5 + self.brightness).clamp(0.0, 1.0);

        Color::from_rgba(r, g, b, color.a)
    }

    fn name(&self) -> &'static str {
        "Brightness/Contrast"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let adj = BrightnessContrast::new(0.0, 0.0);
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.01);
        assert!((result.g - 0.5).abs() < 0.01);
        assert!((result.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_brightness_increase() {
        let adj = BrightnessContrast::new(0.2, 0.0);
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!(result.r > 0.5);
        assert!(result.g > 0.5);
        assert!(result.b > 0.5);
    }

    #[test]
    fn test_contrast_increase() {
        let adj = BrightnessContrast::new(0.0, 0.5);
        let light = Color::from_rgba(0.8, 0.8, 0.8, 1.0);
        let dark = Color::from_rgba(0.2, 0.2, 0.2, 1.0);

        let light_result = adj.apply_pixel(light);
        let dark_result = adj.apply_pixel(dark);

        // Higher contrast should push light colors lighter and dark colors darker
        assert!(light_result.r > light.r);
        assert!(dark_result.r < dark.r);
    }
}
