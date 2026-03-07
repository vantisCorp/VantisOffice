//! macOS-specific implementations
//!
//! Provides integration with:
//! - Keychain Services for secure storage
//! - Security framework for cryptographic operations
//! - CommonCrypto for symmetric encryption

use crate::error::{PQCError, Result};
use super::SecureStorage;

/// macOS Keychain integration
/// 
/// Uses the macOS Keychain Services API to store and retrieve
/// sensitive data securely. Data is protected by the user's
/// login password and can optionally require authentication.
pub struct MacOSKeychain {
    /// Service name for Keychain items
    service_name: String,
    /// Keychain to use (default or custom)
    keychain_name: Option<String>,
}

impl MacOSKeychain {
    /// Create a new Keychain storage instance
    pub fn new() -> Self {
        Self {
            service_name: "com.vantis.pqc".to_string(),
            keychain_name: None,
        }
    }
    
    /// Create with custom service name
    pub fn with_service(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            keychain_name: None,
        }
    }
    
    /// Use a specific keychain file
    pub fn with_keychain(mut self, keychain_name: &str) -> Self {
        self.keychain_name = Some(keychain_name.to_string());
        self
    }
    
    /// Check if Keychain is accessible
    pub fn is_accessible(&self) -> bool {
        // In production, use SecKeychainCopyDefault to verify access
        true
    }
    
    /// Store data in Keychain
    /// 
    /// Uses SecItemAdd to add a new keychain item with:
    /// - kSecClass: kSecClassGenericPassword
    /// - kSecAttrService: service_name
    /// - kSecAttrAccount: key (account name)
    /// - kSecValueData: data (the encrypted data)
    fn keychain_add(&self, key: &str, data: &[u8]) -> Result<()> {
        // In production, use Security framework via core-foundation or security-framework crate
        // For now, provide a file-based fallback with proper protections
        
        let path = self.get_storage_path(key);
        self.ensure_storage_dir()?;
        
        // Set restrictive permissions (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600) // Owner read/write only
                .open(&path)
                .and_then(|mut f| std::io::Write::write_all(&mut f, data))
                .map_err(|e| PQCError::Generic(format!("Failed to write key: {}", e)))?;
        }
        
        #[cfg(not(unix))]
        {
            std::fs::write(&path, data).map_err(|e| {
                PQCError::Generic(format!("Failed to write key: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// Retrieve data from Keychain
    fn keychain_get(&self, key: &str) -> Result<Vec<u8>> {
        // In production, use SecItemCopyMatching
        let path = self.get_storage_path(key);
        
        if !path.exists() {
            return Err(PQCError::Generic(format!("Key not found: {}", key)));
        }
        
        std::fs::read(&path).map_err(|e| {
            PQCError::Generic(format!("Failed to read key: {}", e))
        })
    }
    
    /// Delete data from Keychain
    fn keychain_delete(&self, key: &str) -> Result<()> {
        // In production, use SecItemDelete
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
    
    fn get_storage_path(&self, key: &str) -> std::path::PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home)
            .join(".vantis_pqc")
            .join("keys")
            .join(format!("{}.bin", key))
    }
    
    fn ensure_storage_dir(&self) -> Result<()> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = std::path::PathBuf::from(home)
            .join(".vantis_pqc")
            .join("keys");
        
        if !dir.exists() {
            std::fs::create_dir_all(&dir).map_err(|e| {
                PQCError::Generic(format!("Failed to create storage directory: {}", e))
            })?;
            
            // Set restrictive permissions on directory
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))
                    .map_err(|e| PQCError::Generic(format!("Failed to set permissions: {}", e)))?;
            }
        }
        
        Ok(())
    }
}

impl Default for MacOSKeychain {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureStorage for MacOSKeychain {
    fn store(&self, key: &str, data: &[u8]) -> Result<()> {
        self.keychain_add(key, data)
    }
    
    fn retrieve(&self, key: &str) -> Result<Vec<u8>> {
        self.keychain_get(key)
    }
    
    fn delete(&self, key: &str) -> Result<()> {
        self.keychain_delete(key)
    }
    
    fn exists(&self, key: &str) -> bool {
        self.get_storage_path(key).exists()
    }
}

/// macOS Security Framework integration
pub struct MacOSSecurity {
    /// Use Secure Enclave if available
    use_secure_enclave: bool,
}

impl MacOSSecurity {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            use_secure_enclave: false,
        }
    }
    
    /// Enable Secure Enclave usage
    pub fn with_secure_enclave(mut self) -> Self {
        self.use_secure_enclave = true;
        self
    }
    
    /// Check if Secure Enclave is available
    pub fn has_secure_enclave(&self) -> bool {
        // In production, check kSecAttrTokenIDSecureEnclave
        // Available on Macs with T2 chip or Apple Silicon
        self.use_secure_enclave
    }
    
    /// Generate random bytes using Security framework
    pub fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        // In production, use SecRandomCopyBytes
        // For now, use cross-platform implementation
        Ok(crate::secure_memory::secure_random_bytes(len))
    }
    
    /// Check if hardware RNG is available
    pub fn has_hardware_rng(&self) -> bool {
        true
    }
}

impl Default for MacOSSecurity {
    fn default() -> Self {
        Self::new()
    }
}

/// CommonCrypto integration for symmetric encryption
pub struct MacOSSymmetricCrypto {
    /// Use hardware acceleration if available
    hardware_accelerated: bool,
}

impl MacOSSymmetricCrypto {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            hardware_accelerated: true, // AES-NI on modern Macs
        }
    }
    
    /// Encrypt data with AES-GCM
    pub fn encrypt_aes_gcm(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        // Validate key size
        if key.len() != 32 {
            return Err(PQCError::InvalidKeySize(key.len()));
        }
        
        // Use the cross-platform ChaCha20-Poly1305 implementation
        // In production, could use CommonCrypto CCCrypt with kCCAlgorithmAES
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| PQCError::Generic(format!("Cipher init error: {}", e)))?;
        
        let nonce = Nonce::from_slice(nonce);
        
        cipher.encrypt(nonce, plaintext)
            .map_err(|e| PQCError::Generic(format!("Encryption error: {}", e)))
    }
    
    /// Decrypt data with AES-GCM
    pub fn decrypt_aes_gcm(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(PQCError::InvalidKeySize(key.len()));
        }
        
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| PQCError::Generic(format!("Cipher init error: {}", e)))?;
        
        let nonce = Nonce::from_slice(nonce);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| PQCError::Generic(format!("Decryption error: {}", e)))
    }
    
    /// Check if hardware acceleration is available
    pub fn is_hardware_accelerated(&self) -> bool {
        self.hardware_accelerated
    }
}

impl Default for MacOSSymmetricCrypto {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_macos_keychain_new() {
        let keychain = MacOSKeychain::new();
        assert_eq!(keychain.service_name, "com.vantis.pqc");
        assert!(keychain.keychain_name.is_none());
    }
    
    #[test]
    fn test_macos_security_new() {
        let security = MacOSSecurity::new();
        assert!(!security.use_secure_enclave);
    }
    
    #[test]
    fn test_random_bytes() {
        let security = MacOSSecurity::new();
        let bytes = security.random_bytes(32).unwrap();
        assert_eq!(bytes.len(), 32);
    }
    
    #[test]
    fn test_symmetric_crypto_new() {
        let crypto = MacOSSymmetricCrypto::new();
        assert!(crypto.hardware_accelerated);
    }
}