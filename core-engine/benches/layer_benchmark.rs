//! Layer system benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use drawconnect_core::*;

/// Benchmark layer creation
fn bench_layer_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("layer_creation");

    for size in [256, 512, 1024, 2048].iter() {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", size, size)),
            size,
            |b, &size| {
                b.iter(|| {
                    black_box(layer::Layer::new("Test Layer", size, size))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark pixel operations
fn bench_pixel_operations(c: &mut Criterion) {
    let mut layer = layer::Layer::new("Test", 1024, 1024);
    let color = Color::from_rgba(1.0, 0.5, 0.25, 1.0);

    c.bench_function("set_pixel_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                layer.set_pixel(i % 1024, i / 1024, color);
            }
        })
    });

    c.bench_function("get_pixel_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(layer.get_pixel(i % 1024, i / 1024));
            }
        })
    });
}

/// Benchmark blend modes
fn bench_blend_modes(c: &mut Criterion) {
    let base = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
    let blend = Color::from_rgba(0.8, 0.3, 0.6, 0.7);

    let mut group = c.benchmark_group("blend_modes");

    let modes = [
        ("Normal", BlendMode::Normal),
        ("Multiply", BlendMode::Multiply),
        ("Screen", BlendMode::Screen),
        ("Overlay", BlendMode::Overlay),
        ("SoftLight", BlendMode::SoftLight),
    ];

    for (name, mode) in modes.iter() {
        group.bench_function(*name, |b| {
            b.iter(|| {
                for _ in 0..1000 {
                    black_box(mode.blend(base, blend));
                }
            })
        });
    }

    group.finish();
}

/// Benchmark layer compositing
fn bench_layer_compositing(c: &mut Criterion) {
    let mut manager = layer::LayerManager::with_canvas_size(512, 512);

    // Add multiple layers
    for i in 0..10 {
        let id = manager.add_layer(format!("Layer {}", i));
        if let Some(layer) = manager.get_layer(id) {
            let mut layer = layer.write();
            // Fill with some pattern
            for y in 0..512 {
                for x in 0..512 {
                    if (x + y + i as u32) % 4 == 0 {
                        layer.set_pixel(x, y, Color::from_rgba(
                            i as f32 / 10.0,
                            0.5,
                            1.0 - i as f32 / 10.0,
                            0.5,
                        ));
                    }
                }
            }
        }
    }

    c.bench_function("flatten_10_layers_512x512", |b| {
        b.iter(|| {
            black_box(manager.flatten())
        })
    });
}

/// Benchmark tile manager operations
fn bench_tile_manager(c: &mut Criterion) {
    let mut group = c.benchmark_group("tile_manager");

    // Benchmark sparse vs dense access patterns
    group.bench_function("sparse_access", |b| {
        let mut manager = canvas::TileManager::new(256, 4096, 4096);
        b.iter(|| {
            // Access scattered pixels
            for i in 0..100 {
                let x = (i * 397) % 4096;
                let y = (i * 503) % 4096;
                manager.set_pixel(x, y, Color::red());
            }
        })
    });

    group.bench_function("dense_access", |b| {
        let mut manager = canvas::TileManager::new(256, 4096, 4096);
        b.iter(|| {
            // Access pixels in same tile
            for y in 0..10 {
                for x in 0..10 {
                    manager.set_pixel(x, y, Color::red());
                }
            }
        })
    });

    group.finish();
}

/// Benchmark undo/redo operations
fn bench_undo_redo(c: &mut Criterion) {
    c.bench_function("save_undo_512x512", |b| {
        let mut canvas = Canvas::with_size(512, 512).unwrap();
        // Draw something
        for y in 0..100 {
            for x in 0..100 {
                canvas.set_pixel(x, y, Color::red()).unwrap();
            }
        }

        b.iter(|| {
            canvas.save_undo("test");
        })
    });
}

criterion_group!(
    benches,
    bench_layer_creation,
    bench_pixel_operations,
    bench_blend_modes,
    bench_layer_compositing,
    bench_tile_manager,
    bench_undo_redo,
);

criterion_main!(benches);
