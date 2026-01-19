//! ABR (Adobe Brush) file parser
//!
//! Supports Photoshop brush files in multiple versions:
//! - ABR v1/v2: Older format (Photoshop 6 and earlier)
//! - ABR v6+: Modern format (Photoshop 7 and later)
//!
//! Reference: https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/

use crate::error::{EngineError, EngineResult};
use std::io::{Cursor, Read};

/// Imported brush data from ABR file
#[derive(Debug, Clone)]
pub struct ImportedBrush {
    /// Brush name
    pub name: String,
    /// Brush diameter in pixels
    pub diameter: u32,
    /// Brush hardness (0.0 - 1.0)
    pub hardness: f32,
    /// Brush spacing (0.0 - 1.0, as ratio of diameter)
    pub spacing: f32,
    /// Brush angle in degrees
    pub angle: f32,
    /// Brush roundness (0.0 - 1.0)
    pub roundness: f32,
    /// Brush tip image (grayscale, diameter x diameter)
    /// None for computed/parametric brushes
    pub tip_image: Option<BrushTipImage>,
}

/// Brush tip image data
#[derive(Debug, Clone)]
pub struct BrushTipImage {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Grayscale pixel data (0-255, 0=transparent, 255=opaque)
    pub data: Vec<u8>,
}

/// ABR file parser
pub struct AbrParser;

impl AbrParser {
    /// Parse an ABR file from bytes
    pub fn parse(data: &[u8]) -> EngineResult<Vec<ImportedBrush>> {
        if data.len() < 4 {
            return Err(EngineError::ImportError("ABR file too small".into()));
        }

        // Read version (big-endian u16)
        let version = u16::from_be_bytes([data[0], data[1]]);

        match version {
            1 | 2 => Self::parse_v1_v2(data),
            6..=10 => Self::parse_v6_plus(data),
            _ => Err(EngineError::ImportError(format!(
                "Unsupported ABR version: {}",
                version
            ))),
        }
    }

    /// Parse ABR v1/v2 format (Photoshop 6 and earlier)
    fn parse_v1_v2(data: &[u8]) -> EngineResult<Vec<ImportedBrush>> {
        let mut brushes = Vec::new();
        let mut cursor = Cursor::new(data);

        // Skip version (2 bytes)
        let mut header = [0u8; 4];
        cursor.read_exact(&mut header).map_err(|e| {
            EngineError::ImportError(format!("Failed to read ABR header: {}", e))
        })?;

        let version = u16::from_be_bytes([header[0], header[1]]);
        let brush_count = u16::from_be_bytes([header[2], header[3]]);

        for i in 0..brush_count {
            match Self::parse_v1_brush(&mut cursor, version) {
                Ok(brush) => brushes.push(brush),
                Err(e) => {
                    log::warn!("Failed to parse brush {}: {}", i, e);
                    // Continue parsing other brushes
                }
            }
        }

        Ok(brushes)
    }

    /// Parse a single brush from v1/v2 format
    fn parse_v1_brush(cursor: &mut Cursor<&[u8]>, version: u16) -> EngineResult<ImportedBrush> {
        // Read brush type (2 bytes)
        let mut type_buf = [0u8; 2];
        cursor.read_exact(&mut type_buf).map_err(|e| {
            EngineError::ImportError(format!("Failed to read brush type: {}", e))
        })?;
        let brush_type = u16::from_be_bytes(type_buf);

        // Read brush size (4 bytes)
        let mut size_buf = [0u8; 4];
        cursor.read_exact(&mut size_buf).map_err(|e| {
            EngineError::ImportError(format!("Failed to read brush size: {}", e))
        })?;
        let brush_size = u32::from_be_bytes(size_buf);

        match brush_type {
            1 => Self::parse_computed_brush(cursor, brush_size, version),
            2 => Self::parse_sampled_brush(cursor, brush_size, version),
            _ => Err(EngineError::ImportError(format!(
                "Unknown brush type: {}",
                brush_type
            ))),
        }
    }

