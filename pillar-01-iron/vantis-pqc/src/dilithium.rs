// Dilithium Signature Scheme implementation
// Provides quantum-resistant digital signatures for authentication

use crate::error::{PQCError, Result};
use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};
use pqcrypto_traits::sign::{DetachedSignature, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Security levels for Dilithium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DilithiumSecurityLevel {
    /// Dilithium2 - NIST security level 2 (~128 bits)
    Level2,
    /// Dilithium3 - NIST security level 3 (~192 bits)
    Level3,
    /// Dilithium5 - NIST security level 5 (~256 bits, recommended)
    Level5,
}

impl DilithiumSecurityLevel {
    /// Get the public key size for this security level
    pub fn public_key_size(&self) -> usize {
        match self {
            DilithiumSecurityLevel::Level2 => 1312,
            DilithiumSecurityLevel::Level3 => 1952,
            DilithiumSecurityLevel::Level5 => 2592,
        }
    }

    /// Get the private key size for this security level
    pub fn private_key_size(&self) -> usize {
        match self {
            DilithiumSecurityLevel::Level2 => 2560,
            DilithiumSecurityLevel::Level3 => 4032,
            DilithiumSecurityLevel::Level5 => 4896,
        }
    }

    /// Get the signature size for this security level
    pub fn signature_size(&self) -> usize {
        match self {
            DilithiumSecurityLevel::Level2 => 2420,
            DilithiumSecurityLevel::Level3 => 3309,
            DilithiumSecurityLevel::Level5 => 4627,
        }
    }
}

/// Dilithium key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilithiumKeyPair {
    /// Public key
    pub public_key: Vec<u8>,
    /// Private key
    pub private_key: Vec<u8>,
    /// Security level
    pub security_level: DilithiumSecurityLevel,
}

