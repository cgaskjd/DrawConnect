//! Brush settings

use serde::{Deserialize, Serialize};

/// Brush settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushSettings {
    /// Brush size in pixels
    pub size: f32,
    /// Minimum size ratio (0.0 - 1.0) relative to base size
    pub min_size_ratio: f32,
    /// Brush opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Minimum opacity ratio
    pub min_opacity_ratio: f32,
    /// Brush hardness (0.0 - 1.0), affects edge softness
    pub hardness: f32,
    /// Spacing between brush stamps (ratio of brush size)
    pub spacing: f32,
    /// Flow rate (for accumulative painting)
    pub flow: f32,
    /// Smoothing level (0.0 - 1.0)
    pub smoothing: f32,
    /// Anti-aliasing enabled
    pub anti_aliasing: bool,
    /// Angle in degrees
    pub angle: f32,
    /// Roundness (1.0 = circle, < 1.0 = ellipse)
    pub roundness: f32,
    /// Wet edges effect
    pub wet_edges: bool,
    /// Build-up mode (accumulate opacity)
    pub build_up: bool,
    /// Transfer mode
    pub transfer_mode: TransferMode,
}

/// Transfer mode for brush painting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferMode {
    /// Normal painting
    Normal,
    /// Multiply transfer
    Multiply,
    /// Screen transfer
    Screen,
    /// Build-up (accumulative)
    BuildUp,
}

impl Default for TransferMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            size: 20.0,
            min_size_ratio: 0.1,
            opacity: 1.0,
            min_opacity_ratio: 0.0,
            hardness: 0.8,
            spacing: 0.15,
            flow: 1.0,
            smoothing: 0.5,
            anti_aliasing: true,
            angle: 0.0,
            roundness: 1.0,
            wet_edges: false,
            build_up: false,
            transfer_mode: TransferMode::Normal,
        }
    }
}

impl BrushSettings {
    /// Create settings for a soft round brush
    pub fn soft_round() -> Self {
        Self {
            hardness: 0.0,
            ..Default::default()
        }
    }

    /// Create settings for a hard round brush
    pub fn hard_round() -> Self {
        Self {
            hardness: 1.0,
            ..Default::default()
        }
    }

    /// Create settings for a pencil
    pub fn pencil() -> Self {
        Self {
            size: 3.0,
            hardness: 0.9,
            opacity: 0.8,
            spacing: 0.05,
            build_up: true,
            ..Default::default()
        }
    }

    /// Create settings for ink pen
    pub fn ink_pen() -> Self {
        Self {
            size: 5.0,
            hardness: 1.0,
            opacity: 1.0,
            spacing: 0.05,
            smoothing: 0.7,
            ..Default::default()
        }
    }

    /// Create settings for airbrush
    pub fn airbrush() -> Self {
        Self {
            size: 50.0,
            hardness: 0.0,
            opacity: 0.1,
            spacing: 0.05,
            flow: 0.5,
            build_up: true,
            ..Default::default()
        }
    }

    /// Create settings for watercolor
    pub fn watercolor() -> Self {
        Self {
            size: 30.0,
            hardness: 0.2,
            opacity: 0.6,
            spacing: 0.1,
            wet_edges: true,
            flow: 0.7,
            ..Default::default()
        }
    }

    /// Create settings for eraser
    pub fn eraser() -> Self {
        Self {
            size: 20.0,
            hardness: 0.5,
            opacity: 1.0,
            spacing: 0.1,
            ..Default::default()
        }
    }

    /// Validate and clamp settings to valid ranges
    pub fn validate(&mut self) {
        self.size = self.size.max(1.0).min(5000.0);
        self.min_size_ratio = self.min_size_ratio.clamp(0.0, 1.0);
        self.opacity = self.opacity.clamp(0.0, 1.0);
        self.min_opacity_ratio = self.min_opacity_ratio.clamp(0.0, 1.0);
        self.hardness = self.hardness.clamp(0.0, 1.0);
        self.spacing = self.spacing.max(0.01).min(10.0);
        self.flow = self.flow.clamp(0.0, 1.0);
        self.smoothing = self.smoothing.clamp(0.0, 1.0);
        self.angle = self.angle % 360.0;
        self.roundness = self.roundness.clamp(0.01, 1.0);
    }

    /// Create a builder for brush settings
    pub fn builder() -> BrushSettingsBuilder {
        BrushSettingsBuilder::new()
    }
}

/// Builder for brush settings
#[derive(Default)]
pub struct BrushSettingsBuilder {
    settings: BrushSettings,
}

impl BrushSettingsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set brush size
    pub fn size(mut self, size: f32) -> Self {
        self.settings.size = size;
        self
    }

    /// Set minimum size ratio
    pub fn min_size_ratio(mut self, ratio: f32) -> Self {
        self.settings.min_size_ratio = ratio;
        self
    }

    /// Set opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.settings.opacity = opacity;
        self
    }

    /// Set hardness
    pub fn hardness(mut self, hardness: f32) -> Self {
        self.settings.hardness = hardness;
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.settings.spacing = spacing;
        self
    }

    /// Set flow
    pub fn flow(mut self, flow: f32) -> Self {
        self.settings.flow = flow;
        self
    }

    /// Set smoothing
    pub fn smoothing(mut self, smoothing: f32) -> Self {
        self.settings.smoothing = smoothing;
        self
    }

    /// Enable/disable anti-aliasing
    pub fn anti_aliasing(mut self, enabled: bool) -> Self {
        self.settings.anti_aliasing = enabled;
        self
    }

    /// Set angle
    pub fn angle(mut self, angle: f32) -> Self {
        self.settings.angle = angle;
        self
    }

    /// Set roundness
    pub fn roundness(mut self, roundness: f32) -> Self {
        self.settings.roundness = roundness;
        self
    }

    /// Enable/disable wet edges
    pub fn wet_edges(mut self, enabled: bool) -> Self {
        self.settings.wet_edges = enabled;
        self
    }

    /// Enable/disable build-up mode
    pub fn build_up(mut self, enabled: bool) -> Self {
        self.settings.build_up = enabled;
        self
    }

    /// Set transfer mode
    pub fn transfer_mode(mut self, mode: TransferMode) -> Self {
        self.settings.transfer_mode = mode;
        self
    }

    /// Build the settings
    pub fn build(mut self) -> BrushSettings {
        self.settings.validate();
        self.settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = BrushSettings::default();
        assert_eq!(settings.size, 20.0);
        assert_eq!(settings.opacity, 1.0);
    }

    #[test]
    fn test_builder() {
        let settings = BrushSettings::builder()
            .size(50.0)
            .opacity(0.5)
            .hardness(0.8)
            .build();

        assert_eq!(settings.size, 50.0);
        assert_eq!(settings.opacity, 0.5);
        assert_eq!(settings.hardness, 0.8);
    }

    #[test]
    fn test_validation() {
        let mut settings = BrushSettings::default();
        settings.opacity = 2.0; // Invalid
        settings.hardness = -0.5; // Invalid
        settings.validate();

        assert_eq!(settings.opacity, 1.0);
        assert_eq!(settings.hardness, 0.0);
    }
}
