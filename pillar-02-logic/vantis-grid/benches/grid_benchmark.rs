// Performance benchmarks for Vantis Grid
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_grid::core::{Cell, CellValue, Worksheet, Workbook};

fn benchmark_cell_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell_operations");
    
    group.bench_function("create_cell", |b| {
        b.iter(|| {
            black_box(Cell::new(0, 0))
        })
    });
    
    group.bench_function("create_cell_with_value", |b| {
        b.iter(|| {
            let cell = Cell::new(0, 0);
            black_box(cell.with_value(CellValue::Number(42.0)))
        })
    });
    
    group.bench_function("create_cell_with_formula", |b| {
        b.iter(|| {
            let cell = Cell::new(0, 0);
            black_box(cell.with_formula("=SUM(A1:A10)".to_string()))
        })
    });
    
    group.finish();
}

fn benchmark_worksheet_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("worksheet_operations");
    
    group.bench_function("create_worksheet", |b| {
        b.iter(|| {
            black_box(Worksheet::new("Sheet1".to_string()))
        })
    });
    
    group.bench_function("create_worksheet_with_100_cells", |b| {
        b.iter(|| {
            let mut worksheet = Worksheet::new("Sheet1".to_string());
            for row in 0..10 {
                for col in 0..10 {
                    worksheet.set_cell_value(row, col, CellValue::Number((row * 10 + col) as f64));
                }
            }
            black_box(worksheet)
        })
    });
    
    group.bench_function("create_worksheet_with_1000_cells", |b| {
        b.iter(|| {
            let mut worksheet = Worksheet::new("Sheet1".to_string());
            for row in 0..100 {
                for col in 0..10 {
                    worksheet.set_cell_value(row, col, CellValue::Number((row * 10 + col) as f64));
                }
            }
            black_box(worksheet)
        })
    });
    
    group.finish();
}

fn benchmark_workbook_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("workbook_operations");
    
    group.bench_function("create_workbook", |b| {
        b.iter(|| {
            black_box(Workbook::new("Workbook1".to_string()))
        })
    });
    
    group.bench_function("create_workbook_with_sheets", |b| {
        b.iter(|| {
            let mut workbook = Workbook::new("Workbook1".to_string());
            for i in 0..10 {
                workbook.add_worksheet(format!("Sheet{}", i));
            }
            black_box(workbook)
        })
    });
    
    group.finish();
}

fn benchmark_large_worksheet(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_worksheet");
    
    let sizes = vec![1000, 5000, 10000, 20000]; // Number of cells
    
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut worksheet = Worksheet::new("Sheet1".to_string());
                for i in 0..size {
                    let row = i / 100;
                    let col = i % 100;
                    worksheet.set_cell_value(row, col, CellValue::Number(i as f64));
                }
                black_box(worksheet)
            })
        });
    }
    
    group.finish();
}

fn benchmark_cell_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell_access");
    
    // Create a worksheet with cells
    let mut worksheet = Worksheet::new("Sheet1".to_string());
    for row in 0..100 {
        for col in 0..100 {
            worksheet.set_cell_value(row, col, CellValue::Number((row * 100 + col) as f64));
        }
    }
    
    group.bench_function("get_cell", |b| {
        b.iter(|| {
            black_box(worksheet.get_cell(50, 50))
        })
    });
    
    group.bench_function("set_cell", |b| {
        b.iter(|| {
            let mut ws = worksheet.clone();
            ws.set_cell_value(50, 50, CellValue::Number(999.0))
        })
    });
    
    group.bench_function("get_range_10x10", |b| {
        b.iter(|| {
            black_box(worksheet.get_cell(0, 0))
        })
    });
    
    group.finish();
}

fn benchmark_formula_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("formula_evaluation");
    
    // Create a worksheet with data
    let mut worksheet = Worksheet::new("Sheet1".to_string());
    for row in 0..100 {
        for col in 0..10 {
            worksheet.set_cell_value(row, col, CellValue::Number((row * 10 + col) as f64));
        }
    }
    
    group.bench_function("simple_formula", |b| {
        b.iter(|| {
            let cell = Cell::new(0, 0).with_formula("=A1+1".to_string());
            black_box(cell)
        })
    });
    
    group.bench_function("complex_formula", |b| {
        b.iter(|| {
            let cell = Cell::new(0, 0).with_formula("=SUM(A1:A100)+AVERAGE(B1:B100)".to_string());
            black_box(cell)
        })
    });
    
    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    // Create a simple cell value to serialize
    let cell_value = CellValue::Number(42.0);
    
    group.bench_function("serialize_cell_value", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&cell_value)).unwrap())
        })
    });
    
    group.bench_function("deserialize_cell_value", |b| {
        let json = serde_json::to_string(&cell_value).unwrap();
        b.iter(|| {
            black_box(serde_json::from_str::<CellValue>(&json).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    group.bench_function("parallel_cell_writes_10_threads", |b| {
        b.iter(|| {
            let mut worksheet = Worksheet::new("Sheet1".to_string());
            let handles: Vec<_> = (0..10)
                .map(|i| {
                    let mut ws = worksheet.clone();
                    std::thread::spawn(move || {
                        for row in i * 100..(i + 1) * 100 {
                            for col in 0..10 {
                                ws.set_cell_value(row, col, CellValue::Number((row * 10 + col) as f64));
                            }
                        }
                        ws
                    })
                })
                .collect();
            
            for handle in handles {
                let _ = handle.join();
            }
            black_box(worksheet)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_cell_operations,
    benchmark_worksheet_operations,
    benchmark_workbook_operations,
    benchmark_large_worksheet,
    benchmark_cell_access,
    benchmark_formula_evaluation,
    benchmark_serialization,
    benchmark_concurrent_operations
);
criterion_main!(benches);