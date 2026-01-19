//! # Render Filters Module
//!
//! Provides rendering and effect filters.
//!
//! ## Supported Filters
//!
//! - **Vignette**: Darkens image edges
//! - **LensFlare**: Simulates camera lens flare
//! - **Clouds**: Generates procedural cloud texture

mod vignette;
mod lens_flare;
mod clouds;

pub use vignette::Vignette;
pub use lens_flare::{LensFlare, FlareStyle};
pub use clouds::Clouds;
