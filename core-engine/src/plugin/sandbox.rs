//! Plugin sandbox for security isolation

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use super::permissions::Permission;

/// Plugin sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Memory limit in bytes
    pub memory_limit: usize,
    /// CPU time limit in milliseconds
    pub cpu_time_limit: u64,
    /// Execution timeout in milliseconds
    pub execution_timeout: u64,
    /// Maximum stack size
    pub stack_size: usize,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            memory_limit: 256 * 1024 * 1024,  // 256 MB
            cpu_time_limit: 10_000,            // 10 seconds
            execution_timeout: 30_000,         // 30 seconds
            stack_size: 1024 * 1024,          // 1 MB
        }
    }
}

/// Plugin sandbox - limits plugin access to system resources
#[derive(Debug, Clone)]
pub struct PluginSandbox {
    /// Plugin ID
    plugin_id: String,
    /// Granted permissions
    permissions: HashSet<Permission>,
    /// Sandbox configuration
    config: SandboxConfig,
    /// Allowed file paths (for fs:read/write)
    allowed_paths: Vec<PathBuf>,
    /// Allowed network hosts (for network:fetch)
    allowed_hosts: Vec<String>,
}

impl PluginSandbox {
    /// Create a new sandbox for a plugin
    pub fn new(plugin_id: &str, permissions: &[Permission]) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            permissions: permissions.iter().cloned().collect(),
            config: SandboxConfig::default(),
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
        }
    }

    /// Create sandbox with custom configuration
    pub fn with_config(plugin_id: &str, permissions: &[Permission], config: SandboxConfig) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            permissions: permissions.iter().cloned().collect(),
            config,
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
        }
    }

    /// Get plugin ID
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// Check if a permission is granted
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    /// Check if all permissions are granted
    pub fn has_all_permissions(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|p| self.permissions.contains(p))
    }

    // ========================================================================
    // Permission checks
    // ========================================================================

    /// Check canvas read permission
    pub fn can_read_canvas(&self) -> bool {
        self.has_permission(&Permission::CanvasRead)
    }

    /// Check canvas write permission
    pub fn can_write_canvas(&self) -> bool {
        self.has_permission(&Permission::CanvasWrite)
    }

    /// Check layer read permission
    pub fn can_read_layers(&self) -> bool {
        self.has_permission(&Permission::LayerRead)
    }

    /// Check layer write permission
    pub fn can_write_layers(&self) -> bool {
        self.has_permission(&Permission::LayerWrite)
    }

    /// Check layer pixels access permission
    pub fn can_access_layer_pixels(&self) -> bool {
        self.has_permission(&Permission::LayerPixels)
    }

    /// Check brush register permission
    pub fn can_register_brush(&self) -> bool {
        self.has_permission(&Permission::BrushRegister)
    }

    /// Check filter register permission
    pub fn can_register_filter(&self) -> bool {
        self.has_permission(&Permission::FilterRegister)
    }

    /// Check tool register permission
    pub fn can_register_tool(&self) -> bool {
        self.has_permission(&Permission::ToolRegister)
    }

    /// Check UI panel permission
    pub fn can_add_panel(&self) -> bool {
        self.has_permission(&Permission::UIPanel)
    }

    /// Check UI menu permission
    pub fn can_add_menu(&self) -> bool {
        self.has_permission(&Permission::UIMenu)
    }

    // ========================================================================
    // File system access
    // ========================================================================

    /// Add an allowed path for file access
    pub fn allow_path(&mut self, path: PathBuf) {
        self.allowed_paths.push(path);
    }

    /// Add the plugin's own directory to allowed paths
    pub fn allow_plugin_directory(&mut self, plugin_dir: PathBuf) {
        self.allowed_paths.push(plugin_dir);
    }

    /// Check if file read is allowed for a path
    pub fn can_read_file(&self, path: &Path) -> bool {
        if !self.has_permission(&Permission::FsRead) {
            return false;
        }
        self.is_path_allowed(path)
    }

    /// Check if file write is allowed for a path
    pub fn can_write_file(&self, path: &Path) -> bool {
        if !self.has_permission(&Permission::FsWrite) {
            return false;
        }
        self.is_path_allowed(path)
    }

    /// Check if a path is within allowed directories
    fn is_path_allowed(&self, path: &Path) -> bool {
        // Canonicalize the path to prevent directory traversal attacks
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => return false,
        };

        self.allowed_paths.iter().any(|allowed| {
            if let Ok(allowed_canonical) = allowed.canonicalize() {
                canonical.starts_with(&allowed_canonical)
            } else {
                false
            }
        })
    }

    // ========================================================================
    // Network access
    // ========================================================================

    /// Add an allowed host for network access
    pub fn allow_host(&mut self, host: String) {
        self.allowed_hosts.push(host);
    }

    /// Check if network fetch is allowed for a URL
    pub fn can_fetch(&self, url: &str) -> bool {
        if !self.has_permission(&Permission::NetworkFetch) {
            return false;
        }

        // Parse URL and extract host
        let host = extract_host(url);
        if host.is_none() {
            return false;
        }
        let host = host.unwrap();

        // Check if host is allowed
        self.allowed_hosts.iter().any(|allowed| {
            if allowed == "*" {
                true
            } else if allowed.starts_with("*.") {
                // Wildcard subdomain matching
                let domain = &allowed[2..];
                host == domain || host.ends_with(&format!(".{}", domain))
            } else {
                host == *allowed
            }
        })
    }

    // ========================================================================
    // Resource limits
    // ========================================================================

    /// Get memory limit
    pub fn memory_limit(&self) -> usize {
        self.config.memory_limit
    }

    /// Get CPU time limit
    pub fn cpu_time_limit(&self) -> u64 {
        self.config.cpu_time_limit
    }

    /// Get execution timeout
    pub fn execution_timeout(&self) -> u64 {
        self.config.execution_timeout
    }

    /// Get stack size
    pub fn stack_size(&self) -> usize {
        self.config.stack_size
    }

    /// Set memory limit
    pub fn set_memory_limit(&mut self, limit: usize) {
        self.config.memory_limit = limit;
    }

    /// Set execution timeout
    pub fn set_timeout(&mut self, timeout_ms: u64) {
        self.config.execution_timeout = timeout_ms;
    }
}

