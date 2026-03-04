//! Flux Vector Engine - GPU-accelerated rendering engine for VantisOffice
//!
//! Features:
//! - Vulkan-based rendering pipeline
//! - 120Hz refresh rate support
//! - Vector graphics with hardware acceleration
//! - Low power consumption
//! - Multi-monitor support

pub mod core;
pub mod graphics;
pub mod platform;
pub mod ui;

pub use core::{RenderError, RenderResult, VSyncMode, WindowConfig};
pub use graphics::{Color, FillType, Paint, Path, VectorEngine};
pub use ui::{
    Animation, Button, Component, ComponentContext, EasingFunction, Event, EventType, ListView,
    TextField,
};

/// Flux Vector Engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Flux Vector Engine
pub fn init() -> Result<(), RenderError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Flux Vector Engine v{} initializing...", VERSION);

    // Initialize subsystems
    core::init()?;
    graphics::init()?;
    ui::init()?;

    tracing::info!("Flux Vector Engine initialized successfully");
    Ok(())
}

/// Get engine capabilities
pub fn capabilities() -> EngineCapabilities {
    EngineCapabilities {
        version: VERSION.to_string(),
        max_texture_size: 16384,
        max_framebuffer_size: 16384,
        supports_ray_tracing: false,
        supports_compute_shaders: true,
        max_msaa_samples: 16,
        max_anisotropy: 16.0,
    }
}

/// Engine capabilities
#[derive(Debug, Clone, serde::Serialize)]
pub struct EngineCapabilities {
    pub version: String,
    pub max_texture_size: u32,
    pub max_framebuffer_size: u32,
    pub supports_ray_tracing: bool,
    pub supports_compute_shaders: bool,
    pub max_msaa_samples: u32,
    pub max_anisotropy: f32,
}