    /// Parse a computed (parametric) brush
    fn parse_computed_brush(
        cursor: &mut Cursor<&[u8]>,
        _size: u32,
        _version: u16,
    ) -> EngineResult<ImportedBrush> {
        // Skip misc (4 bytes)
        let mut misc = [0u8; 4];
        cursor.read_exact(&mut misc)?;

        // Read spacing (2 bytes, percentage)
        let mut spacing_buf = [0u8; 2];
        cursor.read_exact(&mut spacing_buf)?;
        let spacing = u16::from_be_bytes(spacing_buf) as f32 / 100.0;

        // Read diameter (2 bytes)
        let mut diameter_buf = [0u8; 2];
        cursor.read_exact(&mut diameter_buf)?;
        let diameter = u16::from_be_bytes(diameter_buf) as u32;

        // Read roundness (2 bytes, percentage)
        let mut roundness_buf = [0u8; 2];
        cursor.read_exact(&mut roundness_buf)?;
        let roundness = u16::from_be_bytes(roundness_buf) as f32 / 100.0;

        // Read angle (2 bytes, degrees)
        let mut angle_buf = [0u8; 2];
        cursor.read_exact(&mut angle_buf)?;
        let angle = i16::from_be_bytes(angle_buf) as f32;

        // Read hardness (2 bytes, percentage)
        let mut hardness_buf = [0u8; 2];
        cursor.read_exact(&mut hardness_buf)?;
        let hardness = u16::from_be_bytes(hardness_buf) as f32 / 100.0;

        Ok(ImportedBrush {
            name: format!("Round {}", diameter),
            diameter,
            hardness,
            spacing,
            angle,
            roundness,
            tip_image: None,
        })
    }

    /// Parse a sampled (image-based) brush
    fn parse_sampled_brush(
        cursor: &mut Cursor<&[u8]>,
        _size: u32,
        _version: u16,
    ) -> EngineResult<ImportedBrush> {
        // Skip misc (4 bytes)
        let mut misc = [0u8; 4];
        cursor.read_exact(&mut misc)?;

        // Read spacing (2 bytes, percentage)
        let mut spacing_buf = [0u8; 2];
        cursor.read_exact(&mut spacing_buf)?;
        let spacing = u16::from_be_bytes(spacing_buf) as f32 / 100.0;

        // Skip antialiasing flag (2 bytes)
        let mut aa_buf = [0u8; 2];
        cursor.read_exact(&mut aa_buf)?;

        // Read bounds (top, left, bottom, right - each 2 bytes)
        let mut bounds = [0u8; 8];
        cursor.read_exact(&mut bounds)?;
        let top = i16::from_be_bytes([bounds[0], bounds[1]]) as i32;
        let left = i16::from_be_bytes([bounds[2], bounds[3]]) as i32;
        let bottom = i16::from_be_bytes([bounds[4], bounds[5]]) as i32;
        let right = i16::from_be_bytes([bounds[6], bounds[7]]) as i32;

        let width = (right - left).max(1) as u32;
        let height = (bottom - top).max(1) as u32;

        // Read depth (2 bytes)
        let mut depth_buf = [0u8; 2];
        cursor.read_exact(&mut depth_buf)?;
        let depth = u16::from_be_bytes(depth_buf);

        // Read image data (RLE compressed or raw)
        let tip_image = if depth == 8 {
            // 8-bit grayscale
            Some(Self::read_brush_image(cursor, width, height)?)
        } else {
            None
        };

        let diameter = width.max(height);

        Ok(ImportedBrush {
            name: format!("Sampled {}", diameter),
            diameter,
            hardness: 1.0,
            spacing,
            angle: 0.0,
            roundness: 1.0,
            tip_image,
        })
    }

    /// Read brush tip image data (RLE compressed)
    fn read_brush_image(
        cursor: &mut Cursor<&[u8]>,
        width: u32,
        height: u32,
    ) -> EngineResult<BrushTipImage> {
        // Read compression flag (2 bytes)
        let mut compression_buf = [0u8; 2];
        cursor.read_exact(&mut compression_buf)?;
        let compression = u16::from_be_bytes(compression_buf);

        let data = if compression == 0 {
            // Raw data
            let size = (width * height) as usize;
            let mut data = vec![0u8; size];
            cursor.read_exact(&mut data)?;
            data
        } else {
            // RLE compressed
            Self::decode_rle(cursor, width, height)?
        };

        Ok(BrushTipImage {
            width,
            height,
            data,
        })
    }

