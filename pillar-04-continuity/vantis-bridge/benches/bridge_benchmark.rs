// Performance benchmarks for Vantis Bridge
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_bridge::{
    Document, ConversionConfig, ConversionResult, DocumentType,
    DocxParser, XlsxParser, PptxParser, Parser,
    DocxConverter, XlsxConverter, PptxConverter, Converter,
    Sanitizer, SanitizationConfig,
    VantisExporter, ExportFormat
};

fn bench_document_creation(c: &mut Criterion) {
    c.bench_function("bridge_document_creation", |b| {
        b.iter(|| {
            Document::new(DocumentType::Docx, "test.docx")
        });
    });
}

fn bench_docx_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bridge_docx_parsing");
    
    for size in [1024, 10240, 102400, 512000].iter() {
        let dummy_data = vec![0u8; *size];
        let mut parser = DocxParser::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                parser.parse(black_box(&dummy_data))
            });
        });
    }
    
    group.finish();
}

fn bench_xlsx_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bridge_xlsx_parsing");
    
    for size in [1024, 10240, 102400, 512000].iter() {
        let dummy_data = vec![0u8; *size];
        let mut parser = XlsxParser::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                parser.parse(black_box(&dummy_data))
            });
        });
    }
    
    group.finish();
}

fn bench_pptx_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bridge_pptx_parsing");
    
    for size in [1024, 10240, 102400, 512000].iter() {
        let dummy_data = vec![0u8; *size];
        let mut parser = PptxParser::new();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                parser.parse(black_box(&dummy_data))
            });
        });
    }
    
    group.finish();
}

