//! UI widgets

use super::{Component, ComponentContext, Event, EventType};
use serde::{Deserialize, Serialize};

/// Widget trait
pub trait Widget: Component {
    /// Get widget bounds
    fn bounds(&self) -> (f32, f32, f32, f32);
    
    /// Set widget bounds
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32);
}

/// Button widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub text: String,
    pub bounds: (f32, f32, f32, f32),
    pub enabled: bool,
    pub hovered: bool,
    pub pressed: bool,
}

impl Button {
    /// Create a new button
    pub fn new(text: String) -> Self {
        Self {
            text,
            bounds: (0.0, 0.0, 100.0, 30.0),
            enabled: true,
            hovered: false,
            pressed: false,
        }
    }
    
    /// Set bounds
    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = (x, y, width, height);
        self
    }
    
    /// Check if button is clicked
    pub fn is_clicked(&self) -> bool {
        self.pressed && self.enabled
    }
}

impl Component for Button {
    fn render(&self, _ctx: &mut ComponentContext) {
        // Render button
    }
    
    fn handle_event(&mut self, event: &Event) {
        match event.event_type {
            EventType::MouseMove => {
                let (x, y, width, height) = self.bounds;
                self.hovered = event.x >= x && event.x <= x + width
                    && event.y >= y && event.y <= y + height;
            }
            EventType::MouseDown => {
                if self.hovered {
                    self.pressed = true;
                }
            }
            EventType::MouseUp => {
                self.pressed = false;
            }
            _ => {}
        }
    }
    
    fn update(&mut self, _dt: f32) {
        // Update button state
    }
}

impl Widget for Button {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds
    }
    
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.bounds = (x, y, width, height);
    }
}

/// Text field widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextField {
    pub text: String,
    pub placeholder: String,
    pub bounds: (f32, f32, f32, f32),
    pub focused: bool,
    pub editable: bool,
}

impl TextField {
    /// Create a new text field
    pub fn new(placeholder: String) -> Self {
        Self {
            text: String::new(),
            placeholder,
            bounds: (0.0, 0.0, 200.0, 30.0),
            focused: false,
            editable: true,
        }
    }
    
    /// Set bounds
    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = (x, y, width, height);
        self
    }
    
    /// Set text
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    
    /// Get text
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Component for TextField {
    fn render(&self, _ctx: &mut ComponentContext) {
        // Render text field
    }
    
    fn handle_event(&mut self, event: &Event) {
        match event.event_type {
            EventType::Click => {
                let (x, y, width, height) = self.bounds;
                self.focused = event.x >= x && event.x <= x + width
                    && event.y >= y && event.y <= y + height;
            }
            EventType::Focus => {
                self.focused = true;
            }
            EventType::Blur => {
                self.focused = false;
            }
            _ => {}
        }
    }
    
    fn update(&mut self, _dt: f32) {
        // Update text field state
    }
}

impl Widget for TextField {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds
    }
    
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.bounds = (x, y, width, height);
    }
}

/// List view widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListView {
    pub items: Vec<String>,
    pub bounds: (f32, f32, f32, f32),
    pub selected_index: Option<usize>,
    pub scroll_offset: f32,
}

impl ListView {
    /// Create a new list view
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            bounds: (0.0, 0.0, 200.0, 300.0),
            selected_index: None,
            scroll_offset: 0.0,
        }
    }
    
    /// Set bounds
    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = (x, y, width, height);
        self
    }
    
    /// Add item
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
    
    /// Remove item
    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }
    
    /// Get selected item
    pub fn selected_item(&self) -> Option<&String> {
        self.selected_index.and_then(|i| self.items.get(i))
    }
}

impl Component for ListView {
    fn render(&self, _ctx: &mut ComponentContext) {
        // Render list view
    }
    
    fn handle_event(&mut self, event: &Event) {
        match event.event_type {
            EventType::Click => {
                let (x, y, width, height) = self.bounds;
                if event.x >= x && event.x <= x + width
                    && event.y >= y && event.y <= y + height
                {
                    // Calculate clicked item index
                    let item_height = 30.0;
                    let relative_y = event.y - y + self.scroll_offset;
                    let index = (relative_y / item_height) as usize;
                    if index < self.items.len() {
                        self.selected_index = Some(index);
                    }
                }
            }
            EventType::Scroll => {
                self.scroll_offset += event.y;
            }
            _ => {}
        }
    }
    
    fn update(&mut self, _dt: f32) {
        // Update list view state
    }
}

impl Widget for ListView {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds
    }
    
    fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.bounds = (x, y, width, height);
    }
}