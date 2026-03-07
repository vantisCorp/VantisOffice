//! Key Derivation Functions for Post-Quantum Cryptography
//!
//! This module provides secure key derivation functions designed to work with
//! PQC key material. These KDFs are essential for:
//! - Deriving multiple keys from a single shared secret
//! - Password-based key derivation for key storage
//! - Key stretching for enhanced security
//!
//! # Security Considerations
//!
//! All KDFs in this module are designed to be quantum-resistant by using
//! hash functions with sufficient output size (at least 256 bits).

use crate::error::{PQCError, Result};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Hash algorithm configuration for KDF operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 (256-bit output)
    Sha256,
    /// SHA-512 (512-bit output)
    Sha512,
    /// BLAKE3 (256-bit output, recommended for PQC)
    Blake3,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Blake3
    }
}

impl HashAlgorithm {
    /// Get the output size in bytes for this hash algorithm
    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha512 => 64,
            HashAlgorithm::Blake3 => 32,
        }
    }

    /// Get the block size in bytes for this hash algorithm
    pub fn block_size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => 64,
            HashAlgorithm::Sha512 => 128,
            HashAlgorithm::Blake3 => 64,
        }
    }
}

/// HKDF (HMAC-based Extract-and-Expand Key Derivation Function)
///
/// Implements RFC 5869 HKDF with support for multiple hash algorithms.
/// This is the recommended KDF for deriving keys from PQC shared secrets.
#[derive(Debug, Clone)]
pub struct Hkdf {
    /// Hash algorithm to use
    hash: HashAlgorithm,
    /// Pseudorandom key (extracted from input key material)
    prk: Option<Vec<u8>>,
}

impl Zeroize for Hkdf {
    fn zeroize(&mut self) {
        if let Some(ref mut prk) = self.prk {
            prk.zeroize();
        }
    }
}

impl Drop for Hkdf {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Hkdf {
    /// Create a new HKDF instance with the specified hash algorithm
    pub fn new(hash: HashAlgorithm) -> Self {
        Self {
            hash,
            prk: None,
        }
    }

    /// Extract phase: Derive a pseudorandom key from input key material
    ///
    /// # Arguments
    /// * `ikm` - Input key material (e.g., PQC shared secret)
    /// * `salt` - Optional salt value (recommended for security)
    ///
    /// # Returns
    /// The HKDF instance with extracted PRK, ready for expansion
    pub fn extract(&mut self, ikm: &[u8], salt: Option<&[u8]>) -> Result<()> {
        if ikm.is_empty() {
            return Err(PQCError::InvalidKeyMaterial("Input key material cannot be empty".to_string()));
        }

        let salt_bytes = salt.unwrap_or(&[0u8; 32][..]);
        
        // Use HMAC-based extraction
        let prk = self.hmac(salt_bytes, ikm)?;
        self.prk = Some(prk);
        
        Ok(())
    }

    /// Expand phase: Derive one or more keys from the PRK
    ///
    /// # Arguments
    /// * `info` - Optional context and application specific information
    /// * `length` - Desired output length in bytes (max 255 * hash_output_size)
    ///
    /// # Returns
    /// Derived key material of the specified length
    pub fn expand(&self, info: Option<&[u8]>, length: usize) -> Result<Vec<u8>> {
        let prk = self.prk.as_ref().ok_or_else(|| {
            PQCError::InvalidKeyMaterial("HKDF not initialized - call extract first".to_string())
        })?;

        let hash_len = self.hash.output_size();
        let max_length = 255 * hash_len;
        
        if length > max_length {
            return Err(PQCError::InvalidKeyMaterial(format!(
                "Requested length {} exceeds maximum {}",
                length, max_length
            )));
        }

        let info_bytes = info.unwrap_or(&[]);
        let mut output = Vec::with_capacity(length);
        let mut t = Vec::new();
        let mut counter: u8 = 1;

        while output.len() < length {
            // T(N) = HMAC(PRK, T(N-1) | info | counter)
            let mut input = t.clone();
            input.extend_from_slice(info_bytes);
            input.push(counter);
            
            t = self.hmac(prk, &input)?;
            output.extend_from_slice(&t);
            
            counter = counter.checked_add(1).ok_or_else(|| {
                PQCError::InvalidKeyMaterial("HKDF expand counter overflow".to_string())
            })?;
        }

        output.truncate(length);
        Ok(output)
    }

