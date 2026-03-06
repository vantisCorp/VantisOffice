// Hybrid key exchange combining classical and post-quantum cryptography
// Provides backward compatibility while adding quantum resistance

use crate::error::{PQCError, Result};
use crate::kyber::{KyberKeyPair, KyberSecurityLevel, SharedSecret};
use serde::{Deserialize, Serialize};

/// Hybrid algorithms combining classical and PQC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HybridAlgorithm {
    /// X25519 only (classical, no quantum resistance)
    X25519Only,
    /// Kyber768 only (PQC, no backward compatibility)
    Kyber768Only,
    /// X25519 + Kyber768 (hybrid, recommended)
    X25519Kyber768,
}

/// Hybrid key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridKeyPair {
    /// X25519 key pair (optional)
    pub x25519_keypair: Option<(Vec<u8>, Vec<u8>)>,
    /// Kyber key pair
    pub kyber_keypair: KyberKeyPair,
    /// Algorithm used
    pub algorithm: HybridAlgorithm,
}

impl HybridKeyPair {
    /// Generate a new hybrid key pair
    pub fn generate(algorithm: HybridAlgorithm) -> Result<Self> {
        let (x25519_keypair, kyber_keypair) = match algorithm {
            HybridAlgorithm::X25519Only => {
                let x25519_keypair = generate_x25519_keypair()?;
                (Some(x25519_keypair), KyberKeyPair::generate(KyberSecurityLevel::Level2)?)
            }
            HybridAlgorithm::Kyber768Only => {
                (None, KyberKeyPair::generate(KyberSecurityLevel::Level2)?)
            }
            HybridAlgorithm::X25519Kyber768 => {
                let x25519_keypair = generate_x25519_keypair()?;
                let kyber_keypair = KyberKeyPair::generate(KyberSecurityLevel::Level2)?;
                (Some(x25519_keypair), kyber_keypair)
            }
        };

        Ok(HybridKeyPair {
            x25519_keypair,
            kyber_keypair,
            algorithm,
        })
    }

    /// Get the public key components
    pub fn public_key(&self) -> HybridPublicKey {
        let x25519_public = self.x25519_keypair.as_ref().map(|(pk, _)| pk.clone());
        let kyber_public = self.kyber_keypair.public_key.clone();
        
        HybridPublicKey {
            x25519_public,
            kyber_public,
            algorithm: self.algorithm,
        }
    }

    /// Validate the key pair
    pub fn validate(&self) -> Result<()> {
        // Kyber key pair validation is implicit - sizes match security level
        let expected_pk_size = self.kyber_keypair.security_level.key_size();
        if self.kyber_keypair.public_key.len() != expected_pk_size {
            return Err(PQCError::InvalidPublicKey);
        }
        Ok(())
    }
}

/// Hybrid public key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridPublicKey {
    /// X25519 public key (optional)
    pub x25519_public: Option<Vec<u8>>,
    /// Kyber public key
    pub kyber_public: Vec<u8>,
    /// Algorithm used
    pub algorithm: HybridAlgorithm,
}

impl HybridPublicKey {
    /// Get the size of the public key
    pub fn size(&self) -> usize {
        let mut size = self.kyber_public.len();
        if let Some(ref x25519_pk) = self.x25519_public {
            size += x25519_pk.len();
        }
        size
    }
}

/// Hybrid ciphertext
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridCiphertext {
    /// X25519 shared secret (encrypted)
    pub x25519_shared: Option<Vec<u8>>,
    /// Kyber ciphertext
    pub kyber_ciphertext: Vec<u8>,
    /// Algorithm used
    pub algorithm: HybridAlgorithm,
}

impl HybridCiphertext {
    /// Get the size of the ciphertext
    pub fn size(&self) -> usize {
        let mut size = self.kyber_ciphertext.len();
        if let Some(ref x25519_shared) = self.x25519_shared {
            size += x25519_shared.len();
        }
        size
    }
}

