//! Vantis Writer: Advanced word processor for VantisOffice
//!
//! Features Babel Typography, Deep Focus Mode, and native Markdown support.

pub mod core;
pub mod typography;
pub mod markdown;
pub mod focus;
pub mod ai;
pub mod ui;

use anyhow::Result;

/// Initialize Vantis Writer
pub fn init() -> Result<()> {
    // Initialize typography engine
    typography::init()?;
    
    // Initialize markdown parser
    markdown::init()?;
    
    // Initialize AI assistant
    ai::init()?;
    
    Ok(())
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
