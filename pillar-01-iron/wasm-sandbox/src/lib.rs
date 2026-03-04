//! WASM-Sandbox: Secure plugin execution environment for VantisOffice
//!
//! This library provides a zero-trust execution environment for plugins and macros
//! using WebAssembly technology.

pub mod api;
pub mod plugins;
pub mod runtime;
pub mod security;

use anyhow::Result;

/// Initialize WASM sandbox
pub fn init() -> Result<()> {
    // Initialize Wasmtime
    runtime::init()?;

    // Initialize security systems
    security::init()?;

    Ok(())
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
