//! File format handling module
//!
//! Supports native .dcpaint format and import/export of common formats

use crate::canvas::Canvas;
use crate::error::{EngineError, EngineResult};
use crate::layer::LayerManager;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// Native DrawConnect format
    DcPaint,
    /// PNG image
    Png,
    /// JPEG image
    Jpeg,
    /// WebP image
    WebP,
    /// TIFF image
    Tiff,
    /// PSD (Photoshop)
    Psd,
    /// SVG vector
    Svg,
    /// PDF document
    Pdf,
    /// GIF animation
    Gif,
}

impl FileFormat {
    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "dcpaint" | "dcp" => Some(Self::DcPaint),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "webp" => Some(Self::WebP),
            "tiff" | "tif" => Some(Self::Tiff),
            "psd" => Some(Self::Psd),
            "svg" => Some(Self::Svg),
            "pdf" => Some(Self::Pdf),
            "gif" => Some(Self::Gif),
            _ => None,
        }
    }

    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::DcPaint => "dcpaint",
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::WebP => "webp",
            Self::Tiff => "tiff",
            Self::Psd => "psd",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
            Self::Gif => "gif",
        }
    }

    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::DcPaint => "application/x-dcpaint",
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::WebP => "image/webp",
            Self::Tiff => "image/tiff",
            Self::Psd => "image/vnd.adobe.photoshop",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
            Self::Gif => "image/gif",
        }
    }

    /// Check if format supports layers
    pub fn supports_layers(&self) -> bool {
        matches!(self, Self::DcPaint | Self::Psd | Self::Tiff)
    }

    /// Check if format supports transparency
    pub fn supports_transparency(&self) -> bool {
        !matches!(self, Self::Jpeg)
    }
}

/// Native file header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcPaintHeader {
    /// Magic number
    pub magic: [u8; 8],
    /// Version
    pub version: u32,
    /// Canvas width
    pub width: u32,
    /// Canvas height
    pub height: u32,
    /// DPI
    pub dpi: u32,
    /// Number of layers
    pub layer_count: u32,
    /// Color profile name
    pub color_profile: String,
    /// Compression type
    pub compression: CompressionType,
}

impl DcPaintHeader {
    /// Magic number for dcpaint files
    pub const MAGIC: [u8; 8] = *b"DCPAINT\0";

    /// Current file format version
    pub const VERSION: u32 = 1;

    /// Create new header
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            width,
            height,
            dpi: 300,
            layer_count: 0,
            color_profile: "sRGB".into(),
            compression: CompressionType::Zstd,
        }
    }

    /// Validate header
    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC && self.version <= Self::VERSION
    }
}

/// Compression type for file storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionType {
    /// No compression
    None,
    /// LZ4 fast compression
    Lz4,
    /// Zstd compression (default)
    Zstd,
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::Zstd
    }
}

/// Export options
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// Output format
    pub format: FileFormat,
    /// Quality (0-100, for lossy formats)
    pub quality: u8,
    /// Include alpha channel
    pub include_alpha: bool,
    /// Flatten layers
    pub flatten: bool,
    /// Color profile
    pub color_profile: Option<String>,
    /// DPI
    pub dpi: u32,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: FileFormat::Png,
            quality: 90,
            include_alpha: true,
            flatten: true,
            color_profile: None,
            dpi: 300,
        }
    }
}

/// File handler for saving and loading
pub struct FileHandler;

impl FileHandler {
    /// Save to native format
    pub fn save_native(
        path: &Path,
        canvas: &Canvas,
        layer_manager: &LayerManager,
    ) -> EngineResult<()> {
        let header = DcPaintHeader::new(canvas.width(), canvas.height());

        // Serialize header
        let header_data = bincode::serialize(&header)?;

        // Serialize layer data
        let layers_data = Self::serialize_layers(layer_manager)?;

        // Compress tile data
        let tiles_data = Self::compress_tiles(canvas)?;

        // Write file
        let mut file_data = Vec::new();
        file_data.extend_from_slice(&header_data);
        file_data.extend_from_slice(&layers_data);
        file_data.extend_from_slice(&tiles_data);

        std::fs::write(path, file_data)?;
        Ok(())
    }

    /// Load from native format
    pub fn load_native(path: &Path) -> EngineResult<(Canvas, LayerManager)> {
        let data = std::fs::read(path)?;

        if data.len() < 8 || &data[0..8] != DcPaintHeader::MAGIC {
            return Err(EngineError::UnsupportedFormat(
                "Invalid dcpaint file".into(),
            ));
        }

        // Parse header
        let header: DcPaintHeader = bincode::deserialize(&data)?;
        if !header.is_valid() {
            return Err(EngineError::UnsupportedFormat(
                "Unsupported dcpaint version".into(),
            ));
        }

        // Create canvas
        let canvas = Canvas::with_size(header.width, header.height)?;
        let layer_manager = LayerManager::with_canvas_size(header.width, header.height);

        Ok((canvas, layer_manager))
    }

    /// Export to image format
    pub fn export_image(
        path: &Path,
        pixels: &[u8],
        width: u32,
        height: u32,
        options: &ExportOptions,
    ) -> EngineResult<()> {
        use image::{ImageBuffer, ImageFormat, Rgba};

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, pixels.to_vec())
                .ok_or_else(|| EngineError::ImageError(image::ImageError::Limits(
                    image::error::LimitError::from_kind(
                        image::error::LimitErrorKind::DimensionError,
                    ),
                )))?;

        let format = match options.format {
            FileFormat::Png => ImageFormat::Png,
            FileFormat::Jpeg => ImageFormat::Jpeg,
            FileFormat::WebP => ImageFormat::WebP,
            FileFormat::Tiff => ImageFormat::Tiff,
            FileFormat::Gif => ImageFormat::Gif,
            _ => {
                return Err(EngineError::UnsupportedFormat(format!(
                    "Export not supported for {:?}",
                    options.format
                )))
            }
        };

        img.save_with_format(path, format)?;
        Ok(())
    }

    /// Import from image format
    pub fn import_image(path: &Path) -> EngineResult<(Vec<u8>, u32, u32)> {
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();

        Ok((pixels, width, height))
    }

    fn serialize_layers(layer_manager: &LayerManager) -> EngineResult<Vec<u8>> {
        // Simplified - would serialize layer metadata
        let data = Vec::new();
        Ok(data)
    }

    fn compress_tiles(canvas: &Canvas) -> EngineResult<Vec<u8>> {
        let pixels = canvas.get_pixels();
        let compressed = zstd::encode_all(pixels.as_slice(), 3)?;
        Ok(compressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(FileFormat::from_extension("png"), Some(FileFormat::Png));
        assert_eq!(FileFormat::from_extension("JPG"), Some(FileFormat::Jpeg));
        assert_eq!(FileFormat::from_extension("dcpaint"), Some(FileFormat::DcPaint));
        assert_eq!(FileFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_header_creation() {
        let header = DcPaintHeader::new(1920, 1080);
        assert!(header.is_valid());
        assert_eq!(header.width, 1920);
        assert_eq!(header.height, 1080);
    }

    #[test]
    fn test_format_features() {
        assert!(FileFormat::Png.supports_transparency());
        assert!(!FileFormat::Jpeg.supports_transparency());
        assert!(FileFormat::DcPaint.supports_layers());
        assert!(!FileFormat::Png.supports_layers());
    }
}
