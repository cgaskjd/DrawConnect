//! Drawing tools module

use crate::color::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tool type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolType {
    /// Brush tool
    Brush,
    /// Eraser tool
    Eraser,
    /// Pencil tool
    Pencil,
    /// Pen tool (vector)
    Pen,
    /// Line tool
    Line,
    /// Rectangle tool
    Rectangle,
    /// Ellipse tool
    Ellipse,
    /// Selection - rectangular
    SelectRect,
    /// Selection - lasso
    SelectLasso,
    /// Selection - magic wand
    SelectMagic,
    /// Move tool
    Move,
    /// Transform tool
    Transform,
    /// Color picker (eyedropper)
    ColorPicker,
    /// Fill (paint bucket)
    Fill,
    /// Gradient tool
    Gradient,
    /// Text tool
    Text,
    /// Smudge tool
    Smudge,
    /// Blur tool
    Blur,
    /// Sharpen tool
    Sharpen,
    /// Clone stamp
    Clone,
    /// Hand tool (pan)
    Hand,
    /// Zoom tool
    Zoom,
}

impl Default for ToolType {
    fn default() -> Self {
        Self::Brush
    }
}

/// Tool state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolState {
    /// Idle, not in use
    Idle,
    /// Active, being used
    Active,
    /// Hovering
    Hover,
}

/// Base trait for all tools
pub trait Tool {
    /// Get tool type
    fn tool_type(&self) -> ToolType;

    /// Get tool name
    fn name(&self) -> &str;

    /// Get tool cursor
    fn cursor(&self) -> &str {
        "default"
    }

    /// Handle mouse/pen down
    fn on_press(&mut self, x: f32, y: f32, pressure: f32);

    /// Handle mouse/pen move
    fn on_move(&mut self, x: f32, y: f32, pressure: f32);

    /// Handle mouse/pen up
    fn on_release(&mut self, x: f32, y: f32);

    /// Cancel current operation
    fn cancel(&mut self);

    /// Reset tool state
    fn reset(&mut self);
}

/// Tool manager
pub struct ToolManager {
    /// Current tool type
    current_tool: ToolType,
    /// Primary color
    primary_color: Color,
    /// Secondary color
    secondary_color: Color,
    /// Tool options
    options: ToolOptions,
}

/// Tool options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOptions {
    /// Brush size
    pub size: f32,
    /// Opacity
    pub opacity: f32,
    /// Hardness
    pub hardness: f32,
    /// Flow
    pub flow: f32,
    /// Smoothing
    pub smoothing: f32,
    /// Anti-aliasing
    pub anti_aliasing: bool,
    /// Sample all layers (for picker/clone)
    pub sample_all_layers: bool,
    /// Contiguous (for fill/select)
    pub contiguous: bool,
    /// Tolerance (for magic wand/fill)
    pub tolerance: f32,
    /// Feather radius
    pub feather: f32,
}

impl Default for ToolOptions {
    fn default() -> Self {
        Self {
            size: 20.0,
            opacity: 1.0,
            hardness: 0.8,
            flow: 1.0,
            smoothing: 0.5,
            anti_aliasing: true,
            sample_all_layers: false,
            contiguous: true,
            tolerance: 32.0,
            feather: 0.0,
        }
    }
}

impl ToolManager {
    /// Create a new tool manager
    pub fn new() -> Self {
        Self {
            current_tool: ToolType::default(),
            primary_color: Color::black(),
            secondary_color: Color::white(),
            options: ToolOptions::default(),
        }
    }

    /// Get current tool type
    pub fn current_tool(&self) -> ToolType {
        self.current_tool
    }

    /// Set current tool
    pub fn set_tool(&mut self, tool: ToolType) {
        self.current_tool = tool;
    }

    /// Get primary color
    pub fn primary_color(&self) -> Color {
        self.primary_color
    }

    /// Set primary color
    pub fn set_primary_color(&mut self, color: Color) {
        self.primary_color = color;
    }

    /// Get secondary color
    pub fn secondary_color(&self) -> Color {
        self.secondary_color
    }

    /// Set secondary color
    pub fn set_secondary_color(&mut self, color: Color) {
        self.secondary_color = color;
    }

    /// Swap primary and secondary colors
    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.primary_color, &mut self.secondary_color);
    }

    /// Reset colors to default (black/white)
    pub fn reset_colors(&mut self) {
        self.primary_color = Color::black();
        self.secondary_color = Color::white();
    }

    /// Get tool options
    pub fn options(&self) -> &ToolOptions {
        &self.options
    }

    /// Get mutable tool options
    pub fn options_mut(&mut self) -> &mut ToolOptions {
        &mut self.options
    }

    /// Set brush size
    pub fn set_size(&mut self, size: f32) {
        self.options.size = size.max(1.0).min(5000.0);
    }

    /// Set opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.options.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Get cursor for current tool
    pub fn cursor(&self) -> &str {
        match self.current_tool {
            ToolType::Brush | ToolType::Pencil | ToolType::Eraser => "crosshair",
            ToolType::ColorPicker => "eyedropper",
            ToolType::Move => "move",
            ToolType::Hand => "grab",
            ToolType::Zoom => "zoom-in",
            ToolType::Text => "text",
            ToolType::Fill => "bucket",
            _ => "default",
        }
    }

    /// Check if current tool is a painting tool
    pub fn is_painting_tool(&self) -> bool {
        matches!(
            self.current_tool,
            ToolType::Brush
                | ToolType::Pencil
                | ToolType::Eraser
                | ToolType::Smudge
                | ToolType::Blur
                | ToolType::Sharpen
                | ToolType::Clone
        )
    }

    /// Check if current tool is a selection tool
    pub fn is_selection_tool(&self) -> bool {
        matches!(
            self.current_tool,
            ToolType::SelectRect | ToolType::SelectLasso | ToolType::SelectMagic
        )
    }

    /// Check if current tool is a shape tool
    pub fn is_shape_tool(&self) -> bool {
        matches!(
            self.current_tool,
            ToolType::Line | ToolType::Rectangle | ToolType::Ellipse
        )
    }
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_manager() {
        let mut manager = ToolManager::new();
        assert_eq!(manager.current_tool(), ToolType::Brush);

        manager.set_tool(ToolType::Eraser);
        assert_eq!(manager.current_tool(), ToolType::Eraser);
    }

    #[test]
    fn test_color_swap() {
        let mut manager = ToolManager::new();
        manager.set_primary_color(Color::red());
        manager.set_secondary_color(Color::blue());

        manager.swap_colors();

        assert!((manager.primary_color().b - 1.0).abs() < 0.01);
        assert!((manager.secondary_color().r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_tool_categories() {
        let manager = ToolManager::new();
        assert!(manager.is_painting_tool());

        let mut manager = ToolManager::new();
        manager.set_tool(ToolType::SelectRect);
        assert!(manager.is_selection_tool());
    }
}
