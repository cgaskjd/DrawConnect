//! Color swatch file parsers
//!
//! Supports:
//! - ACO: Adobe Color (Photoshop swatch files)
//! - ASE: Adobe Swatch Exchange

use crate::error::{EngineError, EngineResult};
use std::io::{Cursor, Read};

/// Imported color swatch
#[derive(Debug, Clone)]
pub struct ColorSwatch {
    /// Color name (may be empty)
    pub name: String,
    /// RGBA color values (0-255)
    pub color: [u8; 4],
}

/// Color space identifier for ACO format
#[derive(Debug, Clone, Copy, PartialEq)]
enum AcoColorSpace {
    RGB = 0,
    HSB = 1,
    CMYK = 2,
    Pantone = 3,
    Focoltone = 4,
    Trumatch = 5,
    Toyo = 6,
    Lab = 7,
    Grayscale = 8,
    HKS = 10,
}

impl TryFrom<u16> for AcoColorSpace {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AcoColorSpace::RGB),
            1 => Ok(AcoColorSpace::HSB),
            2 => Ok(AcoColorSpace::CMYK),
            3 => Ok(AcoColorSpace::Pantone),
            4 => Ok(AcoColorSpace::Focoltone),
            5 => Ok(AcoColorSpace::Trumatch),
            6 => Ok(AcoColorSpace::Toyo),
            7 => Ok(AcoColorSpace::Lab),
            8 => Ok(AcoColorSpace::Grayscale),
            10 => Ok(AcoColorSpace::HKS),
            _ => Err(()),
        }
    }
}

/// Swatch file parser
pub struct SwatchParser;

impl SwatchParser {
    /// Parse an ACO (Adobe Color) file
    pub fn parse_aco(data: &[u8]) -> EngineResult<Vec<ColorSwatch>> {
        if data.len() < 4 {
            return Err(EngineError::ImportError("ACO file too small".into()));
        }

        let mut cursor = Cursor::new(data);
        let mut swatches = Vec::new();

        // Read version (2 bytes, big-endian)
        let mut version_buf = [0u8; 2];
        cursor.read_exact(&mut version_buf)?;
        let version = u16::from_be_bytes(version_buf);

        if version != 1 && version != 2 {
            return Err(EngineError::ImportError(format!(
                "Unsupported ACO version: {}",
                version
            )));
        }

        // Read color count (2 bytes)
        let mut count_buf = [0u8; 2];
        cursor.read_exact(&mut count_buf)?;
        let color_count = u16::from_be_bytes(count_buf);

        // Version 1 colors (no names)
        for _ in 0..color_count {
            match Self::parse_aco_color(&mut cursor, false) {
                Ok(swatch) => swatches.push(swatch),
                Err(e) => log::warn!("Failed to parse ACO color: {}", e),
            }
        }

        // Check for version 2 section (has names)
        if cursor.position() as usize + 4 <= data.len() {
            let mut version2_buf = [0u8; 2];
            if cursor.read_exact(&mut version2_buf).is_ok() {
                let version2 = u16::from_be_bytes(version2_buf);

                if version2 == 2 {
                    // Read color count again
                    let mut count2_buf = [0u8; 2];
                    cursor.read_exact(&mut count2_buf)?;
                    let color_count2 = u16::from_be_bytes(count2_buf);

                    // Replace swatches with named versions
                    swatches.clear();
                    for _ in 0..color_count2 {
                        match Self::parse_aco_color(&mut cursor, true) {
                            Ok(swatch) => swatches.push(swatch),
                            Err(e) => log::warn!("Failed to parse ACO v2 color: {}", e),
                        }
                    }
                }
            }
        }

        Ok(swatches)
    }

