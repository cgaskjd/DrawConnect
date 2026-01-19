//! Lens Flare filter
//!
//! Simulates camera lens flare effect.

use crate::filters::Filter;
use std::f32::consts::PI;

/// Lens flare style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlareStyle {
    /// 50-300mm Zoom lens
    #[default]
    Zoom50_300,
    /// 35mm Prime lens
    Prime35,
    /// 105mm lens
    Lens105,
    /// Movie Prime lens
    MoviePrime,
}

/// Lens Flare filter
#[derive(Debug, Clone)]
pub struct LensFlare {
    /// Flare center X position (0 to 100%)
    pub center_x: f32,
    /// Flare center Y position (0 to 100%)
    pub center_y: f32,
    /// Brightness (0 to 300%)
    pub brightness: f32,
    /// Flare style
    pub style: FlareStyle,
}

impl LensFlare {
    /// Create a new lens flare filter
    pub fn new(center_x: f32, center_y: f32, brightness: f32) -> Self {
        Self {
            center_x: center_x.clamp(0.0, 100.0),
            center_y: center_y.clamp(0.0, 100.0),
            brightness: brightness.clamp(0.0, 300.0),
            style: FlareStyle::default(),
        }
    }
}

impl Default for LensFlare {
    fn default() -> Self {
        Self {
            center_x: 50.0,
            center_y: 50.0,
            brightness: 100.0,
            style: FlareStyle::Zoom50_300,
        }
    }
}

impl Filter for LensFlare {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let cx = width as f32 * self.center_x / 100.0;
        let cy = height as f32 * self.center_y / 100.0;
        let brightness = self.brightness / 100.0;

        // Center of image for flare line calculation
        let img_cx = width as f32 / 2.0;
        let img_cy = height as f32 / 2.0;

        // Direction from flare center to image center
        let dir_x = img_cx - cx;
        let dir_y = img_cy - cy;
        let dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(1.0);
        let dir_x = dir_x / dir_len;
        let dir_y = dir_y / dir_len;

        // Flare elements based on style
        let elements = match self.style {
            FlareStyle::Zoom50_300 => vec![
                (0.0, 80.0, (255, 240, 200), 0.8),   // Main bright center
                (0.3, 40.0, (255, 200, 150), 0.3),  // Secondary ring
                (0.6, 25.0, (200, 150, 255), 0.2),  // Purple halo
                (1.0, 15.0, (150, 200, 255), 0.15), // Blue dot
                (1.5, 20.0, (255, 220, 180), 0.1),  // Far element
            ],
            FlareStyle::Prime35 => vec![
                (0.0, 60.0, (255, 255, 240), 0.7),
                (0.4, 30.0, (200, 255, 200), 0.25),
                (0.8, 20.0, (255, 200, 200), 0.2),
            ],
            FlareStyle::Lens105 => vec![
                (0.0, 70.0, (255, 245, 220), 0.75),
                (0.2, 50.0, (255, 230, 180), 0.4),
                (0.5, 35.0, (180, 200, 255), 0.25),
                (0.9, 25.0, (255, 180, 220), 0.2),
                (1.3, 15.0, (220, 255, 220), 0.1),
            ],
            FlareStyle::MoviePrime => vec![
                (0.0, 100.0, (255, 250, 240), 0.6),
                (0.1, 80.0, (255, 240, 200), 0.4),
                (0.3, 60.0, (180, 220, 255), 0.3),
                (0.6, 40.0, (255, 180, 180), 0.25),
                (1.0, 30.0, (200, 255, 200), 0.2),
                (1.4, 20.0, (180, 180, 255), 0.15),
            ],
        };

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                let mut add_r = 0.0f32;
                let mut add_g = 0.0f32;
                let mut add_b = 0.0f32;

                for (dist_factor, radius, color, intensity) in &elements {
                    // Calculate element position along the flare line
                    let elem_x = cx + dir_x * dir_len * dist_factor;
                    let elem_y = cy + dir_y * dir_len * dist_factor;

                    // Distance from this pixel to element center
                    let dx = x as f32 - elem_x;
                    let dy = y as f32 - elem_y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < *radius {
                        // Soft falloff
                        let t = 1.0 - (dist / radius);
                        let factor = t * t * intensity * brightness;

                        add_r += color.0 as f32 * factor;
                        add_g += color.1 as f32 * factor;
                        add_b += color.2 as f32 * factor;
                    }
                }

                // Add flare to existing pixel
                if add_r > 0.0 || add_g > 0.0 || add_b > 0.0 {
                    pixels[idx] = (pixels[idx] as f32 + add_r).min(255.0) as u8;
                    pixels[idx + 1] = (pixels[idx + 1] as f32 + add_g).min(255.0) as u8;
                    pixels[idx + 2] = (pixels[idx + 2] as f32 + add_b).min(255.0) as u8;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Lens Flare"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lens_flare() {
        let mut pixels = vec![64u8; 100 * 100 * 4];
        let filter = LensFlare::default();
        filter.apply(&mut pixels, 100, 100);
    }
}
