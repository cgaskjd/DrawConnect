//! Curves adjustment
//!
//! Provides precise tonal control using a curve with control points.

use super::{Adjustment, CurveChannel};
use crate::color::Color;

/// A point on the curves adjustment
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CurvePoint {
    /// X position (input value, 0.0 to 1.0)
    pub x: f32,
    /// Y position (output value, 0.0 to 1.0)
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

/// Curves adjustment
#[derive(Debug, Clone)]
pub struct Curves {
    /// Control points defining the curve
    pub points: Vec<CurvePoint>,
    /// Which channel to adjust
    pub channel: CurveChannel,
    /// Lookup table for fast curve evaluation (256 entries)
    lut: Vec<f32>,
}

impl Curves {
    /// Create a new curves adjustment with control points
    pub fn new(mut points: Vec<CurvePoint>, channel: CurveChannel) -> Self {
        // Ensure we have at least start and end points
        if points.is_empty() {
            points = vec![CurvePoint::new(0.0, 0.0), CurvePoint::new(1.0, 1.0)];
        }

        // Sort points by x coordinate
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        // Build lookup table
        let lut = Self::build_lut(&points);

        Self {
            points,
            channel,
            lut,
        }
    }

    /// Build a 256-entry lookup table for the curve
    fn build_lut(points: &[CurvePoint]) -> Vec<f32> {
        let mut lut = Vec::with_capacity(256);

        for i in 0..256 {
            let t = i as f32 / 255.0;
            lut.push(Self::evaluate_curve(points, t));
        }

        lut
    }

    /// Evaluate the curve at a given input value using Catmull-Rom interpolation
    fn evaluate_curve(points: &[CurvePoint], t: f32) -> f32 {
        if points.len() < 2 {
            return t;
        }

        // Find the segment containing t
        let mut i = 0;
        while i < points.len() - 1 && points[i + 1].x < t {
            i += 1;
        }

        if i >= points.len() - 1 {
            return points.last().unwrap().y;
        }

        // Get the four control points for Catmull-Rom
        let p0 = if i > 0 { points[i - 1] } else { points[i] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() { points[i + 2] } else { points[i + 1] };

        // Calculate local t
        let local_t = if (p2.x - p1.x).abs() > 0.0001 {
            (t - p1.x) / (p2.x - p1.x)
        } else {
            0.0
        };

        Self::catmull_rom(p0.y, p1.y, p2.y, p3.y, local_t).clamp(0.0, 1.0)
    }

    /// Catmull-Rom spline interpolation
    fn catmull_rom(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
        let t2 = t * t;
        let t3 = t2 * t;

        0.5 * ((2.0 * p1)
            + (-p0 + p2) * t
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
    }

    /// Apply the curve to a single value using the lookup table
    fn apply_curve(&self, value: f32) -> f32 {
        let idx = (value * 255.0).clamp(0.0, 255.0) as usize;
        self.lut[idx]
    }
}

impl Default for Curves {
    fn default() -> Self {
        Self::new(
            vec![CurvePoint::new(0.0, 0.0), CurvePoint::new(1.0, 1.0)],
            CurveChannel::RGB,
        )
    }
}

impl Adjustment for Curves {
    fn apply_pixel(&self, color: Color) -> Color {
        match self.channel {
            CurveChannel::RGB => {
                Color::from_rgba(
                    self.apply_curve(color.r),
                    self.apply_curve(color.g),
                    self.apply_curve(color.b),
                    color.a,
                )
            }
            CurveChannel::Red => {
                Color::from_rgba(
                    self.apply_curve(color.r),
                    color.g,
                    color.b,
                    color.a,
                )
            }
            CurveChannel::Green => {
                Color::from_rgba(
                    color.r,
                    self.apply_curve(color.g),
                    color.b,
                    color.a,
                )
            }
            CurveChannel::Blue => {
                Color::from_rgba(
                    color.r,
                    color.g,
                    self.apply_curve(color.b),
                    color.a,
                )
            }
        }
    }

    fn name(&self) -> &'static str {
        "Curves"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_curve() {
        let adj = Curves::default();
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let result = adj.apply_pixel(color);
        assert!((result.r - 0.5).abs() < 0.02);
    }

    #[test]
    fn test_s_curve() {
        let points = vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(0.25, 0.15),
            CurvePoint::new(0.75, 0.85),
            CurvePoint::new(1.0, 1.0),
        ];
        let adj = Curves::new(points, CurveChannel::RGB);

        // Dark values should get darker
        let dark = Color::from_rgba(0.25, 0.25, 0.25, 1.0);
        let dark_result = adj.apply_pixel(dark);
        assert!(dark_result.r < 0.25);

        // Light values should get lighter
        let light = Color::from_rgba(0.75, 0.75, 0.75, 1.0);
        let light_result = adj.apply_pixel(light);
        assert!(light_result.r > 0.75);
    }
}
