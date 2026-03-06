// Kyber Key Encapsulation Mechanism implementation
// Provides quantum-resistant key exchange using NIST-standardized Kyber

use crate::error::{PQCError, Result};
use pqcrypto_kyber::{kyber1024, kyber512, kyber768};
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey};
use pqcrypto_traits::kem::SharedSecret as PQSharedSecret;
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
    /// Generate a new Kyber key pair using actual pqcrypto-kyber library
    pub fn generate(security_level: KyberSecurityLevel) -> Result<Self> {
        let (pk, sk) = match security_level {
            KyberSecurityLevel::Level1 => {
                let (pk, sk) = kyber512::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
            KyberSecurityLevel::Level2 => {
                let (pk, sk) = kyber768::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
            KyberSecurityLevel::Level3 => {
                let (pk, sk) = kyber1024::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
        };

        Ok(KyberKeyPair {
            public_key: pk,
            private_key: sk,
            security_level,
        })
    }

    /// Get public key reference
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get private key reference
    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }

    /// Get security level
    pub fn security_level(&self) -> KyberSecurityLevel {
        self.security_level
    }

    /// Clear sensitive data from memory
    pub fn zeroize(&mut self) {
        self.private_key.zeroize();
    }
}

impl Drop for KyberKeyPair {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Kyber ciphertext
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyberCiphertext {
    /// Encapsulated secret data
    pub data: Vec<u8>,
    /// Security level used for encryption
    pub security_level: KyberSecurityLevel,
}

impl KyberCiphertext {
    /// Create new ciphertext
    pub fn new(data: Vec<u8>, security_level: KyberSecurityLevel) -> Self {
        Self { data, security_level }
    }

    /// Get ciphertext data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Validate ciphertext size matches security level
    pub fn validate(&self) -> Result<()> {
        let expected_size = self.security_level.ciphertext_size();
        if self.data.len() != expected_size {
            return Err(PQCError::InvalidCiphertext(format!(
                "Expected {} bytes, got {}",
                expected_size,
                self.data.len()
            )));
        }
        Ok(())
    }
}

/// Shared secret from key encapsulation
pub type SharedSecret = Vec<u8>;

/// Encapsulate a shared secret using public key
pub fn encapsulate(public_key: &[u8]) -> Result<(SharedSecret, KyberCiphertext)> {
    // Determine security level based on public key size
    let security_level = match public_key.len() {
        800 => KyberSecurityLevel::Level1,
        1184 => KyberSecurityLevel::Level2,
        1568 => KyberSecurityLevel::Level3,
        _ => return Err(PQCError::InvalidPublicKey),
    };

    let (ss, ct) = match security_level {
        KyberSecurityLevel::Level1 => {
            let pk = kyber512::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let (ss, ct) = kyber512::encapsulate(&pk);
            (ss.as_bytes().to_vec(), ct.as_bytes().to_vec())
        }
        KyberSecurityLevel::Level2 => {
            let pk = kyber768::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let (ss, ct) = kyber768::encapsulate(&pk);
            (ss.as_bytes().to_vec(), ct.as_bytes().to_vec())
        }
        KyberSecurityLevel::Level3 => {
            let pk = kyber1024::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let (ss, ct) = kyber1024::encapsulate(&pk);
            (ss.as_bytes().to_vec(), ct.as_bytes().to_vec())
        }
    };

    Ok((ss, KyberCiphertext::new(ct, security_level)))
}

/// Decapsulate shared secret using private key
pub fn decapsulate(private_key: &[u8], ciphertext: &[u8]) -> Result<SharedSecret> {
    // Determine security level based on private key size
    let security_level = match private_key.len() {
        1632 => KyberSecurityLevel::Level1, // 800 + 832
        2400 => KyberSecurityLevel::Level2, // 1184 + 1216
        3168 => KyberSecurityLevel::Level3, // 1568 + 1600
        _ => return Err(PQCError::InvalidPrivateKey),
    };

    let ss = match security_level {
        KyberSecurityLevel::Level1 => {
            let sk = kyber512::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let ct = kyber512::Ciphertext::from_bytes(ciphertext)
                .map_err(|_| PQCError::InvalidCiphertext("Invalid ciphertext".to_string()))?;
            kyber512::decapsulate(&ct, &sk).as_bytes().to_vec()
        }
        KyberSecurityLevel::Level2 => {
            let sk = kyber768::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let ct = kyber768::Ciphertext::from_bytes(ciphertext)
                .map_err(|_| PQCError::InvalidCiphertext("Invalid ciphertext".to_string()))?;
            kyber768::decapsulate(&ct, &sk).as_bytes().to_vec()
        }
        KyberSecurityLevel::Level3 => {
            let sk = kyber1024::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let ct = kyber1024::Ciphertext::from_bytes(ciphertext)
                .map_err(|_| PQCError::InvalidCiphertext("Invalid ciphertext".to_string()))?;
            kyber1024::decapsulate(&ct, &sk).as_bytes().to_vec()
        }
    };

    Ok(ss)
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
        let kp = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        assert_eq!(kp.public_key.len(), 1184);
        assert_eq!(kp.private_key.len(), 2400);
        assert_eq!(kp.security_level, KyberSecurityLevel::Level2);
    }

    #[test]
    fn test_encapsulate_decapsulate() {
        let kp = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        let (ss1, ct) = encapsulate(&kp.public_key).unwrap();
        let ss2 = decapsulate(&kp.private_key, &ct.data).unwrap();
        
        assert_eq!(ss1.len(), 32);
        assert_eq!(ss2.len(), 32);
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn test_ciphertext_validation() {
        let kp = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        let (_, ct) = encapsulate(&kp.public_key).unwrap();
        
        assert!(ct.validate().is_ok());
        
        let invalid_ct = KyberCiphertext::new(vec![0u8; 100], KyberSecurityLevel::Level2);
        assert!(invalid_ct.validate().is_err());
    }

    #[test]
    fn test_invalid_key_sizes() {
        let invalid_pk = vec![0u8; 100];
        let result = encapsulate(&invalid_pk);
        assert!(result.is_err());

        let invalid_sk = vec![0u8; 100];
        let ct = vec![0u8; 1088];
        let result = decapsulate(&invalid_sk, &ct);
        assert!(result.is_err());
    }

    #[test]
    fn test_keypair_from_seed() {
        let kp1 = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        let kp2 = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        
        // Different key pairs should be different
        assert_ne!(kp1.public_key, kp2.public_key);
        assert_ne!(kp1.private_key, kp2.private_key);
    }

    #[test]
    fn test_shared_secret_hex() {
        let kp = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
        let (ss, _) = encapsulate(&kp.public_key).unwrap();
        
        assert_eq!(ss.len(), 32);
    }
}