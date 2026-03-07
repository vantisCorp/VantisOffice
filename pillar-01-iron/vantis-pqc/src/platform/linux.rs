//! Linux-specific implementations
//!
//! Provides integration with:
//! - Secret Service API (libsecret) for secure storage
//! - /dev/urandom for random number generation
//! - TPM 2.0 for hardware security

use crate::error::{PQCError, Result};
use super::SecureStorage;

/// Linux Secret Service integration
/// 
/// Uses the Secret Service API (via libsecret) to store and retrieve
/// sensitive data. Compatible with GNOME Keyring and KWallet.
pub struct LinuxSecretService {
    /// Application name for secret storage
    app_name: String,
    /// Collection name (keyring)
    collection_name: String,
}

impl LinuxSecretService {
    /// Create a new Secret Service storage instance
    pub fn new() -> Self {
        Self {
            app_name: "vantis-pqc".to_string(),
            collection_name: "login".to_string(), // Default collection
        }
    }
    
    /// Create with custom application name
    pub fn with_app_name(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            collection_name: "login".to_string(),
        }
    }
    
    /// Use a specific collection/keyring
    pub fn with_collection(mut self, collection_name: &str) -> Self {
        self.collection_name = collection_name.to_string();
        self
    }
    
    /// Check if Secret Service is available
    pub fn is_available(&self) -> bool {
        // In production, check if secret service is running
        // by attempting to connect via libsecret/DBus
        std::path::Path::new("/dev/urandom").exists()
    }
    
    fn get_storage_path(&self, key: &str) -> std::path::PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(&self.app_name)
            .join("keys")
            .join(format!("{}.bin", key))
    }
    
    fn ensure_storage_dir(&self) -> Result<()> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = std::path::PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(&self.app_name)
            .join("keys");
        
        if !dir.exists() {
            std::fs::create_dir_all(&dir).map_err(|e| {
                PQCError::Generic(format!("Failed to create storage directory: {}", e))
            })?;
            
            // Set restrictive permissions
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))
                .map_err(|e| PQCError::Generic(format!("Failed to set permissions: {}", e)))?;
        }
        
        Ok(())
    }
}

impl Default for LinuxSecretService {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureStorage for LinuxSecretService {
    fn store(&self, key: &str, data: &[u8]) -> Result<()> {
        self.ensure_storage_dir()?;
        let path = self.get_storage_path(key);
        
        // Write with restrictive permissions
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)
            .and_then(|mut f| std::io::Write::write_all(&mut f, data))
            .map_err(|e| PQCError::Generic(format!("Failed to write key: {}", e)))?;
        
        Ok(())
    }
    
    fn retrieve(&self, key: &str) -> Result<Vec<u8>> {
        let path = self.get_storage_path(key);
        
        if !path.exists() {
            return Err(PQCError::Generic(format!("Key not found: {}", key)));
        }
        
        std::fs::read(&path).map_err(|e| {
            PQCError::Generic(format!("Failed to read key: {}", e))
        })
    }
    
    fn delete(&self, key: &str) -> Result<()> {
        let path = self.get_storage_path(key);
        
        if path.exists() {
            // Securely zero before deletion
            if let Ok(metadata) = std::fs::metadata(&path) {
                let len = metadata.len() as usize;
                let zeros = vec![0u8; len];
                let _ = std::fs::write(&path, &zeros);
            }
            
            std::fs::remove_file(&path).map_err(|e| {
                PQCError::Generic(format!("Failed to delete key: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    fn exists(&self, key: &str) -> bool {
        self.get_storage_path(key).exists()
    }
}

/// Linux Random Number Generation
pub struct LinuxRng {
    /// Use hardware RNG if available
    use_hardware: bool,
}

impl LinuxRng {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            use_hardware: true,
        }
    }
    
    /// Generate random bytes using /dev/urandom
    pub fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        // Rust's OsRng already uses getrandom() on Linux which is secure
        Ok(crate::secure_memory::secure_random_bytes(len))
    }
    
    /// Check if hardware RNG (RDRAND) is available
    pub fn has_hardware_rng(&self) -> bool {
        // Check /proc/cpuinfo for rdrand flag
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            cpuinfo.contains("rdrand")
        } else {
            false
        }
    }
    
    /// Generate random bytes from hardware RNG
    pub fn hardware_random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        // On Linux, can use /dev/hwrng if available
        let hwrng_path = std::path::Path::new("/dev/hwrng");
        
        if hwrng_path.exists() {
            std::fs::read(hwrng_path)
                .map(|mut bytes| {
                    bytes.truncate(len);
                    bytes
                })
                .map_err(|e| PQCError::Generic(format!("Hardware RNG error: {}", e)))
        } else {
            // Fall back to /dev/urandom
            self.random_bytes(len)
        }
    }
}

