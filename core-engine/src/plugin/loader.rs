//! Plugin loader for installing and loading plugins

use std::path::{Path, PathBuf};
use std::fs;
use crate::error::{EngineError, EngineResult};
use super::manifest::PluginManifest;

/// Plugin loader handles plugin installation and file operations
pub struct PluginLoader {
    /// Base plugins directory
    plugins_dir: PathBuf,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self { plugins_dir }
    }

    /// Get the plugins directory
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }

    /// Get the installed plugins directory
    pub fn installed_dir(&self) -> PathBuf {
        self.plugins_dir.join("installed")
    }

    /// Get the disabled plugins directory
    pub fn disabled_dir(&self) -> PathBuf {
        self.plugins_dir.join("disabled")
    }

    /// Ensure required directories exist
    pub fn ensure_directories(&self) -> EngineResult<()> {
        fs::create_dir_all(self.installed_dir()).map_err(|e| {
            EngineError::PluginError(format!("Failed to create installed directory: {}", e))
        })?;
        fs::create_dir_all(self.disabled_dir()).map_err(|e| {
            EngineError::PluginError(format!("Failed to create disabled directory: {}", e))
        })?;
        Ok(())
    }

    /// Scan for installed plugins
    pub fn scan_installed(&self) -> EngineResult<Vec<(PathBuf, PluginManifest)>> {
        let installed_dir = self.installed_dir();
        let mut plugins = Vec::new();

        if !installed_dir.exists() {
            return Ok(plugins);
        }

        for entry in fs::read_dir(&installed_dir).map_err(|e| {
            EngineError::PluginError(format!("Failed to read plugins directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                EngineError::PluginError(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.json");
                if manifest_path.exists() {
                    match PluginManifest::from_file(&manifest_path) {
                        Ok(manifest) => {
                            plugins.push((path, manifest));
                        }
                        Err(e) => {
                            log::warn!("Failed to load manifest from {:?}: {}", manifest_path, e);
                        }
                    }
                }
            }
        }

        Ok(plugins)
    }

    /// Install plugin from a file (zip or directory)
    pub fn install(&self, source: &Path) -> EngineResult<PathBuf> {
        self.ensure_directories()?;

        if source.is_dir() {
            self.install_from_directory(source)
        } else if source.extension().map(|e| e == "zip" || e == "dcplugin").unwrap_or(false) {
            self.install_from_archive(source)
        } else {
            Err(EngineError::PluginError(
                "Unsupported plugin format. Use .zip, .dcplugin, or a directory".into()
            ))
        }
    }

    /// Install from a directory
    fn install_from_directory(&self, source: &Path) -> EngineResult<PathBuf> {
        // Validate manifest exists
        let manifest_path = source.join("manifest.json");
        if !manifest_path.exists() {
            return Err(EngineError::PluginError(
                "Plugin directory must contain manifest.json".into()
            ));
        }

        // Parse and validate manifest
        let manifest = PluginManifest::from_file(&manifest_path)?;
        manifest.validate()?;

        // Determine destination directory
        let dest = self.installed_dir().join(&manifest.id);

        // Remove existing installation if present
        if dest.exists() {
            fs::remove_dir_all(&dest).map_err(|e| {
                EngineError::PluginError(format!("Failed to remove existing plugin: {}", e))
            })?;
        }

        // Copy directory
        copy_dir_recursive(source, &dest)?;

        Ok(dest)
    }

    /// Install from an archive (.zip or .dcplugin)
    fn install_from_archive(&self, source: &Path) -> EngineResult<PathBuf> {
        // Create temp directory for extraction
        let temp_dir = self.plugins_dir.join("temp");
        fs::create_dir_all(&temp_dir).map_err(|e| {
            EngineError::PluginError(format!("Failed to create temp directory: {}", e))
        })?;

        // Extract archive
        let extract_dir = temp_dir.join(format!("extract_{}", uuid::Uuid::new_v4()));
        extract_archive(source, &extract_dir)?;

        // Find manifest (could be in root or in a subdirectory)
        let manifest_path = find_manifest(&extract_dir)?;
        let plugin_root = manifest_path.parent().unwrap_or(&extract_dir);

        // Install from extracted directory
        let result = self.install_from_directory(plugin_root);

        // Cleanup temp directory
        let _ = fs::remove_dir_all(&extract_dir);

        result
    }

    /// Uninstall a plugin
    pub fn uninstall(&self, plugin_id: &str) -> EngineResult<()> {
        let plugin_dir = self.installed_dir().join(plugin_id);

        if plugin_dir.exists() {
            // Move to disabled directory first (for potential recovery)
            let backup_dir = self.disabled_dir().join(format!("{}_backup", plugin_id));
            if backup_dir.exists() {
                fs::remove_dir_all(&backup_dir).ok();
            }
            fs::rename(&plugin_dir, &backup_dir).map_err(|e| {
                EngineError::PluginError(format!("Failed to move plugin to disabled: {}", e))
            })?;
        }

        Ok(())
    }

    /// Move plugin to disabled directory
    pub fn disable(&self, plugin_id: &str) -> EngineResult<()> {
        let source = self.installed_dir().join(plugin_id);
        let dest = self.disabled_dir().join(plugin_id);

        if source.exists() {
            if dest.exists() {
                fs::remove_dir_all(&dest).ok();
            }
            fs::rename(&source, &dest).map_err(|e| {
                EngineError::PluginError(format!("Failed to disable plugin: {}", e))
            })?;
        }

        Ok(())
    }

    /// Move plugin from disabled to installed directory
    pub fn enable(&self, plugin_id: &str) -> EngineResult<()> {
        let source = self.disabled_dir().join(plugin_id);
        let dest = self.installed_dir().join(plugin_id);

        if source.exists() {
            if dest.exists() {
                fs::remove_dir_all(&dest).ok();
            }
            fs::rename(&source, &dest).map_err(|e| {
                EngineError::PluginError(format!("Failed to enable plugin: {}", e))
            })?;
        }

        Ok(())
    }

    /// Get plugin directory path
    pub fn get_plugin_path(&self, plugin_id: &str) -> Option<PathBuf> {
        let installed = self.installed_dir().join(plugin_id);
        if installed.exists() {
            return Some(installed);
        }

        let disabled = self.disabled_dir().join(plugin_id);
        if disabled.exists() {
            return Some(disabled);
        }

        None
    }

    /// Read plugin file content
    pub fn read_plugin_file(&self, plugin_id: &str, file_path: &str) -> EngineResult<Vec<u8>> {
        let plugin_dir = self.get_plugin_path(plugin_id)
            .ok_or_else(|| EngineError::PluginNotFound(plugin_id.to_string()))?;

        let file_path = plugin_dir.join(file_path);

        // Security check: ensure path is within plugin directory
        let canonical = file_path.canonicalize().map_err(|_| {
            EngineError::PluginError("Invalid file path".into())
        })?;
        let plugin_canonical = plugin_dir.canonicalize().map_err(|_| {
            EngineError::PluginError("Invalid plugin path".into())
        })?;

        if !canonical.starts_with(&plugin_canonical) {
            return Err(EngineError::PluginError("Path traversal not allowed".into()));
        }

        fs::read(&canonical).map_err(|e| {
            EngineError::PluginError(format!("Failed to read file: {}", e))
        })
    }
}