    /// Decode RLE compressed image data
    fn decode_rle(
        cursor: &mut Cursor<&[u8]>,
        width: u32,
        height: u32,
    ) -> EngineResult<Vec<u8>> {
        // Skip row byte counts (2 bytes per row)
        let mut skip = vec![0u8; (height * 2) as usize];
        cursor.read_exact(&mut skip)?;

        let mut data = Vec::with_capacity((width * height) as usize);

        for _row in 0..height {
            let mut col = 0u32;
            while col < width {
                let mut header = [0u8; 1];
                if cursor.read_exact(&mut header).is_err() {
                    break;
                }
                let n = header[0] as i8;

                if n >= 0 {
                    // Literal run: copy n+1 bytes
                    let count = (n as u32 + 1).min(width - col);
                    let mut bytes = vec![0u8; count as usize];
                    cursor.read_exact(&mut bytes)?;
                    data.extend_from_slice(&bytes);
                    col += count;
                } else if n > -128 {
                    // Repeat run: repeat next byte (-n+1) times
                    let mut byte = [0u8; 1];
                    cursor.read_exact(&mut byte)?;
                    let count = (-n as u32 + 1).min(width - col);
                    for _ in 0..count {
                        data.push(byte[0]);
                    }
                    col += count;
                }
                // n == -128: no-op
            }
        }

        // Pad if necessary
        let expected = (width * height) as usize;
        data.resize(expected, 0);

        Ok(data)
    }

    /// Parse ABR v6+ format (Photoshop 7 and later)
    fn parse_v6_plus(data: &[u8]) -> EngineResult<Vec<ImportedBrush>> {
        let mut brushes = Vec::new();
        let mut pos = 4; // Skip version and subversion

        // Look for 8BIM sections
        while pos + 12 < data.len() {
            // Check for "8BIM" signature
            if &data[pos..pos + 4] != b"8BIM" {
                pos += 1;
                continue;
            }
            pos += 4;

            // Read section key (4 bytes)
            let key = &data[pos..pos + 4];
            pos += 4;

            // Read section size (4 bytes, big-endian)
            if pos + 4 > data.len() {
                break;
            }
            let size = u32::from_be_bytes([
                data[pos],
                data[pos + 1],
                data[pos + 2],
                data[pos + 3],
            ]) as usize;
            pos += 4;

            // Process section based on key
            match key {
                b"samp" => {
                    // Brush samples section
                    if pos + size <= data.len() {
                        match Self::parse_samp_section(&data[pos..pos + size]) {
                            Ok(mut sample_brushes) => brushes.append(&mut sample_brushes),
                            Err(e) => log::warn!("Failed to parse samp section: {}", e),
                        }
                    }
                }
                b"desc" | b"patt" | b"phry" => {
                    // Description, pattern, hierarchy sections - skip for now
                }
                _ => {
                    // Unknown section - skip
                }
            }

            pos += size;

            // Align to 4-byte boundary
            if size % 4 != 0 {
                pos += 4 - (size % 4);
            }
        }

        // If no brushes found via sections, try legacy parsing
        if brushes.is_empty() {
            return Self::parse_v6_legacy(data);
        }

        Ok(brushes)
    }

    /// Parse brush samples section
    fn parse_samp_section(data: &[u8]) -> EngineResult<Vec<ImportedBrush>> {
        let mut brushes = Vec::new();
        let mut pos = 0;

        while pos + 12 < data.len() {
            // Read sample size (4 bytes)
            let sample_size = u32::from_be_bytes([
                data[pos],
                data[pos + 1],
                data[pos + 2],
                data[pos + 3],
            ]) as usize;
            pos += 4;

            if sample_size == 0 || pos + sample_size > data.len() {
                break;
            }

            // Parse individual brush sample
            match Self::parse_brush_sample(&data[pos..pos + sample_size]) {
                Ok(brush) => brushes.push(brush),
                Err(e) => log::warn!("Failed to parse brush sample: {}", e),
            }

            pos += sample_size;
        }

        Ok(brushes)
    }

