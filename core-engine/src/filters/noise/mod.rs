//! Noise filters
//!
//! Add and reduce noise effects.

mod add_noise;
mod reduce_noise;

pub use add_noise::{AddNoise, NoiseType};
pub use reduce_noise::ReduceNoise;
