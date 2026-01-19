//! Layer group/folder functionality

use super::BlendMode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Layer group (folder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerGroup {
    /// Unique group identifier
    pub id: Uuid,
    /// Group name
    pub name: String,
    /// Group visibility
    pub visible: bool,
    /// Group opacity
    pub opacity: f32,
    /// Blend mode for the group
    pub blend_mode: BlendMode,
    /// Whether the group is expanded in UI
    pub expanded: bool,
    /// Child layer IDs (in order)
    pub children: Vec<Uuid>,
    /// Parent group ID (for nested groups)
    pub parent_id: Option<Uuid>,
    /// Pass through blending (vs isolated)
    pub pass_through: bool,
}

impl LayerGroup {
    /// Create a new layer group
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            expanded: true,
            children: Vec::new(),
            parent_id: None,
            pass_through: true,
        }
    }

    /// Add a layer to this group
    pub fn add_layer(&mut self, layer_id: Uuid) {
        if !self.children.contains(&layer_id) {
            self.children.push(layer_id);
        }
    }

    /// Remove a layer from this group
    pub fn remove_layer(&mut self, layer_id: Uuid) -> bool {
        if let Some(pos) = self.children.iter().position(|&id| id == layer_id) {
            self.children.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if group contains a layer
    pub fn contains(&self, layer_id: Uuid) -> bool {
        self.children.contains(&layer_id)
    }

    /// Get number of layers in group
    pub fn layer_count(&self) -> usize {
        self.children.len()
    }

    /// Reorder a layer within the group
    pub fn reorder_layer(&mut self, layer_id: Uuid, new_index: usize) -> bool {
        if let Some(current_pos) = self.children.iter().position(|&id| id == layer_id) {
            if new_index < self.children.len() {
                let id = self.children.remove(current_pos);
                self.children.insert(new_index, id);
                return true;
            }
        }
        false
    }

    /// Toggle group expansion
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Toggle visibility
    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

impl Default for LayerGroup {
    fn default() -> Self {
        Self::new("Group")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_creation() {
        let group = LayerGroup::new("Test Group");
        assert_eq!(group.name, "Test Group");
        assert!(group.visible);
        assert!(group.expanded);
        assert_eq!(group.layer_count(), 0);
    }

    #[test]
    fn test_add_remove_layers() {
        let mut group = LayerGroup::new("Test");
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        group.add_layer(id1);
        group.add_layer(id2);
        assert_eq!(group.layer_count(), 2);
        assert!(group.contains(id1));

        group.remove_layer(id1);
        assert_eq!(group.layer_count(), 1);
        assert!(!group.contains(id1));
    }

    #[test]
    fn test_reorder() {
        let mut group = LayerGroup::new("Test");
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        group.add_layer(id1);
        group.add_layer(id2);
        group.add_layer(id3);

        group.reorder_layer(id1, 2);
        assert_eq!(group.children[2], id1);
        assert_eq!(group.children[0], id2);
    }
}
