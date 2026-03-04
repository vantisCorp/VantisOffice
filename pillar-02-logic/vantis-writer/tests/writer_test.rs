//! Comprehensive integration tests for Vantis Writer
//!
//! Tests cover:
//! - Core document model (Document, Paragraph, Editor)
//! - Markdown parsing and live preview
//! - Typography engine (Babel Engine)
//! - Document metadata and word counting
//! - Multiple paragraph operations
//! - Edge cases and integration workflows

use vantis_writer::{init};
use vantis_writer::core::{Document, Editor, Paragraph, ParagraphStyle};
use vantis_writer::markdown::{MarkdownParser, LivePreview};
use vantis_writer::typography::{BabelEngine, FontConfig, TypographySettings, KerningMode, Justification};

// ============================================================================
// Initialization Tests
// ============================================================================

#[test]
fn test_writer_initialization() {
    let result = init();
    assert!(result.is_ok(), "Writer initialization should succeed");
}

// ============================================================================
// Document Tests
// ============================================================================

#[test]
fn test_document_creation() {
    let document = Document::new("Test Document".to_string());
    assert_eq!(document.title, "Test Document");
    assert_eq!(document.paragraphs.len(), 0);
    assert!(!document.id.is_empty());
}

#[test]
fn test_document_with_title() {
    let document = Document::new("My First Document".to_string());
    assert_eq!(document.title, "My First Document");
}

#[test]
fn test_document_metadata_initialization() {
    let document = Document::new("Test".to_string());
    assert_eq!(document.metadata.author, "");
    assert_eq!(document.metadata.word_count, 0);
}

#[test]
fn test_document_id_uniqueness() {
    let doc1 = Document::new("Doc1".to_string());
    let doc2 = Document::new("Doc2".to_string());
    assert_ne!(doc1.id, doc2.id, "Document IDs should be unique");
}

// ============================================================================
// Paragraph Tests
// ============================================================================

#[test]
fn test_paragraph_creation() {
    let paragraph = Paragraph::new("Hello, World!".to_string());
    assert_eq!(paragraph.text, "Hello, World!");
}

#[test]
fn test_paragraph_with_style() {
    let style = ParagraphStyle {
        font_size: 16.0,
        font_family: "Arial".to_string(),
        bold: true,
        italic: false,
        color: "#FF0000".to_string(),
    };
    let paragraph = Paragraph::new("Styled text".to_string()).with_style(style);
    assert_eq!(paragraph.style.font_size, 16.0);
    assert!(paragraph.style.bold);
}

#[test]
fn test_paragraph_default_style() {
    let paragraph = Paragraph::new("Default style".to_string());
    assert_eq!(paragraph.style.font_size, 14.0);
    assert_eq!(paragraph.style.font_family, "Inter");
    assert!(!paragraph.style.bold);
    assert!(!paragraph.style.italic);
    assert_eq!(paragraph.style.color, "#000000");
}

#[test]
fn test_paragraph_style_mutations() {
    let mut paragraph = Paragraph::new("Test".to_string());
    paragraph.style.bold = true;
    paragraph.style.italic = true;
    paragraph.style.font_size = 18.0;
    assert!(paragraph.style.bold);
    assert!(paragraph.style.italic);
    assert_eq!(paragraph.style.font_size, 18.0);
}

// ============================================================================
// Document Operations Tests
// ============================================================================

#[test]
fn test_add_paragraph() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("First paragraph".to_string());

    let result = document.add_paragraph(paragraph);
    assert!(result.is_ok(), "Adding paragraph should succeed");
    assert_eq!(document.paragraphs.len(), 1);
}

#[test]
fn test_add_multiple_paragraphs() {
    let mut document = Document::new("Test".to_string());
    
    for i in 0..5 {
        let paragraph = Paragraph::new(format!("Paragraph {}", i));
        document.add_paragraph(paragraph).unwrap();
    }
    
    assert_eq!(document.paragraphs.len(), 5);
}

#[test]
fn test_word_count_single_paragraph() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("Hello World Test".to_string());
    document.add_paragraph(paragraph).unwrap();

    assert_eq!(document.metadata.word_count, 3);
}

#[test]
fn test_word_count_multiple_paragraphs() {
    let mut document = Document::new("Test".to_string());
    document.add_paragraph(Paragraph::new("First paragraph text".to_string())).unwrap();
    document.add_paragraph(Paragraph::new("Second paragraph here".to_string())).unwrap();
    document.add_paragraph(Paragraph::new("Third paragraph test".to_string())).unwrap();

    assert_eq!(document.metadata.word_count, 9);
}

#[test]
fn test_word_count_empty_document() {
    let document = Document::new("Test".to_string());
    assert_eq!(document.metadata.word_count, 0);
}

