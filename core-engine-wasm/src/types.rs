//! WASM type definitions and conversions

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Stroke point for JavaScript
#[derive(Clone, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct JsStrokePoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
    pub timestamp: f64,
}

#[wasm_bindgen]
impl JsStrokePoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, pressure: f32, tilt_x: f32, tilt_y: f32, timestamp: f64) -> Self {
        Self {
            x,
            y,
            pressure,
            tilt_x,
            tilt_y,
            timestamp,
        }
    }
}

impl From<JsStrokePoint> for drawconnect_core::StrokePoint {
    fn from(point: JsStrokePoint) -> Self {
        drawconnect_core::StrokePoint {
            position: glam::Vec2::new(point.x, point.y),
            pressure: point.pressure,
            tilt: glam::Vec2::new(point.tilt_x, point.tilt_y),
            timestamp: point.timestamp as u64,
        }
    }
}

/// Import result types for PS file formats
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportedBrushInfo {
    pub name: String,
    pub diameter: u32,
    pub hardness: f32,
    pub spacing: f32,
    pub angle: f32,
    pub roundness: f32,
    pub has_tip_image: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportedPatternInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportedSwatchInfo {
    pub name: String,
    pub hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
