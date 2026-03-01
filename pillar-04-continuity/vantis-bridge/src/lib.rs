//! Vantis Bridge - Legacy format converter with security controls
//!
//! Features:
//! - Convert .docx, .xlsx, .pptx to Vantis formats
//! - Security sanitization
//! - Metadata removal
//! - Macro/script removal
//! - Format validation

pub mod core;
pub mod parsers;
pub mod converters;
pub mod sanitization;
pub mod export;

pub use core::{Document, ConversionConfig, ConversionResult, DocumentType, SanitizationResult};
pub use parsers::{DocxParser, XlsxParser, PptxParser, Parser};
pub use converters::{DocxConverter, XlsxConverter, PptxConverter, Converter};
pub use sanitization::{Sanitizer, SanitizationConfig};
pub use export::{VantisExporter, ExportFormat, ExportResult};

/// Vantis Bridge version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Bridge
pub fn init() -> Result<(), BridgeError> {
    // Initialize subsystems
    core::init()?;
    parsers::init()?;
    converters::init()?;
    sanitization::init()?;
    
    Ok(())
}

/// Bridge-specific errors
#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("Parser error: {0}")]
    Parser(String),
    
    #[error("Converter error: {0}")]
    Converter(String),
    
    #[error("Sanitization error: {0}")]
    Sanitization(String),
    
    #[error("Export error: {0}")]
    Export(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("General error: {0}")]
    General(String),
}

impl From<String> for BridgeError {
    fn from(s: String) -> Self {
        BridgeError::General(s)
    }
}