//! Core data structures for Vantis Lens

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

/// PDF Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument {
    pub path: String,
    pub pages: Vec<PdfPage>,
    pub metadata: PdfMetadata,
    pub security: PdfSecurity,
    pub version: String,
    pub is_encrypted: bool,
    pub is_sterilized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
    pub modification_date: Option<chrono::DateTime<chrono::Utc>>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSecurity {
    pub has_password: bool,
    pub has_permissions: bool,
    pub can_print: bool,
    pub can_copy: bool,
    pub can_modify: bool,
    pub can_extract: bool,
    pub encryption_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPage {
    pub index: usize,
    pub width: f64,
    pub height: f64,
    pub rotation: i32,
    pub annotations: Vec<Annotation>,
    pub text_content: Option<String>,
}

impl PdfDocument {
    pub fn new(path: String) -> Self {
        PdfDocument {
            path,
            pages: Vec::new(),
            metadata: PdfMetadata {
                title: None,
                author: None,
                subject: None,
                keywords: None,
                creator: None,
                producer: None,
                creation_date: None,
                modification_date: None,
                custom_properties: HashMap::new(),
            },
            security: PdfSecurity {
                has_password: false,
                has_permissions: false,
                can_print: true,
                can_copy: true,
                can_modify: true,
                can_extract: true,
                encryption_level: None,
            },
            version: "1.7".to_string(),
            is_encrypted: false,
            is_sterilized: false,
        }
    }
    
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
    
    pub fn get_page(&self, index: usize) -> Option<&PdfPage> {
        self.pages.get(index)
    }
    
    pub fn get_page_mut(&mut self, index: usize) -> Option<&mut PdfPage> {
        self.pages.get_mut(index)
    }
    
    pub fn add_page(&mut self, page: PdfPage) {
        self.pages.push(page);
    }
    
    pub fn is_valid(&self) -> bool {
        !self.pages.is_empty()
    }
}

/// Annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub annotation_type: AnnotationType,
    pub page_index: usize,
    pub rect: AnnotationRect,
    pub content: String,
    pub author: Option<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub color: Option<String>,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationType {
    Text,
    Highlight,
    Underline,
    Strikeout,
    Squiggly,
    Comment,
    Stamp,
    Signature,
    Freehand,
    Rectangle,
    Circle,
    Line,
    Arrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Annotation {
    pub fn new(id: String, annotation_type: AnnotationType, page_index: usize) -> Self {
        let now = chrono::Utc::now();
        Annotation {
            id,
            annotation_type,
            page_index,
            rect: AnnotationRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            content: String::new(),
            author: None,
            created: now,
            modified: now,
            color: None,
            is_visible: true,
        }
    }
    
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
    
    pub fn with_position(mut self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.rect = AnnotationRect {
            x,
            y,
            width,
            height,
        };
        self
    }
    
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
}

/// Initialize core module
pub fn init() -> Result<(), String> {
    Ok(())
}