/// Copy directory recursively
fn copy_dir_recursive(source: &Path, dest: &Path) -> EngineResult<()> {
    fs::create_dir_all(dest).map_err(|e| {
        EngineError::PluginError(format!("Failed to create directory: {}", e))
    })?;

    for entry in fs::read_dir(source).map_err(|e| {
        EngineError::PluginError(format!("Failed to read directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            EngineError::PluginError(format!("Failed to read entry: {}", e))
        })?;

        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path).map_err(|e| {
                EngineError::PluginError(format!("Failed to copy file: {}", e))
            })?;
        }
    }

    Ok(())
}

/// Extract archive to destination
fn extract_archive(source: &Path, dest: &Path) -> EngineResult<()> {
    fs::create_dir_all(dest).map_err(|e| {
        EngineError::PluginError(format!("Failed to create extraction directory: {}", e))
    })?;

    let file = fs::File::open(source).map_err(|e| {
        EngineError::PluginError(format!("Failed to open archive: {}", e))
    })?;

    let mut archive = zip::ZipArchive::new(file).map_err(|e| {
        EngineError::PluginError(format!("Failed to read archive: {}", e))
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            EngineError::PluginError(format!("Failed to read archive entry: {}", e))
        })?;

        let outpath = match file.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).ok();
            }

            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                EngineError::PluginError(format!("Failed to create file: {}", e))
            })?;

            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                EngineError::PluginError(format!("Failed to extract file: {}", e))
            })?;
        }
    }

    Ok(())
}

/// Find manifest.json in extracted directory (recursive search)
fn find_manifest(dir: &Path) -> EngineResult<PathBuf> {
    // Check root directory first
    let manifest = dir.join("manifest.json");
    if manifest.exists() {
        return Ok(manifest);
    }

    // Check immediate subdirectories (common case: zip contains a folder)
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest = path.join("manifest.json");
                if manifest.exists() {
                    return Ok(manifest);
                }
            }
        }
    }

    // Recursive search for deeply nested manifest
    if let Some(manifest) = find_manifest_recursive(dir, 3) {
        return Ok(manifest);
    }

    Err(EngineError::PluginError(
        "No manifest.json found in plugin archive".into()
    ))
}

/// Recursively search for manifest.json with depth limit
fn find_manifest_recursive(dir: &Path, max_depth: u32) -> Option<PathBuf> {
    if max_depth == 0 {
        return None;
    }

    let manifest = dir.join("manifest.json");
    if manifest.exists() {
        return Some(manifest);
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories and common non-plugin directories
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') || name == "__MACOSX" || name == "node_modules" {
                    continue;
                }

                if let Some(found) = find_manifest_recursive(&path, max_depth - 1) {
                    return Some(found);
                }
            }
        }
    }

    None
}
