// Performance benchmarks for Vantis Writer
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use vantis_writer::core::{Document, Editor, Paragraph, ParagraphStyle};
use vantis_writer::markdown::{LivePreview, MarkdownParser};
use vantis_writer::typography::{
    BabelEngine, FontConfig, Justification, KerningMode, TypographySettings,
};

fn benchmark_document_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_creation");
    for size in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut doc = Document::new(format!("Test Document {}", size));
                for i in 0..size {
                    let paragraph = Paragraph::new(format!("Paragraph {}", i));
                    doc.add_paragraph(paragraph).unwrap();
                }
                black_box(doc)
            })
        });
    }
    group.finish();
}

fn benchmark_add_paragraph(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_paragraph");
    for initial_size in [0, 100, 1000] {
        let initial_size_for_closure = initial_size;
        group.bench_with_input(
            BenchmarkId::from_parameter(initial_size),
            &initial_size,
            move |b, &_initial_size| {
                b.iter(|| {
                    let mut doc = Document::new("Test Document".to_string());
                    for i in 0..initial_size_for_closure {
                        let paragraph = Paragraph::new(format!("Initial paragraph {}", i));
                        doc.add_paragraph(paragraph).unwrap();
                    }
                    let new_paragraph = Paragraph::new("New paragraph".to_string());
                    doc.add_paragraph(new_paragraph).unwrap();
                    black_box(doc)
                })
            },
        );
    }
    group.finish();
}

fn benchmark_markdown_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("markdown_parsing");

    let small_text = "# Heading\n\nThis is a paragraph.";
    let medium_text = "# Heading\n\nThis is a paragraph.\n\n## Subheading\n\nAnother paragraph with **bold** and *italic* text.";
    let large_text = std::iter::repeat("# Heading\n\nParagraph with [link](https://example.com) and `code`.\n\n## Subheading\n\n- List item 1\n- List item 2\n- List item 3\n\n| Column 1 | Column 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |\n").take(10).collect::<String>();

    group.bench_function("small", |b| {
        b.iter(|| {
            let parser = MarkdownParser::new();
            black_box(parser.parse_to_html(small_text).unwrap())
        })
    });

    group.bench_function("medium", |b| {
        b.iter(|| {
            let parser = MarkdownParser::new();
            black_box(parser.parse_to_html(medium_text).unwrap())
        })
    });

    group.bench_function("large", |b| {
        b.iter(|| {
            let parser = MarkdownParser::new();
            black_box(parser.parse_to_html(&large_text).unwrap())
        })
    });

    group.finish();
}

fn benchmark_live_preview(c: &mut Criterion) {
    let mut group = c.benchmark_group("live_preview");

    let markdown_text = "# Title\n\n**Bold** and *italic* text.\n\n## Subtitle\n\n- Item 1\n- Item 2\n\n> Blockquote\n\n`inline code`";

    group.bench_function("render", |b| {
        let preview = LivePreview::new(MarkdownParser::new());
        b.iter(|| black_box(preview.render(markdown_text).unwrap()))
    });

    group.finish();
}

fn benchmark_typography_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("typography");

    let short_text = "Hello, World!";
    let medium_text =
        "The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs.";
    let long_text = std::iter::repeat("The quick brown fox jumps over the lazy dog. ")
        .take(10)
        .collect::<String>();

    let config = FontConfig::default();
    let engine = BabelEngine::new(config).unwrap();

    group.bench_function("short_text", |b| {
        b.iter(|| black_box(engine.render_text(short_text, &(800.0, 600.0)).unwrap()))
    });

    group.bench_function("medium_text", |b| {
        b.iter(|| black_box(engine.render_text(medium_text, &(800.0, 600.0)).unwrap()))
    });

    group.bench_function("long_text", |b| {
        b.iter(|| black_box(engine.render_text(&long_text, &(800.0, 600.0)).unwrap()))
    });

    group.finish();
}

fn benchmark_babel_engine_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("babel_engine");

    group.bench_function("create_engine", |b| {
        b.iter(|| {
            let config = FontConfig::default();
            black_box(BabelEngine::new(config).unwrap())
        })
    });

    group.finish();
}

fn benchmark_paragraph_style(c: &mut Criterion) {
    let mut group = c.benchmark_group("paragraph_style");

    group.bench_function("default_style", |b| {
        b.iter(|| black_box(ParagraphStyle::default()))
    });

    group.bench_function("custom_style", |b| {
        b.iter(|| {
            let style = ParagraphStyle {
                font_size: 16.0,
                font_family: "Georgia".to_string(),
                bold: true,
                italic: false,
                color: "#333333".to_string(),
            };
            black_box(style)
        })
    });

    group.bench_function("with_style", |b| {
        b.iter(|| {
            let paragraph = Paragraph::new("Test paragraph".to_string());
            let style = ParagraphStyle {
                font_size: 18.0,
                font_family: "Arial".to_string(),
                bold: false,
                italic: true,
                color: "#FF0000".to_string(),
            };
            black_box(paragraph.with_style(style))
        })
    });

    group.finish();
}

fn benchmark_editor_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("editor_operations");

    group.bench_function("create_editor", |b| {
        let doc = Document::new("Test Document".to_string());
        b.iter(|| black_box(Editor::new(doc.clone())))
    });

    group.bench_function("insert_text", |b| {
        let doc = Document::new("Test Document".to_string());
        let mut editor = Editor::new(doc);
        b.iter(|| black_box(editor.insert_text("Hello, World!").unwrap()))
    });

    group.bench_function("delete_text", |b| {
        let doc = Document::new("Test Document".to_string());
        let mut editor = Editor::new(doc);
        b.iter(|| black_box(editor.delete_text(10).unwrap()))
    });

    group.finish();
}

fn benchmark_document_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_metadata");

    group.bench_function("update_metadata", |b| {
        let paragraph = Paragraph::new("Test paragraph with some words".to_string());
        b.iter(|| {
            let mut doc = Document::new("Test Document".to_string());
            doc.add_paragraph(paragraph.clone()).unwrap();
            black_box(doc.metadata.word_count)
        })
    });

    group.finish();
}

fn benchmark_typography_settings(c: &mut Criterion) {
    let mut group = c.benchmark_group("typography_settings");

    group.bench_function("default_settings", |b| {
        b.iter(|| black_box(TypographySettings::default()))
    });

    group.bench_function("custom_settings", |b| {
        b.iter(|| {
            let settings = TypographySettings {
                kerning: KerningMode::Optical,
                ligatures: true,
                hyphenation: true,
                justification: Justification::Justified,
                paragraph_spacing: 1.5,
            };
            black_box(settings)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_document_creation,
    benchmark_add_paragraph,
    benchmark_markdown_parsing,
    benchmark_live_preview,
    benchmark_typography_render,
    benchmark_babel_engine_creation,
    benchmark_paragraph_style,
    benchmark_editor_operations,
    benchmark_document_metadata,
    benchmark_typography_settings
);
criterion_main!(benches);
