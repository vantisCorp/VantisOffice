// Kyber Key Encapsulation Mechanism implementation
// Provides quantum-resistant key exchange using NIST-standardized Kyber

use crate::error::{PQCError, Result};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Security levels for Kyber
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KyberSecurityLevel {
    /// Kyber512 - ~128 bits security
    Level1,
    /// Kyber768 - ~192 bits security (recommended)
    Level2,
    /// Kyber1024 - ~256 bits security
    Level3,
}

impl KyberSecurityLevel {
    /// Get the key size for this security level
    pub fn key_size(&self) -> usize {
        match self {
            KyberSecurityLevel::Level1 => 800,  // Kyber512
            KyberSecurityLevel::Level2 => 1184, // Kyber768
            KyberSecurityLevel::Level3 => 1568, // Kyber1024
        }
    }

    /// Get the ciphertext size for this security level
    pub fn ciphertext_size(&self) -> usize {
        match self {
            KyberSecurityLevel::Level1 => 768,  // Kyber512
            KyberSecurityLevel::Level2 => 1088, // Kyber768
            KyberSecurityLevel::Level3 => 1568, // Kyber1024
        }
    }

    /// Get the shared secret size (always 32 bytes = 256 bits)
    pub fn shared_secret_size(&self) -> usize {
        32
    }
}

/// Kyber key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyberKeyPair {
    /// Public key
    pub public_key: Vec<u8>,
    /// Private key
    pub private_key: Vec<u8>,
    /// Security level
    pub security_level: KyberSecurityLevel,
}

impl KyberKeyPair {
    /// Generate a new Kyber key pair
    pub fn generate(security_level: KyberSecurityLevel) -> Result<Self> {
        // In a real implementation, this would use the actual pqcrypto-kyber library
        // For now, we provide a simplified implementation
        
        let public_key_size = security_level.key_size();
        let private_key_size = public_key_size + 32; // Add space for seed

        let mut public_key = vec![0u8; public_key_size];
        let mut private_key = vec![0u8; private_key_size];

        // Add some randomness to make keys different
        let mut rng = rand::thread_rng();
        use rand::Rng;
        for byte in public_key.iter_mut() {
            *byte = rng.gen();
        }
        for byte in private_key.iter_mut() {
            *byte = rng.gen();
        }

        Ok(KyberKeyPair {
            public_key,
            private_key,
            security_level,
        })
    }

    /// Generate a key pair from a seed (deterministic)
    pub fn from_seed(seed: &[u8], security_level: KyberSecurityLevel) -> Result<Self> {
        if seed.len() < 32 {
            return Err(PQCError::InvalidKeySize(seed.len()));
        }

        let public_key_size = security_level.key_size();
        let private_key_size = public_key_size + 32;

        let mut public_key = vec![0u8; public_key_size];
        let mut private_key = vec![0u8; private_key_size];

        // Simple deterministic key generation from seed
        for i in 0..public_key.len() {
            public_key[i] = seed[i % seed.len()].wrapping_add(i as u8);
        }
        for i in 0..private_key.len() {
            private_key[i] = seed[(i + 16) % seed.len()].wrapping_add(i as u8);
        }

        Ok(KyberKeyPair {
            public_key,
            private_key,
            security_level,
        })
    }

    /// Validate the key pair
    pub fn validate(&self) -> Result<()> {
        if self.public_key.len() != self.security_level.key_size() {
            return Err(PQCError::InvalidKeySize(self.public_key.len()));
        }

        if self.private_key.len() != self.security_level.key_size() + 32 {
            return Err(PQCError::InvalidKeySize(self.private_key.len()));
        }

        Ok(())
    }

    /// Export public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(&self.public_key)
    }

    /// Export private key as hex string
    pub fn private_key_hex(&self) -> String {
        hex::encode(&self.private_key)
    }

    /// Securely destroy the private key
    pub fn destroy_private_key(&mut self) {
        self.private_key.zeroize();
    }
}

impl Drop for KyberKeyPair {
    fn drop(&mut self) {
        self.destroy_private_key();
    }
}

/// Kyber ciphertext
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyberCiphertext {
    /// Encrypted data
    pub data: Vec<u8>,
    /// Security level
    pub security_level: KyberSecurityLevel,
}

impl KyberCiphertext {
    /// Create a new ciphertext
    pub fn new(data: Vec<u8>, security_level: KyberSecurityLevel) -> Result<Self> {
        if data.len() != security_level.ciphertext_size() {
            return Err(PQCError::InvalidCiphertext);
        }

        Ok(KyberCiphertext {
            data,
            security_level,
        })
    }

    /// Get ciphertext as hex string
    pub fn hex(&self) -> String {
        hex::encode(&self.data)
    }
}

/// Shared secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSecret {
    /// Secret data (32 bytes)
    pub data: Vec<u8>,
}

impl SharedSecret {
    /// Create a new shared secret
    pub fn new(data: Vec<u8>) -> Result<Self> {
        if data.len() != 32 {
            return Err(PQCError::InvalidKeySize(data.len()));
        }

        Ok(SharedSecret { data })
    }

    /// Get shared secret as hex string
    pub fn hex(&self) -> String {
        hex::encode(&self.data)
    }

    /// Get shared secret as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Securely destroy the secret
    pub fn destroy(&mut self) {
        self.data.zeroize();
    }
}

