//! Blur filters
//!
//! Provides various blur effects.

mod gaussian;
mod box_blur;
mod motion;
mod radial;

pub use gaussian::GaussianBlur;
pub use box_blur::BoxBlur;
pub use motion::MotionBlur;
pub use radial::{RadialBlur, RadialBlurType};

/// Horizontal box blur pass (shared by GaussianBlur and BoxBlur)
pub(crate) fn box_blur_h(source: &[u8], target: &mut [u8], w: u32, h: u32, r: i32) {
    let iarr = 1.0 / (r + r + 1) as f32;

    for y in 0..h {
        for c in 0..4 {
            let mut acc = 0.0f32;

            let first_val = source[(y * w * 4 + c) as usize] as f32;
            let last_val = source[(y * w * 4 + (w - 1) * 4 + c) as usize] as f32;

            acc += first_val * (r + 1) as f32;
            for x in 0..r.min(w as i32) {
                acc += source[(y * w * 4 + x as u32 * 4 + c) as usize] as f32;
            }

            for x in 0..w as i32 {
                let left = x - r - 1;
                let right = x + r;

                if right < w as i32 {
                    acc += source[(y * w * 4 + right as u32 * 4 + c) as usize] as f32;
                } else {
                    acc += last_val;
                }

                if left >= 0 {
                    acc -= source[(y * w * 4 + left as u32 * 4 + c) as usize] as f32;
                } else {
                    acc -= first_val;
                }

                target[(y * w * 4 + x as u32 * 4 + c) as usize] = (acc * iarr).clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Vertical box blur pass (shared by GaussianBlur and BoxBlur)
pub(crate) fn box_blur_v(source: &[u8], target: &mut [u8], w: u32, h: u32, r: i32) {
    let iarr = 1.0 / (r + r + 1) as f32;

    for x in 0..w {
        for c in 0..4 {
            let mut acc = 0.0f32;

            let first_val = source[(x * 4 + c) as usize] as f32;
            let last_val = source[((h - 1) * w * 4 + x * 4 + c) as usize] as f32;

            acc += first_val * (r + 1) as f32;
            for y in 0..r.min(h as i32) {
                acc += source[(y as u32 * w * 4 + x * 4 + c) as usize] as f32;
            }

            for y in 0..h as i32 {
                let top = y - r - 1;
                let bottom = y + r;

                if bottom < h as i32 {
                    acc += source[(bottom as u32 * w * 4 + x * 4 + c) as usize] as f32;
                } else {
                    acc += last_val;
                }

                if top >= 0 {
                    acc -= source[(top as u32 * w * 4 + x * 4 + c) as usize] as f32;
                } else {
                    acc -= first_val;
                }

                target[(y as u32 * w * 4 + x * 4 + c) as usize] = (acc * iarr).clamp(0.0, 255.0) as u8;
            }
        }
    }
}
