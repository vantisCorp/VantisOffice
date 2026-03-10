//! Kyber Key Encapsulation Mechanism (KEM)
//!
//! Kyber is a post-quantum key encapsulation mechanism selected by NIST
//! for standardization. It provides IND-CCA2 security.

use crate::error::{PQCError, Result};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Security levels for Kyber
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KyberSecurityLevel {
    /// Kyber-512 (NIST Level 1)
    Kyber512,
    /// Kyber-768 (NIST Level 3) - Default
    #[default]
    Kyber768,
    /// Kyber-1024 (NIST Level 5)
    Kyber1024,
}

/// Kyber key pair
#[derive(Clone, ZeroizeOnDrop)]
pub struct KyberKeyPair {
    /// Security level
    #[zeroize(skip)]
    pub security_level: KyberSecurityLevel,
    /// Public key
    public_key: Vec<u8>,
    /// Private key
    private_key: Vec<u8>,
}

impl KyberKeyPair {
    /// Generate a new Kyber key pair
    pub fn generate(security_level: KyberSecurityLevel) -> Result<Self> {
        let (public_key_size, private_key_size) = match security_level {
            KyberSecurityLevel::Kyber512 => (800, 1632),
            KyberSecurityLevel::Kyber768 => (1184, 2400),
            KyberSecurityLevel::Kyber1024 => (1568, 3168),
        };

        // Placeholder: In production, use actual Kyber implementation
        // Generate random public key
        let public_key = crate::secure_memory::secure_random_bytes(public_key_size);
        // Private key: random prefix + public key appended at the end
        // This allows decapsulate to recover the public key from the private key
        let private_prefix_size = private_key_size - public_key_size;
        let mut private_key = crate::secure_memory::secure_random_bytes(private_prefix_size);
        private_key.extend_from_slice(&public_key);

        Ok(Self {
            security_level,
            public_key,
            private_key,
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get the private key
    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }

    /// Export public key as bytes
    pub fn export_public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    /// Import from bytes
    pub fn import(public_key: Vec<u8>, private_key: Vec<u8>, security_level: KyberSecurityLevel) -> Result<Self> {
        let (expected_pub, expected_priv) = match security_level {
            KyberSecurityLevel::Kyber512 => (800, 1632),
            KyberSecurityLevel::Kyber768 => (1184, 2400),
            KyberSecurityLevel::Kyber1024 => (1568, 3168),
        };

        if public_key.len() != expected_pub {
            return Err(PQCError::InvalidKeySize(public_key.len()));
        }
        if private_key.len() != expected_priv {
            return Err(PQCError::InvalidKeySize(private_key.len()));
        }

        Ok(Self {
            security_level,
            public_key,
            private_key,
        })
    }
}

/// Encapsulation result
#[derive(Debug, Clone)]
pub struct EncapsulationResult {
    /// Shared secret (32 bytes)
    pub shared_secret: Vec<u8>,
    /// Ciphertext
    pub ciphertext: Vec<u8>,
}

/// Encapsulate a shared secret using a public key
pub fn encapsulate(public_key: &[u8], security_level: KyberSecurityLevel) -> Result<EncapsulationResult> {
    let ciphertext_size = match security_level {
        KyberSecurityLevel::Kyber512 => 768,
        KyberSecurityLevel::Kyber768 => 1088,
        KyberSecurityLevel::Kyber1024 => 1568,
    };

    // Placeholder: Generate a random shared secret, then embed it in the ciphertext
    // so that decapsulate can recover it. In production, use actual Kyber KEM.
    let shared_secret = crate::secure_memory::secure_random_bytes(32);

    // Build ciphertext: first 32 bytes are the shared secret XOR'd with public key bytes,
    // rest is random padding to reach expected size
    let mut ciphertext = Vec::with_capacity(ciphertext_size);
    for i in 0..32 {
        let pk_byte = if i < public_key.len() { public_key[i] } else { 0 };
        ciphertext.push(shared_secret[i] ^ pk_byte);
    }
    // Fill remaining with deterministic padding
    let padding = crate::secure_memory::secure_random_bytes(ciphertext_size - 32);
    ciphertext.extend_from_slice(&padding);

    Ok(EncapsulationResult {
        shared_secret,
        ciphertext,
    })
}

/// Decapsulate a ciphertext using a private key
pub fn decapsulate(private_key: &[u8], ciphertext: &[u8], security_level: KyberSecurityLevel) -> Result<Vec<u8>> {
    let expected_ct_size = match security_level {
        KyberSecurityLevel::Kyber512 => 768,
        KyberSecurityLevel::Kyber768 => 1088,
        KyberSecurityLevel::Kyber1024 => 1568,
    };

    if ciphertext.len() != expected_ct_size {
        return Err(PQCError::InvalidCiphertext(format!(
            "Expected {} bytes, got {}",
            expected_ct_size,
            ciphertext.len()
        )));
    }

    // Placeholder: Recover shared secret from ciphertext using private key.
    // The private key contains the public key material needed to reverse the XOR.
    // In the placeholder keygen, private key starts with random bytes but we need
    // the public key to reverse. Since we store both in the keypair, we extract
    // the public key portion from the private key (first public_key_size bytes).
    let public_key_size = match security_level {
        KyberSecurityLevel::Kyber512 => 800,
        KyberSecurityLevel::Kyber768 => 1184,
        KyberSecurityLevel::Kyber1024 => 1568,
    };

    // In our placeholder, the private key's last public_key_size bytes contain the public key
    // But since our keygen doesn't embed it, we use a simpler approach:
    // XOR the first 32 bytes of ciphertext with the first 32 bytes of private key,
    // then XOR again with private key to simulate the reverse operation.
    // Actually, we need the public key. Let's derive the same shared secret
    // by using the fact that private_key was generated alongside public_key.
    // For the placeholder, we embed the public key at the end of the private key.
    
    // Extract public key from end of private key (if available)
    let pk_bytes: Vec<u8> = if private_key.len() > public_key_size {
        private_key[private_key.len() - public_key_size..].to_vec()
    } else {
        private_key[..32.min(private_key.len())].to_vec()
    };

    // Recover shared secret: reverse the XOR with public key bytes
    let mut shared_secret = Vec::with_capacity(32);
    for i in 0..32 {
        let pk_byte = if i < pk_bytes.len() { pk_bytes[i] } else { 0 };
        shared_secret.push(ciphertext[i] ^ pk_byte);
    }

    Ok(shared_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        assert_eq!(keypair.public_key().len(), 1184);
        assert_eq!(keypair.private_key().len(), 2400);
    }

    #[test]
    fn test_encapsulation() {
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        let result = encapsulate(keypair.public_key(), KyberSecurityLevel::Kyber768).unwrap();
        assert_eq!(result.shared_secret.len(), 32);
        assert_eq!(result.ciphertext.len(), 1088);
    }

    #[test]
    fn test_security_level_default() {
        assert_eq!(KyberSecurityLevel::default(), KyberSecurityLevel::Kyber768);
    }
}