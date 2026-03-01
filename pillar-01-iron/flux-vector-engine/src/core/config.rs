//! Configuration structures for rendering

use serde::{Deserialize, Serialize};

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub refresh_rate: u32,
    pub vsync: VSyncMode,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub always_on_top: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            title: "Flux Vector Engine".to_string(),
            refresh_rate: 120,
            vsync: VSyncMode::Adaptive,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
        }
    }
}

/// VSync mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VSyncMode {
    Off,
    On,
    Adaptive,
}

/// Render configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub msaa_samples: u32,
    pub anisotropy: f32,
    pub texture_quality: TextureQuality,
    pub shadow_quality: ShadowQuality,
    pub reflection_quality: ReflectionQuality,
    pub ambient_occlusion: bool,
    pub bloom: bool,
    pub anti_aliasing: AntiAliasing,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            msaa_samples: 4,
            anisotropy: 16.0,
            texture_quality: TextureQuality::High,
            shadow_quality: ShadowQuality::High,
            reflection_quality: ReflectionQuality::Medium,
            ambient_occlusion: true,
            bloom: true,
            anti_aliasing: AntiAliasing::FXAA,
        }
    }
}

/// Texture quality
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextureQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Shadow quality
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShadowQuality {
    Off,
    Low,
    Medium,
    High,
    Ultra,
}

/// Reflection quality
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReflectionQuality {
    Off,
    Low,
    Medium,
    High,
}

/// Anti-aliasing mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AntiAliasing {
    Off,
    FXAA,
    MSAA,
    TAA,
}