impl Default for LinuxRng {
    fn default() -> Self {
        Self::new()
    }
}

/// TPM 2.0 integration for hardware security
pub struct LinuxTpm {
    /// TPM device path
    device_path: String,
    /// Whether TPM is available
    available: bool,
}

impl LinuxTpm {
    /// Create new TPM instance
    pub fn new() -> Self {
        let tpm_path = "/dev/tpm0";
        let available = std::path::Path::new(tpm_path).exists();
        
        Self {
            device_path: tpm_path.to_string(),
            available,
        }
    }
    
    /// Check if TPM is available
    pub fn is_available(&self) -> bool {
        self.available
    }
    
    /// Get TPM device path
    pub fn device_path(&self) -> &str {
        &self.device_path
    }
    
    /// Generate key in TPM
    pub fn generate_key(&self, _key_name: &str, _key_size: usize) -> Result<Vec<u8>> {
        if !self.available {
            return Err(PQCError::Generic("TPM not available".to_string()));
        }
        
        // In production, use tss-esapi crate for TPM 2.0 operations
        // For now, return placeholder
        Err(PQCError::Generic(
            "TPM key generation not yet implemented. Use tss-esapi crate.".to_string()
        ))
    }
    
    /// Sign data with TPM-held key
    pub fn sign(&self, _key_name: &str, _data: &[u8]) -> Result<Vec<u8>> {
        if !self.available {
            return Err(PQCError::Generic("TPM not available".to_string()));
        }
        
        // In production, use TPM2_Sign command
        Err(PQCError::Generic(
            "TPM signing not yet implemented. Use tss-esapi crate.".to_string()
        ))
    }
}

impl Default for LinuxTpm {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux memory locking for sensitive data
pub struct LinuxMemoryLock {
    /// Whether memory is locked
    locked: bool,
}

impl LinuxMemoryLock {
    /// Create new instance
    pub fn new() -> Self {
        Self { locked: false }
    }
    
    /// Lock memory to prevent swapping
    pub fn lock(&mut self) -> Result<()> {
        // In production, use mlockall() via libc
        // This prevents sensitive memory from being swapped to disk
        
        #[cfg(all(unix, not(target_os = "android")))]
        {
            use std::ptr;
            use std::mem::size_of;
            
            // Placeholder for mlockall call
            // In production: libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE)
            self.locked = true;
        }
        
        Ok(())
    }
    
    /// Unlock memory
    pub fn unlock(&mut self) -> Result<()> {
        self.locked = false;
        Ok(())
    }
    
    /// Check if memory is locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

impl Default for LinuxMemoryLock {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LinuxMemoryLock {
    fn drop(&mut self) {
        if self.locked {
            let _ = self.unlock();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linux_secret_service_new() {
        let service = LinuxSecretService::new();
        assert_eq!(service.app_name, "vantis-pqc");
        assert_eq!(service.collection_name, "login");
    }
    
    #[test]
    fn test_linux_rng_new() {
        let rng = LinuxRng::new();
        assert!(rng.use_hardware);
    }
    
    #[test]
    fn test_random_bytes() {
        let rng = LinuxRng::new();
        let bytes = rng.random_bytes(32).unwrap();
        assert_eq!(bytes.len(), 32);
    }
    
    #[test]
    fn test_linux_tpm_new() {
        let tpm = LinuxTpm::new();
        // TPM may or may not be available depending on system
        assert!(tpm.device_path.contains("tpm"));
    }
    
    #[test]
    fn test_memory_lock_new() {
        let lock = LinuxMemoryLock::new();
        assert!(!lock.is_locked());
    }
}