    /// One-shot HKDF: Extract and expand in a single call
    ///
    /// This is a convenience method for the common case where you need
    /// to derive key material from input key material in one operation.
    pub fn derive(
        hash: HashAlgorithm,
        ikm: &[u8],
        salt: Option<&[u8]>,
        info: Option<&[u8]>,
        length: usize,
    ) -> Result<Vec<u8>> {
        let mut hkdf = Self::new(hash);
        hkdf.extract(ikm, salt)?;
        hkdf.expand(info, length)
    }

    /// HMAC implementation
    fn hmac(&self, key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
        let block_size = self.hash.block_size();
        
        // Prepare key
        let mut key_padded = vec![0u8; block_size];
        if key.len() > block_size {
            key_padded = self.hash_bytes(key)?;
        } else {
            key_padded[..key.len()].copy_from_slice(key);
        }
        
        // Inner hash: H(key ^ 0x36 || message)
        let mut inner = vec![0x36; block_size];
        for (i, k) in key_padded.iter().enumerate() {
            inner[i] ^= k;
        }
        inner.extend_from_slice(message);
        let inner_hash = self.hash_bytes(&inner)?;
        
        // Outer hash: H(key ^ 0x5c || inner_hash)
        let mut outer = vec![0x5c; block_size];
        for (i, k) in key_padded.iter().enumerate() {
            outer[i] ^= k;
        }
        outer.extend_from_slice(&inner_hash);
        
        self.hash_bytes(&outer)
    }