    /// Parse a single color from ACO format
    fn parse_aco_color(cursor: &mut Cursor<&[u8]>, with_name: bool) -> EngineResult<ColorSwatch> {
        // Read color space (2 bytes)
        let mut space_buf = [0u8; 2];
        cursor.read_exact(&mut space_buf)?;
        let color_space = u16::from_be_bytes(space_buf);

        // Read color values (4 x 2 bytes)
        let mut values = [0u16; 4];
        for v in &mut values {
            let mut val_buf = [0u8; 2];
            cursor.read_exact(&mut val_buf)?;
            *v = u16::from_be_bytes(val_buf);
        }

        // Convert to RGBA based on color space
        let color = Self::convert_aco_color(color_space, values)?;

        // Read name if present
        let name = if with_name {
            // Skip padding (2 bytes - always 0)
            let mut pad_buf = [0u8; 2];
            cursor.read_exact(&mut pad_buf)?;

            // Read name length (2 bytes) - includes null terminator
            let mut len_buf = [0u8; 2];
            cursor.read_exact(&mut len_buf)?;
            let name_len = u16::from_be_bytes(len_buf) as usize;

            if name_len > 1 {
                // Read UTF-16BE string (2 bytes per char)
                let mut chars = Vec::with_capacity(name_len);
                for _ in 0..name_len {
                    let mut char_buf = [0u8; 2];
                    cursor.read_exact(&mut char_buf)?;
                    chars.push(u16::from_be_bytes(char_buf));
                }

                // Convert to String, excluding null terminator
                char::decode_utf16(chars.iter().copied())
                    .filter_map(|r| r.ok())
                    .filter(|&c| c != '\0')
                    .collect()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        Ok(ColorSwatch { name, color })
    }

    /// Convert ACO color values to RGBA
    fn convert_aco_color(color_space: u16, values: [u16; 4]) -> EngineResult<[u8; 4]> {
        let space = AcoColorSpace::try_from(color_space);

        match space {
            Ok(AcoColorSpace::RGB) => {
                // RGB values are 0-65535
                Ok([
                    (values[0] >> 8) as u8,
                    (values[1] >> 8) as u8,
                    (values[2] >> 8) as u8,
                    255,
                ])
            }
            Ok(AcoColorSpace::HSB) => {
                // HSB values: H=0-65535 (0-360°), S=0-65535 (0-100%), B=0-65535 (0-100%)
                let h = values[0] as f32 / 65535.0 * 360.0;
                let s = values[1] as f32 / 65535.0;
                let b = values[2] as f32 / 65535.0;
                let (r, g, bl) = Self::hsb_to_rgb(h, s, b);
                Ok([r, g, bl, 255])
            }
            Ok(AcoColorSpace::CMYK) => {
                // CMYK values are 0-65535 (0-100%)
                let c = 1.0 - (values[0] as f32 / 65535.0);
                let m = 1.0 - (values[1] as f32 / 65535.0);
                let y = 1.0 - (values[2] as f32 / 65535.0);
                let k = 1.0 - (values[3] as f32 / 65535.0);

                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);

                Ok([
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    255,
                ])
            }
            Ok(AcoColorSpace::Lab) => {
                // Lab values: L=0-10000 (0-100), a=-12800 to 12700, b=-12800 to 12700
                let l = values[0] as f32 / 100.0;
                let a = (values[1] as i16) as f32 / 100.0;
                let b = (values[2] as i16) as f32 / 100.0;
                let (r, g, bl) = Self::lab_to_rgb(l, a, b);
                Ok([r, g, bl, 255])
            }
            Ok(AcoColorSpace::Grayscale) => {
                // Grayscale: 0-10000 (0-100%)
                let gray = (values[0] as f32 / 10000.0 * 255.0) as u8;
                Ok([gray, gray, gray, 255])
            }
            _ => {
                // Unknown or spot colors - return a fallback gray
                Ok([128, 128, 128, 255])
            }
        }
    }

    /// Parse an ASE (Adobe Swatch Exchange) file
    pub fn parse_ase(data: &[u8]) -> EngineResult<Vec<ColorSwatch>> {
        if data.len() < 12 {
            return Err(EngineError::ImportError("ASE file too small".into()));
        }

        // Check signature "ASEF"
        if &data[0..4] != b"ASEF" {
            return Err(EngineError::ImportError(
                "Invalid ASE file signature".into(),
            ));
        }

        let mut cursor = Cursor::new(&data[4..]);
        let mut swatches = Vec::new();

        // Read version (4 bytes: major.minor)
        let mut version_buf = [0u8; 4];
        cursor.read_exact(&mut version_buf)?;
        // Version is typically 1.0 = [0, 1, 0, 0]

        // Read block count (4 bytes)
        let mut count_buf = [0u8; 4];
        cursor.read_exact(&mut count_buf)?;
        let block_count = u32::from_be_bytes(count_buf);

        // Parse blocks
        for _ in 0..block_count {
            match Self::parse_ase_block(&mut cursor) {
                Ok(Some(swatch)) => swatches.push(swatch),
                Ok(None) => {} // Group start/end block
                Err(e) => log::warn!("Failed to parse ASE block: {}", e),
            }
        }

        Ok(swatches)
    }

    /// Parse a single ASE block
    fn parse_ase_block(cursor: &mut Cursor<&[u8]>) -> EngineResult<Option<ColorSwatch>> {
        // Read block type (2 bytes)
        let mut type_buf = [0u8; 2];
        cursor.read_exact(&mut type_buf)?;
        let block_type = u16::from_be_bytes(type_buf);

        // Read block length (4 bytes)
        let mut len_buf = [0u8; 4];
        cursor.read_exact(&mut len_buf)?;
        let block_length = u32::from_be_bytes(len_buf) as usize;

        match block_type {
            0xC001 => {
                // Group start - skip
                let mut skip = vec![0u8; block_length];
                cursor.read_exact(&mut skip)?;
                Ok(None)
            }
            0xC002 => {
                // Group end - nothing to read
                Ok(None)
            }
            0x0001 => {
                // Color entry
                Self::parse_ase_color(cursor, block_length)
            }
            _ => {
                // Unknown block type - skip
                let mut skip = vec![0u8; block_length];
                cursor.read_exact(&mut skip)?;
                Ok(None)
            }
        }
    }

    /// Parse an ASE color entry
    fn parse_ase_color(
        cursor: &mut Cursor<&[u8]>,
        _block_length: usize,
    ) -> EngineResult<Option<ColorSwatch>> {
        // Read name length (2 bytes) - in UTF-16 characters
        let mut name_len_buf = [0u8; 2];
        cursor.read_exact(&mut name_len_buf)?;
        let name_len = u16::from_be_bytes(name_len_buf) as usize;

        // Read name (UTF-16BE)
        let name = if name_len > 0 {
            let mut chars = Vec::with_capacity(name_len);
            for _ in 0..name_len {
                let mut char_buf = [0u8; 2];
                cursor.read_exact(&mut char_buf)?;
                chars.push(u16::from_be_bytes(char_buf));
            }
            char::decode_utf16(chars.iter().copied())
                .filter_map(|r| r.ok())
                .filter(|&c| c != '\0')
                .collect()
        } else {
            String::new()
        };

        // Read color model (4 bytes ASCII)
        let mut model_buf = [0u8; 4];
        cursor.read_exact(&mut model_buf)?;
        let model = &model_buf;

        // Read color values based on model
        let color = match model {
            b"RGB " => {
                // 3 x 4-byte floats (0.0-1.0)
                let r = Self::read_f32_be(cursor)?;
                let g = Self::read_f32_be(cursor)?;
                let b = Self::read_f32_be(cursor)?;
                [
                    (r.clamp(0.0, 1.0) * 255.0) as u8,
                    (g.clamp(0.0, 1.0) * 255.0) as u8,
                    (b.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                ]
            }
            b"CMYK" => {
                // 4 x 4-byte floats (0.0-1.0)
                let c = Self::read_f32_be(cursor)?;
                let m = Self::read_f32_be(cursor)?;
                let y = Self::read_f32_be(cursor)?;
                let k = Self::read_f32_be(cursor)?;

                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);

                [
                    (r.clamp(0.0, 1.0) * 255.0) as u8,
                    (g.clamp(0.0, 1.0) * 255.0) as u8,
                    (b.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                ]
            }
            b"LAB " => {
                // 3 x 4-byte floats: L (0-100), a (-128 to 127), b (-128 to 127)
                let l = Self::read_f32_be(cursor)?;
                let a = Self::read_f32_be(cursor)?;
                let b = Self::read_f32_be(cursor)?;
                let (r, g, bl) = Self::lab_to_rgb(l, a, b);
                [r, g, bl, 255]
            }
            b"Gray" => {
                // 1 x 4-byte float (0.0-1.0)
                let gray = Self::read_f32_be(cursor)?;
                let g = (gray.clamp(0.0, 1.0) * 255.0) as u8;
                [g, g, g, 255]
            }
            _ => {
                // Unknown model - skip remaining bytes and return gray
                [128, 128, 128, 255]
            }
        };

        // Read color type (2 bytes): 0=Global, 1=Spot, 2=Normal
        let mut type_buf = [0u8; 2];
        cursor.read_exact(&mut type_buf)?;

        Ok(Some(ColorSwatch { name, color }))
    }

    /// Read a big-endian f32
    fn read_f32_be(cursor: &mut Cursor<&[u8]>) -> EngineResult<f32> {
        let mut buf = [0u8; 4];
        cursor.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    /// Convert HSB to RGB
    fn hsb_to_rgb(h: f32, s: f32, b: f32) -> (u8, u8, u8) {
        if s == 0.0 {
            let v = (b * 255.0) as u8;
            return (v, v, v);
        }

        let h = h % 360.0;
        let h = h / 60.0;
        let i = h.floor() as i32;
        let f = h - i as f32;
        let p = b * (1.0 - s);
        let q = b * (1.0 - s * f);
        let t = b * (1.0 - s * (1.0 - f));

        let (r, g, bl) = match i % 6 {
            0 => (b, t, p),
            1 => (q, b, p),
            2 => (p, b, t),
            3 => (p, q, b),
            4 => (t, p, b),
            _ => (b, p, q),
        };

        (
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (bl * 255.0) as u8,
        )
    }

    /// Convert Lab to RGB
    fn lab_to_rgb(l: f32, a: f32, b: f32) -> (u8, u8, u8) {
        // Lab to XYZ
        let y = (l + 16.0) / 116.0;
        let x = a / 500.0 + y;
        let z = y - b / 200.0;

        fn lab_f_inv(t: f32) -> f32 {
            let delta = 6.0 / 29.0;
            if t > delta {
                t * t * t
            } else {
                3.0 * delta * delta * (t - 4.0 / 29.0)
            }
        }

        // D65 white point
        let x = 0.95047 * lab_f_inv(x);
        let y = 1.0 * lab_f_inv(y);
        let z = 1.08883 * lab_f_inv(z);

        // XYZ to linear RGB
        let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
        let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
        let b = 0.0557 * x - 0.2040 * y + 1.0570 * z;

        // Gamma correction and clamp
        fn gamma(c: f32) -> u8 {
            let c = if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            };
            (c.clamp(0.0, 1.0) * 255.0) as u8
        }

        (gamma(r), gamma(g), gamma(b))
    }

    /// Detect file type and parse accordingly
    pub fn parse_auto(data: &[u8]) -> EngineResult<Vec<ColorSwatch>> {
        if data.len() >= 4 && &data[0..4] == b"ASEF" {
            Self::parse_ase(data)
        } else {
            Self::parse_aco(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsb_to_rgb() {
        // Red
        let (r, g, b) = SwatchParser::hsb_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Green
        let (r, g, b) = SwatchParser::hsb_to_rgb(120.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);

        // Blue
        let (r, g, b) = SwatchParser::hsb_to_rgb(240.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_parse_empty() {
        assert!(SwatchParser::parse_aco(&[]).is_err());
        assert!(SwatchParser::parse_ase(&[]).is_err());
    }

    #[test]
    fn test_ase_signature() {
        let data = [b'A', b'S', b'E', b'F', 0, 1, 0, 0, 0, 0, 0, 0];
        let result = SwatchParser::parse_ase(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
