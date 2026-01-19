//! Color conversion utilities

use super::{Color, ColorSpace};

/// Color converter for converting between color spaces
pub struct ColorConverter;

impl ColorConverter {
    /// Convert color from one space to another
    pub fn convert(color: Color, from: ColorSpace, to: ColorSpace) -> Color {
        if from == to {
            return color;
        }

        // First convert to XYZ (intermediate space)
        let xyz = Self::to_xyz(color, from);

        // Then convert from XYZ to target space
        Self::from_xyz(xyz, to)
    }

    /// Convert to CIE XYZ color space
    fn to_xyz(color: Color, from: ColorSpace) -> (f32, f32, f32) {
        match from {
            ColorSpace::SRGB => Self::srgb_to_xyz(color),
            ColorSpace::AdobeRGB => Self::adobe_rgb_to_xyz(color),
            ColorSpace::DisplayP3 => Self::display_p3_to_xyz(color),
            ColorSpace::ProPhotoRGB => Self::prophoto_to_xyz(color),
            _ => Self::srgb_to_xyz(color), // Fallback
        }
    }

    /// Convert from CIE XYZ color space
    fn from_xyz(xyz: (f32, f32, f32), to: ColorSpace) -> Color {
        match to {
            ColorSpace::SRGB => Self::xyz_to_srgb(xyz),
            ColorSpace::AdobeRGB => Self::xyz_to_adobe_rgb(xyz),
            ColorSpace::DisplayP3 => Self::xyz_to_display_p3(xyz),
            ColorSpace::ProPhotoRGB => Self::xyz_to_prophoto(xyz),
            _ => Self::xyz_to_srgb(xyz), // Fallback
        }
    }

    /// sRGB to XYZ
    fn srgb_to_xyz(color: Color) -> (f32, f32, f32) {
        let r = Self::srgb_to_linear(color.r);
        let g = Self::srgb_to_linear(color.g);
        let b = Self::srgb_to_linear(color.b);

        let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
        let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
        let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

        (x, y, z)
    }

    /// XYZ to sRGB
    fn xyz_to_srgb(xyz: (f32, f32, f32)) -> Color {
        let (x, y, z) = xyz;

        let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

        Color::from_rgb(
            Self::linear_to_srgb(r),
            Self::linear_to_srgb(g),
            Self::linear_to_srgb(b),
        )
    }

    /// Adobe RGB to XYZ
    fn adobe_rgb_to_xyz(color: Color) -> (f32, f32, f32) {
        let r = color.r.powf(2.2);
        let g = color.g.powf(2.2);
        let b = color.b.powf(2.2);

        let x = 0.5767309 * r + 0.1855540 * g + 0.1881852 * b;
        let y = 0.2973769 * r + 0.6273491 * g + 0.0752741 * b;
        let z = 0.0270343 * r + 0.0706872 * g + 0.9911085 * b;

        (x, y, z)
    }

    /// XYZ to Adobe RGB
    fn xyz_to_adobe_rgb(xyz: (f32, f32, f32)) -> Color {
        let (x, y, z) = xyz;

        let r = 2.0413690 * x - 0.5649464 * y - 0.3446944 * z;
        let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b = 0.0134474 * x - 0.1183897 * y + 1.0154096 * z;

        Color::from_rgb(
            r.max(0.0).powf(1.0 / 2.2),
            g.max(0.0).powf(1.0 / 2.2),
            b.max(0.0).powf(1.0 / 2.2),
        )
    }

    /// Display P3 to XYZ
    fn display_p3_to_xyz(color: Color) -> (f32, f32, f32) {
        let r = Self::srgb_to_linear(color.r);
        let g = Self::srgb_to_linear(color.g);
        let b = Self::srgb_to_linear(color.b);

        let x = 0.4865709 * r + 0.2656677 * g + 0.1982173 * b;
        let y = 0.2289746 * r + 0.6917385 * g + 0.0792869 * b;
        let z = 0.0000000 * r + 0.0451134 * g + 1.0439444 * b;

        (x, y, z)
    }

    /// XYZ to Display P3
    fn xyz_to_display_p3(xyz: (f32, f32, f32)) -> Color {
        let (x, y, z) = xyz;

        let r = 2.4934969 * x - 0.9313836 * y - 0.4027108 * z;
        let g = -0.8294890 * x + 1.7626641 * y + 0.0236247 * z;
        let b = 0.0358458 * x - 0.0761724 * y + 0.9568845 * z;

        Color::from_rgb(
            Self::linear_to_srgb(r),
            Self::linear_to_srgb(g),
            Self::linear_to_srgb(b),
        )
    }

    /// ProPhoto RGB to XYZ
    fn prophoto_to_xyz(color: Color) -> (f32, f32, f32) {
        let r = if color.r <= 0.001953125 {
            color.r / 16.0
        } else {
            color.r.powf(1.8)
        };
        let g = if color.g <= 0.001953125 {
            color.g / 16.0
        } else {
            color.g.powf(1.8)
        };
        let b = if color.b <= 0.001953125 {
            color.b / 16.0
        } else {
            color.b.powf(1.8)
        };

        let x = 0.7977666 * r + 0.1351917 * g + 0.0313534 * b;
        let y = 0.2880748 * r + 0.7118432 * g + 0.0000819 * b;
        let z = 0.0000000 * r + 0.0000000 * g + 0.8251046 * b;

        (x, y, z)
    }