    /// Hash bytes using the configured algorithm
    fn hash_bytes(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.hash {
            HashAlgorithm::Sha256 => {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha512 => {
                use sha2::{Sha512, Digest};
                let mut hasher = Sha512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Blake3 => {
                Ok(blake3::hash(data).as_bytes().to_vec())
            }
        }
    }
}

/// PBKDF2 (Password-Based Key Derivation Function 2)
///
/// Implements RFC 8018 PBKDF2 for password-based key derivation.
/// Suitable for deriving encryption keys from user passwords.
#[derive(Debug, Clone)]
pub struct Pbkdf2 {
    /// Hash algorithm to use
    hash: HashAlgorithm,
    /// Number of iterations (minimum 600,000 for SHA-256 as of 2023)
    iterations: u32,
}

impl Pbkdf2 {
    /// Minimum recommended iterations for SHA-256
    pub const MIN_ITERATIONS: u32 = 600_000;
    
    /// Default iterations for PQC applications
    pub const DEFAULT_ITERATIONS: u32 = 1_000_000;

    /// Create a new PBKDF2 instance with the specified hash algorithm
    pub fn new(hash: HashAlgorithm, iterations: u32) -> Self {
        let iterations = iterations.max(Self::MIN_ITERATIONS);
        Self { hash, iterations }
    }

    /// Derive key material from a password
    ///
    /// # Arguments
    /// * `password` - User password
    /// * `salt` - Random salt value (at least 16 bytes recommended)
    /// * `length` - Desired output length in bytes
    ///
    /// # Returns
    /// Derived key material
    pub fn derive(&self, password: &[u8], salt: &[u8], length: usize) -> Result<Vec<u8>> {
        if password.is_empty() {
            return Err(PQCError::InvalidKeyMaterial("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 16 {
            return Err(PQCError::InvalidKeyMaterial("Salt must be at least 16 bytes".to_string()));
        }

        let mut output = vec![0u8; length];
        
        match self.hash {
            HashAlgorithm::Sha256 => {
                use sha2::Sha256;
                pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, self.iterations, &mut output);
            }
            HashAlgorithm::Sha512 => {
                use sha2::Sha512;
                pbkdf2::pbkdf2_hmac::<Sha512>(password, salt, self.iterations, &mut output);
            }
            HashAlgorithm::Blake3 => {
                // BLAKE3 doesn't have direct PBKDF2 support, use SHA-256 fallback
                use sha2::Sha256;
                pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, self.iterations, &mut output);
            }
        }

        Ok(output)
    }

    /// Generate a random salt for PBKDF2
    pub fn generate_salt() -> Vec<u8> {
        use rand::RngCore;
        let mut salt = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }
}

/// Argon2 - Memory-hard password hashing function
///
/// Implements Argon2id (recommended variant) for password-based key derivation.
/// Argon2 is the winner of the Password Hashing Competition (2015) and provides
/// resistance against GPU and ASIC attacks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Config {
    /// Memory cost in KiB (minimum 15 MiB recommended)
    pub memory_cost: u32,
    /// Time cost (number of passes, minimum 2)
    pub time_cost: u32,
    /// Parallelism (number of threads)
    pub parallelism: u32,
    /// Output length in bytes
    pub output_length: usize,
}

impl Default for Argon2Config {
    fn default() -> Self {
        Self {
            memory_cost: 64 * 1024, // 64 MiB
            time_cost: 3,
            parallelism: 4,
            output_length: 32,
        }
    }
}

impl Argon2Config {
    /// Create a new Argon2 configuration for PQC applications
    pub fn for_pqc() -> Self {
        Self {
            memory_cost: 128 * 1024, // 128 MiB for enhanced security
            time_cost: 4,
            parallelism: 4,
            output_length: 64, // 512 bits for PQC keys
        }
    }
}

/// Argon2id password-based key derivation
pub struct Argon2Kdf {
    config: Argon2Config,
}

impl Argon2Kdf {
    /// Create a new Argon2 KDF with the specified configuration
    pub fn new(config: Argon2Config) -> Result<Self> {
        if config.memory_cost < 8 * 1024 {
            return Err(PQCError::InvalidKeyMaterial(
                "Memory cost must be at least 8 MiB".to_string()
            ));
        }
        
        if config.time_cost < 1 {
            return Err(PQCError::InvalidKeyMaterial(
                "Time cost must be at least 1".to_string()
            ));
        }
        
        Ok(Self { config })
    }

    /// Derive key material from a password
    ///
    /// # Arguments
    /// * `password` - User password
    /// * `salt` - Random salt value (16 bytes required)
    ///
    /// # Returns
    /// Derived key material
    pub fn derive(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        if password.is_empty() {
            return Err(PQCError::InvalidKeyMaterial("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 16 {
            return Err(PQCError::InvalidKeyMaterial(
                "Salt must be at least 16 bytes".to_string()
            ));
        }

        let mut output = vec![0u8; self.config.output_length];
        
        argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(
                self.config.memory_cost,
                self.config.time_cost,
                self.config.parallelism,
                Some(self.config.output_length),
            ).map_err(|e| PQCError::KeyDerivationFailed(e.to_string()))?,
        )
        .hash_password_into(password, salt, &mut output)
        .map_err(|e| PQCError::KeyDerivationFailed(e.to_string()))?;

        Ok(output)
    }

    /// Generate a random salt for Argon2
    pub fn generate_salt() -> Vec<u8> {
        use rand::RngCore;
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }
}

/// Derived key container with metadata
#[derive(Debug, Clone, ZeroizeOnDrop)]
pub struct DerivedKey {
    /// The derived key material
    pub key: Vec<u8>,
    /// Algorithm used for derivation
    pub algorithm: String,
    /// Salt used (if applicable)
    pub salt: Vec<u8>,
    /// Iterations (for PBKDF2)
    pub iterations: Option<u32>,
}

impl DerivedKey {
    /// Create a new derived key container
    pub fn new(key: Vec<u8>, algorithm: String, salt: Vec<u8>) -> Self {
        Self {
            key,
            algorithm,
            salt,
            iterations: None,
        }
    }

