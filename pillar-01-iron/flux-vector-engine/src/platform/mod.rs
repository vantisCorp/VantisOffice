//! Platform-specific rendering backends

pub use vulkan_backend::VulkanBackend;
pub use egl_backend::EGLBackend;
pub use metal_backend::MetalBackend;
pub use backend_type::BackendType;

mod vulkan_backend;
mod egl_backend;
mod metal_backend;
mod backend_type;