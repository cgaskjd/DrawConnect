//! Geometry utilities module

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Rectangle structure
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Create from two points
    pub fn from_points(p1: Vec2, p2: Vec2) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs();
        let height = (p1.y - p2.y).abs();
        Self { x, y, width, height }
    }

    /// Get center point
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Check if point is inside rectangle
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    /// Check if rectangles intersect
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.x + self.width < other.x
            || other.x + other.width < self.x
            || self.y + self.height < other.y
            || other.y + other.height < self.y)
    }

    /// Get intersection with another rectangle
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);

        Some(Rect::new(x, y, right - x, bottom - y))
    }

    /// Get union with another rectangle
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);

        Rect::new(x, y, right - x, bottom - y)
    }

    /// Expand rectangle by amount
    pub fn expand(&self, amount: f32) -> Rect {
        Rect::new(
            self.x - amount,
            self.y - amount,
            self.width + amount * 2.0,
            self.height + amount * 2.0,
        )
    }

    /// Get area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// 2D transformation matrix
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// Matrix elements [a, b, c, d, tx, ty]
    /// | a  b  tx |
    /// | c  d  ty |
    /// | 0  0  1  |
    pub matrix: [f32; 6],
}

impl Transform {
    /// Identity transform
    pub fn identity() -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

    /// Translation transform
    pub fn translation(tx: f32, ty: f32) -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 1.0, tx, ty],
        }
    }

    /// Scale transform
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            matrix: [sx, 0.0, 0.0, sy, 0.0, 0.0],
        }
    }

    /// Rotation transform (angle in radians)
    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            matrix: [cos, sin, -sin, cos, 0.0, 0.0],
        }
    }

    /// Combine two transforms
    pub fn multiply(&self, other: &Transform) -> Transform {
        let a = self.matrix;
        let b = other.matrix;

        Transform {
            matrix: [
                a[0] * b[0] + a[1] * b[2],
                a[0] * b[1] + a[1] * b[3],
                a[2] * b[0] + a[3] * b[2],
                a[2] * b[1] + a[3] * b[3],
                a[4] * b[0] + a[5] * b[2] + b[4],
                a[4] * b[1] + a[5] * b[3] + b[5],
            ],
        }
    }

    /// Transform a point
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        let m = &self.matrix;
        Vec2::new(
            m[0] * point.x + m[2] * point.y + m[4],
            m[1] * point.x + m[3] * point.y + m[5],
        )
    }

    /// Get inverse transform
    pub fn inverse(&self) -> Option<Transform> {
        let m = &self.matrix;
        let det = m[0] * m[3] - m[1] * m[2];

        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Transform {
            matrix: [
                m[3] * inv_det,
                -m[1] * inv_det,
                -m[2] * inv_det,
                m[0] * inv_det,
                (m[2] * m[5] - m[3] * m[4]) * inv_det,
                (m[1] * m[4] - m[0] * m[5]) * inv_det,
            ],
        })
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

/// Bezier curve utilities
pub struct Bezier;

impl Bezier {
    /// Evaluate quadratic bezier at t
    pub fn quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;

        p0 * mt2 + p1 * (2.0 * mt * t) + p2 * t2
    }

    /// Evaluate cubic bezier at t
    pub fn cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        p0 * mt3 + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * t3
    }

    /// Get points along cubic bezier
    pub fn cubic_points(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, segments: usize) -> Vec<Vec2> {
        (0..=segments)
            .map(|i| {
                let t = i as f32 / segments as f32;
                Self::cubic(p0, p1, p2, p3, t)
            })
            .collect()
    }

    /// Approximate bezier arc length
    pub fn cubic_length(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, segments: usize) -> f32 {
        let points = Self::cubic_points(p0, p1, p2, p3, segments);
        points.windows(2).map(|w| w[0].distance(w[1])).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10.0, 10.0, 100.0, 100.0);
        assert!(rect.contains(Vec2::new(50.0, 50.0)));
        assert!(!rect.contains(Vec2::new(0.0, 0.0)));
    }

    #[test]
    fn test_rect_intersection() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);

        let intersection = r1.intersection(&r2).unwrap();
        assert_eq!(intersection.x, 50.0);
        assert_eq!(intersection.width, 50.0);
    }

    #[test]
    fn test_transform_point() {
        let translate = Transform::translation(10.0, 20.0);
        let point = Vec2::new(0.0, 0.0);

        let transformed = translate.transform_point(point);
        assert!((transformed.x - 10.0).abs() < 0.01);
        assert!((transformed.y - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_transform_inverse() {
        let transform = Transform::translation(10.0, 20.0);
        let inverse = transform.inverse().unwrap();

        let point = Vec2::new(10.0, 20.0);
        let result = inverse.transform_point(point);

        assert!((result.x - 0.0).abs() < 0.01);
        assert!((result.y - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_bezier() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(50.0, 100.0);
        let p2 = Vec2::new(100.0, 0.0);

        let mid = Bezier::quadratic(p0, p1, p2, 0.5);
        assert!((mid.x - 50.0).abs() < 0.01);
    }
}
