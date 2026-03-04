//! Unit tests for Vantis Lens

use vantis_lens::init;
use vantis_lens::core::{PdfDocument, PdfPage, Annotation, AnnotationType};
use vantis_lens::sterilization::{PdfSterilizer, SterilizationOptions, SterilizationReport};

#[test]
fn test_initialization() {
    let result = init();
    assert!(result.is_ok(), "Initialization should succeed");
}

// Core module tests
#[test]
fn test_pdf_document_creation() {
    let doc = PdfDocument::new("test.pdf".to_string());
    assert_eq!(doc.path, "test.pdf");
    assert_eq!(doc.page_count(), 0);
    assert!(!doc.is_encrypted);
    assert!(!doc.is_sterilized);
}

#[test]
fn test_pdf_document_add_page() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    let page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: Vec::new(),
        text_content: Some("Test content".to_string()),
    };
    doc.add_page(page);
    assert_eq!(doc.page_count(), 1);
    assert!(doc.is_valid());
}

#[test]
fn test_pdf_document_get_page() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    let page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: Vec::new(),
        text_content: Some("Test content".to_string()),
    };
    doc.add_page(page.clone());
    
    let retrieved = doc.get_page(0);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().index, 0);
    
    let invalid = doc.get_page(99);
    assert!(invalid.is_none());
}

#[test]
fn test_annotation_creation() {
    let annotation = Annotation::new("ann1".to_string(), AnnotationType::Text, 0);
    assert_eq!(annotation.id, "ann1");
    assert_eq!(annotation.annotation_type, AnnotationType::Text);
    assert_eq!(annotation.page_index, 0);
    assert!(annotation.content.is_empty());
    assert!(annotation.is_visible);
}

#[test]
fn test_annotation_with_content() {
    let annotation = Annotation::new("ann1".to_string(), AnnotationType::Highlight, 0)
        .with_content("Important text".to_string())
        .with_position(10.0, 20.0, 100.0, 50.0)
        .with_color("#FFFF00".to_string());
    
    assert_eq!(annotation.content, "Important text");
    assert_eq!(annotation.rect.x, 10.0);
    assert_eq!(annotation.rect.y, 20.0);
    assert_eq!(annotation.color, Some("#FFFF00".to_string()));
}

#[test]
fn test_annotation_types() {
    let types = vec![
        AnnotationType::Text,
        AnnotationType::Highlight,
        AnnotationType::Underline,
        AnnotationType::Strikeout,
        AnnotationType::Squiggly,
        AnnotationType::Comment,
        AnnotationType::Stamp,
        AnnotationType::Signature,
        AnnotationType::Freehand,
        AnnotationType::Rectangle,
        AnnotationType::Circle,
        AnnotationType::Line,
        AnnotationType::Arrow,
    ];
    
    // Test that all annotation types can be created
    for (i, ann_type) in types.iter().enumerate() {
        let annotation = Annotation::new(format!("ann{}", i), ann_type.clone(), 0);
        assert_eq!(annotation.annotation_type, *ann_type);
    }
}

// Sterilization module tests
#[test]
fn test_sterilizer_creation() {
    let sterilizer = PdfSterilizer::new();
    assert!(sterilizer.is_enabled());
}

#[test]
fn test_sterilizer_enable_disable() {
    let mut sterilizer = PdfSterilizer::new();
    assert!(sterilizer.is_enabled());
    
    sterilizer.disable();
    assert!(!sterilizer.is_enabled());
    
    sterilizer.enable();
    assert!(sterilizer.is_enabled());
}

#[test]
fn test_sterilization_options_default() {
    let options = SterilizationOptions::default();
    assert!(options.remove_metadata);
    assert!(options.remove_javascript);
    assert!(options.remove_embedded_files);
    assert!(options.remove_external_links);
    assert!(!options.remove_forms);
    assert!(!options.remove_annotations);
    assert!(options.flatten_layers);
}

#[test]
fn test_sterilization_report_creation() {
    let report = SterilizationReport::new();
    assert!(!report.success);
    assert_eq!(report.total_changes(), 0);
    assert!(report.warnings.is_empty());
}

#[test]
fn test_sterilization_report_add_warning() {
    let mut report = SterilizationReport::new();
    report.add_warning("Test warning".to_string());
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0], "Test warning");
}

#[test]
fn test_sterilization_report_total_changes() {
    let mut report = SterilizationReport::new();
    report.metadata_removed = 5;
    report.javascript_removed = 2;
    report.embedded_files_removed = 1;
    report.external_links_removed = 3;
    report.forms_removed = 0;
    report.annotations_removed = 0;
    report.layers_flattened = true;
    
    assert_eq!(report.total_changes(), 12); // 5+2+1+3+0+0+1
}

