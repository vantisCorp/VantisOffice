//! Demo application for Vantis Bridge

use vantis_bridge::*;

fn main() -> Result<(), BridgeError> {
    println!("=== Vantis Bridge Demo ===\n");

    // Initialize Bridge
    init()?;
    println!("✓ Vantis Bridge initialized\n");

    // Test DOCX parsing
    let docx_data = b"PK\x03\x04...DOCX file content...".to_vec();
    let docx_parser = DocxParser;

    if docx_parser.can_parse("document.docx") {
        println!("✓ Can parse DOCX files\n");
        let document = docx_parser.parse(&docx_data)?;
        println!("✓ Parsed DOCX document: {}\n", document.name);
        println!("  Type: {:?}\n", document.document_type);
        println!("  Size: {} bytes\n", document.content.len());
    }

    // Test XLSX parsing
    let xlsx_data = b"PK\x03\x04...XLSX file content...".to_vec();
    let xlsx_parser = XlsxParser;

    if xlsx_parser.can_parse("spreadsheet.xlsx") {
        println!("✓ Can parse XLSX files\n");
        let document = xlsx_parser.parse(&xlsx_data)?;
        println!("✓ Parsed XLSX document: {}\n", document.name);
        println!("  Type: {:?}\n", document.document_type);
        println!("  Size: {} bytes\n", document.content.len());
    }

    // Test PPTX parsing
    let pptx_data = b"PK\x03\x04...PPTX file content...".to_vec();
    let pptx_parser = PptxParser;

    if pptx_parser.can_parse("presentation.pptx") {
        println!("✓ Can parse PPTX files\n");
        let document = pptx_parser.parse(&pptx_data)?;
        println!("✓ Parsed PPTX document: {}\n", document.name);
        println!("  Type: {:?}\n", document.document_type);
        println!("  Size: {} bytes\n", document.content.len());
    }

    // Test conversion
    let mut docx_document = Document::new(
        "test.docx".to_string(),
        DocumentType::Docx,
        b"DOCX content".to_vec(),
    );

    // Add some metadata
    docx_document.metadata.author = Some("John Doe".to_string());
    docx_document.metadata.title = Some("Test Document".to_string());
    docx_document.metadata.has_macros = true;
    docx_document.metadata.has_scripts = true;

    println!("=== Testing Conversion ===\n");

    let docx_converter = DocxConverter;
    let conversion_result = docx_converter.convert(docx_document.clone())?;

    if conversion_result.success {
        println!("✓ Converted DOCX to Vantis Writer\n");
        if let Some(converted_doc) = conversion_result.document {
            println!("  Converted document: {}\n", converted_doc.name);
            println!("  Type: {:?}\n", converted_doc.document_type);
        }
    }

    // Test sanitization
    println!("\n=== Testing Sanitization ===\n");

    let sanitization_config = SanitizationConfig::default();
    let sanitizer = Sanitizer::new(sanitization_config);

    let sanitization_result = sanitizer.sanitize(&mut docx_document);

    println!("✓ Sanitized document\n");
    println!(
        "  Metadata removed: {}\n",
        sanitization_result.metadata_removed
    );
    println!("  Macros removed: {}\n", sanitization_result.macros_removed);
    println!(
        "  Scripts removed: {}\n",
        sanitization_result.scripts_removed
    );
    println!(
        "  Embedded files removed: {}\n",
        sanitization_result.embedded_files_removed
    );
    println!(
        "  Size before: {} bytes\n",
        sanitization_result.total_size_before
    );
    println!(
        "  Size after: {} bytes\n",
        sanitization_result.total_size_after
    );

    // Test export
    println!("\n=== Testing Export ===\n");

    let exporter = VantisExporter;
    let export_result = exporter.export(&docx_document, ExportFormat::VantisWriter)?;

    println!("✓ Exported document\n");
    println!("  Format: {:?}\n", export_result.format);
    println!("  Size: {} bytes\n", export_result.size);

    let json_export_result = exporter.export_to_json(&docx_document)?;
    println!("✓ Exported to JSON\n");
    println!("  Size: {} bytes\n", json_export_result.size);

    // Test conversion config
    println!("\n=== Testing Conversion Config ===\n");

    let conversion_config = ConversionConfig {
        remove_metadata: true,
        remove_macros: true,
        remove_scripts: true,
        remove_embedded_files: true,
        preserve_formatting: true,
        preserve_images: true,
    };

    println!("Conversion Config:\n");
    println!("  Remove metadata: {}\n", conversion_config.remove_metadata);
    println!("  Remove macros: {}\n", conversion_config.remove_macros);
    println!("  Remove scripts: {}\n", conversion_config.remove_scripts);
    println!(
        "  Remove embedded files: {}\n",
        conversion_config.remove_embedded_files
    );
    println!(
        "  Preserve formatting: {}\n",
        conversion_config.preserve_formatting
    );
    println!("  Preserve images: {}\n", conversion_config.preserve_images);

    println!("\n=== Demo Complete ===");
    Ok(())
}
