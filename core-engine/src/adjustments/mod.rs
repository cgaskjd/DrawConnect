//! # Image Adjustments Module
//!
//! Provides professional image adjustment capabilities similar to Photoshop.
//!
//! ## Supported Adjustments
//!
//! - **Tonal**: Brightness/Contrast, Levels, Curves, Exposure
//! - **Color**: Hue/Saturation, Color Balance, Vibrance, Photo Filter
//! - **Special**: Black & White, Invert, Posterize, Threshold

mod brightness_contrast;
mod invert;
mod levels;
mod curves;
mod hue_saturation;
mod color_balance;
mod vibrance;
mod exposure;
mod black_white;
mod photo_filter;
mod posterize;
mod threshold;

pub use brightness_contrast::BrightnessContrast;
pub use invert::Invert;
pub use levels::Levels;
pub use curves::{Curves, CurvePoint};
pub use hue_saturation::HueSaturation;
pub use color_balance::ColorBalance;
pub use vibrance::Vibrance;
pub use exposure::Exposure;
pub use black_white::BlackWhite;
pub use photo_filter::{PhotoFilter, FilterPreset};
pub use posterize::Posterize;
pub use threshold::Threshold;

use crate::color::Color;
use crate::layer::Layer;
use crate::selection::Selection;

/// Common trait for all image adjustments
pub trait Adjustment: Send + Sync {
    /// Apply adjustment to a single pixel color
    fn apply_pixel(&self, color: Color) -> Color;

    /// Apply adjustment to an entire layer
    fn apply_to_layer(&self, layer: &mut Layer) {
        let width = layer.width();
        let height = layer.height();
        let pixels = &mut layer.pixels;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                if idx + 3 < pixels.len() {
                    let color = Color::from_rgba8(
                        pixels[idx],
                        pixels[idx + 1],
                        pixels[idx + 2],
                        pixels[idx + 3],
                    );
                    let adjusted = self.apply_pixel(color);
                    let (r, g, b, a) = adjusted.to_rgba8();
                    pixels[idx] = r;
                    pixels[idx + 1] = g;
                    pixels[idx + 2] = b;
                    pixels[idx + 3] = a;
                }
            }
        }
    }

    /// Apply adjustment respecting a selection mask
    fn apply_with_selection(&self, layer: &mut Layer, selection: &Selection) {
        let width = layer.width();
        let height = layer.height();
        let pixels = &mut layer.pixels;

        for y in 0..height {
            for x in 0..width {
                // Check if pixel is within selection
                if !selection.contains(x as f32, y as f32) {
                    continue;
                }

                let idx = ((y * width + x) * 4) as usize;
                if idx + 3 < pixels.len() {
                    let color = Color::from_rgba8(
                        pixels[idx],
                        pixels[idx + 1],
                        pixels[idx + 2],
                        pixels[idx + 3],
                    );
                    let adjusted = self.apply_pixel(color);
                    let (r, g, b, a) = adjusted.to_rgba8();
                    pixels[idx] = r;
                    pixels[idx + 1] = g;
                    pixels[idx + 2] = b;
                    pixels[idx + 3] = a;
                }
            }
        }
    }

    /// Get adjustment name for UI/history
    fn name(&self) -> &'static str;
}

/// Curve channel selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CurveChannel {
    /// All RGB channels together
    RGB,
    /// Red channel only
    Red,
    /// Green channel only
    Green,
    /// Blue channel only
    Blue,
}

impl Default for CurveChannel {
    fn default() -> Self {
        Self::RGB
    }
}
