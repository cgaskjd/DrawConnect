//! Plugin type definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    /// Brush plugin
    Brush,
    /// Filter/effect plugin
    Filter,
    /// Tool plugin
    Tool,
    /// Mixed plugin (multiple types)
    Mixed,
}

/// Plugin runtime type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginRuntime {
    /// JavaScript runtime
    JavaScript,
    /// WebAssembly runtime
    Wasm,
    /// Both JS and WASM
    Hybrid,
}

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginState {
    /// Plugin is installed but not activated
    Installed,
    /// Plugin is enabled and running
    Enabled,
    /// Plugin is disabled
    Disabled,
    /// Plugin encountered an error
    Error,
}

/// Plugin information for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Unique plugin ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Version string
    pub version: String,
    /// Description
    pub description: String,
    /// Author name
    pub author: String,
    /// Plugin type
    pub plugin_type: PluginType,
    /// Current state
    pub state: PluginState,
    /// Icon path (optional)
    pub icon: Option<String>,
}

/// Detailed plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDetail {
    /// Basic info
    pub info: PluginInfo,
    /// Full manifest
    pub manifest: serde_json::Value,
    /// Settings schema (if any)
    pub settings_schema: Option<serde_json::Value>,
    /// Current settings
    pub current_settings: serde_json::Value,
    /// Readme content (if available)
    pub readme: Option<String>,
    /// Changelog content (if available)
    pub changelog: Option<String>,
}

/// Brush capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushCapability {
    /// Brush ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Category
    pub category: String,
    /// Icon path
    pub icon: Option<String>,
}

/// Filter capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCapability {
    /// Filter ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Category
    pub category: String,
    /// Icon path
    pub icon: Option<String>,
}

/// Tool capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Tool ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Category
    pub category: String,
    /// Icon path
    pub icon: String,
}

/// Panel capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelCapability {
    /// Panel ID
    pub id: String,
    /// Panel title
    pub title: String,
    /// Icon path
    pub icon: Option<String>,
    /// Position (left, right, bottom, float)
    pub position: String,
}

/// Plugin capabilities (what the plugin provides)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginCapabilities {
    /// Brushes provided by this plugin
    #[serde(default)]
    pub brushes: Vec<BrushCapability>,
    /// Filters provided by this plugin
    #[serde(default)]
    pub filters: Vec<FilterCapability>,
    /// Tools provided by this plugin
    #[serde(default)]
    pub tools: Vec<ToolCapability>,
    /// UI panels provided by this plugin
    #[serde(default)]
    pub panels: Vec<PanelCapability>,
}

/// Plugin dependencies
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginDependencies {
    /// Required plugins
    #[serde(default)]
    pub plugins: Vec<String>,
    /// Required features
    #[serde(default)]
    pub features: Vec<String>,
}

/// Plugin settings schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettingsSchema {
    /// Settings schema as JSON Schema
    pub schema: HashMap<String, serde_json::Value>,
}

/// Store-related plugin metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoreMetadata {
    /// Price (0 for free)
    #[serde(default)]
    pub price: f32,
    /// Screenshot URLs
    #[serde(default)]
    pub screenshots: Vec<String>,
    /// Preview video URL
    pub preview: Option<String>,
}

/// Store search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSearchResult {
    /// Found plugins
    pub plugins: Vec<StorePluginInfo>,
    /// Total count
    pub total: u32,
    /// Current page
    pub page: u32,
    /// Page size
    pub page_size: u32,
}

/// Plugin info from store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorePluginInfo {
    /// Plugin ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Version
    pub version: String,
    /// Description
    pub description: String,
    /// Author
    pub author: String,
    /// Download count
    pub downloads: u32,
    /// Rating (0-5)
    pub rating: f32,
    /// Price
    pub price: f32,
    /// Screenshots
    pub screenshots: Vec<String>,
}

/// Plugin update information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginUpdateInfo {
    /// Plugin ID
    pub id: String,
    /// Current installed version
    pub current_version: String,
    /// Latest available version
    pub latest_version: String,
    /// Changelog for the update
    pub changelog: String,
}
