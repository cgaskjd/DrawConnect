//! Levels adjustment
//!
//! Adjusts input and output levels for tonal range control.

use super::{Adjustment, CurveChannel};
use crate::color::Color;

/// Levels adjustment
#[derive(Debug, Clone)]
pub struct Levels {
    /// Input black point (0.0 to 1.0)
    pub input_black: f32,
    /// Input white point (0.0 to 1.0)
    pub input_white: f32,
    /// Gamma (midtone) adjustment (0.1 to 10.0)
    pub gamma: f32,
    /// Output black point (0.0 to 1.0)
    pub output_black: f32,
    /// Output white point (0.0 to 1.0)
    pub output_white: f32,
    /// Which channel to adjust
    pub channel: CurveChannel,
}

impl Levels {
    /// Create a new levels adjustment
    pub fn new(
        input_black: f32,
        input_white: f32,
        gamma: f32,
        output_black: f32,
        output_white: f32,
        channel: CurveChannel,
    ) -> Self {
        Self {
            input_black: input_black.clamp(0.0, 1.0),
            input_white: input_white.clamp(0.0, 1.0).max(input_black + 0.001),
            gamma: gamma.clamp(0.1, 10.0),
            output_black: output_black.clamp(0.0, 1.0),
            output_white: output_white.clamp(0.0, 1.0),
            channel,
        }
    }

    fn apply_levels(&self, value: f32) -> f32 {
        // Step 1: Apply input levels
        let input_range = self.input_white - self.input_black;
        let normalized = ((value - self.input_black) / input_range).clamp(0.0, 1.0);

        // Step 2: Apply gamma correction
        let gamma_corrected = normalized.powf(1.0 / self.gamma);

        // Step 3: Apply output levels
        let output_range = self.output_white - self.output_black;
        self.output_black + gamma_corrected * output_range
    }
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            input_black: 0.0,
            input_white: 1.0,
            gamma: 1.0,
            output_black: 0.0,
            output_white: 1.0,
            channel: CurveChannel::RGB,
        }
    }
}

impl Adjustment for Levels {
    fn apply_pixel(&self, color: Color) -> Color {
        match self.channel {
            CurveChannel::RGB => {
                Color::from_rgba(
                    self.apply_levels(color.r),
                    self.apply_levels(color.g),
                    self.apply_levels(color.b),
                    color.a,
                )
            }
            CurveChannel::Red => {
                Color::from_rgba(
                    self.apply_levels(color.r),
                    color.g,
                    color.b,
                    color.a,
                )
            }
            CurveChannel::Green => {
                Color::from_rgba(
                    color.r,
                    self.apply_levels(color.g),
                    color.b,
                    color.a,
                )
            }
            CurveChannel::Blue => {
                Color::from_rgba(
                    color.r,
                    color.g,
                    self.apply_levels(color.b),
                    color.a,
                )
            }
        }
    }

    fn name(&self) -> &'static str {
        "Levels"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_no_change() {
        let adj = Levels::default();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_input_black_lift() {
        let adj = Levels::new(0.2, 1.0, 1.0, 0.0, 1.0, CurveChannel::RGB);
        let color = Color::from_rgba(0.2, 0.2, 0.2, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.0).abs() < 0.01);
    }
}
