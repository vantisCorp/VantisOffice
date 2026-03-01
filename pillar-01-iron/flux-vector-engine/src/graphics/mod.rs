//! Graphics and vector rendering components

pub use vector_engine::{VectorEngine, Path, Paint, Color, FillType, StrokeCap, StrokeJoin};
pub use shaders::ShaderManager;
pub use textures::TextureManager;
pub use fonts::FontManager;

mod vector_engine;
mod shaders;
mod textures;
mod fonts;

/// Initialize graphics subsystems
pub fn init() -> Result<(), super::RenderError> {
    Ok(())
}