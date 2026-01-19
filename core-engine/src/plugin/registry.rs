//! Plugin registry for persistent storage of plugin states

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::{EngineError, EngineResult};

/// Registry entry for a single plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// Plugin ID
    pub id: String,
    /// Whether the plugin is enabled
    pub enabled: bool,
    /// Installation timestamp
    pub installed_at: u64,
    /// Last updated timestamp
    pub updated_at: u64,
    /// Plugin settings
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

/// Plugin registry - persists plugin states to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRegistry {
    /// Registry version for migrations
    pub version: u32,
    /// Plugin entries
    pub plugins: HashMap<String, RegistryEntry>,
    /// Registry file path
    #[serde(skip)]
    path: Option<PathBuf>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self {
            version: 1,
            plugins: HashMap::new(),
            path: None,
        }
    }
}

impl PluginRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Load registry from file or create new one
    pub fn load_or_create(plugins_dir: &Path) -> EngineResult<Self> {
        let registry_path = plugins_dir.join("registry.json");

        if registry_path.exists() {
            Self::load(&registry_path)
        } else {
            let mut registry = Self::new();
            registry.path = Some(registry_path);
            Ok(registry)
        }
    }

    /// Load registry from file
    pub fn load(path: &Path) -> EngineResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            EngineError::PluginError(format!("Failed to read registry: {}", e))
        })?;

        let mut registry: Self = serde_json::from_str(&content).map_err(|e| {
            EngineError::PluginError(format!("Failed to parse registry: {}", e))
        })?;

        registry.path = Some(path.to_path_buf());
        Ok(registry)
    }

    /// Save registry to file
    pub fn save(&self) -> EngineResult<()> {
        if let Some(ref path) = self.path {
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    EngineError::PluginError(format!("Failed to create directory: {}", e))
                })?;
            }

            let content = serde_json::to_string_pretty(self).map_err(|e| {
                EngineError::PluginError(format!("Failed to serialize registry: {}", e))
            })?;

            std::fs::write(path, content).map_err(|e| {
                EngineError::PluginError(format!("Failed to write registry: {}", e))
            })?;
        }
        Ok(())
    }

    /// Register a new plugin
    pub fn register(&mut self, id: &str) -> EngineResult<()> {
        let now = current_timestamp();
        let entry = RegistryEntry {
            id: id.to_string(),
            enabled: false,
            installed_at: now,
            updated_at: now,
            settings: HashMap::new(),
        };
        self.plugins.insert(id.to_string(), entry);
        self.save()
    }

    /// Remove a plugin from registry
    pub fn remove(&mut self, id: &str) -> EngineResult<()> {
        self.plugins.remove(id);
        self.save()
    }

    /// Check if a plugin is registered
    pub fn is_registered(&self, id: &str) -> bool {
        self.plugins.contains_key(id)
    }

    /// Check if a plugin is enabled
    pub fn is_enabled(&self, id: &str) -> bool {
        self.plugins.get(id).map(|e| e.enabled).unwrap_or(false)
    }

    /// Set plugin enabled state
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> EngineResult<()> {
        if let Some(entry) = self.plugins.get_mut(id) {
            entry.enabled = enabled;
            entry.updated_at = current_timestamp();
            self.save()?;
        }
        Ok(())
    }

    /// Get plugin settings
    pub fn get_settings(&self, id: &str) -> Option<&HashMap<String, serde_json::Value>> {
        self.plugins.get(id).map(|e| &e.settings)
    }

    /// Set a plugin setting
    pub fn set_setting(&mut self, id: &str, key: &str, value: serde_json::Value) -> EngineResult<()> {
        if let Some(entry) = self.plugins.get_mut(id) {
            entry.settings.insert(key.to_string(), value);
            entry.updated_at = current_timestamp();
            self.save()?;
        }
        Ok(())
    }

    /// Get all registered plugin IDs
    pub fn plugin_ids(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Get all enabled plugin IDs
    pub fn enabled_plugin_ids(&self) -> Vec<String> {
        self.plugins
            .iter()
            .filter(|(_, e)| e.enabled)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get plugin entry
    pub fn get(&self, id: &str) -> Option<&RegistryEntry> {
        self.plugins.get(id)
    }

    /// Update timestamp
    pub fn touch(&mut self, id: &str) -> EngineResult<()> {
        if let Some(entry) = self.plugins.get_mut(id) {
            entry.updated_at = current_timestamp();
            self.save()?;
        }
        Ok(())
    }
}

/// Get current timestamp in seconds since UNIX epoch
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_operations() {
        let mut registry = PluginRegistry::new();

        // Register plugin
        registry.plugins.insert("test.plugin".to_string(), RegistryEntry {
            id: "test.plugin".to_string(),
            enabled: false,
            installed_at: 0,
            updated_at: 0,
            settings: HashMap::new(),
        });

        assert!(registry.is_registered("test.plugin"));
        assert!(!registry.is_enabled("test.plugin"));

        // Enable plugin
        if let Some(entry) = registry.plugins.get_mut("test.plugin") {
            entry.enabled = true;
        }
        assert!(registry.is_enabled("test.plugin"));
    }
}
