//! PAT (Photoshop Pattern) file parser
//!
//! Supports Photoshop pattern files (.pat) for importing textures and patterns.

use crate::error::{EngineError, EngineResult};
use std::io::{Cursor, Read};

/// Imported pattern data from PAT file
#[derive(Debug, Clone)]
pub struct ImportedPattern {
    /// Pattern name
    pub name: String,
    /// Pattern width in pixels
    pub width: u32,
    /// Pattern height in pixels
    pub height: u32,
    /// RGBA pixel data
    pub data: Vec<u8>,
}

/// Color mode in PAT file
#[derive(Debug, Clone, Copy, PartialEq)]
enum PatColorMode {
    Bitmap = 0,
    Grayscale = 1,
    Indexed = 2,
    RGB = 3,
    CMYK = 4,
    Multichannel = 7,
    Duotone = 8,
    Lab = 9,
}

impl TryFrom<u16> for PatColorMode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PatColorMode::Bitmap),
            1 => Ok(PatColorMode::Grayscale),
            2 => Ok(PatColorMode::Indexed),
            3 => Ok(PatColorMode::RGB),
            4 => Ok(PatColorMode::CMYK),
            7 => Ok(PatColorMode::Multichannel),
            8 => Ok(PatColorMode::Duotone),
            9 => Ok(PatColorMode::Lab),
            _ => Err(()),
        }
    }
}

/// PAT file parser
pub struct PatParser;

impl PatParser {
    /// Parse a PAT file from bytes
    pub fn parse(data: &[u8]) -> EngineResult<Vec<ImportedPattern>> {
        if data.len() < 8 {
            return Err(EngineError::ImportError("PAT file too small".into()));
        }

        let mut cursor = Cursor::new(data);
        let mut patterns = Vec::new();

        // Read file version (4 bytes, big-endian)
        let mut version_buf = [0u8; 4];
        cursor.read_exact(&mut version_buf).map_err(|e| {
            EngineError::ImportError(format!("Failed to read PAT version: {}", e))
        })?;
        let version = u32::from_be_bytes(version_buf);

        if version != 1 && version != 2 {
            return Err(EngineError::ImportError(format!(
                "Unsupported PAT version: {}",
                version
            )));
        }

        // Read pattern count (4 bytes, big-endian)
        let mut count_buf = [0u8; 4];
        cursor.read_exact(&mut count_buf).map_err(|e| {
            EngineError::ImportError(format!("Failed to read pattern count: {}", e))
        })?;
        let pattern_count = u32::from_be_bytes(count_buf);

        // Parse each pattern
        for i in 0..pattern_count {
            match Self::parse_pattern(&mut cursor, version) {
                Ok(pattern) => patterns.push(pattern),
                Err(e) => {
                    log::warn!("Failed to parse pattern {}: {}", i, e);
                    // Try to continue with remaining patterns
                }
            }
        }

        Ok(patterns)
    }

