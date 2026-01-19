//! Stroke Module
//!
//! Handles stroke data and stroke building with smoothing

use glam::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single point in a stroke
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StrokePoint {
    /// Position (x, y)
    pub position: Vec2,
    /// Pressure (0.0 - 1.0)
    pub pressure: f32,
    /// Tilt X (-1.0 to 1.0)
    pub tilt_x: f32,
    /// Tilt Y (-1.0 to 1.0)
    pub tilt_y: f32,
    /// Rotation angle in radians
    pub rotation: f32,
    /// Timestamp in milliseconds
    pub timestamp: u64,
}

impl StrokePoint {
    /// Create a new stroke point
    pub fn new(x: f32, y: f32, pressure: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            pressure: pressure.clamp(0.0, 1.0),
            tilt_x: 0.0,
            tilt_y: 0.0,
            rotation: 0.0,
            timestamp: 0,
        }
    }

    /// Create with full parameters
    pub fn full(
        x: f32,
        y: f32,
        pressure: f32,
        tilt_x: f32,
        tilt_y: f32,
        rotation: f32,
        timestamp: u64,
    ) -> Self {
        Self {
            position: Vec2::new(x, y),
            pressure: pressure.clamp(0.0, 1.0),
            tilt_x: tilt_x.clamp(-1.0, 1.0),
            tilt_y: tilt_y.clamp(-1.0, 1.0),
            rotation,
            timestamp,
        }
    }

    /// Linear interpolation between two points
    pub fn lerp(a: &StrokePoint, b: &StrokePoint, t: f32) -> Self {
        Self {
            position: a.position.lerp(b.position, t),
            pressure: a.pressure + (b.pressure - a.pressure) * t,
            tilt_x: a.tilt_x + (b.tilt_x - a.tilt_x) * t,
            tilt_y: a.tilt_y + (b.tilt_y - a.tilt_y) * t,
            rotation: a.rotation + (b.rotation - a.rotation) * t,
            timestamp: a.timestamp + ((b.timestamp - a.timestamp) as f32 * t) as u64,
        }
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &StrokePoint) -> f32 {
        self.position.distance(other.position)
    }

    /// Calculate velocity to another point
    pub fn velocity_to(&self, other: &StrokePoint) -> f32 {
        if other.timestamp <= self.timestamp {
            return 0.0;
        }
        let dt = (other.timestamp - self.timestamp) as f32 / 1000.0;
        self.distance_to(other) / dt
    }
}

impl Default for StrokePoint {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

/// A complete stroke
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    /// Unique stroke ID
    pub id: Uuid,
    /// Stroke points
    pub points: Vec<StrokePoint>,
    /// Stroke color (hex)
    pub color: String,
    /// Brush ID used
    pub brush_id: Uuid,
    /// Layer ID
    pub layer_id: Uuid,
    /// Start timestamp
    pub start_time: u64,
    /// End timestamp
    pub end_time: u64,
}

impl Stroke {
    /// Create a new empty stroke
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            points: Vec::new(),
            color: "#000000".into(),
            brush_id: Uuid::nil(),
            layer_id: Uuid::nil(),
            start_time: 0,
            end_time: 0,
        }
    }

    /// Add a point to the stroke
    pub fn add_point(&mut self, point: StrokePoint) {
        if self.points.is_empty() {
            self.start_time = point.timestamp;
        }
        self.end_time = point.timestamp;
        self.points.push(point);
    }

    /// Get stroke length (sum of segments)
    pub fn length(&self) -> f32 {
        if self.points.len() < 2 {
            return 0.0;
        }

        self.points
            .windows(2)
            .map(|w| w[0].distance_to(&w[1]))
            .sum()
    }

    /// Get bounding box (min_x, min_y, max_x, max_y)
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for point in &self.points {
            min_x = min_x.min(point.position.x);
            min_y = min_y.min(point.position.y);
            max_x = max_x.max(point.position.x);
            max_y = max_y.max(point.position.y);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    /// Get point count
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// Check if stroke is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get duration in milliseconds
    pub fn duration(&self) -> u64 {
        self.end_time.saturating_sub(self.start_time)
    }

    /// Clear all points
    pub fn clear(&mut self) {
        self.points.clear();
        self.start_time = 0;
        self.end_time = 0;
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new()
    }
}

/// Stroke builder with smoothing and prediction
pub struct StrokeBuilder {
    /// Current stroke being built
    stroke: Stroke,
    /// Smoothing level (0.0 - 1.0)
    smoothing: f32,
    /// Smoothing window size
    window_size: usize,
    /// Recent points for smoothing
    recent_points: Vec<StrokePoint>,
    /// Is currently drawing
    active: bool,
}

