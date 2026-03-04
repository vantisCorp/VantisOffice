//! Integration tests for Vantis Writer

use vantis_writer::{init};
use vantis_writer::core::{Document, Editor, Paragraph};

#[test]
fn test_writer_initialization() {
    let result = init();
    assert!(result.is_ok(), "Writer initialization should succeed");
}

#[test]
fn test_document_creation() {
    let document = Document::new("Test Document".to_string());
    assert_eq!(document.title, "Test Document");
    assert_eq!(document.paragraphs.len(), 0);
}

#[test]
fn test_paragraph_creation() {
    let paragraph = Paragraph::new("Hello, World!".to_string());
    assert_eq!(paragraph.text, "Hello, World!");
}

#[test]
fn test_add_paragraph() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("First paragraph".to_string());

    let result = document.add_paragraph(paragraph);
    assert!(result.is_ok(), "Adding paragraph should succeed");
    assert_eq!(document.paragraphs.len(), 1);
}

#[test]
fn test_editor_creation() {
    let document = Document::new("Test".to_string());
    let editor = Editor::new(document);
    assert_eq!(editor.document().title, "Test");
}

#[test]
fn test_word_count() {
    let mut document = Document::new("Test".to_string());
    let paragraph = Paragraph::new("Hello World Test".to_string());
    document.add_paragraph(paragraph).unwrap();

    assert_eq!(document.metadata.word_count, 3);
}
