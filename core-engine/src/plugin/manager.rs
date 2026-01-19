//! Plugin manager - main entry point for plugin management

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;

use crate::error::{EngineError, EngineResult};
use super::manifest::{PluginManifest, CURRENT_API_VERSION};
use super::registry::PluginRegistry;
use super::loader::PluginLoader;
use super::sandbox::PluginSandbox;
use super::types::*;

/// Loaded plugin state
pub struct LoadedPlugin {
    /// Plugin ID
    pub id: String,
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Current state
    pub state: PluginState,
    /// Plugin directory path
    pub path: PathBuf,
    /// Sandbox for this plugin
    pub sandbox: PluginSandbox,
    /// Plugin settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Error message if state is Error
    pub error: Option<String>,
}

impl LoadedPlugin {
    /// Create a new loaded plugin
    pub fn new(manifest: PluginManifest, path: PathBuf) -> Self {
        let sandbox = PluginSandbox::new(&manifest.id, &manifest.permissions);
        Self {
            id: manifest.id.clone(),
            manifest,
            state: PluginState::Installed,
            path,
            sandbox,
            settings: HashMap::new(),
            error: None,
        }
    }

    /// Get plugin info for UI display
    pub fn info(&self) -> PluginInfo {
        self.manifest.to_info(self.state)
    }
}

/// Plugin manager - handles all plugin operations
pub struct PluginManager {
    /// Plugins directory
    plugins_dir: PathBuf,
    /// Plugin loader
    loader: PluginLoader,
    /// Plugin registry
    registry: Arc<RwLock<PluginRegistry>>,
    /// Loaded plugins
    plugins: HashMap<String, Arc<RwLock<LoadedPlugin>>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(plugins_dir: PathBuf) -> EngineResult<Self> {
        let loader = PluginLoader::new(plugins_dir.clone());
        loader.ensure_directories()?;

        let registry = PluginRegistry::load_or_create(&plugins_dir)?;

        Ok(Self {
            plugins_dir,
            loader,
            registry: Arc::new(RwLock::new(registry)),
            plugins: HashMap::new(),
        })
    }

