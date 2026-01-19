//! Plugin management commands for Tauri
//!
//! These commands expose the plugin system to the frontend.

use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tauri::State;

use drawconnect_core::plugin::{
    PluginManager, PluginInfo, PluginDetail, PluginState,
};

/// Plugin manager state shared across commands
pub struct PluginManagerState {
    manager: Arc<RwLock<Option<PluginManager>>>,
}

impl Default for PluginManagerState {
    fn default() -> Self {
        Self {
            manager: Arc::new(RwLock::new(None)),
        }
    }
}

impl PluginManagerState {
    /// Initialize the plugin manager with the given plugins directory
    pub fn init(&self, plugins_dir: PathBuf) -> Result<(), String> {
        let manager = PluginManager::new(plugins_dir)
            .map_err(|e| format!("Failed to initialize plugin manager: {}", e))?;

        *self.manager.write() = Some(manager);
        Ok(())
    }

    /// Get reference to the manager
    pub fn with_manager<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&PluginManager) -> Result<R, String>,
    {
        let lock = self.manager.read();
        let manager = lock.as_ref().ok_or("Plugin manager not initialized")?;
        f(manager)
    }

    /// Get mutable reference to the manager
    pub fn with_manager_mut<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut PluginManager) -> Result<R, String>,
    {
        let mut lock = self.manager.write();
        let manager = lock.as_mut().ok_or("Plugin manager not initialized")?;
        f(manager)
    }
}

// ============================================================================
// DTOs for frontend communication
// ============================================================================

/// Plugin info for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfoDto {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub plugin_type: String,
    pub state: String,
    pub icon: Option<String>,
}

impl From<PluginInfo> for PluginInfoDto {
    fn from(info: PluginInfo) -> Self {
        Self {
            id: info.id,
            name: info.name,
            version: info.version,
            description: info.description,
            author: info.author,
            plugin_type: format!("{:?}", info.plugin_type),
            state: match info.state {
                PluginState::Installed => "installed".to_string(),
                PluginState::Enabled => "enabled".to_string(),
                PluginState::Disabled => "disabled".to_string(),
                PluginState::Error => "error".to_string(),
            },
            icon: info.icon,
        }
    }
}

/// Detailed plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDetailDto {
    pub info: PluginInfoDto,
    pub manifest: serde_json::Value,
    pub settings_schema: Option<serde_json::Value>,
    pub current_settings: serde_json::Value,
    pub readme: Option<String>,
    pub changelog: Option<String>,
}

impl From<PluginDetail> for PluginDetailDto {
    fn from(detail: PluginDetail) -> Self {
        Self {
            info: detail.info.into(),
            manifest: detail.manifest,
            settings_schema: detail.settings_schema,
            current_settings: detail.current_settings,
            readme: detail.readme,
            changelog: detail.changelog,
        }
    }
}

/// Plugin capabilities aggregated from all enabled plugins
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginCapabilitiesDto {
    pub brushes: Vec<BrushCapabilityDto>,
    pub filters: Vec<FilterCapabilityDto>,
    pub tools: Vec<ToolCapabilityDto>,
    pub panels: Vec<PanelCapabilityDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushCapabilityDto {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCapabilityDto {
    pub id: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapabilityDto {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelCapabilityDto {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub position: String,
}

// ============================================================================
// Plugin Commands
// ============================================================================

/// Initialize the plugin system
#[tauri::command]
pub fn init_plugin_system(
    state: State<PluginManagerState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Get the app data directory for plugins
    let app_dir = app_handle.path_resolver()
        .app_data_dir()
        .ok_or("Failed to get app data directory")?;

    let plugins_dir = app_dir.join("plugins");

    state.init(plugins_dir)?;

    // Scan and load all installed plugins
    state.with_manager_mut(|manager| {
        manager.scan_and_load()
            .map_err(|e| format!("Failed to scan plugins: {}", e))?;
        Ok(())
    })
}

/// Get list of all plugins
#[tauri::command]
pub fn get_plugins(state: State<PluginManagerState>) -> Result<Vec<PluginInfoDto>, String> {
    state.with_manager(|manager| {
        let plugins: Vec<PluginInfoDto> = manager
            .list_plugins()
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(plugins)
    })
}

/// Get detailed information about a plugin
#[tauri::command]
pub fn get_plugin_detail(
    state: State<PluginManagerState>,
    plugin_id: String,
) -> Result<PluginDetailDto, String> {
    state.with_manager(|manager| {
        manager
            .get_plugin_detail(&plugin_id)
            .map(Into::into)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_id))
    })
}

/// Install a plugin from a file path
#[tauri::command]
pub fn install_plugin(
    state: State<PluginManagerState>,
    path: String,
) -> Result<PluginInfoDto, String> {
    let path = PathBuf::from(path);

    state.with_manager_mut(|manager| {
        manager
            .install(&path)
            .map(Into::into)
            .map_err(|e| format!("Failed to install plugin: {}", e))
    })
}

/// Uninstall a plugin
#[tauri::command]
pub fn uninstall_plugin(
    state: State<PluginManagerState>,
    plugin_id: String,
) -> Result<(), String> {
    state.with_manager_mut(|manager| {
        manager
            .uninstall(&plugin_id)
            .map_err(|e| format!("Failed to uninstall plugin: {}", e))
    })
}

/// Enable a plugin
#[tauri::command]
pub fn enable_plugin(
    state: State<PluginManagerState>,
    plugin_id: String,
) -> Result<(), String> {
    state.with_manager_mut(|manager| {
        manager
            .enable_plugin(&plugin_id)
            .map_err(|e| format!("Failed to enable plugin: {}", e))
    })
}

/// Disable a plugin
#[tauri::command]
pub fn disable_plugin(
    state: State<PluginManagerState>,
    plugin_id: String,
) -> Result<(), String> {
    state.with_manager_mut(|manager| {
        manager
            .disable_plugin(&plugin_id)
            .map_err(|e| format!("Failed to disable plugin: {}", e))
    })
}

/// Get plugin settings
#[tauri::command]
pub fn get_plugin_settings(
    state: State<PluginManagerState>,
    plugin_id: String,
) -> Result<serde_json::Value, String> {
    state.with_manager(|manager| {
        let settings = manager
            .get_plugin_settings(&plugin_id)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_id))?;

        serde_json::to_value(settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))
    })
}

