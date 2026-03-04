//! Core data structures for Vantis Bridge

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub document_type: DocumentType,
    pub content: Vec<u8>,
    pub metadata: DocumentMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
    pub fn new(name: String, document_type: DocumentType, content: Vec<u8>) -> Self {
        let now = Utc::now();
        Document {
            id: Uuid::new_v4().to_string(),
            name,
            document_type,
            content,
            metadata: DocumentMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Document type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    Docx,
    Xlsx,
    Pptx,
    VantisWriter,
    VantisGrid,
    VantisCanvas,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub author: Option<String>,
    pub title: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub page_count: Option<usize>,
    pub word_count: Option<usize>,
    pub has_macros: bool,
    pub has_scripts: bool,
    pub has_embedded_files: bool,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        DocumentMetadata {
            author: None,
            title: None,
            subject: None,
            keywords: None,
            created: None,
            modified: None,
            page_count: None,
            word_count: None,
            has_macros: false,
            has_scripts: false,
            has_embedded_files: false,
        }
    }
}

/// Conversion config
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    pub remove_metadata: bool,
    pub remove_macros: bool,
    pub remove_scripts: bool,
    pub remove_embedded_files: bool,
    pub preserve_formatting: bool,
    pub preserve_images: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        ConversionConfig {
            remove_metadata: true,
            remove_macros: true,
            remove_scripts: true,
            remove_embedded_files: true,
            preserve_formatting: true,
            preserve_images: true,
        }
    }
}

/// Conversion result
#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub success: bool,
    pub document: Option<Document>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub sanitization_result: Option<SanitizationResult>,
}

impl ConversionResult {
    pub fn success(document: Document) -> Self {
        ConversionResult {
            success: true,
            document: Some(document),
            warnings: Vec::new(),
            errors: Vec::new(),
            sanitization_result: None,
        }
    }

    pub fn failure(errors: Vec<String>) -> Self {
        ConversionResult {
            success: false,
            document: None,
            warnings: Vec::new(),
            errors,
            sanitization_result: None,
        }
    }
}

/// Sanitization result
#[derive(Debug, Clone)]
pub struct SanitizationResult {
    pub metadata_removed: bool,
    pub macros_removed: usize,
    pub scripts_removed: usize,
    pub embedded_files_removed: usize,
    pub total_size_before: usize,
    pub total_size_after: usize,
}

/// Initialize core module
pub fn init() -> Result<(), String> {
    Ok(())
}