    /// Parse a single brush sample from v6+ format
    fn parse_brush_sample(data: &[u8]) -> EngineResult<ImportedBrush> {
        if data.len() < 47 {
            return Err(EngineError::ImportError(
                "Brush sample data too small".into(),
            ));
        }

        let mut pos = 0;

        // Skip unknown bytes (1 byte)
        pos += 1;

        // Read bounds (top, left, bottom, right - each 4 bytes)
        let top = i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        let left = i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        let bottom = i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        let right = i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        let width = (right - left).max(1) as u32;
        let height = (bottom - top).max(1) as u32;

        // Skip depth (2 bytes)
        pos += 2;

        // Read compression (1 byte)
        let compression = data[pos];
        pos += 1;

        // Read image data
        let remaining = &data[pos..];
        let tip_data = if compression == 0 {
            // Raw data
            let size = (width * height) as usize;
            if remaining.len() >= size {
                remaining[..size].to_vec()
            } else {
                vec![128u8; size] // Fallback to mid-gray
            }
        } else {
            // RLE compressed
            Self::decode_rle_v6(remaining, width, height)?
        };

        let diameter = width.max(height);

        Ok(ImportedBrush {
            name: format!("Brush {}", diameter),
            diameter,
            hardness: 1.0,
            spacing: 0.25,
            angle: 0.0,
            roundness: 1.0,
            tip_image: Some(BrushTipImage {
                width,
                height,
                data: tip_data,
            }),
        })
    }

    /// Decode RLE compressed data for v6+ format
    fn decode_rle_v6(data: &[u8], width: u32, height: u32) -> EngineResult<Vec<u8>> {
        let mut result = Vec::with_capacity((width * height) as usize);
        let mut pos = 0;

        // Skip row byte counts (can be 2 or 4 bytes per row depending on format)
        let rows_header_size = (height * 2) as usize;
        if data.len() < rows_header_size {
            return Err(EngineError::ImportError("RLE data too small".into()));
        }
        pos += rows_header_size;

        while pos < data.len() && result.len() < (width * height) as usize {
            let header = data[pos] as i8;
            pos += 1;

            if header >= 0 {
                // Literal run
                let count = (header as usize + 1).min(data.len() - pos);
                result.extend_from_slice(&data[pos..pos + count]);
                pos += count;
            } else if header > -128 {
                // Repeat run
                if pos >= data.len() {
                    break;
                }
                let byte = data[pos];
                pos += 1;
                let count = -header as usize + 1;
                for _ in 0..count {
                    result.push(byte);
                }
            }
            // header == -128: no-op
        }

        // Pad if necessary
        let expected = (width * height) as usize;
        result.resize(expected, 0);

        Ok(result)
    }

    /// Fallback parser for v6+ files without proper section headers
    fn parse_v6_legacy(data: &[u8]) -> EngineResult<Vec<ImportedBrush>> {
        // Try to find brush data by scanning for common patterns
        let mut brushes = Vec::new();

        // Simple heuristic: look for plausible brush dimensions
        let mut pos = 4; // Skip version
        while pos + 20 < data.len() {
            // Look for reasonable dimension values (1-2048 pixels)
            let potential_width =
                u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            let potential_height =
                u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);

            if potential_width > 0
                && potential_width <= 2048
                && potential_height > 0
                && potential_height <= 2048
            {
                let diameter = potential_width.max(potential_height);
                brushes.push(ImportedBrush {
                    name: format!("Imported Brush {}", brushes.len() + 1),
                    diameter,
                    hardness: 0.5,
                    spacing: 0.25,
                    angle: 0.0,
                    roundness: 1.0,
                    tip_image: None,
                });
            }

            pos += 1;
        }

        if brushes.is_empty() {
            // Return a default brush if nothing was found
            brushes.push(ImportedBrush {
                name: "Imported Brush".into(),
                diameter: 20,
                hardness: 0.5,
                spacing: 0.25,
                angle: 0.0,
                roundness: 1.0,
                tip_image: None,
            });
        }

        Ok(brushes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        // Test that parser can handle empty/invalid data gracefully
        let result = AbrParser::parse(&[]);
        assert!(result.is_err());

        let result = AbrParser::parse(&[0, 1]); // Too small
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_version() {
        // Version 255 should be unsupported
        let data = [0, 255, 0, 0];
        let result = AbrParser::parse(&data);
        assert!(result.is_err());
    }
}
