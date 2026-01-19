//! Brush engine benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use drawconnect_core::*;

/// Benchmark brush stamp generation
fn bench_stamp_generation(c: &mut Criterion) {
    let brush = Brush::new("Test Brush");

    let mut group = c.benchmark_group("stamp_generation");

    for size in [10, 20, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("size", size),
            size,
            |b, &size| {
                b.iter(|| {
                    black_box(brush.generate_stamp(size as f32, 0.5, 0.0))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark pressure calculations
fn bench_pressure_calculation(c: &mut Criterion) {
    let brush = Brush::new("Test Brush");

    c.bench_function("size_at_pressure", |b| {
        b.iter(|| {
            for p in 0..100 {
                black_box(brush.size_at_pressure(p as f32 / 100.0));
            }
        })
    });

    c.bench_function("opacity_at_pressure", |b| {
        b.iter(|| {
            for p in 0..100 {
                black_box(brush.opacity_at_pressure(p as f32 / 100.0));
            }
        })
    });
}

/// Benchmark stroke building with smoothing
fn bench_stroke_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("stroke_building");

    for smoothing in [0.0, 0.5, 1.0].iter() {
        group.bench_with_input(
            BenchmarkId::new("smoothing", format!("{:.1}", smoothing)),
            smoothing,
            |b, &smoothing| {
                b.iter(|| {
                    let mut builder = StrokeBuilder::new();
                    builder.set_smoothing(smoothing);
                    builder.begin(
                        uuid::Uuid::new_v4(),
                        uuid::Uuid::new_v4(),
                        "#000000",
                    );

                    for i in 0..100 {
                        builder.add_point(StrokePoint::new(
                            i as f32,
                            i as f32 + (i as f32 * 0.1).sin() * 5.0,
                            0.5 + (i as f32 * 0.05).sin() * 0.3,
                        ));
                    }

                    black_box(builder.end())
                })
            },
        );
    }

    group.finish();
}

/// Benchmark dynamics curve evaluation
fn bench_dynamics_curve(c: &mut Criterion) {
    use drawconnect_core::brush::BrushDynamics;

    let dynamics = BrushDynamics::default();

    c.bench_function("curve_evaluation_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f32 / 1000.0;
                black_box(dynamics.size_pressure_curve.evaluate(t));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_stamp_generation,
    bench_pressure_calculation,
    bench_stroke_building,
    bench_dynamics_curve,
);

criterion_main!(benches);