#[test]
fn test_word_count_with_extra_whitespace() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("  Extra   spaces   between   words  ".to_string());
    document.add_paragraph(paragraph).unwrap();

    assert_eq!(document.metadata.word_count, 4);
}

#[test]
fn test_word_count_special_characters() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("Hello, world! This is a test...".to_string());
    document.add_paragraph(paragraph).unwrap();

    assert_eq!(document.metadata.word_count, 6); // "Hello," "world!" "This" "is" "a" "test..."
}

// ============================================================================
// Editor Tests
// ============================================================================

#[test]
fn test_editor_creation() {
    let document = Document::new("Test".to_string());
    let editor = Editor::new(document);
    assert_eq!(editor.document().title, "Test");
}

#[test]
fn test_editor_with_paragraphs() {
    let mut document = Document::new("Test".to_string());
    document.add_paragraph(Paragraph::new("First".to_string())).unwrap();
    document.add_paragraph(Paragraph::new("Second".to_string())).unwrap();
    
    let editor = Editor::new(document);
    assert_eq!(editor.document().paragraphs.len(), 2);
}

#[test]
fn test_editor_insert_text() {
    let mut document = Document::new("Test".to_string());
    document.add_paragraph(Paragraph::new("Original".to_string())).unwrap();
    
    let mut editor = Editor::new(document);
    let result = editor.insert_text(" text");
    assert!(result.is_ok(), "Insert text should succeed");
}

#[test]
fn test_editor_delete_text() {
    let mut document = Document::new("Test".to_string());
    document.add_paragraph(Paragraph::new("Original text".to_string())).unwrap();
    
    let mut editor = Editor::new(document);
    let result = editor.delete_text(5);
    assert!(result.is_ok(), "Delete text should succeed");
}

// ============================================================================
// Markdown Parser Tests
// ============================================================================

#[test]
fn test_markdown_parser_creation() {
    let _parser = MarkdownParser::new();
    // Parser should be created successfully
    assert!(true);
}

#[test]
fn test_markdown_parser_default() {
    let _parser = MarkdownParser::default();
    // Default parser should work
    assert!(true);
}

#[test]
fn test_parse_simple_text() {
    let parser = MarkdownParser::new();
    let markdown = "Hello, World!";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Simple text parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("Hello, World!"));
}

#[test]
fn test_parse_bold_text() {
    let parser = MarkdownParser::new();
    let markdown = "**Bold text**";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Bold text parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("<strong>") || html.contains("<b>"));
}

#[test]
fn test_parse_italic_text() {
    let parser = MarkdownParser::new();
    let markdown = "*Italic text*";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Italic text parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("<em>") || html.contains("<i>"));
}

#[test]
fn test_parse_headers() {
    let parser = MarkdownParser::new();
    let markdown = "# Header 1\n\n## Header 2";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Header parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("<h1>") || html.contains("<h2>"));
}

#[test]
fn test_parse_links() {
    let parser = MarkdownParser::new();
    let markdown = "[Link text](https://example.com)";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Link parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("href"));
}

#[test]
fn test_parse_code() {
    let parser = MarkdownParser::new();
    let markdown = "`code snippet`";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Code parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("<code>") || html.contains("<pre>"));
}

#[test]
fn test_parse_lists() {
    let parser = MarkdownParser::new();
    let markdown = "- Item 1\n- Item 2\n- Item 3";
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "List parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("<li>") || html.contains("<ul>"));
}

// ============================================================================
// Live Preview Tests
// ============================================================================

#[test]
fn test_live_preview_creation() {
    let parser = MarkdownParser::new();
    let _preview = LivePreview::new(parser);
    // Preview should be created successfully
    assert!(true);
}

#[test]
fn test_live_preview_default() {
    let _preview = LivePreview::default();
    // Default preview should work
    assert!(true);
}

#[test]
fn test_live_preview_render() {
    let parser = MarkdownParser::new();
    let preview = LivePreview::new(parser);
    let markdown = "# Test\n\nContent here";
    let result = preview.render(markdown);
    assert!(result.is_ok(), "Live preview rendering should succeed");
}

// ============================================================================
// Typography Tests
// ============================================================================

#[test]
fn test_font_config_default() {
    let config = FontConfig::default();
    assert_eq!(config.primary_font, "Inter");
    assert_eq!(config.size, 14.0);
    assert_eq!(config.weight, 400);
    assert_eq!(config.line_height, 1.5);
}

#[test]
fn test_font_config_custom() {
    let config = FontConfig {
        primary_font: "Arial".to_string(),
        fallback_fonts: vec!["Segoe UI".to_string()],
        size: 16.0,
        weight: 700,
        line_height: 1.2,
    };
    assert_eq!(config.primary_font, "Arial");
    assert_eq!(config.size, 16.0);
    assert_eq!(config.weight, 700);
}

