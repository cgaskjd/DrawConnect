//! # Plugin System
//!
//! DrawConnect plugin system for extending functionality with custom brushes, filters, and tools.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                   Plugin Manager                         │
//! ├─────────────┬─────────────┬─────────────┬───────────────┤
//! │   Loader    │  Registry   │   Sandbox   │    Runtime    │
//! │  (install)  │  (persist)  │ (security)  │  (JS/WASM)    │
//! └─────────────┴─────────────┴─────────────┴───────────────┘
//! ```

pub mod types;
pub mod permissions;
pub mod manifest;
pub mod registry;
pub mod manager;
pub mod loader;
pub mod sandbox;

// Re-exports
pub use types::*;
pub use permissions::Permission;
pub use manifest::PluginManifest;
pub use registry::PluginRegistry;
pub use manager::PluginManager;
pub use loader::PluginLoader;
pub use sandbox::PluginSandbox;
