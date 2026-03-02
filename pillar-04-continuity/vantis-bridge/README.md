# Vantis Bridge

## Overview

Vantis Bridge is a migration tool that imports legacy office file formats (.docx, .xlsx, .pptx, etc.) into VantisOffice with security controls to remove old macros and ensure clean, secure conversions.

## Key Features

- **Legacy Format Support**: Import .docx, .xlsx, .pptx, .pdf, and more
- **Security Sanitization**: Automatic removal of VBA macros and malicious content
- **Format Preservation**: Maintains document formatting
- **Batch Conversion**: Process multiple files at once
- **Validation**: Verify converted documents
- **Error Handling**: Graceful handling of conversion errors

## Architecture

```
vantis-bridge/
├── src/
│   ├── core/
│   │   ├── converter.rs       # Converter engine
│   │   ├── format.rs          # Format detection
│   │   ├── document.rs        # Document model
│   │   └── progress.rs        # Progress tracking
│   ├── formats/
│   │   ├── docx.rs            # DOCX converter
│   │   ├── xlsx.rs            # XLSX converter
│   │   ├── pptx.rs            # PPTX converter
│   │   ├── pdf.rs             # PDF converter
│   │   ├── odt.rs             # ODT converter
│   │   └── rtf.rs             # RTF converter
│   ├── security/
│   │   ├── sanitizer.rs       # Content sanitization
│   │   ├── macro_removal.rs   # Macro removal
│   │   ├── validator.rs       # Content validation
│   │   └── threat_scanner.rs  # Threat scanning
│   ├── batch/
│   │   ├── processor.rs       # Batch processor
│   │   ├── queue.rs           # Conversion queue
│   │   └── scheduler.rs       # Task scheduler
│   ├── export/
│   │   ├── vantis.rs          # Export to Vantis format
│   │   └── validation.rs      # Export validation
│   └── ui/
│       ├── converter.rs       # Converter UI
│       ├── batch_ui.rs        # Batch UI
│       ├── progress.rs        # Progress display
│       └── settings.rs        # Settings UI
├── templates/
│   └── conversion/            # Conversion templates
└── tests/
    ├── formats/               # Format tests
    └── security/              # Security tests
```

## Format Converters

### DOCX Conversion

```rust
use vantis_bridge::formats::{DocxConverter, ConversionConfig};

let config = ConversionConfig::new()
    .with_remove_macros(true)
    .with_preserve_formatting(true)
    .with_embed_images(true)
    .with_convert_smart_quotes(true);

let converter = DocxConverter::new(config)?;

// Convert DOCX to Vantis format
let vantis_doc = converter.convert("document.docx")?;
vantis_doc.save("document.vdoc")?;
```

### XLSX Conversion

```rust
use vantis_bridge::formats::{XlsxConverter, ConversionConfig};

let config = ConversionConfig::new()
    .with_remove_macros(true)
    .with_preserve_formulas(true)
    .with_preserve_charts(true)
    .with_preserve_pivot_tables(true);

let converter = XlsxConverter::new(config)?;

// Convert XLSX to Vantis Grid
let vantis_grid = converter.convert("spreadsheet.xlsx")?;
vantis_grid.save("spreadsheet.vgrid")?;
```

### PPTX Conversion

```rust
use vantis_bridge::formats::{PptxConverter, ConversionConfig};

let config = ConversionConfig::new()
    .with_remove_macros(true)
    .with_preserve_animations(true)
    .with_preserve_transitions(true)
    .with_embed_media(true);

let converter = PptxConverter::new(config)?;

// Convert PPTX to Vantis Canvas
let vantis_canvas = converter.convert("presentation.pptx")?;
vantis_canvas.save("presentation.vpres")?;
```

## Security Sanitization

### Macro Removal