/// Extract host from URL
fn extract_host(url: &str) -> Option<String> {
    // Simple URL host extraction
    let url = url.trim();

    // Skip protocol
    let without_protocol = if url.starts_with("https://") {
        &url[8..]
    } else if url.starts_with("http://") {
        &url[7..]
    } else {
        url
    };

    // Get host (before path/query)
    let host_end = without_protocol.find('/').unwrap_or(without_protocol.len());
    let host_with_port = &without_protocol[..host_end];

    // Remove port if present
    let host = host_with_port.split(':').next()?;

    if host.is_empty() {
        None
    } else {
        Some(host.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_checks() {
        let sandbox = PluginSandbox::new(
            "test.plugin",
            &[Permission::CanvasRead, Permission::BrushRegister],
        );

        assert!(sandbox.can_read_canvas());
        assert!(!sandbox.can_write_canvas());
        assert!(sandbox.can_register_brush());
    }

    #[test]
    fn test_host_extraction() {
        assert_eq!(extract_host("https://example.com/path"), Some("example.com".into()));
        assert_eq!(extract_host("http://api.example.com:8080/"), Some("api.example.com".into()));
        assert_eq!(extract_host("example.com"), Some("example.com".into()));
    }

    #[test]
    fn test_network_permission() {
        let mut sandbox = PluginSandbox::new("test.plugin", &[Permission::NetworkFetch]);
        sandbox.allow_host("api.example.com".into());
        sandbox.allow_host("*.trusted.com".into());

        assert!(sandbox.can_fetch("https://api.example.com/data"));
        assert!(!sandbox.can_fetch("https://evil.com/"));
        assert!(sandbox.can_fetch("https://sub.trusted.com/api"));
        assert!(sandbox.can_fetch("https://trusted.com/api"));
    }
}
