//! Wave filter
//!
//! Creates sinusoidal wave distortion effect.

use crate::filters::Filter;
use crate::filters::pixel_ops::{bilinear_sample_rgba, write_pixel, write_transparent};
use std::f32::consts::PI;

/// Wave type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WaveType {
    /// Sine wave
    #[default]
    Sine,
    /// Triangle wave
    Triangle,
    /// Square wave
    Square,
}

/// Wave filter - creates wave distortion
#[derive(Debug, Clone)]
pub struct Wave {
    /// Wave type
    pub wave_type: WaveType,
    /// Number of generators (1 to 5)
    pub generators: u32,
    /// Horizontal wavelength
    pub wavelength_x: f32,
    /// Vertical wavelength
    pub wavelength_y: f32,
    /// Horizontal amplitude
    pub amplitude_x: f32,
    /// Vertical amplitude
    pub amplitude_y: f32,
    /// Horizontal scale percentage
    pub scale_x: f32,
    /// Vertical scale percentage
    pub scale_y: f32,
}

impl Wave {
    /// Create a new wave filter
    pub fn new(wave_type: WaveType, wavelength: f32, amplitude: f32) -> Self {
        Self {
            wave_type,
            generators: 1,
            wavelength_x: wavelength,
            wavelength_y: wavelength,
            amplitude_x: amplitude,
            amplitude_y: amplitude,
            scale_x: 100.0,
            scale_y: 100.0,
        }
    }

    fn wave_value(&self, t: f32) -> f32 {
        match self.wave_type {
            WaveType::Sine => (t * 2.0 * PI).sin(),
            WaveType::Triangle => {
                let t = t.fract();
                if t < 0.25 {
                    t * 4.0
                } else if t < 0.75 {
                    2.0 - t * 4.0
                } else {
                    t * 4.0 - 4.0
                }
            }
            WaveType::Square => {
                if (t * 2.0 * PI).sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            wave_type: WaveType::Sine,
            generators: 1,
            wavelength_x: 120.0,
            wavelength_y: 120.0,
            amplitude_x: 35.0,
            amplitude_y: 35.0,
            scale_x: 100.0,
            scale_y: 100.0,
        }
    }
}

impl Filter for Wave {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();
        let scale_x = self.scale_x / 100.0;
        let scale_y = self.scale_y / 100.0;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Calculate wave displacement
                let wave_x = if self.wavelength_y > 0.0 {
                    self.wave_value(y as f32 / self.wavelength_y) * self.amplitude_x
                } else {
                    0.0
                };
                let wave_y = if self.wavelength_x > 0.0 {
                    self.wave_value(x as f32 / self.wavelength_x) * self.amplitude_y
                } else {
                    0.0
                };

                let src_x = x as f32 - wave_x * scale_x;
                let src_y = y as f32 - wave_y * scale_y;

                // Bilinear interpolation using shared utility
                if let Some(rgba) = bilinear_sample_rgba(&original, width, height, src_x, src_y) {
                    write_pixel(pixels, idx, rgba);
                } else {
                    // Set transparent for out of bounds
                    write_transparent(pixels, idx);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Wave"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave() {
        let mut pixels = vec![128u8; 100 * 100 * 4];
        let filter = Wave::default();
        filter.apply(&mut pixels, 100, 100);
    }
}
