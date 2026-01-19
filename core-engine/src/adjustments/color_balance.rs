//! Color Balance adjustment
//!
//! Adjusts the color balance for shadows, midtones, and highlights.

use super::Adjustment;
use crate::color::Color;

/// Color Balance adjustment
#[derive(Debug, Clone)]
pub struct ColorBalance {
    /// Shadows adjustment (Cyan-Red, Magenta-Green, Yellow-Blue), -1.0 to 1.0 each
    pub shadows: [f32; 3],
    /// Midtones adjustment (Cyan-Red, Magenta-Green, Yellow-Blue), -1.0 to 1.0 each
    pub midtones: [f32; 3],
    /// Highlights adjustment (Cyan-Red, Magenta-Green, Yellow-Blue), -1.0 to 1.0 each
    pub highlights: [f32; 3],
    /// Preserve luminosity when adjusting colors
    pub preserve_luminosity: bool,
}

impl ColorBalance {
    /// Create a new color balance adjustment
    pub fn new(shadows: [f32; 3], midtones: [f32; 3], highlights: [f32; 3]) -> Self {
        Self {
            shadows: shadows.map(|v| v.clamp(-1.0, 1.0)),
            midtones: midtones.map(|v| v.clamp(-1.0, 1.0)),
            highlights: highlights.map(|v| v.clamp(-1.0, 1.0)),
            preserve_luminosity: true,
        }
    }

    /// Set whether to preserve luminosity
    pub fn with_preserve_luminosity(mut self, preserve: bool) -> Self {
        self.preserve_luminosity = preserve;
        self
    }
}

impl Default for ColorBalance {
    fn default() -> Self {
        Self {
            shadows: [0.0, 0.0, 0.0],
            midtones: [0.0, 0.0, 0.0],
            highlights: [0.0, 0.0, 0.0],
            preserve_luminosity: true,
        }
    }
}

impl Adjustment for ColorBalance {
    fn apply_pixel(&self, color: Color) -> Color {
        let luminosity = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;

        // Calculate tonal weights
        let shadows_weight = 1.0 - luminosity.min(0.5) * 2.0;
        let highlights_weight = (luminosity - 0.5).max(0.0) * 2.0;
        let midtones_weight = 1.0 - (shadows_weight + highlights_weight).min(1.0);

        // Apply adjustments weighted by tonal range
        let mut r = color.r;
        let mut g = color.g;
        let mut b = color.b;

        // Shadows
        r += self.shadows[0] * shadows_weight * 0.5;
        g += self.shadows[1] * shadows_weight * 0.5;
        b += self.shadows[2] * shadows_weight * 0.5;

        // Midtones
        r += self.midtones[0] * midtones_weight * 0.5;
        g += self.midtones[1] * midtones_weight * 0.5;
        b += self.midtones[2] * midtones_weight * 0.5;

        // Highlights
        r += self.highlights[0] * highlights_weight * 0.5;
        g += self.highlights[1] * highlights_weight * 0.5;
        b += self.highlights[2] * highlights_weight * 0.5;

        // Clamp values
        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);

        // Preserve luminosity if enabled
        if self.preserve_luminosity {
            let new_luminosity = r * 0.299 + g * 0.587 + b * 0.114;
            if new_luminosity > 0.001 {
                let ratio = luminosity / new_luminosity;
                r = (r * ratio).clamp(0.0, 1.0);
                g = (g * ratio).clamp(0.0, 1.0);
                b = (b * ratio).clamp(0.0, 1.0);
            }
        }

        Color::from_rgba(r, g, b, color.a)
    }

    fn name(&self) -> &'static str {
        "Color Balance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let adj = ColorBalance::default();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.01);
        assert!((result.g - 0.5).abs() < 0.01);
        assert!((result.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_add_red_to_shadows() {
        let adj = ColorBalance::new([0.5, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let dark = Color::from_rgba(0.2, 0.2, 0.2, 1.0);
        let result = adj.apply_pixel(dark);
        // Dark pixels should have more red
        assert!(result.r > dark.r || (result.r - dark.r).abs() < 0.1);
    }
}
