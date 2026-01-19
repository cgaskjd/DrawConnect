//! Emboss filter
//!
//! Creates a 3D embossed effect.

use crate::filters::Filter;

/// Emboss filter
#[derive(Debug, Clone)]
pub struct Emboss {
    /// Light angle in degrees
    pub angle: f32,
    /// Effect height
    pub height: f32,
    /// Effect amount
    pub amount: f32,
}

impl Emboss {
    /// Create a new emboss filter
    pub fn new(angle: f32, height: f32, amount: f32) -> Self {
        Self {
            angle: angle % 360.0,
            height: height.clamp(1.0, 10.0),
            amount: amount.clamp(0.0, 500.0),
        }
    }
}

impl Default for Emboss {
    fn default() -> Self {
        Self {
            angle: 135.0,
            height: 1.0,
            amount: 100.0,
        }
    }
}

impl Filter for Emboss {
    fn apply(&self, pixels: &mut [u8], width: u32, height: u32) {
        let original = pixels.to_vec();

        // Calculate direction from angle
        let angle_rad = self.angle.to_radians();
        let dx = angle_rad.cos();
        let dy = angle_rad.sin();

        let offset = self.height.ceil() as i32;

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;

                // Get sample positions
                let x1 = (x as i32 - (dx * offset as f32) as i32).clamp(0, width as i32 - 1) as u32;
                let y1 = (y as i32 - (dy * offset as f32) as i32).clamp(0, height as i32 - 1) as u32;
                let x2 = (x as i32 + (dx * offset as f32) as i32).clamp(0, width as i32 - 1) as u32;
                let y2 = (y as i32 + (dy * offset as f32) as i32).clamp(0, height as i32 - 1) as u32;

                let idx1 = ((y1 * width + x1) * 4) as usize;
                let idx2 = ((y2 * width + x2) * 4) as usize;

                for c in 0..3 {
                    let p1 = original[idx1 + c] as f32;
                    let p2 = original[idx2 + c] as f32;

                    // Calculate emboss value (difference + 128 for neutral gray)
                    let diff = (p1 - p2) * (self.amount / 100.0);
                    let value = 128.0 + diff;

                    pixels[idx + c] = value.clamp(0.0, 255.0) as u8;
                }
                // Alpha unchanged
            }
        }
    }

    fn name(&self) -> &'static str {
        "Emboss"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emboss() {
        let mut pixels = vec![128u8; 50 * 50 * 4];
        let filter = Emboss::new(135.0, 1.0, 100.0);
        filter.apply(&mut pixels, 50, 50);
    }
}