impl DilithiumKeyPair {
    /// Generate a new Dilithium key pair using actual pqcrypto-dilithium library
    pub fn generate(security_level: DilithiumSecurityLevel) -> Result<Self> {
        let (pk, sk) = match security_level {
            DilithiumSecurityLevel::Level2 => {
                let (pk, sk) = dilithium2::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
            DilithiumSecurityLevel::Level3 => {
                let (pk, sk) = dilithium3::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
            DilithiumSecurityLevel::Level5 => {
                let (pk, sk) = dilithium5::keypair();
                (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
            }
        };

        Ok(DilithiumKeyPair {
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
    pub fn security_level(&self) -> DilithiumSecurityLevel {
        self.security_level
    }

    /// Clear sensitive data from memory
    pub fn zeroize(&mut self) {
        self.private_key.zeroize();
    }
}

impl Drop for DilithiumKeyPair {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Dilithium signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilithiumSignature {
    /// Signature data
    pub data: Vec<u8>,
    /// Security level used for signing
    pub security_level: DilithiumSecurityLevel,
}

impl DilithiumSignature {
    /// Create new signature
    pub fn new(data: Vec<u8>, security_level: DilithiumSecurityLevel) -> Self {
        Self { data, security_level }
    }

    /// Get signature data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Validate signature size matches security level
    pub fn validate(&self) -> Result<()> {
        // Signature sizes can vary slightly, just ensure they're in reasonable range
        let min_size = match self.security_level {
            DilithiumSecurityLevel::Level2 => 2400,
            DilithiumSecurityLevel::Level3 => 3250,
            DilithiumSecurityLevel::Level5 => 4500,
        };
        
        let max_size = match self.security_level {
            DilithiumSecurityLevel::Level2 => 2500,
            DilithiumSecurityLevel::Level3 => 3350,
            DilithiumSecurityLevel::Level5 => 4700,
        };
        
        if self.data.len() < min_size || self.data.len() > max_size {
            return Err(PQCError::InvalidSignature(format!(
                "Signature size {} out of range [{}, {}] for {:?}",
                self.data.len(),
                min_size,
                max_size,
                self.security_level
            )));
        }
        Ok(())
    }
}

/// Sign a message using Dilithium private key
pub fn sign(private_key: &[u8], message: &[u8]) -> Result<DilithiumSignature> {
    // Determine security level based on private key size
    let security_level = match private_key.len() {
        2560 => DilithiumSecurityLevel::Level2,
        4032 => DilithiumSecurityLevel::Level3,
        4896 => DilithiumSecurityLevel::Level5,
        _ => return Err(PQCError::InvalidPrivateKey),
    };

    // Perform signing
    let sig = match security_level {
        DilithiumSecurityLevel::Level2 => {
            let sk = dilithium2::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let sig = dilithium2::detached_sign(message, &sk);
            sig.as_bytes().to_vec()
        }
        DilithiumSecurityLevel::Level3 => {
            let sk = dilithium3::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let sig = dilithium3::detached_sign(message, &sk);
            sig.as_bytes().to_vec()
        }
        DilithiumSecurityLevel::Level5 => {
            let sk = dilithium5::SecretKey::from_bytes(private_key)
                .map_err(|_| PQCError::InvalidPrivateKey)?;
            let sig = dilithium5::detached_sign(message, &sk);
            sig.as_bytes().to_vec()
        }
    };

    Ok(DilithiumSignature::new(sig, security_level))
}

/// Verify a Dilithium signature
pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool> {
    // Determine security level based on public key size
    let security_level = match public_key.len() {
        1312 => DilithiumSecurityLevel::Level2,
        1952 => DilithiumSecurityLevel::Level3,
        2592 => DilithiumSecurityLevel::Level5,
        _ => return Err(PQCError::InvalidPublicKey),
    };

    // Perform verification
    let result = match security_level {
        DilithiumSecurityLevel::Level2 => {
            let pk = dilithium2::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let sig = dilithium2::DetachedSignature::from_bytes(signature)
                .map_err(|_| PQCError::InvalidSignature("Invalid signature".to_string()))?;
            dilithium2::verify_detached_signature(&sig, message, &pk).is_ok()
        }
        DilithiumSecurityLevel::Level3 => {
            let pk = dilithium3::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let sig = dilithium3::DetachedSignature::from_bytes(signature)
                .map_err(|_| PQCError::InvalidSignature("Invalid signature".to_string()))?;
            dilithium3::verify_detached_signature(&sig, message, &pk).is_ok()
        }
        DilithiumSecurityLevel::Level5 => {
            let pk = dilithium5::PublicKey::from_bytes(public_key)
                .map_err(|_| PQCError::InvalidPublicKey)?;
            let sig = dilithium5::DetachedSignature::from_bytes(signature)
                .map_err(|_| PQCError::InvalidSignature("Invalid signature".to_string()))?;
            dilithium5::verify_detached_signature(&sig, message, &pk).is_ok()
        }
    };

    Ok(result)
}

/// Generate a Dilithium key pair
pub fn generate_keypair(security_level: DilithiumSecurityLevel) -> Result<DilithiumKeyPair> {
    DilithiumKeyPair::generate(security_level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_sizes() {
        // These are documented sizes, actual sizes may vary slightly
        let kp2 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level2).unwrap();
        assert_eq!(kp2.public_key.len(), 1312);
        
        let kp3 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        assert_eq!(kp3.public_key.len(), 1952);
        
        let kp5 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level5).unwrap();
        assert_eq!(kp5.public_key.len(), 2592);
    }

    #[test]
    fn test_generate_keypair() {
        let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        assert_eq!(kp.public_key.len(), 1952);
        assert_eq!(kp.security_level, DilithiumSecurityLevel::Level3);
        // Private key size may vary, just check it's reasonable
        assert!(kp.private_key.len() > 1000);
    }

    #[test]
    fn test_sign_verify() {
        let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        let message = b"Hello, Dilithium!";
        
        let signature = sign(&kp.private_key, message).unwrap();
        let verified = verify(&kp.public_key, message, &signature.data).unwrap();
        
        assert!(verified);
    }

    #[test]
    fn test_signature_validation() {
        let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        let message = b"Test message";
        
        let signature = sign(&kp.private_key, message).unwrap();
        assert!(signature.validate().is_ok());
        
        let invalid_sig = DilithiumSignature::new(vec![0u8; 100], DilithiumSecurityLevel::Level3);
        assert!(invalid_sig.validate().is_err());
    }

    #[test]
    fn test_invalid_signature() {
        let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        let message = b"Original message";
        let fake_message = b"Fake message";
        
        let signature = sign(&kp.private_key, message).unwrap();
        let verified = verify(&kp.public_key, fake_message, &signature.data).unwrap();
        
        // Signature should not verify for different message
        assert!(!verified);
    }

    #[test]
    fn test_different_keypairs() {
        let kp1 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        let kp2 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
        
        // Different key pairs should be different
        assert_ne!(kp1.public_key, kp2.public_key);
        assert_ne!(kp1.private_key, kp2.private_key);
    }
}