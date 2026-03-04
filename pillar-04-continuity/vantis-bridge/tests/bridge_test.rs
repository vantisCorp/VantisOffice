//! Integration tests for Vantis Bridge
//! Tests document parsing, conversion, sanitization, and export

use vantis_bridge::{
    ConversionConfig, ConversionResult, Document, DocumentType, ExportFormat, ExportResult,
    SanitizationConfig, SanitizationResult, Sanitizer, VantisExporter,
};

#[test]
fn test_document_creation() {
    let document = Document::new(
        "test_document".to_string(),
        DocumentType::Docx,
        b"Test content".to_vec(),
    );

    assert_eq!(document.name, "test_document");
    assert_eq!(document.document_type, DocumentType::Docx);
    assert!(!document.id.is_empty());
}

#[test]
fn test_conversion_config_default() {
    let config = ConversionConfig::default();

    assert!(config.remove_metadata);
    assert!(config.remove_macros);
    assert!(config.remove_scripts);
    assert!(config.remove_embedded_files);
}

#[test]
fn test_sanitization_config_default() {
    let config = SanitizationConfig::default();

    assert!(config.remove_metadata);
    assert!(config.remove_macros);
    assert!(config.remove_scripts);
    assert!(config.remove_embedded_files);
}

#[test]
fn test_sanitizer_creation() {
    let config = SanitizationConfig::default();
    let _sanitizer = Sanitizer::new(config);
}

#[test]
fn test_sanitization_result() {
    let result = SanitizationResult {
        metadata_removed: true,
        macros_removed: 2,
        scripts_removed: 1,
        embedded_files_removed: 0,
        total_size_before: 1000,
        total_size_after: 950,
    };

    assert!(result.metadata_removed);
    assert_eq!(result.macros_removed, 2);
    assert_eq!(result.scripts_removed, 1);
    assert_eq!(result.embedded_files_removed, 0);
}

#[test]
fn test_export_format_variants() {
    // Test that all export format variants exist
    let _json = ExportFormat::Json;
    let _vantis_writer = ExportFormat::VantisWriter;
    let _vantis_grid = ExportFormat::VantisGrid;
    let _vantis_canvas = ExportFormat::VantisCanvas;
}

#[test]
fn test_export_result() {
    let result = ExportResult {
        format: ExportFormat::Json,
        data: b"{&quot;test&quot;: &quot;data&quot;}".to_vec(),
        size: 17,
        success: true,
    };

    assert_eq!(result.format, ExportFormat::Json);
    assert_eq!(result.size, 17);
    assert!(result.success);
}

#[test]
fn test_document_type_variants() {
    // Test that all document type variants exist
    let _docx = DocumentType::Docx;
    let _xlsx = DocumentType::Xlsx;
    let _pptx = DocumentType::Pptx;
}

#[test]
fn test_conversion_result() {
    let result = ConversionResult {
        success: true,
        document: Some(Document::new(
            "test".to_string(),
            DocumentType::Docx,
            b"content".to_vec(),
        )),
        warnings: vec![],
        errors: vec![],
        sanitization_result: None,
    };

    assert!(result.success);
    assert!(result.document.is_some());
}

#[test]
fn test_full_conversion_workflow() {
    // Create document
    let mut document = Document::new(
        "test_document".to_string(),
        DocumentType::Docx,
        b"Test content".to_vec(),
    );

    // Create sanitization config
    let sanitization_config = SanitizationConfig::default();

    // Create sanitizer
    let sanitizer = Sanitizer::new(sanitization_config);

    // Sanitize document
    let result = sanitizer.sanitize(&mut document);

    // Verify document was created
    assert_eq!(document.name, "test_document");
    assert_eq!(document.document_type, DocumentType::Docx);

    // Verify sanitization result
    assert!(result.metadata_removed);
}
