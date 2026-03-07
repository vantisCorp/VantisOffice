//! Platform-specific implementations
//!
//! This module provides platform-specific functionality for:
//! - Windows: DPAPI integration, CryptoAPI support
//! - macOS: Keychain integration, Security framework support
//! - Linux: Secret service integration, /dev/urandom

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

/// Platform-independent secure storage trait
pub trait SecureStorage: Send + Sync {
    /// Store data securely
    fn store(&self, key: &str, data: &[u8]) -> crate::error::Result<()>;
    
    /// Retrieve stored data
    fn retrieve(&self, key: &str) -> crate::error::Result<Vec<u8>>;
    
    /// Delete stored data
    fn delete(&self, key: &str) -> crate::error::Result<()>;
    
    /// Check if data exists
    fn exists(&self, key: &str) -> bool;
}

/// Get the platform-specific secure storage implementation
#[cfg(target_os = "windows")]
pub fn get_secure_storage() -> Box<dyn SecureStorage> {
    Box::new(windows::WindowsSecureStorage::new())
}

/// Get the platform-specific secure storage implementation
#[cfg(target_os = "macos")]
pub fn get_secure_storage() -> Box<dyn SecureStorage> {
    Box::new(macos::MacOSKeychain::new())
}

/// Get the platform-specific secure storage implementation
#[cfg(target_os = "linux")]
pub fn get_secure_storage() -> Box<dyn SecureStorage> {
    Box::new(linux::LinuxSecretService::new())
}

/// Fallback for unsupported platforms
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_secure_storage() -> Box<dyn SecureStorage> {
    Box::new(FallbackStorage::new())
}

/// Fallback in-memory storage for unsupported platforms
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub struct FallbackStorage {
    storage: std::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>,
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
impl FallbackStorage {
    pub fn new() -> Self {
        Self {
            storage: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
impl SecureStorage for FallbackStorage {
    fn store(&self, key: &str, data: &[u8]) -> crate::error::Result<()> {
        let mut storage = self.storage.write().map_err(|e| {
            crate::error::PQCError::Generic(format!("Storage lock error: {}", e))
        })?;
        storage.insert(key.to_string(), data.to_vec());
        Ok(())
    }
    
    fn retrieve(&self, key: &str) -> crate::error::Result<Vec<u8>> {
        let storage = self.storage.read().map_err(|e| {
            crate::error::PQCError::Generic(format!("Storage lock error: {}", e))
        })?;
        storage.get(key).cloned().ok_or_else(|| {
            crate::error::PQCError::Generic(format!("Key not found: {}", key))
        })
    }
    
    fn delete(&self, key: &str) -> crate::error::Result<()> {
        let mut storage = self.storage.write().map_err(|e| {
            crate::error::PQCError::Generic(format!("Storage lock error: {}", e))
        })?;
        storage.remove(key);
        Ok(())
    }
    
    fn exists(&self, key: &str) -> bool {
        let storage = self.storage.read().unwrap();
        storage.contains_key(key)
    }
}

/// Platform information
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    /// Operating system name
    pub os: String,
    /// Architecture
    pub arch: String,
    /// Whether secure storage is available
    pub secure_storage_available: bool,
    /// Whether hardware RNG is available
    pub hardware_rng_available: bool,
}

impl PlatformInfo {
    /// Get current platform information
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            secure_storage_available: cfg!(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "linux"
            )),
            hardware_rng_available: true, // Rust's OsRng uses hardware RNG on all platforms
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_info() {
        let info = PlatformInfo::current();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
    }
}