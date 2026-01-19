//! Plugin permission system

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Plugin permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    // Canvas permissions
    /// Read canvas pixels
    #[serde(rename = "canvas:read")]
    CanvasRead,
    /// Write canvas pixels
    #[serde(rename = "canvas:write")]
    CanvasWrite,

    // Layer permissions
    /// Read layer information
    #[serde(rename = "layer:read")]
    LayerRead,
    /// Modify layers
    #[serde(rename = "layer:write")]
    LayerWrite,
    /// Access active layer
    #[serde(rename = "layer:active")]
    LayerActive,
    /// Access layer pixel data
    #[serde(rename = "layer:pixels")]
    LayerPixels,

    // Brush permissions
    /// Register custom brushes
    #[serde(rename = "brush:register")]
    BrushRegister,
    /// Custom brush rendering
    #[serde(rename = "brush:render")]
    BrushRender,

    // Filter permissions
    /// Register filters
    #[serde(rename = "filter:register")]
    FilterRegister,
    /// Apply filters
    #[serde(rename = "filter:apply")]
    FilterApply,

    // Tool permissions
    /// Register custom tools
    #[serde(rename = "tool:register")]
    ToolRegister,

    // UI permissions
    /// Add UI panels
    #[serde(rename = "ui:panel")]
    UIPanel,
    /// Add menu items
    #[serde(rename = "ui:menu")]
    UIMenu,
    /// Add toolbar items
    #[serde(rename = "ui:toolbar")]
    UIToolbar,
    /// Show dialogs
    #[serde(rename = "ui:dialog")]
    UIDialog,

    // File system permissions
    /// Read files (sandboxed)
    #[serde(rename = "fs:read")]
    FsRead,
    /// Write files (sandboxed)
    #[serde(rename = "fs:write")]
    FsWrite,

    // Network permissions
    /// Fetch from network (restricted)
    #[serde(rename = "network:fetch")]
    NetworkFetch,

    // History permissions
    /// Access undo/redo history
    #[serde(rename = "history:access")]
    HistoryAccess,

    // Selection permissions
    /// Read selection info
    #[serde(rename = "selection:read")]
    SelectionRead,
    /// Modify selection
    #[serde(rename = "selection:write")]
    SelectionWrite,
}

impl Permission {
    /// Check if this is a dangerous permission
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            Permission::FsWrite | Permission::NetworkFetch | Permission::CanvasWrite
        )
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Permission::CanvasRead => "Read canvas pixel data",
            Permission::CanvasWrite => "Modify canvas pixels",
            Permission::LayerRead => "Read layer information",
            Permission::LayerWrite => "Create, delete, or modify layers",
            Permission::LayerActive => "Access the active layer",
            Permission::LayerPixels => "Direct access to layer pixel data",
            Permission::BrushRegister => "Register custom brushes",
            Permission::BrushRender => "Custom brush rendering",
            Permission::FilterRegister => "Register image filters",
            Permission::FilterApply => "Apply filters to images",
            Permission::ToolRegister => "Register custom tools",
            Permission::UIPanel => "Add custom UI panels",
            Permission::UIMenu => "Add menu items",
            Permission::UIToolbar => "Add toolbar buttons",
            Permission::UIDialog => "Show dialog windows",
            Permission::FsRead => "Read files from disk (sandboxed)",
            Permission::FsWrite => "Write files to disk (sandboxed)",
            Permission::NetworkFetch => "Make network requests",
            Permission::HistoryAccess => "Access undo/redo history",
            Permission::SelectionRead => "Read selection information",
            Permission::SelectionWrite => "Modify selection",
        }
    }

    /// Get permission category
    pub fn category(&self) -> &'static str {
        match self {
            Permission::CanvasRead | Permission::CanvasWrite => "Canvas",
            Permission::LayerRead
            | Permission::LayerWrite
            | Permission::LayerActive
            | Permission::LayerPixels => "Layers",
            Permission::BrushRegister | Permission::BrushRender => "Brushes",
            Permission::FilterRegister | Permission::FilterApply => "Filters",
            Permission::ToolRegister => "Tools",
            Permission::UIPanel | Permission::UIMenu | Permission::UIToolbar | Permission::UIDialog => "UI",
            Permission::FsRead | Permission::FsWrite => "File System",
            Permission::NetworkFetch => "Network",
            Permission::HistoryAccess => "History",
            Permission::SelectionRead | Permission::SelectionWrite => "Selection",
        }
    }
}

/// Permission checker utility
#[derive(Debug, Clone)]
pub struct PermissionChecker {
    granted: HashSet<Permission>,
}

impl PermissionChecker {
    /// Create a new permission checker with granted permissions
    pub fn new(permissions: &[Permission]) -> Self {
        Self {
            granted: permissions.iter().cloned().collect(),
        }
    }

    /// Check if a permission is granted
    pub fn has(&self, permission: &Permission) -> bool {
        self.granted.contains(permission)
    }

    /// Check if all permissions are granted
    pub fn has_all(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|p| self.granted.contains(p))
    }

    /// Check if any permission is granted
    pub fn has_any(&self, permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| self.granted.contains(p))
    }

    /// Get all granted permissions
    pub fn granted_permissions(&self) -> &HashSet<Permission> {
        &self.granted
    }

    /// Check canvas read permission
    pub fn can_read_canvas(&self) -> bool {
        self.has(&Permission::CanvasRead)
    }

    /// Check canvas write permission
    pub fn can_write_canvas(&self) -> bool {
        self.has(&Permission::CanvasWrite)
    }

    /// Check layer pixels permission
    pub fn can_access_layer_pixels(&self) -> bool {
        self.has(&Permission::LayerPixels)
    }

    /// Check file read permission
    pub fn can_read_files(&self) -> bool {
        self.has(&Permission::FsRead)
    }

    /// Check file write permission
    pub fn can_write_files(&self) -> bool {
        self.has(&Permission::FsWrite)
    }

    /// Check network permission
    pub fn can_fetch(&self) -> bool {
        self.has(&Permission::NetworkFetch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_checker() {
        let checker = PermissionChecker::new(&[
            Permission::CanvasRead,
            Permission::LayerRead,
        ]);

        assert!(checker.can_read_canvas());
        assert!(!checker.can_write_canvas());
        assert!(checker.has(&Permission::LayerRead));
    }

    #[test]
    fn test_dangerous_permissions() {
        assert!(Permission::FsWrite.is_dangerous());
        assert!(Permission::NetworkFetch.is_dangerous());
        assert!(!Permission::CanvasRead.is_dangerous());
    }
}
