//! Font management

use std::collections::HashMap;
use std::sync::Arc;

/// Font manager
pub struct FontManager {
    fonts: HashMap<String, Arc<Font>>,
    default_font: Option<Arc<Font>>,
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            default_font: None,
        }
    }
    
    /// Load font from bytes
    pub fn load_font(&mut self, name: String, _data: &[u8]) -> Result<(), String> {
        let font = Arc::new(Font::new(name.clone()));
        self.fonts.insert(name.clone(), font);
        Ok(())
    }
    
    /// Get font by name
    pub fn get_font(&self, name: &str) -> Option<&Arc<Font>> {
        self.fonts.get(name)
    }
    
    /// Set default font
    pub fn set_default_font(&mut self, name: &str) {
        if let Some(font) = self.fonts.get(name).cloned() {
            self.default_font = Some(font);
        }
    }
    
    /// Get default font
    pub fn default_font(&self) -> Option<&Arc<Font>> {
        self.default_font.as_ref()
    }
}

/// Font
#[derive(Debug, Clone)]
pub struct Font {
    name: String,
}

impl Font {
    /// Create a new font
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    /// Get font name
    pub fn name(&self) -> &str {
        &self.name
    }
}