//! UI components and widgets

pub use animation::{Animation, AnimationEngine, EasingFunction};
pub use component::{Component, ComponentContext, Event, EventType};
pub use widgets::{Button, ListView, TextField, Widget};

mod animation;
mod component;
mod widgets;

/// Initialize UI subsystems
pub fn init() -> Result<(), super::RenderError> {
    Ok(())
}
