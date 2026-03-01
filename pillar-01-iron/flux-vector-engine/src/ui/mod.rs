//! UI components and widgets

pub use component::{Component, ComponentContext, Event, EventType};
pub use animation::{Animation, EasingFunction, AnimationEngine};
pub use widgets::{Widget, Button, TextField, ListView};

mod component;
mod animation;
mod widgets;

/// Initialize UI subsystems
pub fn init() -> Result<(), super::RenderError> {
    Ok(())
}