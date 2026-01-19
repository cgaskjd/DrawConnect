//! Blend modes for layer compositing

use crate::color::Color;
use serde::{Deserialize, Serialize};

/// Layer blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    // Basic modes
    /// Normal blending
    Normal,
    /// Dissolve (random dithering)
    Dissolve,

    // Darken modes
    /// Darken - keeps darker pixels
    Darken,
    /// Multiply
    Multiply,
    /// Color Burn
    ColorBurn,
    /// Linear Burn
    LinearBurn,
    /// Darker Color
    DarkerColor,

    // Lighten modes
    /// Lighten - keeps lighter pixels
    Lighten,
    /// Screen
    Screen,
    /// Color Dodge
    ColorDodge,
    /// Linear Dodge (Add)
    LinearDodge,
    /// Lighter Color
    LighterColor,

    // Contrast modes
    /// Overlay
    Overlay,
    /// Soft Light
    SoftLight,
    /// Hard Light
    HardLight,
    /// Vivid Light
    VividLight,
    /// Linear Light
    LinearLight,
    /// Pin Light
    PinLight,
    /// Hard Mix
    HardMix,

    // Comparative modes
    /// Difference
    Difference,
    /// Exclusion
    Exclusion,
    /// Subtract
    Subtract,
    /// Divide
    Divide,

    // Composite modes
    /// Hue
    Hue,
    /// Saturation
    Saturation,
    /// Color
    Color,
    /// Luminosity
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl BlendMode {
    /// Get all blend modes
    pub fn all() -> Vec<BlendMode> {
        vec![
            BlendMode::Normal,
            BlendMode::Dissolve,
            BlendMode::Darken,
            BlendMode::Multiply,
            BlendMode::ColorBurn,
            BlendMode::LinearBurn,
            BlendMode::DarkerColor,
            BlendMode::Lighten,
            BlendMode::Screen,
            BlendMode::ColorDodge,
            BlendMode::LinearDodge,
            BlendMode::LighterColor,
            BlendMode::Overlay,
            BlendMode::SoftLight,
            BlendMode::HardLight,
            BlendMode::VividLight,
            BlendMode::LinearLight,
            BlendMode::PinLight,
            BlendMode::HardMix,
            BlendMode::Difference,
            BlendMode::Exclusion,
            BlendMode::Subtract,
            BlendMode::Divide,
            BlendMode::Hue,
            BlendMode::Saturation,
            BlendMode::Color,
            BlendMode::Luminosity,
        ]
    }

    /// Get blend mode name
    pub fn name(&self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Dissolve => "Dissolve",
            BlendMode::Darken => "Darken",
            BlendMode::Multiply => "Multiply",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::LinearBurn => "Linear Burn",
            BlendMode::DarkerColor => "Darker Color",
            BlendMode::Lighten => "Lighten",
            BlendMode::Screen => "Screen",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::LinearDodge => "Linear Dodge (Add)",
            BlendMode::LighterColor => "Lighter Color",
            BlendMode::Overlay => "Overlay",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::HardLight => "Hard Light",
            BlendMode::VividLight => "Vivid Light",
            BlendMode::LinearLight => "Linear Light",
            BlendMode::PinLight => "Pin Light",
            BlendMode::HardMix => "Hard Mix",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Subtract => "Subtract",
            BlendMode::Divide => "Divide",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Blend two colors using this blend mode
    /// `base` is the bottom layer, `blend` is the top layer
    pub fn blend(&self, base: Color, blend: Color) -> Color {
        if blend.a == 0.0 {
            return base;
        }

        let result = match self {
            BlendMode::Normal => blend,
            BlendMode::Multiply => self.blend_multiply(base, blend),
            BlendMode::Screen => self.blend_screen(base, blend),
            BlendMode::Overlay => self.blend_overlay(base, blend),
            BlendMode::Darken => self.blend_darken(base, blend),
            BlendMode::Lighten => self.blend_lighten(base, blend),
            BlendMode::ColorDodge => self.blend_color_dodge(base, blend),
            BlendMode::ColorBurn => self.blend_color_burn(base, blend),
            BlendMode::HardLight => self.blend_hard_light(base, blend),
            BlendMode::SoftLight => self.blend_soft_light(base, blend),
            BlendMode::Difference => self.blend_difference(base, blend),
            BlendMode::Exclusion => self.blend_exclusion(base, blend),
            BlendMode::LinearDodge => self.blend_linear_dodge(base, blend),
            BlendMode::LinearBurn => self.blend_linear_burn(base, blend),
            BlendMode::Subtract => self.blend_subtract(base, blend),
            BlendMode::Divide => self.blend_divide(base, blend),
            _ => blend, // Fallback to normal for unimplemented modes
        };

        // Alpha compositing
        let out_alpha = blend.a + base.a * (1.0 - blend.a);
        if out_alpha == 0.0 {
            return Color::transparent();
        }

        Color::from_rgba(
            (result.r * blend.a + base.r * base.a * (1.0 - blend.a)) / out_alpha,
            (result.g * blend.a + base.g * base.a * (1.0 - blend.a)) / out_alpha,
            (result.b * blend.a + base.b * base.a * (1.0 - blend.a)) / out_alpha,
            out_alpha,
        )
    }

    fn blend_multiply(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            base.r * blend.r,
            base.g * blend.g,
            base.b * blend.b,
            blend.a,
        )
    }

    fn blend_screen(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            1.0 - (1.0 - base.r) * (1.0 - blend.r),
            1.0 - (1.0 - base.g) * (1.0 - blend.g),
            1.0 - (1.0 - base.b) * (1.0 - blend.b),
            blend.a,
        )
    }

    fn blend_overlay(&self, base: Color, blend: Color) -> Color {
        let overlay_channel = |b: f32, s: f32| -> f32 {
            if b < 0.5 {
                2.0 * b * s
            } else {
                1.0 - 2.0 * (1.0 - b) * (1.0 - s)
            }
        };

        Color::from_rgba(
            overlay_channel(base.r, blend.r),
            overlay_channel(base.g, blend.g),
            overlay_channel(base.b, blend.b),
            blend.a,
        )
    }

    fn blend_darken(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            base.r.min(blend.r),
            base.g.min(blend.g),
            base.b.min(blend.b),
            blend.a,
        )
    }

    fn blend_lighten(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            base.r.max(blend.r),
            base.g.max(blend.g),
            base.b.max(blend.b),
            blend.a,
        )
    }

    fn blend_color_dodge(&self, base: Color, blend: Color) -> Color {
        let dodge_channel = |b: f32, s: f32| -> f32 {
            if s >= 1.0 {
                1.0
            } else {
                (b / (1.0 - s)).min(1.0)
            }
        };

        Color::from_rgba(
            dodge_channel(base.r, blend.r),
            dodge_channel(base.g, blend.g),
            dodge_channel(base.b, blend.b),
            blend.a,
        )
    }

    fn blend_color_burn(&self, base: Color, blend: Color) -> Color {
        let burn_channel = |b: f32, s: f32| -> f32 {
            if s <= 0.0 {
                0.0
            } else {
                (1.0 - (1.0 - b) / s).max(0.0)
            }
        };

        Color::from_rgba(
            burn_channel(base.r, blend.r),
            burn_channel(base.g, blend.g),
            burn_channel(base.b, blend.b),
            blend.a,
        )
    }

    fn blend_hard_light(&self, base: Color, blend: Color) -> Color {
        let hard_light_channel = |b: f32, s: f32| -> f32 {
            if s < 0.5 {
                2.0 * b * s
            } else {
                1.0 - 2.0 * (1.0 - b) * (1.0 - s)
            }
        };

        Color::from_rgba(
            hard_light_channel(base.r, blend.r),
            hard_light_channel(base.g, blend.g),
            hard_light_channel(base.b, blend.b),
            blend.a,
        )
    }

    fn blend_soft_light(&self, base: Color, blend: Color) -> Color {
        let soft_light_channel = |b: f32, s: f32| -> f32 {
            if s < 0.5 {
                b - (1.0 - 2.0 * s) * b * (1.0 - b)
            } else {
                let d = if b < 0.25 {
                    ((16.0 * b - 12.0) * b + 4.0) * b
                } else {
                    b.sqrt()
                };
                b + (2.0 * s - 1.0) * (d - b)
            }
        };

        Color::from_rgba(
            soft_light_channel(base.r, blend.r),
            soft_light_channel(base.g, blend.g),
            soft_light_channel(base.b, blend.b),
            blend.a,
        )
    }

    fn blend_difference(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            (base.r - blend.r).abs(),
            (base.g - blend.g).abs(),
            (base.b - blend.b).abs(),
            blend.a,
        )
    }

    fn blend_exclusion(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            base.r + blend.r - 2.0 * base.r * blend.r,
            base.g + blend.g - 2.0 * base.g * blend.g,
            base.b + blend.b - 2.0 * base.b * blend.b,
            blend.a,
        )
    }

    fn blend_linear_dodge(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            (base.r + blend.r).min(1.0),
            (base.g + blend.g).min(1.0),
            (base.b + blend.b).min(1.0),
            blend.a,
        )
    }

    fn blend_linear_burn(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            (base.r + blend.r - 1.0).max(0.0),
            (base.g + blend.g - 1.0).max(0.0),
            (base.b + blend.b - 1.0).max(0.0),
            blend.a,
        )
    }

    fn blend_subtract(&self, base: Color, blend: Color) -> Color {
        Color::from_rgba(
            (base.r - blend.r).max(0.0),
            (base.g - blend.g).max(0.0),
            (base.b - blend.b).max(0.0),
            blend.a,
        )
    }

    fn blend_divide(&self, base: Color, blend: Color) -> Color {
        let divide_channel = |b: f32, s: f32| -> f32 {
            if s <= 0.0 {
                1.0
            } else {
                (b / s).min(1.0)
            }
        };

        Color::from_rgba(
            divide_channel(base.r, blend.r),
            divide_channel(base.g, blend.g),
            divide_channel(base.b, blend.b),
            blend.a,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_blend() {
        let base = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
        let blend = Color::from_rgba(0.0, 1.0, 0.0, 1.0);

        let result = BlendMode::Normal.blend(base, blend);
        assert!((result.g - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_multiply_blend() {
        let base = Color::from_rgba(1.0, 0.5, 0.5, 1.0);
        let blend = Color::from_rgba(0.5, 0.5, 0.5, 1.0);

        let result = BlendMode::Multiply.blend(base, blend);
        assert!((result.r - 0.5).abs() < 0.01);
        assert!((result.g - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_screen_blend() {
        let base = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let blend = Color::from_rgba(0.5, 0.5, 0.5, 1.0);

        let result = BlendMode::Screen.blend(base, blend);
        assert!((result.r - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_blend_mode_names() {
        assert_eq!(BlendMode::Normal.name(), "Normal");
        assert_eq!(BlendMode::Multiply.name(), "Multiply");
        assert_eq!(BlendMode::Screen.name(), "Screen");
    }
}