    /// Parse a single pattern
    fn parse_pattern(cursor: &mut Cursor<&[u8]>, file_version: u32) -> EngineResult<ImportedPattern> {
        // Read pattern version (4 bytes)
        let mut version_buf = [0u8; 4];
        cursor.read_exact(&mut version_buf)?;
        let pattern_version = u32::from_be_bytes(version_buf);

        // Read color mode (4 bytes)
        let mut mode_buf = [0u8; 4];
        cursor.read_exact(&mut mode_buf)?;
        let color_mode_value = u32::from_be_bytes(mode_buf) as u16;
        let color_mode = PatColorMode::try_from(color_mode_value)
            .map_err(|_| EngineError::ImportError(format!("Unknown color mode: {}", color_mode_value)))?;

        // Read dimensions
        let mut height_buf = [0u8; 2];
        cursor.read_exact(&mut height_buf)?;
        let height = u16::from_be_bytes(height_buf) as u32;

        let mut width_buf = [0u8; 2];
        cursor.read_exact(&mut width_buf)?;
        let width = u16::from_be_bytes(width_buf) as u32;

        if width == 0 || height == 0 || width > 8192 || height > 8192 {
            return Err(EngineError::ImportError(format!(
                "Invalid pattern dimensions: {}x{}",
                width, height
            )));
        }

        // Read pattern name (Pascal string - length prefix + UTF-16BE chars)
        let name = Self::read_unicode_string(cursor)?;

        // Read pattern ID (Pascal string)
        let _pattern_id = Self::read_unicode_string(cursor)?;

        // For version 3+, there may be additional color table data
        if pattern_version >= 3 {
            // Skip color table if present
            if color_mode == PatColorMode::Indexed {
                let mut color_table = [0u8; 768]; // 256 * 3 bytes
                let _ = cursor.read_exact(&mut color_table);
            }
        }

        // Read Virtual Memory Array List (VMAL)
        let data = Self::read_pattern_data(cursor, width, height, color_mode, file_version)?;

        Ok(ImportedPattern {
            name: if name.is_empty() {
                format!("Pattern {}x{}", width, height)
            } else {
                name
            },
            width,
            height,
            data,
        })
    }

    /// Read a Unicode string (Pascal-style with length prefix)
    fn read_unicode_string(cursor: &mut Cursor<&[u8]>) -> EngineResult<String> {
        // Read length (4 bytes)
        let mut len_buf = [0u8; 4];
        cursor.read_exact(&mut len_buf)?;
        let char_count = u32::from_be_bytes(len_buf) as usize;

        if char_count == 0 {
            return Ok(String::new());
        }

        if char_count > 1024 {
            // Sanity check
            return Err(EngineError::ImportError("String too long".into()));
        }

        // Read UTF-16BE characters
        let mut chars = Vec::with_capacity(char_count);
        for _ in 0..char_count {
            let mut char_buf = [0u8; 2];
            cursor.read_exact(&mut char_buf)?;
            chars.push(u16::from_be_bytes(char_buf));
        }

        // Convert UTF-16 to String, filtering null terminators
        let string: String = char::decode_utf16(chars.iter().copied())
            .filter_map(|r| r.ok())
            .filter(|&c| c != '\0')
            .collect();

        Ok(string)
    }