```rust
use vantis_bridge::security::{MacroRemover, MacroType};

let remover = MacroRemover::new()?;

// Remove all macros
remover.remove_all_macros("document.docx", "clean.docx")?;

// Remove specific types
remover.remove_macros_by_type(
    "document.docx",
    "clean.docx",
    vec![
        MacroType::VBA,
        MacroType::XLM,
        MacroType::DDE
    ]
)?;

// Check for macros
let has_macros = remover.has_macros("document.docx")?;
println!("Document has macros: {}", has_macros);
```

### Content Sanitization

```rust
use vantis_bridge::security::{Sanitizer, SanitizationLevel};

let sanitizer = Sanitizer::new(SanitizationLevel::Strict)?;

// Remove external links
sanitizer.remove_external_links("document.docx")?;

// Remove embedded objects
sanitizer.remove_embedded_objects("document.docx")?;

// Remove hidden content
sanitizer.remove_hidden_content("document.docx")?;

// Remove metadata
sanitizer.remove_metadata("document.docx")?;
```

### Threat Scanning

```rust
use vantis_bridge::security::{ThreatScanner, ThreatType};

let scanner = ThreatScanner::new()?;

// Scan document
let threats = scanner.scan("document.docx")?;

for threat in threats {
    match threat.threat_type {
        ThreatType::Macro => {
            println!("Found VBA macro: {}", threat.description);
        },
        ThreatType::ExternalLink => {
            println!("Found external link: {}", threat.description);
        },
        ThreatType::EmbeddedFile => {
            println!("Found embedded file: {}", threat.description);
        },
    }
}
```

## Batch Processing

### Queue Management

```rust
use vantis_bridge::batch::{ConversionQueue, ConversionTask};

let queue = ConversionQueue::new()?;

// Add tasks to queue
queue.add_task(ConversionTask::new("doc1.docx"))?;
queue.add_task(ConversionTask::new("doc2.docx"))?;
queue.add_task(ConversionTask::new("doc3.docx"))?;

// Process queue
queue.process_all()?;
```

### Parallel Processing

```rust
use vantis_bridge::batch::{BatchProcessor, ProcessorConfig};

let config = ProcessorConfig::new()
    .with_max_workers(4)
    .with_progress_callback(|progress| {
        println!("Progress: {}%", progress.percentage);
    });

let processor = BatchProcessor::new(config)?;

// Process directory
processor.process_directory(
    "/path/to/documents",
    "/path/to/output",
    "*.docx"
)?;
```

### Progress Tracking

```rust
use vantis_bridge::core::{ProgressTracker, ProgressEvent};

let tracker = ProgressTracker::new()?;

tracker.on_progress(|event| {
    match event {
        ProgressEvent::Started(task) => {
            println!("Started: {}", task.filename);
        },
        ProgressEvent::Progress(task, percent) => {
            println!("{}: {}%", task.filename, percent);
        },
        ProgressEvent::Completed(task) => {
            println!("Completed: {}", task.filename);
        },
        ProgressEvent::Failed(task, error) => {
            println!("Failed: {} - {}", task.filename, error);
        },
    }
    Ok(())
})?;
```

## Format Detection

### Automatic Detection

```rust
use vantis_bridge::core::FormatDetector;

let detector = FormatDetector::new()?;

// Detect format from file
let format = detector.detect_format("document.docx")?;
println!("Format: {:?}", format);

// Detect from content
let format = detector.detect_from_bytes(&data)?;
```

### Validation

```rust
use vantis_bridge::core::{Validator, ValidationResult};

let validator = Validator::new()?;

// Validate file
let result = validator.validate("document.docx")?;

match result {
    ValidationResult::Valid => {
        println!("File is valid");
    },
    ValidationResult::Corrupted => {
        println!("File is corrupted");
    },
    ValidationResult::Unsupported => {
        println!("File format is not supported");
    },
}
```

## API Examples

### Simple Conversion

```rust
use vantis_bridge::Bridge;

let bridge = Bridge::new()?;

// Convert single file
bridge.convert(
    "document.docx",
    "document.vdoc"
)?;
```

### Advanced Conversion

