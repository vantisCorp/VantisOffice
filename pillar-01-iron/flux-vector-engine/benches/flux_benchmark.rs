// Performance benchmarks for Flux Vector Engine
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use flux_vector_engine::{
    VectorEngine, Path, Paint, Color, FillType,
    WindowConfig, VSyncMode, Component, Animation, EasingFunction
};

fn bench_engine_initialization(c: &mut Criterion) {
    c.bench_function("flux_engine_initialization", |b| {
        b.iter(|| {
            VectorEngine::new()
        });
    });
}

fn bench_path_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_path_creation");
    
    for points in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(points), points, |b, count| {
            b.iter(|| {
                let mut path = Path::new();
                for i in 0..*count {
                    let angle = (i as f32 / *count as f32) * 2.0 * std::f32::consts::PI;
                    let x = 100.0 + angle.cos() * 50.0;
                    let y = 100.0 + angle.sin() * 50.0;
                    path.line_to(x, y);
                }
                path
            });
        });
    }
    
    group.finish();
}

fn bench_path_rendering_simple(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 100.0);
    path.line_to(200.0, 0.0);
    path.close();
    
    let paint = Paint {
        color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        fill_type: FillType::EvenOdd,
        ..Default::default()
    };
    
    c.bench_function("flux_path_rendering_simple", |b| {
        b.iter(|| {
            engine.draw_path(black_box(&path), black_box(&paint))
        });
    });
}

fn bench_path_rendering_complex(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let path = create_complex_path(500);
    
    let paint = Paint {
        color: Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
        fill_type: FillType::Winding,
        ..Default::default()
    };
    
    c.bench_function("flux_path_rendering_complex", |b| {
        b.iter(|| {
            engine.draw_path(black_box(&path), black_box(&paint))
        });
    });
}

fn bench_multiple_paths_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_multiple_paths_rendering");
    
    for count in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            b.iter(|| {
                let mut engine = VectorEngine::new().unwrap();
                let paint = Paint {
                    color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
                    fill_type: FillType::EvenOdd,
                    ..Default::default()
                };
                
                for i in 0..*count {
                    let mut path = Path::new();
                    let x = (i as f32 * 10.0) % 500.0;
                    let y = (i as f32 * 10.0) % 500.0;
                    path.move_to(x, y);
                    path.line_to(x + 50.0, y + 50.0);
                    path.line_to(x + 100.0, y);
                    path.close();
                    engine.draw_path(&path, &paint).unwrap();
                }
                
                engine
            });
        });
    }
    
    group.finish();
}

fn bench_fill_stroke_operations(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let path = create_complex_path(100);
    
    let mut group = c.benchmark_group("flux_fill_stroke_operations");
    
    group.bench_function("fill", |b| {
        b.iter(|| {
            let paint = Paint {
                color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
                fill_type: FillType::EvenOdd,
                ..Default::default()
            };
            engine.fill_path(black_box(&path), black_box(&paint))
        });
    });
    
    group.bench_function("stroke", |b| {
        b.iter(|| {
            let paint = Paint {
                color: Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
                fill_type: FillType::EvenOdd,
                ..Default::default()
            };
            engine.stroke_path(black_box(&path), black_box(2.0), black_box(&paint))
        });
    });
    
    group.finish();
}

fn bench_color_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_color_operations");
    
    let colors = vec![
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
        Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
        Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 },
        Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 },
    ];
    
    group.bench_function("color_blend", |b| {
        b.iter(|| {
            Color::blend(
                black_box(colors[0]),
                black_box(colors[1]),
                black_box(0.5)
            )
        });
    });
    
    group.bench_function("color_lerp", |b| {
        b.iter(|| {
            Color::lerp(
                black_box(colors[0]),
                black_box(colors[1]),
                black_box(0.5)
            )
        });
    });
    
    group.finish();
}

fn bench_animation_system(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let animation = Animation::new(1.0);
    
    c.bench_function("flux_animation_step", |b| {
        b.iter(|| {
            animation.step(black_box(0.016))
        });
    });
}

fn bench_easing_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_easing_functions");
    
    let easing_functions = vec![
        EasingFunction::Linear,
        EasingFunction::EaseIn,
        EasingFunction::EaseOut,
        EasingFunction::EaseInOut,
        EasingFunction::Bounce,
        EasingFunction::Elastic,
    ];
    
    for func in easing_functions.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{:?}", func)), func, |b, func| {
            b.iter(|| {
                func.apply(black_box(0.5))
            });
        });
    }
    
    group.finish();
}

