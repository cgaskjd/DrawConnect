//! Photo Filter adjustment
//!
//! Applies a color filter overlay like a camera lens filter.

use super::Adjustment;
use crate::color::Color;

/// Preset filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FilterPreset {
    /// Warming filter (orange)
    Warming85,
    /// Cooling filter (blue)
    Cooling80,
    /// Warm LBA
    WarmingLBA,
    /// Cool LBB
    CoolingLBB,
    /// Underwater
    Underwater,
    /// Sepia
    Sepia,
    /// Custom color
    Custom,
}

impl Default for FilterPreset {
    fn default() -> Self {
        Self::Custom
    }
}

/// Photo Filter adjustment
#[derive(Debug, Clone)]
pub struct PhotoFilter {
    /// Filter color
    pub color: Color,
    /// Filter density (0.0 to 1.0)
    pub density: f32,
    /// Preserve luminosity
    pub preserve_luminosity: bool,
    /// Preset type (for UI reference)
    pub preset: FilterPreset,
}

impl PhotoFilter {
    /// Create a new photo filter with custom color
    pub fn new(color: Color, density: f32, preserve_luminosity: bool) -> Self {
        Self {
            color,
            density: density.clamp(0.0, 1.0),
            preserve_luminosity,
            preset: FilterPreset::Custom,
        }
    }

    /// Create a warming filter (85)
    pub fn warming() -> Self {
        Self {
            color: Color::from_hex("#EC8A00").unwrap_or(Color::from_rgba(0.93, 0.54, 0.0, 1.0)),
            density: 0.25,
            preserve_luminosity: true,
            preset: FilterPreset::Warming85,
        }
    }

    /// Create a cooling filter (80)
    pub fn cooling() -> Self {
        Self {
            color: Color::from_hex("#00B5FF").unwrap_or(Color::from_rgba(0.0, 0.71, 1.0, 1.0)),
            density: 0.25,
            preserve_luminosity: true,
            preset: FilterPreset::Cooling80,
        }
    }

    /// Create a sepia filter
    pub fn sepia() -> Self {
        Self {
            color: Color::from_hex("#AC8760").unwrap_or(Color::from_rgba(0.67, 0.53, 0.38, 1.0)),
            density: 0.4,
            preserve_luminosity: true,
            preset: FilterPreset::Sepia,
        }
    }

    /// Create an underwater filter
    pub fn underwater() -> Self {
        Self {
            color: Color::from_hex("#00C2B1").unwrap_or(Color::from_rgba(0.0, 0.76, 0.69, 1.0)),
            density: 0.3,
            preserve_luminosity: true,
            preset: FilterPreset::Underwater,
        }
    }
}

impl Default for PhotoFilter {
    fn default() -> Self {
        Self::warming()
    }
}

impl Adjustment for PhotoFilter {
    fn apply_pixel(&self, color: Color) -> Color {
        let original_lum = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;

        // Blend with filter color
        let r = color.r * (1.0 - self.density) + self.color.r * self.density;
        let g = color.g * (1.0 - self.density) + self.color.g * self.density;
        let b = color.b * (1.0 - self.density) + self.color.b * self.density;

        if self.preserve_luminosity {
            let new_lum = r * 0.299 + g * 0.587 + b * 0.114;
            if new_lum > 0.001 {
                let ratio = original_lum / new_lum;
                return Color::from_rgba(
                    (r * ratio).clamp(0.0, 1.0),
                    (g * ratio).clamp(0.0, 1.0),
                    (b * ratio).clamp(0.0, 1.0),
                    color.a,
                );
            }
        }

        Color::from_rgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            color.a,
        )
    }

    fn name(&self) -> &'static str {
        "Photo Filter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warming_filter() {
        let adj = PhotoFilter::warming();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        // Warming filter should add orange tint
        assert!(result.r > result.b);
    }

    #[test]
    fn test_cooling_filter() {
        let adj = PhotoFilter::cooling();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        // Cooling filter should add blue tint
        assert!(result.b > result.r);
    }

    #[test]
    fn test_zero_density() {
        let adj = PhotoFilter::new(Color::from_rgba(1.0, 0.0, 0.0, 1.0), 0.0, true);
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.01);
        assert!((result.g - 0.5).abs() < 0.01);
        assert!((result.b - 0.5).abs() < 0.01);
    }
}