    /// Read pattern pixel data
    fn read_pattern_data(
        cursor: &mut Cursor<&[u8]>,
        width: u32,
        height: u32,
        color_mode: PatColorMode,
        _file_version: u32,
    ) -> EngineResult<Vec<u8>> {
        // Read Virtual Memory Array List header
        let mut vmal_version_buf = [0u8; 4];
        cursor.read_exact(&mut vmal_version_buf)?;
        let _vmal_version = u32::from_be_bytes(vmal_version_buf);

        // Read length
        let mut vmal_length_buf = [0u8; 4];
        cursor.read_exact(&mut vmal_length_buf)?;
        let _vmal_length = u32::from_be_bytes(vmal_length_buf);

        // Read rectangle (top, left, bottom, right)
        let mut rect_buf = [0u8; 16];
        cursor.read_exact(&mut rect_buf)?;

        // Read number of channels
        let mut channels_buf = [0u8; 4];
        cursor.read_exact(&mut channels_buf)?;
        let channel_count = u32::from_be_bytes(channels_buf);

        // Determine bytes per pixel based on color mode
        let channels_needed = match color_mode {
            PatColorMode::Bitmap | PatColorMode::Grayscale | PatColorMode::Duotone => 1,
            PatColorMode::Indexed => 1,
            PatColorMode::RGB => 3,
            PatColorMode::CMYK => 4,
            PatColorMode::Lab => 3,
            PatColorMode::Multichannel => channel_count.min(4),
        };

        let pixel_count = (width * height) as usize;
        let mut rgba_data = vec![0u8; pixel_count * 4];

        // Read channel data
        let mut channel_data: Vec<Vec<u8>> = Vec::new();

        for _ch in 0..channel_count {
            // Read channel header
            let mut ch_written_buf = [0u8; 4];
            cursor.read_exact(&mut ch_written_buf)?;
            let is_written = u32::from_be_bytes(ch_written_buf) != 0;

            // Read channel length and pixel depth
            let mut ch_length_buf = [0u8; 4];
            cursor.read_exact(&mut ch_length_buf)?;
            let ch_length = u32::from_be_bytes(ch_length_buf) as usize;

            let mut depth_buf = [0u8; 4];
            cursor.read_exact(&mut depth_buf)?;
            let _depth = u32::from_be_bytes(depth_buf);

            // Read rectangle again
            let mut ch_rect_buf = [0u8; 16];
            cursor.read_exact(&mut ch_rect_buf)?;

            // Skip pixel depth (2 bytes)
            let mut pd_buf = [0u8; 2];
            cursor.read_exact(&mut pd_buf)?;

            // Read compression (1 byte)
            let mut comp_buf = [0u8; 1];
            cursor.read_exact(&mut comp_buf)?;
            let compression = comp_buf[0];

            if is_written && ch_length > 0 {
                // Read channel pixel data
                let data = if compression == 1 {
                    // RLE compressed
                    Self::decode_rle_channel(cursor, width, height)?
                } else {
                    // Raw data
                    let mut raw = vec![0u8; pixel_count];
                    cursor.read_exact(&mut raw)?;
                    raw
                };
                channel_data.push(data);
            } else {
                // Empty channel, fill with zeros or white depending on channel
                channel_data.push(vec![255u8; pixel_count]);
            }
        }

        // Convert to RGBA based on color mode
        match color_mode {
            PatColorMode::Grayscale | PatColorMode::Duotone => {
                if let Some(gray) = channel_data.first() {
                    for i in 0..pixel_count {
                        let g = gray.get(i).copied().unwrap_or(0);
                        rgba_data[i * 4] = g;
                        rgba_data[i * 4 + 1] = g;
                        rgba_data[i * 4 + 2] = g;
                        rgba_data[i * 4 + 3] = 255;
                    }
                }
                // Apply alpha channel if present
                if channel_data.len() > 1 {
                    if let Some(alpha) = channel_data.get(1) {
                        for i in 0..pixel_count {
                            rgba_data[i * 4 + 3] = alpha.get(i).copied().unwrap_or(255);
                        }
                    }
                }
            }
            PatColorMode::RGB => {
                for i in 0..pixel_count {
                    rgba_data[i * 4] = channel_data.get(0).and_then(|c| c.get(i).copied()).unwrap_or(0);
                    rgba_data[i * 4 + 1] = channel_data.get(1).and_then(|c| c.get(i).copied()).unwrap_or(0);
                    rgba_data[i * 4 + 2] = channel_data.get(2).and_then(|c| c.get(i).copied()).unwrap_or(0);
                    rgba_data[i * 4 + 3] = channel_data.get(3).and_then(|c| c.get(i).copied()).unwrap_or(255);
                }
            }
            PatColorMode::CMYK => {
                // Convert CMYK to RGB
                for i in 0..pixel_count {
                    let c = channel_data.get(0).and_then(|ch| ch.get(i).copied()).unwrap_or(0) as f32 / 255.0;
                    let m = channel_data.get(1).and_then(|ch| ch.get(i).copied()).unwrap_or(0) as f32 / 255.0;
                    let y = channel_data.get(2).and_then(|ch| ch.get(i).copied()).unwrap_or(0) as f32 / 255.0;
                    let k = channel_data.get(3).and_then(|ch| ch.get(i).copied()).unwrap_or(0) as f32 / 255.0;

                    let r = (1.0 - c) * (1.0 - k);
                    let g = (1.0 - m) * (1.0 - k);
                    let b = (1.0 - y) * (1.0 - k);

                    rgba_data[i * 4] = (r * 255.0) as u8;
                    rgba_data[i * 4 + 1] = (g * 255.0) as u8;
                    rgba_data[i * 4 + 2] = (b * 255.0) as u8;
                    rgba_data[i * 4 + 3] = channel_data.get(4).and_then(|c| c.get(i).copied()).unwrap_or(255);
                }
            }
            PatColorMode::Lab => {
                // Convert Lab to RGB (simplified conversion)
                for i in 0..pixel_count {
                    let l = channel_data.get(0).and_then(|ch| ch.get(i).copied()).unwrap_or(128) as f32;
                    let a = channel_data.get(1).and_then(|ch| ch.get(i).copied()).unwrap_or(128) as f32;
                    let b = channel_data.get(2).and_then(|ch| ch.get(i).copied()).unwrap_or(128) as f32;

                    // Simplified Lab to RGB conversion
                    let (r, g, bl) = Self::lab_to_rgb(l, a, b);
                    rgba_data[i * 4] = r;
                    rgba_data[i * 4 + 1] = g;
                    rgba_data[i * 4 + 2] = bl;
                    rgba_data[i * 4 + 3] = channel_data.get(3).and_then(|c| c.get(i).copied()).unwrap_or(255);
                }
            }
            _ => {
                // For other modes, just use grayscale or first channel
                if let Some(data) = channel_data.first() {
                    for i in 0..pixel_count {
                        let g = data.get(i).copied().unwrap_or(128);
                        rgba_data[i * 4] = g;
                        rgba_data[i * 4 + 1] = g;
                        rgba_data[i * 4 + 2] = g;
                        rgba_data[i * 4 + 3] = 255;
                    }
                }
            }
        }

        Ok(rgba_data)
    }

