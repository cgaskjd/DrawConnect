//! Error types for the DrawConnect Core Engine

use thiserror::Error;

/// Engine error type
#[derive(Error, Debug)]
pub enum EngineError {
    /// Canvas dimension exceeds maximum allowed size
    #[error("Canvas size {0}x{1} exceeds maximum allowed {2}x{2}")]
    CanvasTooLarge(u32, u32, u32),

    /// Invalid canvas dimensions
    #[error("Invalid canvas dimensions: {0}x{1}")]
    InvalidCanvasSize(u32, u32),

    /// Layer not found
    #[error("Layer not found: {0}")]
    LayerNotFound(uuid::Uuid),

    /// Layer index out of bounds
    #[error("Layer index {0} out of bounds (max: {1})")]
    LayerIndexOutOfBounds(usize, usize),

    /// Invalid blend mode
    #[error("Invalid blend mode: {0}")]
    InvalidBlendMode(String),

    /// Brush not found
    #[error("Brush not found: {0}")]
    BrushNotFound(String),

    /// Invalid brush settings
    #[error("Invalid brush settings: {0}")]
    InvalidBrushSettings(String),

    /// Render error
    #[error("Render error: {0}")]
    RenderError(String),

    /// GPU initialization failed
    #[error("GPU initialization failed: {0}")]
    GpuInitFailed(String),

    /// Memory allocation failed
    #[error("Memory allocation failed: requested {0} bytes")]
    MemoryAllocationFailed(usize),

    /// File I/O error
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Image processing error
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    /// Color conversion error
    #[error("Color conversion error: {0}")]
    ColorConversionError(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Tile not found
    #[error("Tile not found at position ({0}, {1})")]
    TileNotFound(u32, u32),

    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Format not supported
    #[error("Format not supported: {0}")]
    UnsupportedFormat(String),

    /// Plugin error
    #[error("Plugin error: {0}")]
    PluginError(String),

    /// Plugin not found
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    /// Import error
    #[error("Import error: {0}")]
    ImportError(String),
}

/// Result type alias for engine operations
pub type EngineResult<T> = Result<T, EngineError>;

impl From<serde_json::Error> for EngineError {
    fn from(err: serde_json::Error) -> Self {
        EngineError::SerializationError(err.to_string())
    }
}

impl From<bincode::Error> for EngineError {
    fn from(err: bincode::Error) -> Self {
        EngineError::SerializationError(err.to_string())
    }
}
