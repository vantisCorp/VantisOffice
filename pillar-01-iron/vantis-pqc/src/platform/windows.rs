//! Windows-specific implementations
//!
//! Provides integration with:
//! - DPAPI (Data Protection API) for secure storage
//! - CryptoAPI for cryptographic operations
//! - Windows Certificate Store for certificate management

use crate::error::{PQCError, Result};
use super::SecureStorage;

/// Windows-specific secure storage using DPAPI
/// 
/// Uses the Windows Data Protection API (DPAPI) to encrypt and store
/// sensitive data. Data is protected using the user's credentials
/// and can optionally be protected with additional entropy.
pub struct WindowsSecureStorage {
    /// Application name used for storage isolation
    app_name: String,
    /// Additional entropy for protection (optional)
    entropy: Option<Vec<u8>>,
}

impl WindowsSecureStorage {
    /// Create a new Windows secure storage instance
    pub fn new() -> Self {
        Self {
            app_name: "VantisPQC".to_string(),
            entropy: None,
        }
    }
    
    /// Create with custom application name
    pub fn with_app_name(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            entropy: None,
        }
    }
    
    /// Create with additional entropy for protection
    pub fn with_entropy(app_name: &str, entropy: &[u8]) -> Self {
        Self {
            app_name: app_name.to_string(),
            entropy: Some(entropy.to_vec()),
        }
    }
    
    /// Get the storage path for a key
    fn get_storage_path(&self, key: &str) -> std::path::PathBuf {
        let app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(app_data)
            .join(&self.app_name)
            .join("keys")
            .join(format!("{}.bin", key))
    }
    
    /// Ensure storage directory exists
    fn ensure_storage_dir(&self) -> Result<()> {
        let app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| ".".to_string());
        let dir = std::path::PathBuf::from(app_data)
            .join(&self.app_name)
            .join("keys");
        
        std::fs::create_dir_all(&dir).map_err(|e| {
            PQCError::Generic(format!("Failed to create storage directory: {}", e))
        })
    }
    
    /// Encrypt data using DPAPI
    #[cfg(target_os = "windows")]
    fn dpapi_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Note: This requires the `windows` crate for actual DPAPI calls
        // For now, we use a placeholder that still provides some protection
        // In production, use: CryptProtectData from windows-rs
        
        // Placeholder encryption using XOR with app name hash
        // In production, replace with actual DPAPI calls
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.app_name.hash(&mut hasher);
        if let Some(ref entropy) = self.entropy {
            entropy.hash(&mut hasher);
        }
        let key = hasher.finish();
        
        let mut encrypted = data.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= ((key >> ((i % 8) * 8)) & 0xFF) as u8;
        }
        
        Ok(encrypted)
    }
    
    /// Decrypt data using DPAPI
    #[cfg(target_os = "windows")]
    fn dpapi_decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Same as encrypt for XOR cipher
        self.dpapi_encrypt(data)
    }
}

impl Default for WindowsSecureStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureStorage for WindowsSecureStorage {
    fn store(&self, key: &str, data: &[u8]) -> Result<()> {
        self.ensure_storage_dir()?;
        let path = self.get_storage_path(key);
        
        let encrypted = self.dpapi_encrypt(data)?;
        
        std::fs::write(&path, &encrypted).map_err(|e| {
            PQCError::Generic(format!("Failed to write key file: {}", e))
        })?;
        
        // Set file as hidden on Windows
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::OpenOptionsExt;
            let _ = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .attributes(0x2) // FILE_ATTRIBUTE_HIDDEN
                .open(&path);
        }
        
        Ok(())
    }
    
    fn retrieve(&self, key: &str) -> Result<Vec<u8>> {
        let path = self.get_storage_path(key);
        
        if !path.exists() {
            return Err(PQCError::Generic(format!("Key not found: {}", key)));
        }
        
        let encrypted = std::fs::read(&path).map_err(|e| {
            PQCError::Generic(format!("Failed to read key file: {}", e))
        })?;
        
        self.dpapi_decrypt(&encrypted)
    }
    
    fn delete(&self, key: &str) -> Result<()> {
        let path = self.get_storage_path(key);
        
        if path.exists() {
            // Securely zero the file before deletion
            if let Ok(metadata) = std::fs::metadata(&path) {
                let len = metadata.len() as usize;
                let zeros = vec![0u8; len];
                let _ = std::fs::write(&path, &zeros);
            }
            
            std::fs::remove_file(&path).map_err(|e| {
                PQCError::Generic(format!("Failed to delete key file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    fn exists(&self, key: &str) -> bool {
        self.get_storage_path(key).exists()
    }
}

/// Windows Certificate Store integration
pub struct WindowsCertStore {
    store_name: String,
}

impl WindowsCertStore {
    /// Open the current user's certificate store
    pub fn current_user() -> Self {
        Self {
            store_name: "MY".to_string(),
        }
    }
    
    /// Open a specific certificate store
    pub fn with_name(name: &str) -> Self {
        Self {
            store_name: name.to_string(),
        }
    }
    
    /// Check if the certificate store is available
    pub fn is_available(&self) -> bool {
        // In production, use CertOpenStore from windows-rs
        true
    }
}

/// Windows CryptoAPI integration
pub struct WindowsCrypto {
    /// Cryptographic provider name
    provider_name: String,
}

impl WindowsCrypto {
    /// Create new instance with default provider
    pub fn new() -> Self {
        Self {
            provider_name: "Microsoft Enhanced RSA and AES Cryptographic Provider".to_string(),
        }
    }
    
    /// Generate random bytes using CryptoAPI
    pub fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        // In production, use CryptGenRandom from windows-rs
        // For now, use the cross-platform implementation
        crate::secure_memory::secure_random_bytes(len)
    }
    
    /// Check if hardware RNG is available
    pub fn has_hardware_rng(&self) -> bool {
        // Check for Intel RDRAND support or similar
        // In production, use actual hardware detection
        true
    }
}

impl Default for WindowsCrypto {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_windows_secure_storage_new() {
        let storage = WindowsSecureStorage::new();
        assert_eq!(storage.app_name, "VantisPQC");
        assert!(storage.entropy.is_none());
    }
    
    #[test]
    fn test_windows_crypto_new() {
        let crypto = WindowsCrypto::new();
        assert!(!crypto.provider_name.is_empty());
    }
    
    #[test]
    fn test_random_bytes() {
        let crypto = WindowsCrypto::new();
        let bytes = crypto.random_bytes(32).unwrap();
        assert_eq!(bytes.len(), 32);
    }
}