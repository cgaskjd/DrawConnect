//! Color Management Module
//!
//! Provides comprehensive color handling including:
//! - Multiple color spaces (RGB, HSB/HSV, HSL, CMYK, Lab)
//! - Color conversion between spaces
//! - ICC profile support
//! - Color picker utilities

mod convert;
mod palette;
mod profile;

pub use convert::ColorConverter;
pub use palette::ColorPalette;
pub use profile::IccProfile;

use serde::{Deserialize, Serialize};

/// Color space enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSpace {
    /// sRGB color space
    SRGB,
    /// Adobe RGB color space
    AdobeRGB,
    /// Display P3 color space
    DisplayP3,
    /// ProPhoto RGB color space
    ProPhotoRGB,
    /// CMYK color space
    CMYK,
    /// CIE Lab color space
    Lab,
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::SRGB
    }
}

/// Color structure with RGBA values (0.0 - 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red component (0.0 - 1.0)
    pub r: f32,
    /// Green component (0.0 - 1.0)
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    pub b: f32,
    /// Alpha component (0.0 - 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color from RGBA values (0.0 - 1.0)
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create a new color from RGB values (0.0 - 1.0), alpha = 1.0
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Create a new color from RGBA u8 values (0 - 255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Create a new color from RGB u8 values (0 - 255)
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

    /// Create a new color from hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let len = hex.len();

        match len {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::from_rgb8(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::from_rgba8(r, g, b, a))
            }
            3 => {
                // Short form: #RGB
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::from_rgb8(r, g, b))
            }
            _ => None,
        }
    }

    /// Create a new color from HSB/HSV values
    /// h: 0-360, s: 0-1, b: 0-1
    pub fn from_hsb(h: f32, s: f32, b: f32) -> Self {
        Self::from_hsba(h, s, b, 1.0)
    }

    /// Create a new color from HSBA values
    pub fn from_hsba(h: f32, s: f32, b: f32, a: f32) -> Self {
        let h = h % 360.0;
        let s = s.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);

        let c = b * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = b - c;

        let (r, g, bl) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Self::from_rgba(r + m, g + m, bl + m, a)
    }

    /// Create a new color from HSL values
    /// h: 0-360, s: 0-1, l: 0-1
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self::from_hsla(h, s, l, 1.0)
    }

    /// Create a new color from HSLA values
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let h = h % 360.0;
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Self::from_rgba(r + m, g + m, b + m, a)
    }

    /// Convert to RGBA u8 values (0 - 255)
    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        )
    }

    /// Convert to RGB u8 values (0 - 255)
    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        let (r, g, b, _) = self.to_rgba8();
        (r, g, b)
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb8();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert to hex string with alpha
    pub fn to_hex_rgba(&self) -> String {
        let (r, g, b, a) = self.to_rgba8();
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }

    /// Convert to HSB/HSV values
    /// Returns (h: 0-360, s: 0-1, b: 0-1)
    pub fn to_hsb(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let b = max;
        let s = if max == 0.0 { 0.0 } else { delta / max };

        let h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (((self.g - self.b) / delta) % 6.0)
        } else if max == self.g {
            60.0 * ((self.b - self.r) / delta + 2.0)
        } else {
            60.0 * ((self.r - self.g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        (h, s, b)
    }

    /// Convert to HSL values
    /// Returns (h: 0-360, s: 0-1, l: 0-1)
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let l = (max + min) / 2.0;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (((self.g - self.b) / delta) % 6.0)
        } else if max == self.g {
            60.0 * ((self.b - self.r) / delta + 2.0)
        } else {
            60.0 * ((self.r - self.g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        (h, s, l)
    }

    /// Get luminance (perceived brightness)
    pub fn luminance(&self) -> f32 {
        0.299 * self.r + 0.587 * self.g + 0.114 * self.b
    }

    /// Check if color is dark
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }

    /// Check if color is light
    pub fn is_light(&self) -> bool {
        self.luminance() >= 0.5
    }

    /// Create a new color with different alpha
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::from_rgba(self.r, self.g, self.b, alpha)
    }

    /// Lighten the color by a factor
    pub fn lighten(&self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l + factor).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, self.a)
    }

    /// Darken the color by a factor
    pub fn darken(&self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l - factor).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, self.a)
    }

    /// Saturate the color by a factor
    pub fn saturate(&self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s + factor).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, self.a)
    }

    /// Desaturate the color by a factor
    pub fn desaturate(&self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s - factor).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, self.a)
    }

    /// Get the complementary color
    pub fn complement(&self) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_h = (h + 180.0) % 360.0;
        Self::from_hsla(new_h, s, l, self.a)
    }

    /// Interpolate between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::from_rgba(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Blend this color over another (Porter-Duff "over" operation)
    pub fn blend_over(&self, other: Color) -> Self {
        let src_a = other.a;
        let dst_a = self.a;
        let out_a = src_a + dst_a * (1.0 - src_a);

        if out_a == 0.0 {
            return Self::transparent();
        }

        let out_r = (other.r * src_a + self.r * dst_a * (1.0 - src_a)) / out_a;
        let out_g = (other.g * src_a + self.g * dst_a * (1.0 - src_a)) / out_a;
        let out_b = (other.b * src_a + self.b * dst_a * (1.0 - src_a)) / out_a;

        Self::from_rgba(out_r, out_g, out_b, out_a)
    }

    // Common colors
    /// Black color
    pub fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }

    /// White color
    pub fn white() -> Self {
        Self::from_rgb(1.0, 1.0, 1.0)
    }

    /// Red color
    pub fn red() -> Self {
        Self::from_rgb(1.0, 0.0, 0.0)
    }

    /// Green color
    pub fn green() -> Self {
        Self::from_rgb(0.0, 1.0, 0.0)
    }

    /// Blue color
    pub fn blue() -> Self {
        Self::from_rgb(0.0, 0.0, 1.0)
    }

    /// Yellow color
    pub fn yellow() -> Self {
        Self::from_rgb(1.0, 1.0, 0.0)
    }

    /// Cyan color
    pub fn cyan() -> Self {
        Self::from_rgb(0.0, 1.0, 1.0)
    }

    /// Magenta color
    pub fn magenta() -> Self {
        Self::from_rgb(1.0, 0.0, 1.0)
    }

    /// Transparent color
    pub fn transparent() -> Self {
        Self::from_rgba(0.0, 0.0, 0.0, 0.0)
    }

    /// Gray color
    pub fn gray(value: f32) -> Self {
        Self::from_rgb(value, value, value)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

/// Color manager for handling color operations and profiles
pub struct ColorManager {
    /// Current working color space
    working_space: ColorSpace,
    /// ICC profile (if loaded)
    profile: Option<IccProfile>,
    /// Color history
    history: Vec<Color>,
    /// Maximum history size
    max_history: usize,
}

impl ColorManager {
    /// Create a new color manager
    pub fn new() -> Self {
        Self {
            working_space: ColorSpace::default(),
            profile: None,
            history: Vec::new(),
            max_history: 50,
        }
    }

    /// Set working color space
    pub fn set_working_space(&mut self, space: ColorSpace) {
        self.working_space = space;
    }

    /// Get working color space
    pub fn working_space(&self) -> ColorSpace {
        self.working_space
    }

    /// Add color to history
    pub fn add_to_history(&mut self, color: Color) {
        // Remove if already exists
        self.history.retain(|c| {
            (c.r - color.r).abs() > 0.001
                || (c.g - color.g).abs() > 0.001
                || (c.b - color.b).abs() > 0.001
        });

        // Add to front
        self.history.insert(0, color);

        // Trim to max size
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }
    }

    /// Get color history
    pub fn history(&self) -> &[Color] {
        &self.history
    }

    /// Clear color history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Load ICC profile
    pub fn load_profile(&mut self, profile: IccProfile) {
        self.profile = Some(profile);
    }

    /// Get loaded ICC profile
    pub fn profile(&self) -> Option<&IccProfile> {
        self.profile.as_ref()
    }
}

impl Default for ColorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::from_rgba(1.0, 0.5, 0.25, 1.0);
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.5).abs() < 0.01);
        assert!((color.b - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.0).abs() < 0.01);
        assert!((color.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        assert_eq!(color.to_hex(), "#FF0000");
    }

    #[test]
    fn test_hsb_conversion() {
        let color = Color::from_hsb(0.0, 1.0, 1.0); // Red
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.0).abs() < 0.01);
        assert!((color.b - 0.0).abs() < 0.01);

        let (h, s, b) = color.to_hsb();
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((b - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_lerp() {
        let black = Color::black();
        let white = Color::white();

        let gray = black.lerp(&white, 0.5);
        assert!((gray.r - 0.5).abs() < 0.01);
        assert!((gray.g - 0.5).abs() < 0.01);
        assert!((gray.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_luminance() {
        let white = Color::white();
        let black = Color::black();

        assert!(white.luminance() > 0.9);
        assert!(black.luminance() < 0.1);
        assert!(white.is_light());
        assert!(black.is_dark());
    }
}