fn bench_docx_conversion(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let config = ConversionConfig::default();
    let mut converter = DocxConverter::new();
    
    c.bench_function("bridge_docx_conversion", |b| {
        b.iter(|| {
            converter.convert(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_xlsx_conversion(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Xlsx, 10);
    let config = ConversionConfig::default();
    let mut converter = XlsxConverter::new();
    
    c.bench_function("bridge_xlsx_conversion", |b| {
        b.iter(|| {
            converter.convert(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_pptx_conversion(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Pptx, 10);
    let config = ConversionConfig::default();
    let mut converter = PptxConverter::new();
    
    c.bench_function("bridge_pptx_conversion", |b| {
        b.iter(|| {
            converter.convert(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_sanitization(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let config = SanitizationConfig {
        remove_metadata: true,
        remove_macros: true,
        remove_scripts: true,
        remove_embedded_objects: true,
        remove_external_references: true,
        remove_hidden_content: true,
    };
    let mut sanitizer = Sanitizer::new();
    
    c.bench_function("bridge_sanitization", |b| {
        b.iter(|| {
            sanitizer.sanitize(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_sanitization_levels(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let mut sanitizer = Sanitizer::new();
    
    let sanitization_levels = vec![
        ("minimal", SanitizationConfig {
            remove_metadata: true,
            remove_macros: false,
            remove_scripts: false,
            remove_embedded_objects: false,
            remove_external_references: false,
            remove_hidden_content: false,
        }),
        ("standard", SanitizationConfig {
            remove_metadata: true,
            remove_macros: true,
            remove_scripts: true,
            remove_embedded_objects: false,
            remove_external_references: false,
            remove_hidden_content: false,
        }),
        ("aggressive", SanitizationConfig {
            remove_metadata: true,
            remove_macros: true,
            remove_scripts: true,
            remove_embedded_objects: true,
            remove_external_references: true,
            remove_hidden_content: true,
        }),
    ];
    
    let mut group = c.benchmark_group("bridge_sanitization_levels");
    
    for (level, config) in sanitization_levels.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(level), config, |b, cfg| {
            b.iter(|| {
                sanitizer.sanitize(black_box(&doc), black_box(cfg))
            });
        });
    }
    
    group.finish();
}

fn bench_export_vantis_writer(c: &mut Criterion) {
    let result = ConversionResult::new(create_test_document(DocumentType::Docx, 10));
    let mut exporter = VantisExporter::new();
    
    c.bench_function("bridge_export_vantis_writer", |b| {
        b.iter(|| {
            exporter.export(black_box(&result), black_box(ExportFormat::VantisWriter))
        });
    });
}

fn bench_export_vantis_grid(c: &mut Criterion) {
    let result = ConversionResult::new(create_test_document(DocumentType::Xlsx, 10));
    let mut exporter = VantisExporter::new();
    
    c.bench_function("bridge_export_vantis_grid", |b| {
        b.iter(|| {
            exporter.export(black_box(&result), black_box(ExportFormat::VantisGrid))
        });
    });
}

fn bench_export_vantis_canvas(c: &mut Criterion) {
    let result = ConversionResult::new(create_test_document(DocumentType::Pptx, 10));
    let mut exporter = VantisExporter::new();
    
    c.bench_function("bridge_export_vantis_canvas", |b| {
        b.iter(|| {
            exporter.export(black_box(&result), black_box(ExportFormat::VantisCanvas))
        });
    });
}

fn bench_batch_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bridge_batch_processing");
    
    for count in [5, 10, 20, 50].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            b.iter(|| {
                let documents: Vec<_> = (0..*count)
                    .map(|i| create_test_document(DocumentType::Docx, 5))
                    .collect();
                
                let config = ConversionConfig::default();
                let mut converter = DocxConverter::new();
                let mut results = Vec::new();
                
                for doc in documents {
                    results.push(converter.convert(&doc, &config).unwrap());
                }
                
                results
            });
        });
    }
    
    group.finish();
}

fn bench_format_validation(c: &mut Criterion) {
    let dummy_data = vec![0u8; 10240];
    let mut parser = DocxParser::new();
    
    c.bench_function("bridge_format_validation", |b| {
        b.iter(|| {
            parser.validate(black_box(&dummy_data))
        });
    });
}

fn bench_metadata_removal(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let mut sanitizer = Sanitizer::new();
    
    c.bench_function("bridge_metadata_removal", |b| {
        b.iter(|| {
            let config = SanitizationConfig {
                remove_metadata: true,
                remove_macros: false,
                remove_scripts: false,
                remove_embedded_objects: false,
                remove_external_references: false,
                remove_hidden_content: false,
            };
            sanitizer.sanitize(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_macro_removal(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let mut sanitizer = Sanitizer::new();
    
    c.bench_function("bridge_macro_removal", |b| {
        b.iter(|| {
            let config = SanitizationConfig {
                remove_metadata: false,
                remove_macros: true,
                remove_scripts: false,
                remove_embedded_objects: false,
                remove_external_references: false,
                remove_hidden_content: false,
            };
            sanitizer.sanitize(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_script_removal(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let mut sanitizer = Sanitizer::new();
    
    c.bench_function("bridge_script_removal", |b| {
        b.iter(|| {
            let config = SanitizationConfig {
                remove_metadata: false,
                remove_macros: false,
                remove_scripts: true,
                remove_embedded_objects: false,
                remove_external_references: false,
                remove_hidden_content: false,
            };
            sanitizer.sanitize(black_box(&doc), black_box(&config))
        });
    });
}

fn bench_external_reference_removal(c: &mut Criterion) {
    let doc = create_test_document(DocumentType::Docx, 10);
    let mut sanitizer = Sanitizer::new();
    
    c.bench_function("bridge_external_reference_removal", |b| {
        b.iter(|| {
            let config = SanitizationConfig {
                remove_metadata: false,
                remove_macros: false,
                remove_scripts: false,
                remove_embedded_objects: false,
                remove_external_references: true,
                remove_hidden_content: false,
            };
            sanitizer.sanitize(black_box(&doc), black_box(&config))
        });
    });
}

// Helper function to create test documents
fn create_test_document(doc_type: DocumentType, num_elements: usize) -> Document {
    let mut doc = Document::new(doc_type, &format!("test_{}.docx", num_elements));
    for i in 0..num_elements {
        doc.add_element(format!("element_{}", i));
    }
    doc
}

criterion_group!(
    benches,
    bench_document_creation,
    bench_docx_parsing,
    bench_xlsx_parsing,
    bench_pptx_parsing,
    bench_docx_conversion,
    bench_xlsx_conversion,
    bench_pptx_conversion,
    bench_sanitization,
    bench_sanitization_levels,
    bench_export_vantis_writer,
    bench_export_vantis_grid,
    bench_export_vantis_canvas,
    bench_batch_processing,
    bench_format_validation,
    bench_metadata_removal,
    bench_macro_removal,
    bench_script_removal,
    bench_external_reference_removal
);

criterion_main!(benches);