    /// Get plugins directory
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }

    /// Scan and load all installed plugins
    pub fn scan_and_load(&mut self) -> EngineResult<Vec<String>> {
        let installed = self.loader.scan_installed()?;
        let mut loaded_ids = Vec::new();

        for (path, manifest) in installed {
            match self.load_plugin_internal(&path, manifest) {
                Ok(id) => {
                    loaded_ids.push(id.clone());

                    // Auto-enable if registry says so
                    if self.registry.read().is_enabled(&id) {
                        if let Err(e) = self.enable_plugin(&id) {
                            log::warn!("Failed to auto-enable plugin {}: {}", id, e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to load plugin from {:?}: {}", path, e);
                }
            }
        }

        Ok(loaded_ids)
    }

    /// Load a single plugin
    fn load_plugin_internal(&mut self, path: &Path, manifest: PluginManifest) -> EngineResult<String> {
        // Validate manifest
        manifest.validate()?;

        // Check API compatibility
        if !manifest.is_compatible_with(CURRENT_API_VERSION) {
            return Err(EngineError::PluginError(format!(
                "Plugin {} requires API version {}, but current version is {}",
                manifest.id, manifest.api_version, CURRENT_API_VERSION
            )));
        }

        let id = manifest.id.clone();
        let mut plugin = LoadedPlugin::new(manifest, path.to_path_buf());

        // Allow access to plugin's own directory
        plugin.sandbox.allow_plugin_directory(path.to_path_buf());

        // Load saved settings from registry
        if let Some(settings) = self.registry.read().get_settings(&id) {
            plugin.settings = settings.clone();
        }

        // Register in registry if not already
        if !self.registry.read().is_registered(&id) {
            self.registry.write().register(&id)?;
        }

        self.plugins.insert(id.clone(), Arc::new(RwLock::new(plugin)));

        Ok(id)
    }

    /// Install a plugin from file
    pub fn install(&mut self, source: &Path) -> EngineResult<PluginInfo> {
        // Install plugin files
        let plugin_path = self.loader.install(source)?;

        // Load manifest
        let manifest_path = plugin_path.join("manifest.json");
        let manifest = PluginManifest::from_file(&manifest_path)?;
        let id = manifest.id.clone();

        // Load the plugin
        self.load_plugin_internal(&plugin_path, manifest)?;

        // Get plugin info
        let plugin = self.plugins.get(&id)
            .ok_or_else(|| EngineError::PluginNotFound(id.clone()))?;

        Ok(plugin.read().info())
    }

    /// Uninstall a plugin
    pub fn uninstall(&mut self, id: &str) -> EngineResult<()> {
        // Disable first if enabled
        if let Some(plugin) = self.plugins.get(id) {
            if plugin.read().state == PluginState::Enabled {
                self.disable_plugin(id)?;
            }
        }

        // Remove from registry
        self.registry.write().remove(id)?;

        // Uninstall files
        self.loader.uninstall(id)?;

        // Remove from loaded plugins
        self.plugins.remove(id);

        Ok(())
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, id: &str) -> EngineResult<()> {
        let plugin = self.plugins.get(id)
            .ok_or_else(|| EngineError::PluginNotFound(id.to_string()))?;

        {
            let mut plugin = plugin.write();

            if plugin.state == PluginState::Enabled {
                return Ok(());
            }

            // TODO: Initialize JS/WASM runtime based on plugin.manifest.runtime
            // For now, just mark as enabled

            plugin.state = PluginState::Enabled;
        }

        // Update registry
        self.registry.write().set_enabled(id, true)?;

        log::info!("Plugin {} enabled", id);
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, id: &str) -> EngineResult<()> {
        let plugin = self.plugins.get(id)
            .ok_or_else(|| EngineError::PluginNotFound(id.to_string()))?;

        {
            let mut plugin = plugin.write();

            if plugin.state != PluginState::Enabled {
                return Ok(());
            }

            // TODO: Call plugin deactivate and cleanup
            // For now, just mark as disabled

            plugin.state = PluginState::Disabled;
        }

        // Update registry
        self.registry.write().set_enabled(id, false)?;

        log::info!("Plugin {} disabled", id);
        Ok(())
    }

    /// Get list of all plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.values()
            .map(|p| p.read().info())
            .collect()
    }

    /// Get plugin detail
    pub fn get_plugin_detail(&self, id: &str) -> Option<PluginDetail> {
        let plugin = self.plugins.get(id)?;
        let plugin = plugin.read();

        // Read readme if exists
        let readme = self.loader.read_plugin_file(id, "README.md")
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok());

        // Read changelog if exists
        let changelog = self.loader.read_plugin_file(id, "CHANGELOG.md")
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok());

        Some(PluginDetail {
            info: plugin.info(),
            manifest: serde_json::to_value(&plugin.manifest).unwrap_or_default(),
            settings_schema: plugin.manifest.settings.as_ref()
                .map(|s| serde_json::to_value(&s.schema).unwrap_or_default()),
            current_settings: serde_json::to_value(&plugin.settings).unwrap_or_default(),
            readme,
            changelog,
        })
    }

    /// Get plugin settings
    pub fn get_plugin_settings(&self, id: &str) -> Option<HashMap<String, serde_json::Value>> {
        let plugin = self.plugins.get(id)?;
        Some(plugin.read().settings.clone())
    }

    /// Set a plugin setting
    pub fn set_plugin_setting(&mut self, id: &str, key: &str, value: serde_json::Value) -> EngineResult<()> {
        let plugin = self.plugins.get(id)
            .ok_or_else(|| EngineError::PluginNotFound(id.to_string()))?;

        plugin.write().settings.insert(key.to_string(), value.clone());

        // Persist to registry
        self.registry.write().set_setting(id, key, value)?;

        Ok(())
    }

    /// Get all enabled plugins
    pub fn enabled_plugins(&self) -> Vec<&Arc<RwLock<LoadedPlugin>>> {
        self.plugins.values()
            .filter(|p| p.read().state == PluginState::Enabled)
            .collect()
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<&Arc<RwLock<LoadedPlugin>>> {
        self.plugins.get(id)
    }

    /// Get all contributions from enabled plugins
    pub fn get_contributions(&self) -> PluginCapabilities {
        let mut capabilities = PluginCapabilities::default();

        for plugin in self.enabled_plugins() {
            let plugin = plugin.read();
            capabilities.brushes.extend(plugin.manifest.capabilities.brushes.clone());
            capabilities.filters.extend(plugin.manifest.capabilities.filters.clone());
            capabilities.tools.extend(plugin.manifest.capabilities.tools.clone());
            capabilities.panels.extend(plugin.manifest.capabilities.panels.clone());
        }

        capabilities
    }

    /// Refresh plugins (rescan)
    pub fn refresh(&mut self) -> EngineResult<Vec<PluginInfo>> {
        self.plugins.clear();
        self.scan_and_load()?;
        Ok(self.list_plugins())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_plugin_manager_creation() {
        let dir = temp_dir().join("drawconnect_test_plugins");
        let manager = PluginManager::new(dir.clone());
        assert!(manager.is_ok());

        // Cleanup
        let _ = std::fs::remove_dir_all(dir);
    }
}