#[test]
fn test_sterilization_remove_metadata() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    doc.metadata.title = Some("Test Title".to_string());
    doc.metadata.author = Some("Test Author".to_string());
    doc.metadata.custom_properties.insert("key1".to_string(), "value1".to_string());
    
    let sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_metadata: true,
        ..Default::default()
    };
    
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_ok());
    
    let report = result.unwrap();
    assert_eq!(report.metadata_removed, 3); // title, author, custom_properties
    assert!(doc.is_sterilized);
}

#[test]
fn test_sterilization_remove_javascript() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    let page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: Vec::new(),
        text_content: Some("Check javascript:app.alert('test') and app.launchURL('http://example.com')".to_string()),
    };
    doc.add_page(page);
    
    let sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_javascript: true,
        ..Default::default()
    };
    
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_ok());
    
    let report = result.unwrap();
    assert_eq!(report.javascript_removed, 1);
    
    // Verify JavaScript was removed
    let retrieved = doc.get_page(0).unwrap();
    let text = retrieved.text_content.as_ref().unwrap();
    assert!(!text.contains("javascript:"));
    assert!(!text.contains("app.alert"));
    assert!(!text.contains("app.launchURL"));
}

#[test]
fn test_sterilization_remove_external_links() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    let page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: Vec::new(),
        text_content: Some("Visit http://example.com or https://secure.example.com".to_string()),
    };
    doc.add_page(page);
    
    let sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_external_links: true,
        ..Default::default()
    };
    
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_ok());
    
    let report = result.unwrap();
    assert_eq!(report.external_links_removed, 1);
    
    // Verify links were removed
    let retrieved = doc.get_page(0).unwrap();
    let text = retrieved.text_content.as_ref().unwrap();
    assert!(!text.contains("http://"));
    assert!(!text.contains("https://"));
}

#[test]
fn test_sterilization_remove_annotations() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    let mut page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: Vec::new(),
        text_content: Some("Test content".to_string()),
    };
    
    // Add annotations
    page.annotations.push(Annotation::new("ann1".to_string(), AnnotationType::Comment, 0));
    page.annotations.push(Annotation::new("ann2".to_string(), AnnotationType::Highlight, 0));
    doc.add_page(page);
    
    let sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_annotations: true,
        ..Default::default()
    };
    
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_ok());
    
    let report = result.unwrap();
    assert_eq!(report.annotations_removed, 2);
    
    // Verify annotations were removed
    let retrieved = doc.get_page(0).unwrap();
    assert_eq!(retrieved.annotations.len(), 0);
}

#[test]
fn test_sterilization_disabled() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    doc.metadata.title = Some("Test Title".to_string());
    
    let mut sterilizer = PdfSterilizer::new();
    sterilizer.disable();
    
    let options = SterilizationOptions::default();
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_err());
}

#[test]
fn test_pdf_document_security() {
    let doc = PdfDocument::new("test.pdf".to_string());
    assert!(!doc.security.has_password);
    assert!(!doc.security.has_permissions);
    assert!(doc.security.can_print);
    assert!(doc.security.can_copy);
    assert!(doc.security.can_modify);
    assert!(doc.security.can_extract);
}

#[test]
fn test_annotation_rect() {
    let annotation = Annotation::new("ann1".to_string(), AnnotationType::Rectangle, 0)
        .with_position(50.0, 100.0, 200.0, 150.0);
    
    assert_eq!(annotation.rect.x, 50.0);
    assert_eq!(annotation.rect.y, 100.0);
    assert_eq!(annotation.rect.width, 200.0);
    assert_eq!(annotation.rect.height, 150.0);
}

#[test]
fn test_complete_sterilization_workflow() {
    let mut doc = PdfDocument::new("test.pdf".to_string());
    
    // Add metadata
    doc.metadata.title = Some("Confidential Document".to_string());
    doc.metadata.author = Some("John Doe".to_string());
    
    // Add page with content
    let page = PdfPage {
        index: 0,
        width: 595.0,
        height: 842.0,
        rotation: 0,
        annotations: vec![
            Annotation::new("ann1".to_string(), AnnotationType::Comment, 0),
        ],
        text_content: Some("Click javascript:app.alert('hello') for info".to_string()),
    };
    doc.add_page(page);
    
    // Sterilize with all options
    let sterilizer = PdfSterilizer::new();
    let options = SterilizationOptions {
        remove_metadata: true,
        remove_javascript: true,
        remove_external_links: true,
        remove_annotations: true,
        ..Default::default()
    };
    
    let result = sterilizer.sterilize(&mut doc, &options);
    assert!(result.is_ok());
    
    let report = result.unwrap();
    assert!(report.success);
    assert_eq!(report.metadata_removed, 2); // title, author
    assert_eq!(report.javascript_removed, 1);
    assert_eq!(report.annotations_removed, 1);
    assert!(doc.is_sterilized);
}