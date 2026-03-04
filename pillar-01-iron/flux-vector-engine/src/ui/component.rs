//! Component system

use serde::{Deserialize, Serialize};

/// Component trait
pub trait Component {
    /// Render the component
    fn render(&self, _ctx: &mut ComponentContext);

    /// Handle events
    fn handle_event(&mut self, _event: &Event);

    /// Update the component
    fn update(&mut self, _dt: f32);
}

/// Component context
pub struct ComponentContext {
    pub bounds: (f32, f32, f32, f32), // x, y, width, height
}

impl ComponentContext {
    /// Create a new component context
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bounds: (x, y, width, height),
        }
    }

    /// Get bounds
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        self.bounds
    }
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub x: f32,
    pub y: f32,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, x: f32, y: f32) -> Self {
        Self { event_type, x, y }
    }
}

/// Event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Click,
    DoubleClick,
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
    Scroll,
    Resize,
    Focus,
    Blur,
}