    /// Split the derived key into multiple keys
    ///
    /// This is useful when you need multiple keys (e.g., encryption key, MAC key)
    /// from a single key derivation operation.
    pub fn split(&self, lengths: &[usize]) -> Result<Vec<Vec<u8>>> {
        let total: usize = lengths.iter().sum();
        
        if total > self.key.len() {
            return Err(PQCError::InvalidKeyMaterial(format!(
                "Requested total length {} exceeds key length {}",
                total, self.key.len()
            )));
        }

        let mut keys = Vec::new();
        let mut offset = 0;
        
        for &len in lengths {
            keys.push(self.key[offset..offset + len].to_vec());
            offset += len;
        }
        
        Ok(keys)
    }
}

/// Convenience function to derive keys from a PQC shared secret
///
/// This function uses HKDF-BLAKE3 by default, which is recommended for PQC applications.
///
/// # Arguments
/// * `shared_secret` - The shared secret from a PQC key exchange
/// * `context` - Application-specific context string
/// * `key_count` - Number of keys to derive
/// * `key_length` - Length of each key in bytes
///
/// # Returns
/// Vector of derived keys
pub fn derive_keys_from_shared_secret(
    shared_secret: &[u8],
    context: &str,
    key_count: usize,
    key_length: usize,
) -> Result<Vec<Vec<u8>>> {
    if shared_secret.is_empty() {
        return Err(PQCError::InvalidKeyMaterial("Shared secret cannot be empty".to_string()));
    }

    let total_length = key_count * key_length;
    let info = context.as_bytes();
    
    // Use a deterministic derivation by using the context as part of the salt
    // This ensures the same shared secret + context always produces the same keys
    // For additional security in production, consider including a stored salt
    let salt = blake3::hash(context.as_bytes());
    let salt_bytes = salt.as_bytes();
    
    let derived = Hkdf::derive(HashAlgorithm::Blake3, shared_secret, Some(salt_bytes), Some(info), total_length)?;
    
    let mut keys = Vec::new();
    for i in 0..key_count {
        let start = i * key_length;
        let end = start + key_length;
        keys.push(derived[start..end].to_vec());
    }
    
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm_output_sizes() {
        assert_eq!(HashAlgorithm::Sha256.output_size(), 32);
        assert_eq!(HashAlgorithm::Sha512.output_size(), 64);
        assert_eq!(HashAlgorithm::Blake3.output_size(), 32);
    }

    #[test]
    fn test_hkdf_extract_expand() {
        let mut hkdf = Hkdf::new(HashAlgorithm::Sha256);
        let ikm = b"input key material";
        let salt = b"salt value";
        
        hkdf.extract(ikm, Some(salt.as_ref())).unwrap();
        
        let info = b"application context";
        let derived = hkdf.expand(Some(info.as_ref()), 32).unwrap();
        
        assert_eq!(derived.len(), 32);
    }

    #[test]
    fn test_hkdf_derive() {
        let ikm = b"input key material for testing";
        let salt = b"salt";
        let info = b"test context";
        
        let derived = Hkdf::derive(
            HashAlgorithm::Sha256,
            ikm,
            Some(salt.as_ref()),
            Some(info.as_ref()),
            64,
        ).unwrap();
        
        assert_eq!(derived.len(), 64);
        
        // Same inputs should produce same output
        let derived2 = Hkdf::derive(
            HashAlgorithm::Sha256,
            ikm,
            Some(salt.as_ref()),
            Some(info.as_ref()),
            64,
        ).unwrap();
        
        assert_eq!(derived, derived2);
    }

    #[test]
    fn test_hkdf_different_salts() {
        let ikm = b"input key material";
        let info = b"context";
        
        let derived1 = Hkdf::derive(HashAlgorithm::Sha256, ikm, Some(b"salt1"), Some(info), 32).unwrap();
        let derived2 = Hkdf::derive(HashAlgorithm::Sha256, ikm, Some(b"salt2"), Some(info), 32).unwrap();
        
        assert_ne!(derived1, derived2, "Different salts should produce different outputs");
    }

    #[test]
    fn test_pbkdf2_derive() {
        let pbkdf2 = Pbkdf2::new(HashAlgorithm::Sha256, 600_000);
        let password = b"secure password";
        let salt = Pbkdf2::generate_salt();
        
        let derived = pbkdf2.derive(password, &salt, 32).unwrap();
        
        assert_eq!(derived.len(), 32);
        
        // Same password and salt should produce same key
        let derived2 = pbkdf2.derive(password, &salt, 32).unwrap();
        assert_eq!(derived, derived2);
    }

    #[test]
    fn test_pbkdf2_salt_length() {
        let pbkdf2 = Pbkdf2::new(HashAlgorithm::Sha256, 600_000);
        let password = b"password";
        let short_salt = b"short";
        
        let result = pbkdf2.derive(password, short_salt, 32);
        assert!(result.is_err(), "Should reject short salt");
    }

    #[test]
    fn test_argon2_config_default() {
        let config = Argon2Config::default();
        assert!(config.memory_cost >= 8 * 1024);
        assert!(config.time_cost >= 1);
        assert_eq!(config.output_length, 32);
    }

    #[test]
    fn test_argon2_config_pqc() {
        let config = Argon2Config::for_pqc();
        assert_eq!(config.memory_cost, 128 * 1024);
        assert_eq!(config.time_cost, 4);
        assert_eq!(config.output_length, 64);
    }

    #[test]
    fn test_argon2_derive() {
        let config = Argon2Config::default();
        let argon2 = Argon2Kdf::new(config).unwrap();
        let password = b"secure password";
        let salt = Argon2Kdf::generate_salt();
        
        let derived = argon2.derive(password, &salt).unwrap();
        
        assert_eq!(derived.len(), 32);
    }

    #[test]
    fn test_derived_key_split() {
        let key = DerivedKey::new(
            vec![1u8; 64],
            "test".to_string(),
            vec![0u8; 16],
        );
        
        let split = key.split(&[16, 16, 32]).unwrap();
        
        assert_eq!(split.len(), 3);
        assert_eq!(split[0].len(), 16);
        assert_eq!(split[1].len(), 16);
        assert_eq!(split[2].len(), 32);
    }

    #[test]
    fn test_derive_keys_from_shared_secret() {
        let shared_secret = vec![42u8; 32];
        let keys = derive_keys_from_shared_secret(
            &shared_secret,
            "test_context",
            3,
            32,
        ).unwrap();
        
        assert_eq!(keys.len(), 3);
        assert_eq!(keys[0].len(), 32);
        assert_eq!(keys[1].len(), 32);
        assert_eq!(keys[2].len(), 32);
        
        // Each key should be different
        assert_ne!(keys[0], keys[1]);
        assert_ne!(keys[1], keys[2]);
    }

    #[test]
    fn test_hkdf_empty_ikm() {
        let mut hkdf = Hkdf::new(HashAlgorithm::Sha256);
        let result = hkdf.extract(&[], None);
        assert!(result.is_err());
    }

    #[test]
    fn test_hkdf_expand_without_extract() {
        let hkdf = Hkdf::new(HashAlgorithm::Sha256);
        let result = hkdf.expand(None, 32);
        assert!(result.is_err());
    }

    #[test]
    fn test_pbkdf2_minimum_iterations() {
        let pbkdf2 = Pbkdf2::new(HashAlgorithm::Sha256, 100);
        assert!(pbkdf2.iterations >= Pbkdf2::MIN_ITERATIONS);
    }
}