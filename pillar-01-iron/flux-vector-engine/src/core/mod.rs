//! Core rendering components

pub use config::{WindowConfig, VSyncMode, RenderConfig};
pub use error::{RenderError, RenderResult};

mod config;
mod error;

/// Initialize core subsystems
pub fn init() -> Result<(), RenderError> {
    Ok(())
}