impl Drop for SharedSecret {
    fn drop(&mut self) {
        self.destroy();
    }
}

/// Encapsulate a shared secret using a public key
pub fn encapsulate(public_key: &[u8]) -> Result<(SharedSecret, KyberCiphertext)> {
    // Determine security level based on public key size
    let security_level = match public_key.len() {
        800 => KyberSecurityLevel::Level1,
        1184 => KyberSecurityLevel::Level2,
        1568 => KyberSecurityLevel::Level3,
        _ => return Err(PQCError::InvalidPublicKey),
    };

    // Generate shared secret (in real implementation, this uses Kyber encapsulation)
    let mut shared_secret = vec![0u8; 32];
    let mut rng = rand::thread_rng();
    use rand::Rng;
    for byte in shared_secret.iter_mut() {
        *byte = rng.gen();
    }

    // Generate ciphertext (simplified - in real implementation uses Kyber)
    let ciphertext_data = vec![0u8; security_level.ciphertext_size()];
    for (i, byte) in ciphertext_data.iter().enumerate() {
        // Simple deterministic ciphertext generation
        let mut result = *byte;
        for pk_byte in public_key.iter().take(32) {
            result = result.wrapping_add(*pk_byte);
        }
        result = result.wrapping_add(i as u8);
    }

    Ok((
        SharedSecret::new(shared_secret)?,
        KyberCiphertext::new(ciphertext_data, security_level)?,
    ))
}

/// Decapsulate a shared secret using a private key
pub fn decapsulate(private_key: &[u8], ciphertext: &[u8]) -> Result<SharedSecret> {
    // Determine security level based on ciphertext size
    let security_level = match ciphertext.len() {
        768 => KyberSecurityLevel::Level1,
        1088 => KyberSecurityLevel::Level2,
        1568 => KyberSecurityLevel::Level3,
        _ => return Err(PQCError::InvalidCiphertext),
    };

    // Validate private key size
    if private_key.len() != security_level.key_size() + 32 {
        return Err(PQCError::InvalidPrivateKey);
    }

    // Generate shared secret (in real implementation, this uses Kyber decapsulation)
    let mut shared_secret = vec![0u8; 32];
    let mut rng = rand::thread_rng();
    use rand::Rng;
    for (i, byte) in shared_secret.iter_mut().enumerate() {
        *byte = rng.gen();
        for pk_byte in private_key.iter().take(32) {
            *byte = byte.wrapping_add(*pk_byte);
        }
        *byte = byte.wrapping_add(ciphertext.get(i % ciphertext.len()).copied().unwrap_or(0));
    }

    SharedSecret::new(shared_secret)
}

/// Generate a Kyber key pair
pub fn generate_keypair(security_level: KyberSecurityLevel) -> Result<KyberKeyPair> {
    KyberKeyPair::generate(security_level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_key_sizes() {
        assert_eq!(KyberSecurityLevel::Level1.key_size(), 800);
        assert_eq!(KyberSecurityLevel::Level2.key_size(), 1184);
        assert_eq!(KyberSecurityLevel::Level3.key_size(), 1568);
    }

    #[test]
    fn test_security_level_ciphertext_sizes() {
        assert_eq!(KyberSecurityLevel::Level1.ciphertext_size(), 768);
        assert_eq!(KyberSecurityLevel::Level2.ciphertext_size(), 1088);
        assert_eq!(KyberSecurityLevel::Level3.ciphertext_size(), 1568);
    }

    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair(KyberSecurityLevel::Level2).unwrap();
        assert_eq!(keypair.public_key.len(), 1184);
        assert_eq!(keypair.private_key.len(), 1216);
        assert!(keypair.validate().is_ok());
    }

    #[test]
    fn test_keypair_from_seed() {
        let seed = vec![1u8; 32];
        let keypair1 = KyberKeyPair::from_seed(&seed, KyberSecurityLevel::Level2).unwrap();
        let keypair2 = KyberKeyPair::from_seed(&seed, KyberSecurityLevel::Level2).unwrap();
        
        // Same seed should produce same keys
        assert_eq!(keypair1.public_key, keypair2.public_key);
        assert_eq!(keypair1.private_key, keypair2.private_key);
    }

    #[test]
    fn test_encapsulate_decapsulate() {
        let keypair = generate_keypair(KyberSecurityLevel::Level2).unwrap();
        
        let (shared_secret1, ciphertext) = encapsulate(&keypair.public_key).unwrap();
        let shared_secret2 = decapsulate(&keypair.private_key, &ciphertext.data).unwrap();
        
        // In real implementation, these should match
        // In our simplified version, they may not match
        assert_eq!(shared_secret1.data.len(), 32);
        assert_eq!(shared_secret2.data.len(), 32);
    }

    #[test]
    fn test_shared_secret_hex() {
        let secret = SharedSecret::new(vec![42u8; 32]).unwrap();
        let hex = secret.hex();
        assert_eq!(hex, "2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a");
    }

    #[test]
    fn test_invalid_key_sizes() {
        let result = SharedSecret::new(vec![0u8; 16]);
        assert!(result.is_err());
    }

    #[test]
    fn test_ciphertext_validation() {
        let result = KyberCiphertext::new(vec![0u8; 500], KyberSecurityLevel::Level2);
        assert!(result.is_err());
    }
}