/// Set a plugin setting
#[tauri::command]
pub fn set_plugin_setting(
    state: State<PluginManagerState>,
    plugin_id: String,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    state.with_manager_mut(|manager| {
        manager
            .set_plugin_setting(&plugin_id, &key, value)
            .map_err(|e| format!("Failed to set plugin setting: {}", e))
    })
}

/// Get all contributions from enabled plugins
#[tauri::command]
pub fn get_plugin_contributions(
    state: State<PluginManagerState>,
) -> Result<PluginCapabilitiesDto, String> {
    state.with_manager(|manager| {
        let caps = manager.get_contributions();

        Ok(PluginCapabilitiesDto {
            brushes: caps.brushes.into_iter().map(|b| BrushCapabilityDto {
                id: b.id,
                name: b.name,
                category: b.category,
                icon: b.icon,
            }).collect(),
            filters: caps.filters.into_iter().map(|f| FilterCapabilityDto {
                id: f.id,
                name: f.name,
                category: f.category,
            }).collect(),
            tools: caps.tools.into_iter().map(|t| ToolCapabilityDto {
                id: t.id,
                name: t.name,
                icon: Some(t.icon),
            }).collect(),
            panels: caps.panels.into_iter().map(|p| PanelCapabilityDto {
                id: p.id,
                title: p.title,
                icon: p.icon,
                position: p.position,
            }).collect(),
        })
    })
}

/// Open the plugins folder in file explorer
#[tauri::command]
pub fn open_plugins_folder(
    state: State<PluginManagerState>,
) -> Result<(), String> {
    state.with_manager(|manager| {
        let plugins_dir = manager.plugins_dir();

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer")
                .arg(plugins_dir)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }

        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(plugins_dir)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(plugins_dir)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }

        Ok(())
    })
}

/// Refresh plugins (rescan directory)
#[tauri::command]
pub fn refresh_plugins(
    state: State<PluginManagerState>,
) -> Result<Vec<PluginInfoDto>, String> {
    state.with_manager_mut(|manager| {
        manager
            .refresh()
            .map(|list| list.into_iter().map(Into::into).collect())
            .map_err(|e| format!("Failed to refresh plugins: {}", e))
    })
}

// ============================================================================
// Store placeholder commands (for future expansion)
// ============================================================================

/// Search plugins in store (placeholder for future)
#[tauri::command]
pub fn search_store_plugins(
    _query: String,
    _page: Option<u32>,
    _per_page: Option<u32>,
) -> Result<StoreSearchResultDto, String> {
    // Placeholder - return empty results for now
    Ok(StoreSearchResultDto {
        plugins: vec![],
        total: 0,
        page: 1,
        per_page: 20,
    })
}

/// Check for plugin updates (placeholder for future)
#[tauri::command]
pub fn check_plugin_updates(
    _state: State<PluginManagerState>,
) -> Result<Vec<PluginUpdateInfoDto>, String> {
    // Placeholder - return empty list for now
    Ok(vec![])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSearchResultDto {
    pub plugins: Vec<StorePluginInfoDto>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorePluginInfoDto {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub downloads: u32,
    pub rating: f32,
    pub icon_url: Option<String>,
    pub is_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginUpdateInfoDto {
    pub plugin_id: String,
    pub current_version: String,
    pub new_version: String,
    pub changelog: Option<String>,
}

// ============================================================================
// Command registration helper
// ============================================================================

/// Get all plugin commands for registration
pub fn plugin_commands() -> impl Fn(tauri::Invoke) {
    tauri::generate_handler![
        init_plugin_system,
        get_plugins,
        get_plugin_detail,
        install_plugin,
        uninstall_plugin,
        enable_plugin,
        disable_plugin,
        get_plugin_settings,
        set_plugin_setting,
        get_plugin_contributions,
        open_plugins_folder,
        refresh_plugins,
        search_store_plugins,
        check_plugin_updates,
    ]
}
