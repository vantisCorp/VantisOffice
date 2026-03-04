//! Graphics and vector rendering components

pub use fonts::FontManager;
pub use shaders::ShaderManager;
pub use textures::TextureManager;
pub use vector_engine::{Color, FillType, Paint, Path, StrokeCap, StrokeJoin, VectorEngine};

mod fonts;
mod shaders;
mod textures;
mod vector_engine;

/// Initialize graphics subsystems
pub fn init() -> Result<(), super::RenderError> {
    Ok(())
}