    /// XYZ to ProPhoto RGB
    fn xyz_to_prophoto(xyz: (f32, f32, f32)) -> Color {
        let (x, y, z) = xyz;

        let r = 1.3459433 * x - 0.2556075 * y - 0.0511118 * z;
        let g = -0.5445989 * x + 1.5081673 * y + 0.0205351 * z;
        let b = 0.0000000 * x + 0.0000000 * y + 1.2118128 * z;

        let r = if r <= 0.001953125 / 16.0 {
            r * 16.0
        } else {
            r.max(0.0).powf(1.0 / 1.8)
        };
        let g = if g <= 0.001953125 / 16.0 {
            g * 16.0
        } else {
            g.max(0.0).powf(1.0 / 1.8)
        };
        let b = if b <= 0.001953125 / 16.0 {
            b * 16.0
        } else {
            b.max(0.0).powf(1.0 / 1.8)
        };

        Color::from_rgb(r, g, b)
    }

    /// Convert sRGB component to linear
    fn srgb_to_linear(c: f32) -> f32 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Convert linear component to sRGB
    fn linear_to_srgb(c: f32) -> f32 {
        let c = c.clamp(0.0, 1.0);
        if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    }

    /// Convert RGB to CMYK
    pub fn rgb_to_cmyk(color: Color) -> (f32, f32, f32, f32) {
        let k = 1.0 - color.r.max(color.g).max(color.b);

        if k >= 1.0 {
            return (0.0, 0.0, 0.0, 1.0);
        }

        let c = (1.0 - color.r - k) / (1.0 - k);
        let m = (1.0 - color.g - k) / (1.0 - k);
        let y = (1.0 - color.b - k) / (1.0 - k);

        (c, m, y, k)
    }

    /// Convert CMYK to RGB
    pub fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> Color {
        let r = (1.0 - c) * (1.0 - k);
        let g = (1.0 - m) * (1.0 - k);
        let b = (1.0 - y) * (1.0 - k);

        Color::from_rgb(r, g, b)
    }

    /// Convert RGB to Lab
    pub fn rgb_to_lab(color: Color) -> (f32, f32, f32) {
        let (x, y, z) = Self::srgb_to_xyz(color);

        // D65 white point
        let xn = 0.95047;
        let yn = 1.0;
        let zn = 1.08883;

        let fx = Self::lab_f(x / xn);
        let fy = Self::lab_f(y / yn);
        let fz = Self::lab_f(z / zn);

        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        (l, a, b)
    }

    /// Convert Lab to RGB
    pub fn lab_to_rgb(l: f32, a: f32, b: f32) -> Color {
        // D65 white point
        let xn = 0.95047;
        let yn = 1.0;
        let zn = 1.08883;

        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;

        let x = xn * Self::lab_f_inv(fx);
        let y = yn * Self::lab_f_inv(fy);
        let z = zn * Self::lab_f_inv(fz);

        Self::xyz_to_srgb((x, y, z))
    }

    fn lab_f(t: f32) -> f32 {
        let delta: f32 = 6.0 / 29.0;
        if t > delta.powi(3) {
            t.powf(1.0 / 3.0)
        } else {
            t / (3.0 * delta * delta) + 4.0 / 29.0
        }
    }

    fn lab_f_inv(t: f32) -> f32 {
        let delta: f32 = 6.0 / 29.0;
        if t > delta {
            t.powi(3)
        } else {
            3.0 * delta * delta * (t - 4.0 / 29.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_roundtrip() {
        let original = Color::from_rgb(0.5, 0.3, 0.8);
        let converted = ColorConverter::convert(original, ColorSpace::SRGB, ColorSpace::AdobeRGB);
        let back = ColorConverter::convert(converted, ColorSpace::AdobeRGB, ColorSpace::SRGB);

        assert!((original.r - back.r).abs() < 0.01);
        assert!((original.g - back.g).abs() < 0.01);
        assert!((original.b - back.b).abs() < 0.01);
    }

    #[test]
    fn test_cmyk_conversion() {
        let color = Color::from_rgb(1.0, 0.0, 0.0); // Red
        let (c, m, y, k) = ColorConverter::rgb_to_cmyk(color);

        assert!((c - 0.0).abs() < 0.01);
        assert!((m - 1.0).abs() < 0.01);
        assert!((y - 1.0).abs() < 0.01);
        assert!((k - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_lab_conversion() {
        let white = Color::white();
        let (l, a, b) = ColorConverter::rgb_to_lab(white);

        assert!((l - 100.0).abs() < 1.0);
        assert!(a.abs() < 1.0);
        assert!(b.abs() < 1.0);
    }
}
