//! Vantis-Core-IO: Low-level file handling library for Vantis OS
//! 
//! This library provides secure, memory-safe file operations without
//! using standard OS syscalls, ensuring no memory leaks during I/O operations.

pub mod core;
pub mod fs;
pub mod crypto;

pub use core::{FileHandle, OpenFlags};
pub use fs::{VirtualFileSystem, FileSystemError};
pub use crypto::{EncryptionLevel, IntegrityCheck};

use anyhow::Result;

/// Vantis-Core-IO library initialization
pub fn init() -> Result<()> {
    // Initialize custom allocator
    core::allocator::init()?;
    
    // Initialize virtual file system
    fs::init()?;
    
    // Initialize crypto primitives
    crypto::init()?;
    
    Ok(())
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
