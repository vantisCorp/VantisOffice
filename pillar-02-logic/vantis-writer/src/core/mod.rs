//! Core document model and editor engine

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Document model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<Paragraph>,
    pub metadata: DocumentMetadata,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub word_count: usize,
}

impl Document {
    /// Create a new document
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now();
        Document {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            paragraphs: vec![],
            metadata: DocumentMetadata {
                created_at: now,
                modified_at: now,
                author: String::new(),
                word_count: 0,
            },
        }
    }

    /// Add a paragraph
    pub fn add_paragraph(&mut self, paragraph: Paragraph) -> Result<()> {
        self.paragraphs.push(paragraph);
        self.update_metadata();
        Ok(())
    }

    /// Update document metadata
    fn update_metadata(&mut self) {
        self.metadata.modified_at = chrono::Utc::now();
        self.metadata.word_count = self
            .paragraphs
            .iter()
            .map(|p| p.text.split_whitespace().count())
            .sum();
    }
}

/// Paragraph model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub text: String,
    pub style: ParagraphStyle,
}

/// Paragraph style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphStyle {
    pub font_size: f32,
    pub font_family: String,
    pub bold: bool,
    pub italic: bool,
    pub color: String,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        ParagraphStyle {
            font_size: 14.0,
            font_family: "Inter".to_string(),
            bold: false,
            italic: false,
            color: "#000000".to_string(),
        }
    }
}

impl Paragraph {
    /// Create a new paragraph
    pub fn new(text: String) -> Self {
        Paragraph {
            text,
            style: ParagraphStyle::default(),
        }
    }

    /// Set style
    pub fn with_style(mut self, style: ParagraphStyle) -> Self {
        self.style = style;
        self
    }
}

/// Editor engine
pub struct Editor {
    document: Document,
    cursor_position: usize,
}

impl Editor {
    /// Create a new editor
    pub fn new(document: Document) -> Self {
        Editor {
            document,
            cursor_position: 0,
        }
    }

    /// Insert text at cursor position
    pub fn insert_text(&mut self, text: &str) -> Result<()> {
        // Implementation would insert text at cursor position
        Ok(())
    }

    /// Delete text at cursor position
    pub fn delete_text(&mut self, count: usize) -> Result<()> {
        // Implementation would delete text
        Ok(())
    }

    /// Get document
    pub fn document(&self) -> &Document {
        &self.document
    }
}
