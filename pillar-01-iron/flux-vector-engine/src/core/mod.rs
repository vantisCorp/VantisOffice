//! Core rendering components

pub use config::{RenderConfig, VSyncMode, WindowConfig};
pub use error::{RenderError, RenderResult};

mod config;
mod error;

/// Initialize core subsystems
pub fn init() -> Result<(), RenderError> {
    Ok(())
}
