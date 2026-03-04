//! Vantis Lens - Secure PDF viewer with automatic sterilization
//!
//! Features:
//! - Automatic PDF sterilization (removes metadata, scripts, embedded files)
//! - E-signature support (eIDAS compliant)
//! - Secure rendering with sandbox
//! - Annotation support
//! - Search and navigation

pub mod annotation;
pub mod core;
pub mod export;
pub mod rendering;
pub mod signature;
pub mod sterilization;

pub use annotation::{Annotation, AnnotationManager, AnnotationType};
pub use core::{PdfDocument, PdfMetadata, PdfPage};
pub use export::{ExportFormat, PdfExporter};
pub use rendering::{PdfRenderer, RenderOptions, RenderTarget};
pub use signature::{DigitalSignature, SignatureManager, SignatureStatus};
pub use sterilization::{PdfSterilizer, SterilizationOptions, SterilizationReport};

/// Vantis Lens version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Lens
pub fn init() -> Result<(), LensError> {
    // Initialize subsystems
    core::init()?;
    rendering::init()?;
    sterilization::init()?;
    signature::init()?;

    Ok(())
}

/// Lens-specific errors
#[derive(Debug, thiserror::Error)]
pub enum LensError {
    #[error("PDF parsing error: {0}")]
    PdfParsing(String),

    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("Sterilization error: {0}")]
    Sterilization(String),

    #[error("Signature error: {0}")]
    Signature(String),

    #[error("Annotation error: {0}")]
    Annotation(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<String> for LensError {
    fn from(s: String) -> Self {
        LensError::General(s)
    }
}
