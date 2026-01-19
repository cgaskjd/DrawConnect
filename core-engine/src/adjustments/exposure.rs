//! Exposure adjustment
//!
//! Adjusts exposure, offset, and gamma correction.

use super::Adjustment;
use crate::color::Color;

/// Exposure adjustment
#[derive(Debug, Clone)]
pub struct Exposure {
    /// Exposure adjustment in stops (-5.0 to 5.0)
    pub exposure: f32,
    /// Offset adjustment (-0.5 to 0.5)
    pub offset: f32,
    /// Gamma correction (0.1 to 10.0)
    pub gamma: f32,
}

impl Exposure {
    /// Create a new exposure adjustment
    pub fn new(exposure: f32, offset: f32, gamma: f32) -> Self {
        Self {
            exposure: exposure.clamp(-5.0, 5.0),
            offset: offset.clamp(-0.5, 0.5),
            gamma: gamma.clamp(0.1, 10.0),
        }
    }
}

impl Default for Exposure {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            offset: 0.0,
            gamma: 1.0,
        }
    }
}

impl Adjustment for Exposure {
    fn apply_pixel(&self, color: Color) -> Color {
        // Calculate exposure multiplier (2^exposure)
        let multiplier = 2.0_f32.powf(self.exposure);

        // Apply exposure and offset
        let r = color.r * multiplier + self.offset;
        let g = color.g * multiplier + self.offset;
        let b = color.b * multiplier + self.offset;

        // Apply gamma correction
        let r = r.max(0.0).powf(1.0 / self.gamma).clamp(0.0, 1.0);
        let g = g.max(0.0).powf(1.0 / self.gamma).clamp(0.0, 1.0);
        let b = b.max(0.0).powf(1.0 / self.gamma).clamp(0.0, 1.0);

        Color::from_rgba(r, g, b, color.a)
    }

    fn name(&self) -> &'static str {
        "Exposure"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let adj = Exposure::default();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_increase_exposure() {
        let adj = Exposure::new(1.0, 0.0, 1.0);
        let color = Color::from_rgba(0.25, 0.25, 0.25, 1.0);
        let result = adj.apply_pixel(color);
        // +1 stop should double the value (clamped to 0.5)
        assert!((result.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_decrease_exposure() {
        let adj = Exposure::new(-1.0, 0.0, 1.0);
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        // -1 stop should halve the value
        assert!((result.r - 0.25).abs() < 0.01);
    }
}
