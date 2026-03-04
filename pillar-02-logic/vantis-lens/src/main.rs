//! Vantis Lens - Secure PDF viewer with automatic sterilization
//!
//! Main entry point for the Vantis Lens application

use std::path::PathBuf;
use vantis_lens::core::{Annotation, AnnotationType};
use vantis_lens::rendering::RenderOptions;
use vantis_lens::sterilization::SterilizationOptions;
use vantis_lens::{
    AnnotationManager, ExportFormat, PdfDocument, PdfRenderer, PdfSterilizer, SignatureManager,
};

fn main() {
    println!("Vantis Lens v{}", vantis_lens::VERSION);
    println!("Secure PDF viewer for VantisOffice\n");

    // Initialize subsystems
    if let Err(e) = vantis_lens::init() {
        eprintln!("Initialization error: {}", e);
        std::process::exit(1);
    }

    println!("✓ Vantis Lens initialized successfully\n");

    // Create a sample PDF document
    let mut document = PdfDocument::new("sample_document.pdf".to_string());
    println!("✓ Created sample PDF document\n");

    // Add some metadata
    document.metadata.title = Some("Sample Document".to_string());
    document.metadata.author = Some("Vantis Corporation".to_string());
    document.metadata.subject = Some("Demonstration of Vantis Lens capabilities".to_string());
    document.metadata.keywords = Some("PDF, security, sterilization".to_string());

    // Add sample pages
    for i in 0..3 {
        let mut page = vantis_lens::core::PdfPage {
            index: i,
            width: 595.0,  // A4 width in points
            height: 842.0, // A4 height in points
            rotation: 0,
            annotations: Vec::new(),
            text_content: Some(format!("This is page {} of the sample document.\n\nVantis Lens provides secure PDF viewing with automatic sterilization to remove metadata, JavaScript, and other security risks.", i + 1)),
        };

        document.add_page(page);
    }

    println!("✓ Added {} pages to document", document.page_count());
    println!("  Document is sterilized: {}", document.is_sterilized);
    println!();

    // Test Sterilization
    println!("Testing PDF Sterilization:");
    let sterilizer = PdfSterilizer::new();
    println!("  ✓ Sterilizer created");

    let options = SterilizationOptions::default();
    match sterilizer.sterilize(&mut document, &options) {
        Ok(report) => {
            println!("  ✓ Document sterilized successfully");
            println!("    Metadata removed: {}", report.metadata_removed);
            println!("    JavaScript removed: {}", report.javascript_removed);
            println!(
                "    External links removed: {}",
                report.external_links_removed
            );
            println!("    Total changes: {}", report.total_changes());
            println!("    Timestamp: {}", report.timestamp);
        }
        Err(e) => println!("  ✗ Sterilization error: {}", e),
    }
    println!();

    // Test Renderer
    println!("Testing PDF Renderer:");
    let renderer = PdfRenderer::new();
    println!("  ✓ Renderer created");
    println!("  ✓ Renderer enabled: {}", renderer.is_enabled());
    println!("  ✓ Sandbox enabled: {}", renderer.is_sandbox_enabled());

    let render_options = RenderOptions::default();
    println!("  ✓ Render options configured");
    println!();

    // Test Signature Manager
    println!("Testing Signature Manager:");
    let sig_manager = SignatureManager::new();
    println!("  ✓ Signature manager created");

    match sig_manager.create_signature(
        "sample_document.pdf".to_string(),
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
    ) {
        Ok(signature) => {
            println!("  ✓ Created signature: {}", signature.id);
            println!(
                "    Signer: {} <{}>",
                signature.signer_name, signature.signer_email
            );
            println!("    Created at: {}", signature.created_at);

            // Verify signature
            match sig_manager.verify_signature(&signature.id) {
                Ok(status) => println!("    Signature status: {:?}", status),
                Err(e) => println!("    Verification error: {}", e),
            }
        }
        Err(e) => println!("  ✗ Error creating signature: {}", e),
    }
    println!();

    // Test Annotation Manager
    println!("Testing Annotation Manager:");
    let ann_manager = AnnotationManager::new();
    println!("  ✓ Annotation manager created");

    // Create a highlight annotation
    match ann_manager.create_annotation(AnnotationType::Highlight, 0) {
        Ok(mut annotation) => {
            annotation = annotation
                .with_content("Important text".to_string())
                .with_position(100.0, 200.0, 300.0, 20.0)
                .with_color("yellow".to_string());

            ann_manager.update_annotation(annotation.clone()).unwrap();
            println!("  ✓ Created highlight annotation: {}", annotation.id);
            println!("    Type: {:?}", annotation.annotation_type);
            println!("    Content: {}", annotation.content);
        }
        Err(e) => println!("  ✗ Error creating annotation: {}", e),
    }

    // Create a text annotation
    match ann_manager.create_annotation(AnnotationType::Text, 1) {
        Ok(mut annotation) => {
            annotation = annotation
                .with_content("Note: This is important".to_string())
                .with_position(150.0, 300.0, 200.0, 50.0)
                .with_color("blue".to_string());

            ann_manager.update_annotation(annotation.clone()).unwrap();
            println!("  ✓ Created text annotation: {}", annotation.id);
            println!("    Type: {:?}", annotation.annotation_type);
            println!("    Content: {}", annotation.content);
        }
        Err(e) => println!("  ✗ Error creating annotation: {}", e),
    }

    // Get page annotations
    let page_annotations = ann_manager.get_page_annotations(0);
    println!("  ✓ Page 0 has {} annotations", page_annotations.len());
    println!();

    // Test Export
    println!("Testing Export:");
    let text_path = PathBuf::from("/tmp/sample_document.txt");
    let text_exporter = vantis_lens::export::PdfExporter::new(ExportFormat::Text);

    match text_exporter.export(&document, &text_path) {
        Ok(_) => println!("  ✓ Exported to text: {}", text_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }

    let html_path = PathBuf::from("/tmp/sample_document.html");
    let html_exporter = vantis_lens::export::PdfExporter::new(ExportFormat::Html);

    match html_exporter.export(&document, &html_path) {
        Ok(_) => println!("  ✓ Exported to HTML: {}", html_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }

    let json_path = PathBuf::from("/tmp/sample_document.json");
    let json_exporter = vantis_lens::export::PdfExporter::new(ExportFormat::Pdf);

    match json_exporter.export(&document, &json_path) {
        Ok(_) => println!("  ✓ Exported to JSON: {}", json_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }
    println!();

    println!("─────────────────────────────────");
    println!("Vantis Lens demo completed successfully!");
    println!("─────────────────────────────────");
    println!();
    println!("Document Statistics:");
    println!("  Total pages: {}", document.page_count());
    println!("  Is sterilized: {}", document.is_sterilized);
    println!("  Is encrypted: {}", document.is_encrypted);
    println!("  Version: {}", document.version);
    println!();
    println!("Security Features:");
    println!("  ✓ Automatic sterilization");
    println!("  ✓ Sandbox rendering");
    println!("  ✓ E-signature support (eIDAS compliant)");
    println!("  ✓ Annotation support");
    println!("  ✓ Multiple export formats");
}
