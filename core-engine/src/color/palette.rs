//! Color palette functionality

use super::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Color palette for organizing colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Unique palette identifier
    pub id: Uuid,
    /// Palette name
    pub name: String,
    /// Colors in the palette
    pub colors: Vec<PaletteColor>,
    /// Number of columns for display
    pub columns: u32,
    /// Palette description
    pub description: String,
}

/// A color entry in a palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteColor {
    /// The color value
    pub color: Color,
    /// Optional name for the color
    pub name: Option<String>,
}

impl PaletteColor {
    /// Create a new palette color
    pub fn new(color: Color) -> Self {
        Self { color, name: None }
    }

    /// Create a named palette color
    pub fn named(color: Color, name: impl Into<String>) -> Self {
        Self {
            color,
            name: Some(name.into()),
        }
    }
}

impl ColorPalette {
    /// Create a new empty palette
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            colors: Vec::new(),
            columns: 8,
            description: String::new(),
        }
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, color: Color) {
        self.colors.push(PaletteColor::new(color));
    }

    /// Add a named color to the palette
    pub fn add_named_color(&mut self, color: Color, name: impl Into<String>) {
        self.colors.push(PaletteColor::named(color, name));
    }

    /// Remove a color at index
    pub fn remove_color(&mut self, index: usize) -> Option<PaletteColor> {
        if index < self.colors.len() {
            Some(self.colors.remove(index))
        } else {
            None
        }
    }

    /// Get number of colors
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Check if palette is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Create default color palette
    pub fn default_palette() -> Self {
        let mut palette = Self::new("Default");
        palette.description = "Basic color palette".into();

        // Basic colors
        palette.add_named_color(Color::black(), "Black");
        palette.add_named_color(Color::white(), "White");
        palette.add_named_color(Color::red(), "Red");
        palette.add_named_color(Color::green(), "Green");
        palette.add_named_color(Color::blue(), "Blue");
        palette.add_named_color(Color::yellow(), "Yellow");
        palette.add_named_color(Color::cyan(), "Cyan");
        palette.add_named_color(Color::magenta(), "Magenta");

        // Grays
        for i in 1..=7 {
            let v = i as f32 / 8.0;
            palette.add_named_color(Color::gray(v), format!("Gray {}", (v * 100.0) as u32));
        }

        palette
    }

    /// Create a skin tones palette
    pub fn skin_tones() -> Self {
        let mut palette = Self::new("Skin Tones");
        palette.description = "Human skin tone palette".into();
        palette.columns = 6;

        let tones = [
            ("#FFDBAC", "Light 1"),
            ("#F5C5A1", "Light 2"),
            ("#E5B795", "Light 3"),
            ("#D4A984", "Medium 1"),
            ("#C19A6B", "Medium 2"),
            ("#A1724F", "Medium 3"),
            ("#8D5524", "Tan 1"),
            ("#765339", "Tan 2"),
            ("#5C4033", "Dark 1"),
            ("#4A332C", "Dark 2"),
            ("#3B2219", "Dark 3"),
            ("#2C1810", "Dark 4"),
        ];

        for (hex, name) in tones {
            if let Some(color) = Color::from_hex(hex) {
                palette.add_named_color(color, name);
            }
        }

        palette
    }

    /// Create a pastel palette
    pub fn pastel() -> Self {
        let mut palette = Self::new("Pastel");
        palette.description = "Soft pastel colors".into();

        let colors = [
            "#FFB3BA", "#FFDFBA", "#FFFFBA", "#BAFFC9", "#BAE1FF",
            "#E8BAFF", "#FFB3E6", "#C4E8FF", "#D4FFBA", "#FFEEBA",
        ];

        for hex in colors {
            if let Some(color) = Color::from_hex(hex) {
                palette.add_color(color);
            }
        }

        palette
    }

    /// Sort colors by hue
    pub fn sort_by_hue(&mut self) {
        self.colors.sort_by(|a, b| {
            let (h1, _, _) = a.color.to_hsb();
            let (h2, _, _) = b.color.to_hsb();
            h1.partial_cmp(&h2).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Sort colors by luminance
    pub fn sort_by_luminance(&mut self) {
        self.colors.sort_by(|a, b| {
            let l1 = a.color.luminance();
            let l2 = b.color.luminance();
            l1.partial_cmp(&l2).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::default_palette()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_creation() {
        let palette = ColorPalette::new("Test");
        assert_eq!(palette.name, "Test");
        assert!(palette.is_empty());
    }

    #[test]
    fn test_add_colors() {
        let mut palette = ColorPalette::new("Test");
        palette.add_color(Color::red());
        palette.add_named_color(Color::blue(), "Blue");

        assert_eq!(palette.len(), 2);
        assert!(palette.colors[0].name.is_none());
        assert_eq!(palette.colors[1].name, Some("Blue".to_string()));
    }

    #[test]
    fn test_default_palette() {
        let palette = ColorPalette::default_palette();
        assert!(!palette.is_empty());
        assert!(palette.len() >= 8);
    }
}
