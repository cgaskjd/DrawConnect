//! Bridge utilities for type conversion between WASM and core engine

use drawconnect_core::{Color, BlendMode};

/// Convert hex color string to Color
pub fn hex_to_color(hex: &str) -> Result<Color, String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 8 {
        return Err("Invalid hex color format".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?
    } else {
        255
    };

    Ok(Color::from_rgba8(r, g, b, a))
}

/// Convert Color to hex string
pub fn color_to_hex(color: &Color) -> String {
    let (r, g, b, a) = color.to_rgba8();
    if a == 255 {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

/// Convert blend mode to string
pub fn blend_mode_to_string(mode: BlendMode) -> String {
    match mode {
        BlendMode::Normal => "normal".to_string(),
        BlendMode::Multiply => "multiply".to_string(),
        BlendMode::Screen => "screen".to_string(),
        BlendMode::Overlay => "overlay".to_string(),
        BlendMode::Darken => "darken".to_string(),
        BlendMode::Lighten => "lighten".to_string(),
        BlendMode::ColorDodge => "color_dodge".to_string(),
        BlendMode::ColorBurn => "color_burn".to_string(),
        BlendMode::HardLight => "hard_light".to_string(),
        BlendMode::SoftLight => "soft_light".to_string(),
        BlendMode::Difference => "difference".to_string(),
        BlendMode::Exclusion => "exclusion".to_string(),
        BlendMode::Hue => "hue".to_string(),
        BlendMode::Saturation => "saturation".to_string(),
        BlendMode::Color => "color".to_string(),
        BlendMode::Luminosity => "luminosity".to_string(),
    }
}

/// Convert string to blend mode
pub fn string_to_blend_mode(s: &str) -> BlendMode {
    match s.to_lowercase().as_str() {
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "overlay" => BlendMode::Overlay,
        "darken" => BlendMode::Darken,
        "lighten" => BlendMode::Lighten,
        "color_dodge" | "colordodge" => BlendMode::ColorDodge,
        "color_burn" | "colorburn" => BlendMode::ColorBurn,
        "hard_light" | "hardlight" => BlendMode::HardLight,
        "soft_light" | "softlight" => BlendMode::SoftLight,
        "difference" => BlendMode::Difference,
        "exclusion" => BlendMode::Exclusion,
        "hue" => BlendMode::Hue,
        "saturation" => BlendMode::Saturation,
        "color" => BlendMode::Color,
        "luminosity" => BlendMode::Luminosity,
        _ => BlendMode::Normal,
    }
}