fn bench_frame_rendering(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let path = create_complex_path(100);
    let paint = Paint {
        color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        fill_type: FillType::EvenOdd,
        ..Default::default()
    };
    
    c.bench_function("flux_frame_rendering", |b| {
        b.iter(|| {
            engine.begin_frame().unwrap();
            engine.draw_path(&path, &paint).unwrap();
            engine.end_frame().unwrap();
        });
    });
}

fn bench_vsync_modes(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_vsync_modes");
    
    let vsync_modes = vec![
        VSyncMode::Off,
        VSyncMode::On,
        VSyncMode::Adaptive,
    ];
    
    for mode in vsync_modes.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{:?}", mode)), mode, |b, mode| {
            b.iter(|| {
                let config = WindowConfig {
                    vsync: *mode,
                    ..Default::default()
                };
                VectorEngine::with_config(black_box(config))
            });
        });
    }
    
    group.finish();
}

fn bench_gradient_rendering(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let path = create_complex_path(50);
    
    let mut group = c.benchmark_group("flux_gradient_rendering");
    
    group.bench_function("linear_gradient", |b| {
        b.iter(|| {
            let gradient = Paint::linear_gradient(
                Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
                Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
                0.0, 0.0, 100.0, 100.0
            );
            engine.draw_path(black_box(&path), black_box(&gradient))
        });
    });
    
    group.bench_function("radial_gradient", |b| {
        b.iter(|| {
            let gradient = Paint::radial_gradient(
                Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
                Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
                50.0, 50.0, 0.0, 50.0
            );
            engine.draw_path(black_box(&path), black_box(&gradient))
        });
    });
    
    group.finish();
}

fn bench_transformation_operations(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    let path = create_complex_path(100);
    
    let mut group = c.benchmark_group("flux_transformation_operations");
    
    group.bench_function("translate", |b| {
        b.iter(|| {
            engine.translate(black_box(10.0), black_box(20.0));
            engine.draw_path(&path, &Paint::default()).unwrap();
        });
    });
    
    group.bench_function("rotate", |b| {
        b.iter(|| {
            engine.rotate(black_box(std::f32::consts::PI / 4.0));
            engine.draw_path(&path, &Paint::default()).unwrap();
        });
    });
    
    group.bench_function("scale", |b| {
        b.iter(|| {
            engine.scale(black_box(2.0), black_box(2.0));
            engine.draw_path(&path, &Paint::default()).unwrap();
        });
    });
    
    group.finish();
}

fn bench_text_rendering(c: &mut Criterion) {
    let mut engine = VectorEngine::new().unwrap();
    
    c.bench_function("flux_text_rendering", |b| {
        b.iter(|| {
            engine.draw_text(
                black_box("Hello, World!"),
                black_box(10.0),
                black_box(10.0),
                black_box(24.0),
                black_box(&Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 })
            )
        });
    });
}

fn bench_batch_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("flux_batch_rendering");
    
    for count in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            b.iter(|| {
                let mut engine = VectorEngine::new().unwrap();
                let paths: Vec<_> = (0..*count)
                    .map(|i| {
                        let mut path = Path::new();
                        let x = (i as f32 * 10.0) % 500.0;
                        let y = (i as f32 * 10.0) % 500.0;
                        path.move_to(x, y);
                        path.line_to(x + 50.0, y + 50.0);
                        path.close();
                        path
                    })
                    .collect();
                
                let paint = Paint::default();
                engine.draw_paths(black_box(&paths), black_box(&paint))
            });
        });
    }
    
    group.finish();
}

// Helper function to create complex paths
fn create_complex_path(points: usize) -> Path {
    let mut path = Path::new();
    path.move_to(100.0, 100.0);
    
    for i in 0..points {
        let angle = (i as f32 / points as f32) * 2.0 * std::f32::consts::PI;
        let x = 100.0 + angle.cos() * 50.0;
        let y = 100.0 + angle.sin() * 50.0;
        
        if i % 2 == 0 {
            path.line_to(x, y);
        } else {
            path.quad_to(
                x + 10.0,
                y + 10.0,
                x + 20.0,
                y
            );
        }
    }
    
    path.close();
    path
}

criterion_group!(
    benches,
    bench_engine_initialization,
    bench_path_creation,
    bench_path_rendering_simple,
    bench_path_rendering_complex,
    bench_multiple_paths_rendering,
    bench_fill_stroke_operations,
    bench_color_operations,
    bench_animation_system,
    bench_easing_functions,
    bench_frame_rendering,
    bench_vsync_modes,
    bench_gradient_rendering,
    bench_transformation_operations,
    bench_text_rendering,
    bench_batch_rendering
);

criterion_main!(benches);