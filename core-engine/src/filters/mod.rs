//! # Filter Effects Module
//!
//! Provides image filter effects similar to Photoshop.
//!
//! ## Supported Filters
//!
//! - **Blur**: Gaussian, Box, Motion, Radial
//! - **Sharpen**: Unsharp Mask, High Pass
//! - **Noise**: Add Noise, Reduce Noise
//! - **Stylize**: Find Edges, Emboss, Pixelate, Oil Paint
//! - **Distort**: Spherize, Twirl, Wave, Ripple
//! - **Render**: Vignette, Lens Flare, Clouds

pub mod blur;
pub mod sharpen;
pub mod noise;
pub mod stylize;
pub mod distort;
pub mod render;
pub mod pixel_ops;

pub use blur::{GaussianBlur, BoxBlur, MotionBlur, RadialBlur, RadialBlurType};
pub use sharpen::{UnsharpMask, HighPass};
pub use noise::{AddNoise, ReduceNoise, NoiseType};
pub use stylize::{FindEdges, Emboss, Pixelate, OilPaint};
pub use distort::{Spherize, SpherizeMode, Twirl, Wave, WaveType, Ripple, RippleSize};
pub use render::{Vignette, LensFlare, FlareStyle, Clouds};

use crate::layer::Layer;
use crate::selection::Selection;

/// Common trait for all image filters
pub trait Filter: Send + Sync {
    /// Apply filter to a layer's pixel buffer
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32);

    /// Apply filter to an entire layer
    fn apply_to_layer(&self, layer: &mut Layer) {
        let width = layer.width();
        let height = layer.height();
        self.apply(&mut layer.pixels, width, height);
    }

    /// Apply filter respecting a selection mask
    fn apply_with_selection(&self, layer: &mut Layer, selection: &Selection) {
        let width = layer.width();
        let height = layer.height();

        // Create a copy for the filter to work on
        let mut filtered = layer.pixels.clone();
        self.apply(&mut filtered, width, height);

        // Blend filtered result with original based on selection
        for y in 0..height {
            for x in 0..width {
                if selection.contains(x as f32, y as f32) {
                    let idx = ((y * width + x) * 4) as usize;
                    if idx + 3 < layer.pixels.len() {
                        layer.pixels[idx] = filtered[idx];
                        layer.pixels[idx + 1] = filtered[idx + 1];
                        layer.pixels[idx + 2] = filtered[idx + 2];
                        layer.pixels[idx + 3] = filtered[idx + 3];
                    }
                }
            }
        }
    }

    /// Get filter name for UI/history
    fn name(&self) -> &'static str;
}
