//! Render-specific errors

use thiserror::Error;

/// Render-specific errors
#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Vulkan error: {0}")]
    Vulkan(String),
    
    #[error("Window error: {0}")]
    Window(String),
    
    #[error("Swapchain error: {0}")]
    Swapchain(String),
    
    #[error("Device error: {0}")]
    Device(String),
    
    #[error("Surface error: {0}")]
    Surface(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("General error: {0}")]
    General(String),
}

impl From<String> for RenderError {
    fn from(s: String) -> Self {
        RenderError::General(s)
    }
}

pub type RenderResult<T> = Result<T, RenderError>;