    /// Decode RLE compressed channel data
    fn decode_rle_channel(
        cursor: &mut Cursor<&[u8]>,
        width: u32,
        height: u32,
    ) -> EngineResult<Vec<u8>> {
        let pixel_count = (width * height) as usize;
        let mut result = Vec::with_capacity(pixel_count);

        // Read row byte counts (2 bytes per row)
        let row_count = height as usize;
        let mut row_lengths = Vec::with_capacity(row_count);
        for _ in 0..row_count {
            let mut len_buf = [0u8; 2];
            cursor.read_exact(&mut len_buf)?;
            row_lengths.push(u16::from_be_bytes(len_buf) as usize);
        }

        // Decode each row
        for row_len in row_lengths {
            let row_start = result.len();
            let target_len = row_start + width as usize;

            let mut bytes_read = 0;
            while bytes_read < row_len && result.len() < target_len {
                let mut header_buf = [0u8; 1];
                cursor.read_exact(&mut header_buf)?;
                bytes_read += 1;

                let n = header_buf[0] as i8;

                if n >= 0 {
                    // Literal run: copy n+1 bytes
                    let count = (n as usize + 1).min(target_len - result.len());
                    let mut data = vec![0u8; count];
                    cursor.read_exact(&mut data)?;
                    bytes_read += count;
                    result.extend_from_slice(&data);
                } else if n > -128 {
                    // Repeat run: repeat next byte (-n+1) times
                    let mut byte_buf = [0u8; 1];
                    cursor.read_exact(&mut byte_buf)?;
                    bytes_read += 1;
                    let count = (-n as usize + 1).min(target_len - result.len());
                    for _ in 0..count {
                        result.push(byte_buf[0]);
                    }
                }
                // n == -128: no-op
            }

            // Pad row if needed
            while result.len() < target_len {
                result.push(0);
            }
        }

        // Ensure correct size
        result.resize(pixel_count, 0);
        Ok(result)
    }

    /// Simplified Lab to RGB conversion
    fn lab_to_rgb(l: f32, a: f32, b: f32) -> (u8, u8, u8) {
        // Normalize Lab values
        let l = l / 255.0 * 100.0;
        let a = a - 128.0;
        let b = b - 128.0;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        // Test that parser can handle empty/invalid data gracefully
        let result = PatParser::parse(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_version() {
        // Version 99 should be unsupported
        let data = [0, 0, 0, 99, 0, 0, 0, 0];
        let result = PatParser::parse(&data);
        assert!(result.is_err());
    }
}
