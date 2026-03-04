//! Platform-specific rendering backends

pub use backend_type::BackendType;
pub use egl_backend::EGLBackend;
pub use metal_backend::MetalBackend;
pub use vulkan_backend::VulkanBackend;

mod backend_type;
mod egl_backend;
mod metal_backend;
mod vulkan_backend;