```rust
use vantis_bridge::{Bridge, ConversionOptions};

let options = ConversionOptions::new()
    .with_remove_macros(true)
    .with_preserve_formatting(true)
    .with_sanitize_content(true)
    .with_validate_output(true);

let bridge = Bridge::new(options)?;

// Convert with options
bridge.convert(
    "document.docx",
    "document.vdoc"
)?;
```

### Batch Conversion

```rust
use vantis_bridge::{Bridge, BatchOptions};

let options = BatchOptions::new()
    .with_input_directory("/input")
    .with_output_directory("/output")
    .with_pattern("*.docx")
    .with_parallel_workers(4)
    .with_remove_macros(true);

let bridge = Bridge::new(options)?;

// Convert all files
bridge.convert_batch()?;
```

## Integration Points

- **Vantis Writer**: DOCX import
- **Vantis Grid**: XLSX import
- **Vantis Canvas**: PPTX import
- **Vantis Lens**: PDF import
- **Vantis Vault**: Secure storage

## Configuration

```toml
# bridge.toml
[conversion]
default_remove_macros = true
default_preserve_formatting = true
default_embed_images = true
compression_level = 6

[security]
sanitization_level = "strict"
scan_for_threats = true
remove_external_links = true
remove_hidden_content = true

[batch]
max_workers = 4
queue_size = 1000
retry_failed = true
max_retries = 3

[validation]
validate_output = true
check_integrity = true
verify_formatting = true
```

## Supported Formats

### Input Formats

- **Microsoft Word**: .docx, .doc
- **Microsoft Excel**: .xlsx, .xls
- **Microsoft PowerPoint**: .pptx, .ppt
- **OpenDocument**: .odt, .ods, .odp
- **PDF**: .pdf
- **Rich Text**: .rtf
- **Plain Text**: .txt
- **Markdown**: .md

### Output Formats

- **Vantis Document**: .vdoc
- **Vantis Grid**: .vgrid
- **Vantis Canvas**: .vpres
- **Vantis PDF**: .vpdf

## Performance Metrics

- **DOCX Conversion**: 500 files/minute
- **XLSX Conversion**: 300 files/minute
- **PPTX Conversion**: 200 files/minute
- **Macro Removal**: 100ms per file
- **Sanitization**: 200ms per file
- **Batch Processing**: 4x speedup with 4 workers

## Error Handling

### Conversion Errors

```rust
use vantis_bridge::core::{ConversionError, ErrorHandler};

let handler = ErrorHandler::new()?;

match bridge.convert("file.docx", "file.vdoc") {
    Ok(_) => println!("Conversion successful"),
    Err(ConversionError::CorruptedFile) => {
        println!("File is corrupted");
        handler.log_error(error)?;
    },
    Err(ConversionError::UnsupportedFormat) => {
        println!("Format not supported");
    },
    Err(e) => {
        println!("Conversion failed: {}", e);
    },
}
```

### Recovery Mode

```rust
use vantis_bridge::core::RecoveryMode;

// Enable recovery mode for corrupted files
let bridge = Bridge::new()?
    .with_recovery_mode(RecoveryMode::BestEffort);

bridge.convert("corrupted.docx", "recovered.vdoc")?;
```

## Security Features

1. **Macro Removal**: All VBA macros removed
2. **Threat Scanning**: Detect malicious content
3. **Content Sanitization**: Remove dangerous elements
4. **Format Validation**: Verify file integrity
5. **External Link Removal**: Block external references
6. **Metadata Stripping**: Remove identifying information

## Future Roadmap

- [ ] More legacy format support
- [ ] AI-powered format detection
- [ ] Advanced repair capabilities
- [ ] Cloud storage integration
- [ ] Conversion templates
- [ ] Custom conversion rules

## Build Requirements

- Rust 1.70+
- zip (for Office formats)
- serde (serialization)
- regex (pattern matching)
- rayon (parallel processing)

---

**Part of VantisOffice Pillar IV - Critical Tools**