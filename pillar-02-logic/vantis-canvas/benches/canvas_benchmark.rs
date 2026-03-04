// Performance benchmarks for Vantis Canvas
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use vantis_canvas::core::{Canvas, Layer, Shape, ShapeType, Text};

fn benchmark_canvas_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas_creation");

    group.bench_function("create_canvas", |b| {
        b.iter(|| black_box(Canvas::new("Presentation".to_string())))
    });

    group.bench_function("create_canvas_with_slide", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Presentation".to_string());
            canvas.add_slide();
            black_box(canvas)
        })
    });

    group.finish();
}

fn benchmark_shape_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("shape_creation");

    group.bench_function("create_rectangle", |b| {
        b.iter(|| black_box(Shape::new(ShapeType::Rectangle)))
    });

    group.bench_function("create_circle", |b| {
        b.iter(|| black_box(Shape::new(ShapeType::Circle)))
    });

    group.bench_function("create_triangle", |b| {
        b.iter(|| black_box(Shape::new(ShapeType::Triangle)))
    });

    group.bench_function("create_star", |b| {
        b.iter(|| {
            black_box(Shape::new(ShapeType::Star {
                points: 5,
                inner_radius: 0.5,
            }))
        })
    });

    group.finish();
}

fn benchmark_slide_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("slide_operations");

    let slide_counts = vec![1, 5, 10, 20];

    for count in slide_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut canvas = Canvas::new("Presentation".to_string());
                for _ in 0..count {
                    canvas.add_slide();
                }
                black_box(canvas)
            })
        });
    }

    group.finish();
}

fn benchmark_shape_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("shape_operations");

    let shape_counts = vec![10, 50, 100, 500];

    for count in shape_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut canvas = Canvas::new("Presentation".to_string());
                let slide = canvas.add_slide();
                for i in 0..count {
                    let mut shape = Shape::new(ShapeType::Rectangle);
                    shape.position.x = (i % 10) as f64 * 100.0;
                    shape.position.y = (i / 10) as f64 * 100.0;
                    slide.add_shape(shape);
                }
                black_box(canvas)
            })
        });
    }

    group.finish();
}

fn benchmark_text_elements(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_elements");

    group.bench_function("create_text", |b| {
        b.iter(|| black_box(Text::new("Sample text".to_string())))
    });

    group.bench_function("create_text_with_styling", |b| {
        b.iter(|| {
            let mut text = Text::new("Styled text".to_string());
            text.font.size = 24.0;
            text.font.family = "Arial".to_string();
            black_box(text)
        })
    });

    group.finish();
}

fn benchmark_layer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("layer_operations");

    group.bench_function("create_layer", |b| b.iter(|| black_box(Layer::new(0))));

    group.bench_function("create_multiple_layers", |b| {
        b.iter(|| {
            let mut layers = Vec::new();
            for i in 0..10 {
                layers.push(Layer::new(i));
            }
            black_box(layers)
        })
    });

    group.finish();
}

fn benchmark_complex_canvas(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_canvas");

    group.bench_function("canvas_with_100_shapes", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Complex Presentation".to_string());
            let slide = canvas.add_slide();

            for i in 0..100 {
                let shape_type = match i % 4 {
                    0 => ShapeType::Rectangle,
                    1 => ShapeType::Circle,
                    2 => ShapeType::Triangle,
                    _ => ShapeType::Star {
                        points: 5,
                        inner_radius: 0.5,
                    },
                };
                let mut shape = Shape::new(shape_type);
                shape.position.x = (i % 10) as f64 * 150.0;
                shape.position.y = (i / 10) as f64 * 100.0;
                slide.add_shape(shape);
            }

            black_box(canvas)
        })
    });

    group.bench_function("canvas_with_text_and_shapes", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Mixed Content".to_string());
            let slide = canvas.add_slide();

            for i in 0..50 {
                let mut shape = Shape::new(ShapeType::Rectangle);
                shape.position.x = i as f64 * 20.0;
                shape.position.y = 50.0;
                slide.add_shape(shape);

                let mut text = Text::new(format!("Text {}", i));
                text.position.x = i as f64 * 20.0;
                text.position.y = 100.0;
                slide.add_text(text);
            }

            black_box(canvas)
        })
    });

    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    group.bench_function("serialize_simple_canvas", |b| {
        let canvas = Canvas::new("Test".to_string());
        b.iter(|| black_box(serde_json::to_string(black_box(&canvas)).unwrap()))
    });

    group.bench_function("deserialize_simple_canvas", |b| {
        let canvas = Canvas::new("Test".to_string());
        let json = serde_json::to_string(&canvas).unwrap();
        b.iter(|| black_box(serde_json::from_str::<Canvas>(black_box(&json)).unwrap()))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_canvas_creation,
    benchmark_shape_creation,
    benchmark_slide_operations,
    benchmark_shape_operations,
    benchmark_text_elements,
    benchmark_layer_operations,
    benchmark_complex_canvas,
    benchmark_serialization
);
criterion_main!(benches);
