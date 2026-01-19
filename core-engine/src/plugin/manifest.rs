//! Plugin manifest parsing and validation

use serde::{Deserialize, Serialize};
use crate::error::{EngineError, EngineResult};
use super::permissions::Permission;
use super::types::*;

/// Plugin author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAuthor {
    /// Author name
    pub name: String,
    /// Author email
    pub email: Option<String>,
    /// Author website
    pub url: Option<String>,
}

/// Plugin manifest (manifest.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier (e.g., "com.example.my-plugin")
    pub id: String,

    /// Display name
    pub name: String,

    /// Semantic version string
    pub version: String,

    /// Required API version
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    /// Plugin description
    pub description: String,

    /// Author information
    pub author: PluginAuthor,

    /// License identifier
    pub license: String,

    /// Repository URL
    pub repository: Option<String>,

    /// Plugin type
    #[serde(rename = "type")]
    pub plugin_type: PluginType,

    /// Runtime type
    pub runtime: PluginRuntime,

    /// Main JavaScript entry point
    pub main: Option<String>,

    /// WASM module path
    pub wasm: Option<String>,

    /// Required permissions
    #[serde(default)]
    pub permissions: Vec<Permission>,

    /// Plugin capabilities
    #[serde(default)]
    pub capabilities: PluginCapabilities,

    /// Settings schema
    pub settings: Option<PluginSettingsSchema>,

    /// Plugin dependencies
    pub dependencies: Option<PluginDependencies>,

    /// Supported locales
    #[serde(default)]
    pub locales: Vec<String>,

    /// Keywords for search
    #[serde(default)]
    pub keywords: Vec<String>,

    /// Category for organization
    pub category: Option<String>,

    /// Store metadata
    pub store: Option<StoreMetadata>,
}

impl PluginManifest {
    /// Parse manifest from JSON string
    pub fn from_json(json: &str) -> EngineResult<Self> {
        serde_json::from_str(json).map_err(|e| {
            EngineError::PluginError(format!("Failed to parse manifest: {}", e))
        })
    }

    /// Parse manifest from file
    pub fn from_file(path: &std::path::Path) -> EngineResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            EngineError::PluginError(format!("Failed to read manifest file: {}", e))
        })?;
        Self::from_json(&content)
    }

    /// Validate the manifest
    pub fn validate(&self) -> EngineResult<()> {
        // Validate ID format
        if self.id.is_empty() {
            return Err(EngineError::PluginError("Plugin ID cannot be empty".into()));
        }
        if !self.id.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_') {
            return Err(EngineError::PluginError(
                "Plugin ID can only contain alphanumeric characters, dots, hyphens, and underscores".into()
            ));
        }

        // Validate version format (semver)
        if !is_valid_semver(&self.version) {
            return Err(EngineError::PluginError(format!(
                "Invalid version format: {}. Must be semver (e.g., 1.0.0)",
                self.version
            )));
        }

        // Validate API version
        if !is_valid_semver(&self.api_version) {
            return Err(EngineError::PluginError(format!(
                "Invalid API version format: {}",
                self.api_version
            )));
        }

        // Validate entry points based on runtime
        match self.runtime {
            PluginRuntime::JavaScript => {
                if self.main.is_none() {
                    return Err(EngineError::PluginError(
                        "JavaScript plugins must specify 'main' entry point".into()
                    ));
                }
            }
            PluginRuntime::Wasm => {
                if self.wasm.is_none() {
                    return Err(EngineError::PluginError(
                        "WASM plugins must specify 'wasm' module path".into()
                    ));
                }
            }
            PluginRuntime::Hybrid => {
                if self.main.is_none() && self.wasm.is_none() {
                    return Err(EngineError::PluginError(
                        "Hybrid plugins must specify at least 'main' or 'wasm'".into()
                    ));
                }
            }
        }

        // Validate capabilities match plugin type
        self.validate_capabilities()?;

        Ok(())
    }

    /// Check if the manifest is compatible with the given API version
    pub fn is_compatible_with(&self, api_version: &str) -> bool {
        // Simple major version check
        let manifest_major = self.api_version.split('.').next().unwrap_or("0");
        let current_major = api_version.split('.').next().unwrap_or("0");
        manifest_major == current_major
    }

    /// Get all dangerous permissions
    pub fn dangerous_permissions(&self) -> Vec<&Permission> {
        self.permissions.iter().filter(|p| p.is_dangerous()).collect()
    }

    /// Validate capabilities against plugin type
    fn validate_capabilities(&self) -> EngineResult<()> {
        match self.plugin_type {
            PluginType::Brush => {
                if self.capabilities.brushes.is_empty() {
                    return Err(EngineError::PluginError(
                        "Brush plugins must define at least one brush".into()
                    ));
                }
            }
            PluginType::Filter => {
                if self.capabilities.filters.is_empty() {
                    return Err(EngineError::PluginError(
                        "Filter plugins must define at least one filter".into()
                    ));
                }
            }
            PluginType::Tool => {
                if self.capabilities.tools.is_empty() {
                    return Err(EngineError::PluginError(
                        "Tool plugins must define at least one tool".into()
                    ));
                }
            }
            PluginType::Mixed => {
                // Mixed plugins can have any combination
            }
        }
        Ok(())
    }

    /// Convert to PluginInfo for UI display
    pub fn to_info(&self, state: PluginState) -> PluginInfo {
        PluginInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            author: self.author.name.clone(),
            plugin_type: self.plugin_type,
            state,
            icon: None, // TODO: Get from resources
        }
    }
}

/// Simple semver validation
fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u32>().is_ok())
}

/// Current API version supported by this engine
pub const CURRENT_API_VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parsing() {
        let json = r#"{
            "id": "com.example.test-plugin",
            "name": "Test Plugin",
            "version": "1.0.0",
            "apiVersion": "1.0",
            "description": "A test plugin",
            "author": { "name": "Test Author" },
            "license": "MIT",
            "type": "brush",
            "runtime": "javascript",
            "main": "main.js",
            "permissions": ["canvas:read", "brush:register"],
            "capabilities": {
                "brushes": [
                    { "id": "test-brush", "name": "Test Brush", "category": "Test" }
                ]
            }
        }"#;

        let manifest = PluginManifest::from_json(json).unwrap();
        assert_eq!(manifest.id, "com.example.test-plugin");
        assert_eq!(manifest.plugin_type, PluginType::Brush);
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_semver_validation() {
        assert!(is_valid_semver("1.0.0"));
        assert!(is_valid_semver("1.0"));
        assert!(is_valid_semver("0.1.2"));
        assert!(!is_valid_semver("1"));
        assert!(!is_valid_semver("1.0.0.0"));
        assert!(!is_valid_semver("a.b.c"));
    }
}
