//! DrawConnect Core Engine - WebAssembly Bindings
//!
//! This crate provides WASM bindings for the DrawConnect core engine,
//! allowing it to run in web browsers.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;

mod bridge;
mod engine;
mod types;

pub use engine::WasmDrawEngine;
pub use types::*;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Get the engine version
#[wasm_bindgen]
pub fn get_version() -> String {
    drawconnect_core::VERSION.to_string()
}
