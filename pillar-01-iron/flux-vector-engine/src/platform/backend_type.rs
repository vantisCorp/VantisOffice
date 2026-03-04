//! Backend type

use serde::{Deserialize, Serialize};

/// Backend type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackendType {
    Vulkan,
    EGL,
    Metal,
}
