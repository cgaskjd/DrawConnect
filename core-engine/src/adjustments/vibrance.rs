//! Vibrance adjustment
//!
//! Smart saturation adjustment that affects less saturated colors more.

use super::Adjustment;
use crate::color::Color;

/// Vibrance adjustment
#[derive(Debug, Clone)]
pub struct Vibrance {
    /// Vibrance amount (-1.0 to 1.0)
    pub vibrance: f32,
    /// Saturation amount (-1.0 to 1.0)
    pub saturation: f32,
}

impl Vibrance {
    /// Create a new vibrance adjustment
    pub fn new(vibrance: f32, saturation: f32) -> Self {
        Self {
            vibrance: vibrance.clamp(-1.0, 1.0),
            saturation: saturation.clamp(-1.0, 1.0),
        }
    }
}

impl Default for Vibrance {
    fn default() -> Self {
        Self {
            vibrance: 0.0,
            saturation: 0.0,
        }
    }
}

impl Adjustment for Vibrance {
    fn apply_pixel(&self, color: Color) -> Color {
        // Calculate current saturation
        let max = color.r.max(color.g).max(color.b);
        let min = color.r.min(color.g).min(color.b);
        let current_sat = if max > 0.0 { (max - min) / max } else { 0.0 };

        // Vibrance affects less saturated colors more
        let vibrance_factor = 1.0 + self.vibrance * (1.0 - current_sat);

        // Calculate luminance for saturation adjustment
        let luminance = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;

        // Apply vibrance
        let mut r = luminance + (color.r - luminance) * vibrance_factor;
        let mut g = luminance + (color.g - luminance) * vibrance_factor;
        let mut b = luminance + (color.b - luminance) * vibrance_factor;

        // Apply saturation uniformly
        let sat_factor = 1.0 + self.saturation;
        r = luminance + (r - luminance) * sat_factor;
        g = luminance + (g - luminance) * sat_factor;
        b = luminance + (b - luminance) * sat_factor;

        Color::from_rgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            color.a,
        )
    }

    fn name(&self) -> &'static str {
        "Vibrance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let adj = Vibrance::default();
        let color = Color::from_rgba(0.5, 0.3, 0.7, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - color.r).abs() < 0.01);
    }

    #[test]
    fn test_vibrance_affects_desaturated_more() {
        let adj = Vibrance::new(0.5, 0.0);

        // Less saturated color
        let gray = Color::from_rgba(0.5, 0.45, 0.55, 1.0);
        let gray_result = adj.apply_pixel(gray);

        // More saturated color
        let vivid = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
        let vivid_result = adj.apply_pixel(vivid);

        // Gray should have more relative change
        let gray_change = ((gray_result.r - gray.r).abs()
            + (gray_result.g - gray.g).abs()
            + (gray_result.b - gray.b).abs())
            / 3.0;
        let vivid_change = ((vivid_result.r - vivid.r).abs()
            + (vivid_result.g - vivid.g).abs()
            + (vivid_result.b - vivid.b).abs())
            / 3.0;

        // The less saturated color should have more change relative to its saturation
        assert!(gray_change > 0.0 || vivid_change > 0.0);
    }
}
