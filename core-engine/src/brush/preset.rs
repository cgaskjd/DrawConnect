//! Brush presets - built-in brush configurations

use super::{Brush, BrushDynamics, BrushSettings, BrushShape, BrushTip, DynamicsCurve};
use uuid::Uuid;

/// Brush preset factory
pub struct BrushPreset;

impl BrushPreset {
    /// Create a soft round brush
    pub fn round_soft() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Soft Round".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 20.0,
                hardness: 0.0,
                opacity: 1.0,
                spacing: 0.1,
                smoothing: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Basic".into(),
            is_custom: false,
        }
    }

    /// Create a hard round brush
    pub fn round_hard() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Hard Round".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 20.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.1,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Basic".into(),
            is_custom: false,
        }
    }

    /// Create a pencil brush
    pub fn pencil() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pencil".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 3.0,
                hardness: 0.9,
                opacity: 0.8,
                spacing: 0.05,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics::pencil(),
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a mechanical pencil brush
    pub fn mechanical_pencil() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Mechanical Pencil".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 1.5,
                min_size_ratio: 0.8,
                hardness: 1.0,
                opacity: 0.9,
                spacing: 0.03,
                smoothing: 0.3,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create an ink pen brush
    pub fn ink_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Ink Pen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 5.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.05,
                smoothing: 0.7,
                ..Default::default()
            },
            dynamics: BrushDynamics::ink(),
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create a calligraphy brush
    pub fn calligraphy() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Calligraphy".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 15.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.05,
                angle: 45.0,
                roundness: 0.3,
                smoothing: 0.8,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create an oil brush
    pub fn oil_brush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Oil Brush".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 30.0,
                hardness: 0.6,
                opacity: 0.9,
                spacing: 0.08,
                flow: 0.8,
                wet_edges: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                tilt_angle_enabled: true,
                tilt_angle_sensitivity: 0.5,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a watercolor brush
    pub fn watercolor() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Watercolor".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 40.0,
                hardness: 0.2,
                opacity: 0.5,
                spacing: 0.1,
                flow: 0.6,
                wet_edges: true,
                ..Default::default()
            },
            dynamics: BrushDynamics::watercolor(),
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create an acrylic brush
    pub fn acrylic() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Acrylic".into(),
            shape: BrushShape::Flat,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 25.0,
                hardness: 0.7,
                opacity: 0.95,
                spacing: 0.1,
                flow: 0.9,
                roundness: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                tilt_angle_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create an airbrush
    pub fn airbrush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Airbrush".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Airbrush,
            settings: BrushSettings {
                size: 100.0,
                hardness: 0.0,
                opacity: 0.1,
                spacing: 0.05,
                flow: 0.3,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics::airbrush(),
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create an eraser brush
    pub fn eraser() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Eraser".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 20.0,
                hardness: 0.5,
                opacity: 1.0,
                spacing: 0.1,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a smudge brush
    pub fn smudge() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Smudge".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 25.0,
                hardness: 0.3,
                opacity: 0.5,
                spacing: 0.05,
                flow: 0.7,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a blur brush
    pub fn blur() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Blur".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 30.0,
                hardness: 0.0,
                opacity: 0.5,
                spacing: 0.1,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a marker brush
    pub fn marker() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Marker".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 20.0,
                hardness: 0.8,
                opacity: 0.7,
                spacing: 0.05,
                roundness: 0.5,
                angle: 30.0,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a pixel brush
    pub fn pixel() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pixel".into(),
            shape: BrushShape::Square,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 1.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 1.0,
                anti_aliasing: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Pixel Art".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Basic Brushes
    // ========================================================================

    /// Create a medium round brush
    pub fn medium_round() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Medium Round".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 50.0,
                hardness: 0.5,
                opacity: 1.0,
                spacing: 0.1,
                smoothing: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Basic".into(),
            is_custom: false,
        }
    }

    /// Create a large soft brush
    pub fn large_soft() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Large Soft".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 150.0,
                hardness: 0.0,
                opacity: 0.8,
                spacing: 0.08,
                smoothing: 0.6,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Basic".into(),
            is_custom: false,
        }
    }

    /// Create a fine point brush
    pub fn fine_point() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Fine Point".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 2.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.03,
                smoothing: 0.4,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Basic".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Sketching Brushes
    // ========================================================================

    /// Create a charcoal brush
    pub fn charcoal() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Charcoal".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 15.0,
                hardness: 0.3,
                opacity: 0.7,
                spacing: 0.05,
                build_up: true,
                angle: 30.0,
                roundness: 0.6,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                tilt_angle_enabled: true,
                tilt_angle_sensitivity: 0.6,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a pastel brush
    pub fn pastel() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pastel".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 20.0,
                hardness: 0.4,
                opacity: 0.6,
                spacing: 0.06,
                build_up: true,
                roundness: 0.7,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                tilt_angle_enabled: true,
                tilt_size_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a conte crayon brush
    pub fn conte_crayon() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Conte Crayon".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 12.0,
                hardness: 0.5,
                opacity: 0.8,
                spacing: 0.04,
                build_up: true,
                angle: 45.0,
                roundness: 0.4,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                tilt_angle_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a sketch pencil brush
    pub fn sketch_pencil() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Sketch Pencil".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 5.0,
                hardness: 0.7,
                opacity: 0.6,
                spacing: 0.04,
                build_up: true,
                smoothing: 0.3,
                ..Default::default()
            },
            dynamics: BrushDynamics::pencil(),
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    /// Create a color pencil brush
    pub fn color_pencil() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Color Pencil".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 4.0,
                hardness: 0.6,
                opacity: 0.5,
                spacing: 0.03,
                build_up: true,
                smoothing: 0.4,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Sketching".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Inking Brushes
    // ========================================================================

    /// Create a G pen brush (manga style)
    pub fn g_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "G Pen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 8.0,
                min_size_ratio: 0.1,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.03,
                smoothing: 0.8,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                size_pressure_curve: DynamicsCurve::ease_out(),
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create a maru pen brush
    pub fn maru_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Maru Pen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 3.0,
                min_size_ratio: 0.3,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.02,
                smoothing: 0.6,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                size_pressure_curve: DynamicsCurve::smooth(),
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create a brush pen
    pub fn brush_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Brush Pen".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 12.0,
                min_size_ratio: 0.05,
                hardness: 0.9,
                opacity: 1.0,
                spacing: 0.04,
                smoothing: 0.7,
                roundness: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                size_pressure_curve: DynamicsCurve::ease_out(),
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create a comic pen brush
    pub fn comic_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Comic Pen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 6.0,
                min_size_ratio: 0.2,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.03,
                smoothing: 0.75,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    /// Create a technical pen brush
    pub fn technical_pen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Technical Pen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 2.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.02,
                smoothing: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Inking".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Painting Brushes
    // ========================================================================

    /// Create a gouache brush
    pub fn gouache() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Gouache".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 35.0,
                hardness: 0.5,
                opacity: 0.95,
                spacing: 0.08,
                flow: 0.85,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create an impasto brush
    pub fn impasto() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Impasto".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 40.0,
                hardness: 0.8,
                opacity: 1.0,
                spacing: 0.06,
                flow: 1.0,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                tilt_angle_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a palette knife brush
    pub fn palette_knife() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Palette Knife".into(),
            shape: BrushShape::Flat,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 50.0,
                hardness: 0.9,
                opacity: 0.9,
                spacing: 0.1,
                flow: 1.0,
                angle: 0.0,
                roundness: 0.2,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                tilt_angle_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a fan brush
    pub fn fan_brush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Fan Brush".into(),
            shape: BrushShape::Flat,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 60.0,
                hardness: 0.4,
                opacity: 0.7,
                spacing: 0.12,
                flow: 0.6,
                roundness: 0.15,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a filbert brush
    pub fn filbert() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Filbert".into(),
            shape: BrushShape::Ellipse,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 30.0,
                hardness: 0.6,
                opacity: 0.9,
                spacing: 0.08,
                flow: 0.85,
                roundness: 0.6,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                tilt_angle_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a flat brush
    pub fn flat_brush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Flat Brush".into(),
            shape: BrushShape::Flat,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 35.0,
                hardness: 0.7,
                opacity: 0.9,
                spacing: 0.08,
                flow: 0.8,
                roundness: 0.3,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                tilt_angle_enabled: true,
                rotation_follow_stroke: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a round painting brush
    pub fn round_brush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Round Brush".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Bristle,
            settings: BrushSettings {
                size: 25.0,
                hardness: 0.5,
                opacity: 0.9,
                spacing: 0.08,
                flow: 0.85,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    /// Create a detail brush
    pub fn detail_brush() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Detail Brush".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 8.0,
                hardness: 0.7,
                opacity: 0.95,
                spacing: 0.05,
                flow: 0.9,
                smoothing: 0.6,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Painting".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Special Brushes
    // ========================================================================

    /// Create a spray brush
    pub fn spray() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Spray".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Airbrush,
            settings: BrushSettings {
                size: 80.0,
                hardness: 0.2,
                opacity: 0.3,
                spacing: 0.03,
                flow: 0.4,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                scatter: 0.5,
                size_jitter: 0.3,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create a splatter brush
    pub fn splatter() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Splatter".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 50.0,
                hardness: 0.8,
                opacity: 0.8,
                spacing: 0.3,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                scatter: 1.5,
                size_jitter: 0.6,
                opacity_jitter: 0.3,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create a glow brush
    pub fn glow() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Glow".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 60.0,
                hardness: 0.0,
                opacity: 0.2,
                spacing: 0.05,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create a soft glow brush
    pub fn soft_glow() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Soft Glow".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 100.0,
                hardness: 0.0,
                opacity: 0.1,
                spacing: 0.04,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create a noise brush
    pub fn noise() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Noise".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Textured,
            settings: BrushSettings {
                size: 30.0,
                hardness: 0.5,
                opacity: 0.6,
                spacing: 0.15,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                scatter: 0.8,
                size_jitter: 0.4,
                opacity_jitter: 0.5,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    /// Create a stipple brush
    pub fn stipple() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Stipple".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 5.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.5,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                scatter: 1.0,
                size_jitter: 0.3,
                ..Default::default()
            },
            texture: None,
            category: "Special".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Pixel Art Brushes
    // ========================================================================

    /// Create a 2x2 pixel brush
    pub fn pixel_2x2() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pixel 2x2".into(),
            shape: BrushShape::Square,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 2.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 1.0,
                anti_aliasing: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Pixel Art".into(),
            is_custom: false,
        }
    }

    /// Create a 3x3 pixel brush
    pub fn pixel_3x3() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pixel 3x3".into(),
            shape: BrushShape::Square,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 3.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 1.0,
                anti_aliasing: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Pixel Art".into(),
            is_custom: false,
        }
    }

    /// Create a dither brush
    pub fn dither() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Dither".into(),
            shape: BrushShape::Square,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 1.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 2.0,
                anti_aliasing: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Pixel Art".into(),
            is_custom: false,
        }
    }

    /// Create a pixel line brush
    pub fn pixel_line() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Pixel Line".into(),
            shape: BrushShape::Square,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 1.0,
                min_size_ratio: 1.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.5,
                anti_aliasing: false,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: false,
                opacity_pressure_enabled: false,
                ..Default::default()
            },
            texture: None,
            category: "Pixel Art".into(),
            is_custom: false,
        }
    }

    // ========================================================================
    // Additional Utility Brushes
    // ========================================================================

    /// Create a hard eraser brush
    pub fn hard_eraser() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Hard Eraser".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Solid,
            settings: BrushSettings {
                size: 20.0,
                hardness: 1.0,
                opacity: 1.0,
                spacing: 0.1,
                ..Default::default()
            },
            dynamics: BrushDynamics::default(),
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a soft eraser brush
    pub fn soft_eraser() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Soft Eraser".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 30.0,
                hardness: 0.0,
                opacity: 0.8,
                spacing: 0.08,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a sharpen brush
    pub fn sharpen() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Sharpen".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 25.0,
                hardness: 0.3,
                opacity: 0.5,
                spacing: 0.1,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Create a dodge/burn brush
    pub fn dodge_burn() -> Brush {
        Brush {
            id: Uuid::new_v4(),
            name: "Dodge Burn".into(),
            shape: BrushShape::Round,
            tip: BrushTip::Soft,
            settings: BrushSettings {
                size: 40.0,
                hardness: 0.2,
                opacity: 0.3,
                spacing: 0.08,
                build_up: true,
                ..Default::default()
            },
            dynamics: BrushDynamics {
                size_pressure_enabled: true,
                opacity_pressure_enabled: true,
                ..Default::default()
            },
            texture: None,
            category: "Utility".into(),
            is_custom: false,
        }
    }

    /// Get all default presets
    pub fn all_presets() -> Vec<Brush> {
        vec![
            // Basic (5)
            Self::round_soft(),
            Self::round_hard(),
            Self::medium_round(),
            Self::large_soft(),
            Self::fine_point(),
            // Sketching (8)
            Self::pencil(),
            Self::mechanical_pencil(),
            Self::charcoal(),
            Self::pastel(),
            Self::conte_crayon(),
            Self::sketch_pencil(),
            Self::color_pencil(),
            Self::marker(),
            // Inking (7)
            Self::ink_pen(),
            Self::calligraphy(),
            Self::g_pen(),
            Self::maru_pen(),
            Self::brush_pen(),
            Self::comic_pen(),
            Self::technical_pen(),
            // Painting (11)
            Self::oil_brush(),
            Self::watercolor(),
            Self::acrylic(),
            Self::gouache(),
            Self::impasto(),
            Self::palette_knife(),
            Self::fan_brush(),
            Self::filbert(),
            Self::flat_brush(),
            Self::round_brush(),
            Self::detail_brush(),
            // Special (7)
            Self::airbrush(),
            Self::spray(),
            Self::splatter(),
            Self::glow(),
            Self::soft_glow(),
            Self::noise(),
            Self::stipple(),
            // Pixel Art (5)
            Self::pixel(),
            Self::pixel_2x2(),
            Self::pixel_3x3(),
            Self::dither(),
            Self::pixel_line(),
            // Utility (7)
            Self::eraser(),
            Self::hard_eraser(),
            Self::soft_eraser(),
            Self::smudge(),
            Self::blur(),
            Self::sharpen(),
            Self::dodge_burn(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presets_creation() {
        let presets = BrushPreset::all_presets();
        assert!(!presets.is_empty());
        assert_eq!(presets.len(), 50, "Should have exactly 50 brush presets");
    }

    #[test]
    fn test_preset_uniqueness() {
        let presets = BrushPreset::all_presets();
        let ids: Vec<_> = presets.iter().map(|b| b.id).collect();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(ids.len(), unique_ids.len());
    }

    #[test]
    fn test_preset_categories() {
        let presets = BrushPreset::all_presets();
        let categories: std::collections::HashSet<_> = presets.iter().map(|b| b.category.as_str()).collect();

        // Verify all expected categories exist
        assert!(categories.contains("Basic"));
        assert!(categories.contains("Sketching"));
        assert!(categories.contains("Inking"));
        assert!(categories.contains("Painting"));
        assert!(categories.contains("Special"));
        assert!(categories.contains("Pixel Art"));
        assert!(categories.contains("Utility"));
    }
}
