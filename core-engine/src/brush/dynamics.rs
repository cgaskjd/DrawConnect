//! Brush dynamics - pressure, tilt, velocity response curves

use serde::{Deserialize, Serialize};

/// Curve point for dynamics mapping
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CurvePoint {
    /// Input value (0.0 - 1.0)
    pub x: f32,
    /// Output value (0.0 - 1.0)
    pub y: f32,
}

impl CurvePoint {
    /// Create a new curve point
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
        }
    }
}

/// Dynamics curve for mapping input to output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicsCurve {
    /// Control points for the curve
    points: Vec<CurvePoint>,
    /// Curve type
    curve_type: CurveType,
}

/// Type of interpolation curve
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurveType {
    /// Linear interpolation
    Linear,
    /// Smooth (S-curve) interpolation
    Smooth,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Custom bezier curve
    Custom,
}

impl Default for CurveType {
    fn default() -> Self {
        Self::Linear
    }
}

impl DynamicsCurve {
    /// Create a linear curve (identity mapping)
    pub fn linear() -> Self {
        Self {
            points: vec![CurvePoint::new(0.0, 0.0), CurvePoint::new(1.0, 1.0)],
            curve_type: CurveType::Linear,
        }
    }

    /// Create a smooth S-curve
    pub fn smooth() -> Self {
        Self {
            points: vec![
                CurvePoint::new(0.0, 0.0),
                CurvePoint::new(0.25, 0.1),
                CurvePoint::new(0.75, 0.9),
                CurvePoint::new(1.0, 1.0),
            ],
            curve_type: CurveType::Smooth,
        }
    }

    /// Create an ease-in curve (slow start)
    pub fn ease_in() -> Self {
        Self {
            points: vec![
                CurvePoint::new(0.0, 0.0),
                CurvePoint::new(0.5, 0.25),
                CurvePoint::new(1.0, 1.0),
            ],
            curve_type: CurveType::EaseIn,
        }
    }

    /// Create an ease-out curve (slow end)
    pub fn ease_out() -> Self {
        Self {
            points: vec![
                CurvePoint::new(0.0, 0.0),
                CurvePoint::new(0.5, 0.75),
                CurvePoint::new(1.0, 1.0),
            ],
            curve_type: CurveType::EaseOut,
        }
    }

    /// Create a custom curve with points
    pub fn custom(points: Vec<CurvePoint>) -> Self {
        Self {
            points,
            curve_type: CurveType::Custom,
        }
    }

    /// Evaluate the curve at a given input value
    pub fn evaluate(&self, input: f32) -> f32 {
        let input = input.clamp(0.0, 1.0);

        if self.points.is_empty() {
            return input;
        }

        if self.points.len() == 1 {
            return self.points[0].y;
        }

        // Find the two points to interpolate between
        let mut p0 = &self.points[0];
        let mut p1 = &self.points[self.points.len() - 1];

        for window in self.points.windows(2) {
            if input >= window[0].x && input <= window[1].x {
                p0 = &window[0];
                p1 = &window[1];
                break;
            }
        }

        // Calculate interpolation parameter
        let t = if (p1.x - p0.x).abs() < f32::EPSILON {
            0.0
        } else {
            (input - p0.x) / (p1.x - p0.x)
        };

        // Interpolate based on curve type
        match self.curve_type {
            CurveType::Linear => Self::lerp(p0.y, p1.y, t),
            CurveType::Smooth => Self::smoothstep(p0.y, p1.y, t),
            CurveType::EaseIn => Self::ease_in_interpolate(p0.y, p1.y, t),
            CurveType::EaseOut => Self::ease_out_interpolate(p0.y, p1.y, t),
            CurveType::Custom => Self::smoothstep(p0.y, p1.y, t),
        }
    }

