//! Stylize filters
//!
//! Artistic and stylized effects.

mod find_edges;
mod emboss;
mod pixelate;
mod oil_paint;

pub use find_edges::FindEdges;
pub use emboss::Emboss;
pub use pixelate::Pixelate;
pub use oil_paint::OilPaint;