#[test]
fn test_typography_settings_default() {
    let settings = TypographySettings::default();
    // Check that kerning is set to Optical (cannot use == as enum doesn't implement PartialEq)
    match settings.kerning {
        KerningMode::Optical => {},
        _ => panic!("Expected Optical kerning mode"),
    }
    assert!(settings.ligatures);
    assert!(settings.hyphenation);
}

#[test]
fn test_typography_settings_custom() {
    let settings = TypographySettings {
        kerning: KerningMode::None,
        ligatures: false,
        hyphenation: false,
        justification: Justification::Center,
        paragraph_spacing: 1.5,
    };
    // Check kerning mode using pattern matching
    match settings.kerning {
        KerningMode::None => {},
        _ => panic!("Expected None kerning mode"),
    }
    assert!(!settings.ligatures);
    // Check justification using pattern matching
    match settings.justification {
        Justification::Center => {},
        _ => panic!("Expected Center justification"),
    }
}

#[test]
fn test_babel_engine_creation() {
    let config = FontConfig::default();
    let result = BabelEngine::new(config);
    assert!(result.is_ok(), "Babel engine creation should succeed");
}

#[test]
fn test_kerning_modes() {
    let modes = vec![
        KerningMode::None,
        KerningMode::Standard,
        KerningMode::Optical,
    ];
    for mode in modes {
        let _ = mode; // Test that all modes can be created
    }
}

#[test]
fn test_justification_modes() {
    let modes = vec![
        Justification::Left,
        Justification::Center,
        Justification::Right,
        Justification::Justified,
        Justification::Auto,
    ];
    for mode in modes {
        let _ = mode; // Test that all modes can be created
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_document_workflow() {
    // Initialize writer
    let init_result = init();
    assert!(init_result.is_ok(), "Writer initialization should succeed");

    // Create document
    let mut document = Document::new("Complete Workflow Test".to_string());
    
    // Add multiple paragraphs with different styles
    let title_style = ParagraphStyle {
        font_size: 24.0,
        font_family: "Arial".to_string(),
        bold: true,
        italic: false,
        color: "#000000".to_string(),
    };
    
    document.add_paragraph(
        Paragraph::new("Document Title".to_string()).with_style(title_style)
    ).unwrap();
    
    document.add_paragraph(
        Paragraph::new("This is the first paragraph of our document.".to_string())
    ).unwrap();
    
    document.add_paragraph(
        Paragraph::new("Second paragraph with more content.".to_string())
    ).unwrap();

    // Verify document structure
    assert_eq!(document.paragraphs.len(), 3);
    assert_eq!(document.title, "Complete Workflow Test");
    assert!(document.metadata.word_count > 0);

    // Create editor
    let editor = Editor::new(document);
    assert_eq!(editor.document().paragraphs.len(), 3);
}

#[test]
fn test_markdown_to_document_workflow() {
    let parser = MarkdownParser::new();
    let markdown = "# Title\n\nFirst paragraph.\n\nSecond paragraph.";
    
    let result = parser.parse_to_html(markdown);
    assert!(result.is_ok(), "Markdown parsing should succeed");
    
    let html = result.unwrap();
    assert!(html.contains("Title"));
    assert!(html.contains("First paragraph"));
    assert!(html.contains("Second paragraph"));
}

#[test]
fn test_typography_with_document() {
    let config = FontConfig::default();
    let _engine = BabelEngine::new(config).unwrap();
    
    let mut document = Document::new("Typography Test".to_string());
    document.add_paragraph(
        Paragraph::new("Text with typography settings".to_string())
    ).unwrap();
    
    // Document should exist with typography engine available
    assert_eq!(document.paragraphs.len(), 1);
}

#[test]
fn test_large_document_handling() {
    let mut document = Document::new("Large Document".to_string());
    
    // Add 100 paragraphs
    for i in 0..100 {
        let paragraph = Paragraph::new(format!("Paragraph {} with some content", i));
        document.add_paragraph(paragraph).unwrap();
    }
    
    assert_eq!(document.paragraphs.len(), 100);
    assert_eq!(document.metadata.word_count, 500); // 5 words per paragraph
}

#[test]
fn test_empty_paragraphs() {
    let mut document = Document::new("Empty Test".to_string());
    
    // Add empty paragraphs
    document.add_paragraph(Paragraph::new("".to_string())).unwrap();
    document.add_paragraph(Paragraph::new("   ".to_string())).unwrap();
    
    assert_eq!(document.paragraphs.len(), 2);
    assert_eq!(document.metadata.word_count, 0);
}