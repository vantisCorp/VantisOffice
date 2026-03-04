//! Vantis Canvas - 3D-accelerated presentation application
//!
//! Features:
//! - Infinite Canvas for non-linear navigation
//! - GPU-accelerated rendering (Vulkan-based)
//! - Real-time collaboration
//! - Advanced animations and transitions
//! - Support for 4K and 8K resolutions

pub mod animation;
pub mod collaboration;
pub mod core;
pub mod export;
pub mod rendering;

pub use animation::{Animation, AnimationManager, Timeline, Transition};
pub use collaboration::{CanvasCollaboration, Cursor, User};
pub use core::{Canvas, Image, Layer, Shape, Slide, Text};
pub use export::{CanvasExporter, ExportFormat};
pub use rendering::{RenderContext, RenderTarget, Renderer};

/// Vantis Canvas version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Canvas
pub fn init() -> Result<(), CanvasError> {
    // Initialize subsystems
    core::init()?;
    rendering::init()?;
    animation::init()?;

    Ok(())
}

/// Canvas-specific errors
#[derive(Debug, thiserror::Error)]
pub enum CanvasError {
    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("Animation error: {0}")]
    Animation(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Collaboration error: {0}")]
    Collaboration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<String> for CanvasError {
    fn from(s: String) -> Self {
        CanvasError::General(s)
    }
}
