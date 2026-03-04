// Performance benchmarks for Vantis Lens
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use vantis_lens::{
    Annotation, AnnotationManager, AnnotationType, ExportFormat, PdfDocument, PdfExporter,
    PdfRenderer, PdfSterilizer, RenderOptions, SignatureManager, SterilizationOptions,
};

fn bench_pdf_document_creation(c: &mut Criterion) {
    c.bench_function("lens_pdf_document_creation", |b| {
        b.iter(|| PdfDocument::new("test.pdf"));
    });
}

fn bench_pdf_document_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("lens_pdf_document_parse");

    for size in [1024, 10240, 102400, 1024000].iter() {
        let dummy_data = vec![0u8; *size];
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| PdfDocument::parse(black_box(&dummy_data)));
        });
    }

    group.finish();
}

fn bench_pdf_rendering(c: &mut Criterion) {
    let doc = PdfDocument::new("test.pdf");
    let mut renderer = PdfRenderer::new();

    c.bench_function("lens_pdf_rendering_single_page", |b| {
        b.iter(|| renderer.render_page(black_box(&doc), 0, &RenderOptions::default()));
    });
}

fn bench_pdf_rendering_multiple_pages(c: &mut Criterion) {
    let mut group = c.benchmark_group("lens_pdf_rendering_multiple_pages");

    for pages in [5, 10, 20, 50].iter() {
        let doc = create_test_document(*pages);
        let mut renderer = PdfRenderer::new();

        group.bench_with_input(BenchmarkId::from_parameter(pages), pages, |b, _| {
            b.iter(|| renderer.render_document(black_box(&doc), &RenderOptions::default()));
        });
    }

    group.finish();
}

fn bench_pdf_sterilization(c: &mut Criterion) {
    let doc = create_test_document(10);
    let mut sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_metadata: true,
        remove_scripts: true,
        remove_embedded_files: true,
        remove_forms: false,
        remove_links: false,
    };

    c.bench_function("lens_pdf_sterilization", |b| {
        b.iter(|| sterilizer.sterilize(black_box(&doc), black_box(&options)));
    });
}

fn bench_pdf_sterilization_options(c: &mut Criterion) {
    let doc = create_test_document(10);
    let mut sterilizer = PdfSterilizer::new();

    let options_vec = vec![
        (
            "minimal",
            SterilizationOptions {
                remove_metadata: true,
                remove_scripts: false,
                remove_embedded_files: false,
                remove_forms: false,
                remove_links: false,
            },
        ),
        (
            "standard",
            SterilizationOptions {
                remove_metadata: true,
                remove_scripts: true,
                remove_embedded_files: true,
                remove_forms: false,
                remove_links: false,
            },
        ),
        (
            "aggressive",
            SterilizationOptions {
                remove_metadata: true,
                remove_scripts: true,
                remove_embedded_files: true,
                remove_forms: true,
                remove_links: true,
            },
        ),
    ];

    let mut group = c.benchmark_group("lens_pdf_sterilization_options");

    for (name, options) in options_vec.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), options, |b, opts| {
            b.iter(|| sterilizer.sterilize(black_box(&doc), black_box(opts)));
        });
    }

    group.finish();
}

fn bench_pdf_signing(c: &mut Criterion) {
    let doc = create_test_document(5);
    let mut signature_manager = SignatureManager::new();

    c.bench_function("lens_pdf_signing", |b| {
        b.iter(|| {
            signature_manager.sign_document(
                black_box(&doc),
                black_box("test_key"),
                black_box("Test Signature"),
            )
        });
    });
}

fn bench_pdf_signature_verification(c: &mut Criterion) {
    let doc = create_test_document(5);
    let mut signature_manager = SignatureManager::new();
    let signed_doc = signature_manager
        .sign_document(&doc, "test_key", "Test Signature")
        .unwrap();

    c.bench_function("lens_pdf_signature_verification", |b| {
        b.iter(|| signature_manager.verify_signature(black_box(&signed_doc)));
    });
}

fn bench_pdf_annotation_creation(c: &mut Criterion) {
    let doc = PdfDocument::new("test.pdf");

    c.bench_function("lens_pdf_annotation_creation", |b| {
        b.iter(|| Annotation::new(AnnotationType::Text, (100, 200), "Test annotation"));
    });
}

fn bench_pdf_annotation_addition(c: &mut Criterion) {
    let mut doc = create_test_document(5);
    let mut annotation_manager = AnnotationManager::new();

    c.bench_function("lens_pdf_annotation_addition", |b| {
        b.iter(|| {
            let annotation = Annotation::new(AnnotationType::Text, (100, 200), "Test annotation");
            annotation_manager.add_annotation(
                black_box(&mut doc),
                black_box(0),
                black_box(annotation),
            )
        });
    });
}

fn bench_pdf_annotation_batch_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("lens_pdf_annotation_batch_addition");

    for count in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            b.iter(|| {
                let mut doc = create_test_document(5);
                let mut annotation_manager = AnnotationManager::new();

                for i in 0..*count {
                    let annotation = Annotation::new(
                        AnnotationType::Text,
                        (100 + i, 200 + i),
                        &format!("Test annotation {}", i),
                    );
                    annotation_manager
                        .add_annotation(&mut doc, 0, annotation)
                        .unwrap();
                }
            });
        });
    }

    group.finish();
}

fn bench_pdf_export_pdf(c: &mut Criterion) {
    let doc = create_test_document(10);
    let mut exporter = PdfExporter::new();

    c.bench_function("lens_pdf_export_pdf", |b| {
        b.iter(|| exporter.export(black_box(&doc), black_box(ExportFormat::Pdf)));
    });
}

fn bench_pdf_export_image(c: &mut Criterion) {
    let doc = create_test_document(10);
    let mut exporter = PdfExporter::new();

    c.bench_function("lens_pdf_export_image", |b| {
        b.iter(|| exporter.export(black_box(&doc), black_box(ExportFormat::Png)));
    });
}

fn bench_pdf_export_text(c: &mut Criterion) {
    let doc = create_test_document(10);
    let mut exporter = PdfExporter::new();

    c.bench_function("lens_pdf_export_text", |b| {
        b.iter(|| exporter.export(black_box(&doc), black_box(ExportFormat::Text)));
    });
}

fn bench_pdf_search(c: &mut Criterion) {
    let doc = create_test_document(20);

    c.bench_function("lens_pdf_search", |b| {
        b.iter(|| doc.search(black_box("test")));
    });
}

fn bench_pdf_navigate(c: &mut Criterion) {
    let doc = create_test_document(50);

    c.bench_function("lens_pdf_navigate", |b| {
        b.iter(|| doc.navigate_to(black_box(25)));
    });
}

// Helper function to create test documents
fn create_test_document(num_pages: usize) -> PdfDocument {
    let mut doc = PdfDocument::new(&format!("test_{}.pdf", num_pages));
    for _ in 0..num_pages {
        doc.add_page();
    }
    doc
}

criterion_group!(
    benches,
    bench_pdf_document_creation,
    bench_pdf_document_parse,
    bench_pdf_rendering,
    bench_pdf_rendering_multiple_pages,
    bench_pdf_sterilization,
    bench_pdf_sterilization_options,
    bench_pdf_signing,
    bench_pdf_signature_verification,
    bench_pdf_annotation_creation,
    bench_pdf_annotation_addition,
    bench_pdf_annotation_batch_addition,
    bench_pdf_export_pdf,
    bench_pdf_export_image,
    bench_pdf_export_text,
    bench_pdf_search,
    bench_pdf_navigate
);

criterion_main!(benches);