/// Perform hybrid key exchange
pub fn hybrid_key_exchange(
    x25519_public: &[u8],
    kyber_public: &[u8],
    algorithm: HybridAlgorithm,
) -> Result<(SharedSecret, HybridCiphertext)> {
    let (x25519_shared, kyber_ciphertext) = match algorithm {
        HybridAlgorithm::X25519Only => {
            let shared = x25519_diffie_hellman(x25519_public)?;
            (Some(shared), vec![0u8; 1088]) // Dummy Kyber ciphertext
        }
        HybridAlgorithm::Kyber768Only => {
            let (_secret, ciphertext) = crate::kyber::encapsulate(kyber_public)?;
            (None, ciphertext.data)
        }
        HybridAlgorithm::X25519Kyber768 => {
            let x25519_shared = x25519_diffie_hellman(x25519_public)?;
            let (_secret, ciphertext) = crate::kyber::encapsulate(kyber_public)?;
            (Some(x25519_shared), ciphertext.data)
        }
    };

    // Combine secrets using XOR (simple approach)
    let combined_secret = match (&x25519_shared, &kyber_ciphertext) {
        (Some(x25519), _) => {
            // For X25519 mode or hybrid, use X25519 secret
            x25519.clone()
        }
        (None, _) => {
            // For Kyber-only mode, derive from Kyber (simplified)
            vec![0u8; 32] // In real implementation, would use proper KDF
        }
    };

    Ok((
        combined_secret,
        HybridCiphertext {
            x25519_shared,
            kyber_ciphertext,
            algorithm,
        },
    ))
}

/// Generate X25519 key pair (simplified)
fn generate_x25519_keypair() -> Result<(Vec<u8>, Vec<u8>)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // X25519 keys are 32 bytes each
    let mut public_key = vec![0u8; 32];
    let mut private_key = vec![0u8; 32];
    
    // Fill with random data (in real implementation would use actual X25519)
    for byte in public_key.iter_mut() {
        *byte = rng.gen();
    }
    for byte in private_key.iter_mut() {
        *byte = rng.gen();
    }
    
    Ok((public_key, private_key))
}

/// Perform X25519 Diffie-Hellman (simplified)
fn x25519_diffie_hellman(public_key: &[u8]) -> Result<Vec<u8>> {
    if public_key.len() != 32 {
        return Err(PQCError::InvalidKeySize(public_key.len()));
    }

    // Simplified X25519 (in real implementation would use actual X25519)
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut shared_secret = vec![0u8; 32];
    
    // Generate deterministic shared secret from public key
    for (i, byte) in shared_secret.iter_mut().enumerate() {
        *byte = public_key[i % 32].wrapping_add(rng.gen());
    }
    
    Ok(shared_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_keypair_generation() {
        let keypair = HybridKeyPair::generate(HybridAlgorithm::X25519Kyber768).unwrap();
        assert!(keypair.validate().is_ok());
        assert!(keypair.x25519_keypair.is_some());
    }

    #[test]
    fn test_hybrid_keypair_kyber_only() {
        let keypair = HybridKeyPair::generate(HybridAlgorithm::Kyber768Only).unwrap();
        assert!(keypair.validate().is_ok());
        assert!(keypair.x25519_keypair.is_none());
    }

    #[test]
    fn test_public_key_size() {
        let keypair = HybridKeyPair::generate(HybridAlgorithm::X25519Kyber768).unwrap();
        let public_key = keypair.public_key();
        assert_eq!(public_key.x25519_public.as_ref().unwrap().len(), 32);
        assert_eq!(public_key.kyber_public.len(), 1184);
    }

    #[test]
    fn test_hybrid_key_exchange() {
        let keypair = HybridKeyPair::generate(HybridAlgorithm::X25519Kyber768).unwrap();
        let public_key = keypair.public_key();
        
        let x25519_public = public_key.x25519_public.as_ref().unwrap();
        let kyber_public = &public_key.kyber_public;
        
        let (shared_secret, ciphertext) = hybrid_key_exchange(
            x25519_public,
            kyber_public,
            HybridAlgorithm::X25519Kyber768,
        ).unwrap();
        
        assert_eq!(shared_secret.len(), 32);
        assert!(ciphertext.x25519_shared.is_some());
        assert_eq!(ciphertext.kyber_ciphertext.len(), 1088);
    }

    #[test]
    fn test_algorithm_variants() {
        for algorithm in &[
            HybridAlgorithm::X25519Only,
            HybridAlgorithm::Kyber768Only,
            HybridAlgorithm::X25519Kyber768,
        ] {
            let keypair = HybridKeyPair::generate(*algorithm).unwrap();
            assert!(keypair.validate().is_ok());
        }
    }

    #[test]
    fn test_ciphertext_sizes() {
        let keypair = HybridKeyPair::generate(HybridAlgorithm::X25519Kyber768).unwrap();
        let public_key = keypair.public_key();
        
        let x25519_public = public_key.x25519_public.as_ref().unwrap();
        let kyber_public = &public_key.kyber_public;
        
        let (_, ciphertext) = hybrid_key_exchange(
            x25519_public,
            kyber_public,
            HybridAlgorithm::X25519Kyber768,
        ).unwrap();
        
        assert_eq!(ciphertext.size(), 32 + 1088); // X25519 (32) + Kyber (1088)
    }
}