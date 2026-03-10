//! Vantis Writer: Advanced word processor for VantisOffice
//!
//! Features Babel Typography, Deep Focus Mode, native Markdown support,
//! post-quantum document encryption via Vantis Vault integration,
//! and real-time collaboration via Vantis Link integration.

pub mod ai;
pub mod collaboration;
pub mod core;
pub mod encryption;
pub mod focus;
pub mod markdown;
pub mod typography;
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