impl StrokeBuilder {
    /// Create a new stroke builder
    pub fn new() -> Self {
        Self {
            stroke: Stroke::new(),
            smoothing: 0.5,
            window_size: 5,
            recent_points: Vec::with_capacity(10),
            active: false,
        }
    }

    /// Set smoothing level
    pub fn set_smoothing(&mut self, level: f32) {
        self.smoothing = level.clamp(0.0, 1.0);
        self.window_size = (level * 10.0) as usize + 1;
    }

    /// Begin a new stroke
    pub fn begin(&mut self, brush_id: Uuid, layer_id: Uuid, color: &str) {
        self.stroke = Stroke::new();
        self.stroke.brush_id = brush_id;
        self.stroke.layer_id = layer_id;
        self.stroke.color = color.to_string();
        self.recent_points.clear();
        self.active = true;
    }

    /// Add a point to the stroke
    pub fn add_point(&mut self, point: StrokePoint) -> Option<StrokePoint> {
        if !self.active {
            return None;
        }

        self.recent_points.push(point);

        // Trim to window size
        while self.recent_points.len() > self.window_size {
            self.recent_points.remove(0);
        }

        // Apply smoothing
        let smoothed = self.smooth_point(&point);
        self.stroke.add_point(smoothed);

        Some(smoothed)
    }

    /// Smooth a point using recent points
    fn smooth_point(&self, point: &StrokePoint) -> StrokePoint {
        if self.recent_points.len() < 2 || self.smoothing == 0.0 {
            return *point;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_pressure = 0.0;
        let mut weight_sum = 0.0;

        for (i, p) in self.recent_points.iter().enumerate() {
            let weight = (i + 1) as f32; // More recent = more weight
            sum_x += p.position.x * weight;
            sum_y += p.position.y * weight;
            sum_pressure += p.pressure * weight;
            weight_sum += weight;
        }

        let smoothed_x = sum_x / weight_sum;
        let smoothed_y = sum_y / weight_sum;
        let smoothed_pressure = sum_pressure / weight_sum;

        // Blend smoothed with original based on smoothing level
        StrokePoint {
            position: Vec2::new(
                point.position.x * (1.0 - self.smoothing) + smoothed_x * self.smoothing,
                point.position.y * (1.0 - self.smoothing) + smoothed_y * self.smoothing,
            ),
            pressure: point.pressure * (1.0 - self.smoothing) + smoothed_pressure * self.smoothing,
            tilt_x: point.tilt_x,
            tilt_y: point.tilt_y,
            rotation: point.rotation,
            timestamp: point.timestamp,
        }
    }

    /// End the stroke
    pub fn end(&mut self) -> Stroke {
        self.active = false;
        self.recent_points.clear();
        std::mem::take(&mut self.stroke)
    }

    /// Cancel the stroke
    pub fn cancel(&mut self) {
        self.active = false;
        self.stroke.clear();
        self.recent_points.clear();
    }

    /// Check if currently building a stroke
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get current stroke reference
    pub fn current_stroke(&self) -> &Stroke {
        &self.stroke
    }
}

impl Default for StrokeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stroke_point_creation() {
        let point = StrokePoint::new(100.0, 200.0, 0.5);
        assert_eq!(point.position.x, 100.0);
        assert_eq!(point.pressure, 0.5);
    }

    #[test]
    fn test_stroke_point_lerp() {
        let a = StrokePoint::new(0.0, 0.0, 0.0);
        let b = StrokePoint::new(100.0, 100.0, 1.0);

        let mid = StrokePoint::lerp(&a, &b, 0.5);
        assert!((mid.position.x - 50.0).abs() < 0.01);
        assert!((mid.pressure - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_stroke_building() {
        let mut builder = StrokeBuilder::new();
        builder.begin(Uuid::new_v4(), Uuid::new_v4(), "#FF0000");

        builder.add_point(StrokePoint::new(0.0, 0.0, 1.0));
        builder.add_point(StrokePoint::new(10.0, 10.0, 0.8));
        builder.add_point(StrokePoint::new(20.0, 20.0, 0.6));

        let stroke = builder.end();
        assert_eq!(stroke.point_count(), 3);
    }

    #[test]
    fn test_stroke_bounds() {
        let mut stroke = Stroke::new();
        stroke.add_point(StrokePoint::new(10.0, 20.0, 1.0));
        stroke.add_point(StrokePoint::new(50.0, 80.0, 1.0));
        stroke.add_point(StrokePoint::new(30.0, 40.0, 1.0));

        let (min_x, min_y, max_x, max_y) = stroke.bounds().unwrap();
        assert_eq!(min_x, 10.0);
        assert_eq!(min_y, 20.0);
        assert_eq!(max_x, 50.0);
        assert_eq!(max_y, 80.0);
    }
}
