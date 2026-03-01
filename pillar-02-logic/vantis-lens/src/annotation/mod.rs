//! Annotation module for PDF annotations
//! 
//! Provides support for various annotation types

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Annotation Manager
pub struct AnnotationManager {
    annotations: Arc<RwLock<HashMap<String, crate::core::Annotation>>>,
    enabled: bool,
}

impl AnnotationManager {
    pub fn new() -> Self {
        AnnotationManager {
            annotations: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Create a new annotation
    pub fn create_annotation(&self, annotation_type: crate::core::AnnotationType, page_index: usize) -> Result<crate::core::Annotation, String> {
        if !self.enabled {
            return Err("Annotation manager is disabled".to_string());
        }
        
        let id = Uuid::new_v4().to_string();
        let annotation = crate::core::Annotation::new(id.clone(), annotation_type, page_index);
        
        let mut annotations = self.annotations.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        annotations.insert(id, annotation.clone());
        
        Ok(annotation)
    }
    
    /// Update an annotation
    pub fn update_annotation(&self, annotation: crate::core::Annotation) -> Result<(), String> {
        let mut annotations = self.annotations.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        annotations.insert(annotation.id.clone(), annotation);
        
        Ok(())
    }
    
    /// Delete an annotation
    pub fn delete_annotation(&self, annotation_id: &str) -> Result<(), String> {
        let mut annotations = self.annotations.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        annotations.remove(annotation_id)
            .ok_or_else(|| format!("Annotation '{}' not found", annotation_id))?;
        
        Ok(())
    }
    
    /// Get annotation by ID
    pub fn get_annotation(&self, annotation_id: &str) -> Option<crate::core::Annotation> {
        let annotations = self.annotations.read().ok()?;
        annotations.get(annotation_id).cloned()
    }
    
    /// Get all annotations for a page
    pub fn get_page_annotations(&self, page_index: usize) -> Vec<crate::core::Annotation> {
        let annotations = self.annotations.read().ok();
        match annotations {
            Some(anns) => anns.values()
                .filter(|ann| ann.page_index == page_index)
                .cloned()
                .collect(),
            None => Vec::new(),
        }
    }
    
    /// Get all annotations for a document
    pub fn get_document_annotations(&self) -> Vec<crate::core::Annotation> {
        let annotations = self.annotations.read().ok();
        match annotations {
            Some(anns) => anns.values().cloned().collect(),
            None => Vec::new(),
        }
    }
    
    /// Search annotations by content
    pub fn search_annotations(&self, query: &str) -> Vec<crate::core::Annotation> {
        let annotations = self.annotations.read().ok();
        match annotations {
            Some(anns) => anns.values()
                .filter(|ann| ann.content.to_lowercase().contains(&query.to_lowercase()))
                .cloned()
                .collect(),
            None => Vec::new(),
        }
    }
}

// Re-export Annotation and AnnotationType from core
pub use crate::core::{Annotation, AnnotationType};

/// Initialize annotation module
pub fn init() -> Result<(), String> {
    Ok(())
}