    /// Linear interpolation
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// Smooth step interpolation
    fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
        let t = t * t * (3.0 - 2.0 * t);
        a + (b - a) * t
    }

    /// Ease-in interpolation
    fn ease_in_interpolate(a: f32, b: f32, t: f32) -> f32 {
        let t = t * t;
        a + (b - a) * t
    }

    /// Ease-out interpolation
    fn ease_out_interpolate(a: f32, b: f32, t: f32) -> f32 {
        let t = 1.0 - (1.0 - t) * (1.0 - t);
        a + (b - a) * t
    }

    /// Add a control point to the curve
    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point);
        self.points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        self.curve_type = CurveType::Custom;
    }

    /// Remove a control point by index
    pub fn remove_point(&mut self, index: usize) -> Option<CurvePoint> {
        if index < self.points.len() && self.points.len() > 2 {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    /// Get control points
    pub fn points(&self) -> &[CurvePoint] {
        &self.points
    }
}

impl Default for DynamicsCurve {
    fn default() -> Self {
        Self::linear()
    }
}

/// Brush dynamics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushDynamics {
    /// Pressure affects size
    pub size_pressure_enabled: bool,
    /// Size pressure curve
    pub size_pressure_curve: DynamicsCurve,

    /// Pressure affects opacity
    pub opacity_pressure_enabled: bool,
    /// Opacity pressure curve
    pub opacity_pressure_curve: DynamicsCurve,

    /// Pressure affects hardness
    pub hardness_pressure_enabled: bool,
    /// Hardness pressure curve
    pub hardness_pressure_curve: DynamicsCurve,

    /// Tilt affects angle
    pub tilt_angle_enabled: bool,
    /// Tilt angle sensitivity
    pub tilt_angle_sensitivity: f32,

    /// Tilt affects size
    pub tilt_size_enabled: bool,
    /// Tilt size curve
    pub tilt_size_curve: DynamicsCurve,

    /// Velocity affects size
    pub velocity_size_enabled: bool,
    /// Velocity size curve
    pub velocity_size_curve: DynamicsCurve,

    /// Velocity affects opacity
    pub velocity_opacity_enabled: bool,
    /// Velocity opacity curve
    pub velocity_opacity_curve: DynamicsCurve,

    /// Rotation follows stroke direction
    pub rotation_follow_stroke: bool,

    /// Random jitter for size
    pub size_jitter: f32,
    /// Random jitter for opacity
    pub opacity_jitter: f32,
    /// Random jitter for angle
    pub angle_jitter: f32,
    /// Random jitter for position (scatter)
    pub scatter: f32,
}

impl Default for BrushDynamics {
    fn default() -> Self {
        Self {
            size_pressure_enabled: true,
            size_pressure_curve: DynamicsCurve::linear(),
            opacity_pressure_enabled: true,
            opacity_pressure_curve: DynamicsCurve::linear(),
            hardness_pressure_enabled: false,
            hardness_pressure_curve: DynamicsCurve::linear(),
            tilt_angle_enabled: false,
            tilt_angle_sensitivity: 1.0,
            tilt_size_enabled: false,
            tilt_size_curve: DynamicsCurve::linear(),
            velocity_size_enabled: false,
            velocity_size_curve: DynamicsCurve::linear(),
            velocity_opacity_enabled: false,
            velocity_opacity_curve: DynamicsCurve::linear(),
            rotation_follow_stroke: false,
            size_jitter: 0.0,
            opacity_jitter: 0.0,
            angle_jitter: 0.0,
            scatter: 0.0,
        }
    }
}

impl BrushDynamics {
    /// Create dynamics for a pencil-like brush
    pub fn pencil() -> Self {
        Self {
            size_pressure_enabled: true,
            size_pressure_curve: DynamicsCurve::ease_in(),
            opacity_pressure_enabled: true,
            opacity_pressure_curve: DynamicsCurve::smooth(),
            tilt_angle_enabled: true,
            tilt_angle_sensitivity: 0.8,
            ..Default::default()
        }
    }

    /// Create dynamics for a pen/ink brush
    pub fn ink() -> Self {
        Self {
            size_pressure_enabled: true,
            size_pressure_curve: DynamicsCurve::smooth(),
            opacity_pressure_enabled: false,
            ..Default::default()
        }
    }

    /// Create dynamics for an airbrush
    pub fn airbrush() -> Self {
        Self {
            size_pressure_enabled: true,
            size_pressure_curve: DynamicsCurve::linear(),
            opacity_pressure_enabled: true,
            opacity_pressure_curve: DynamicsCurve::ease_out(),
            ..Default::default()
        }
    }

    /// Create dynamics for a watercolor brush
    pub fn watercolor() -> Self {
        Self {
            size_pressure_enabled: true,
            size_pressure_curve: DynamicsCurve::smooth(),
            opacity_pressure_enabled: true,
            opacity_pressure_curve: DynamicsCurve::ease_out(),
            velocity_opacity_enabled: true,
            velocity_opacity_curve: DynamicsCurve::ease_in(),
            scatter: 0.1,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_curve() {
        let curve = DynamicsCurve::linear();
        assert!((curve.evaluate(0.0) - 0.0).abs() < 0.01);
        assert!((curve.evaluate(0.5) - 0.5).abs() < 0.01);
        assert!((curve.evaluate(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_curve_clamping() {
        let curve = DynamicsCurve::linear();
        assert!((curve.evaluate(-0.5) - 0.0).abs() < 0.01);
        assert!((curve.evaluate(1.5) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_smooth_curve() {
        let curve = DynamicsCurve::smooth();
        let mid = curve.evaluate(0.5);
        // Smooth curve should be around 0.5 at midpoint
        assert!(mid > 0.3 && mid < 0.7);
    }
}
