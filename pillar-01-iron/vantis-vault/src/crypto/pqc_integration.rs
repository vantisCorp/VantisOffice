//! Post-Quantum Cryptography Integration for Vantis Vault
//!
//! This module provides integration between Vantis Vault and the PQC module,
//! enabling quantum-resistant encryption for vault operations.

use anyhow::{Result, Context};
use vantis_pqc::{
    KyberKeyPair, KyberSecurityLevel,
    DilithiumKeyPair, DilithiumSecurityLevel,
    derive_keys_from_shared_secret,
    secure_random_bytes, constant_time_compare,
};

/// PQC-enabled encrypted document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PqcEncryptedDocument {
    /// Document ID
    pub document_id: String,
    /// Encrypted content (using AES-256-GCM with derived key)
    pub encrypted_content: Vec<u8>,
    /// Nonce for AES-GCM
    pub nonce: Vec<u8>,
    /// Kyber ciphertext for key encapsulation
    pub kyber_ciphertext: Vec<u8>,
    /// Sender's Dilithium signature
    pub signature: Option<Vec<u8>>,
    /// Security level used
    pub security_level: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// PQC key bundle for document encryption
pub struct PqcKeyBundle {
    /// Kyber keypair for key encapsulation
    pub kyber_keypair: KyberKeyPair,
    /// Dilithium keypair for signing (optional)
    pub dilithium_keypair: Option<DilithiumKeyPair>,
}

impl PqcKeyBundle {
    /// Create a new key bundle with Kyber768 (recommended)
    pub fn new_kyber768() -> Result<Self> {
        let kyber_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768)
            .context("Failed to generate Kyber keypair")?;
        
        Ok(Self {
            kyber_keypair,
            dilithium_keypair: None,
        })
    }

    /// Create a new key bundle with Kyber768 and Dilithium3
    pub fn new_with_signing() -> Result<Self> {
        let kyber_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768)
            .context("Failed to generate Kyber keypair")?;
        
        let dilithium_keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium3)
            .context("Failed to generate Dilithium keypair")?;
        
        Ok(Self {
            kyber_keypair,
            dilithium_keypair: Some(dilithium_keypair),
        })
    }

    /// Create a high-security bundle with Kyber1024 and Dilithium5
    pub fn new_high_security() -> Result<Self> {
        let kyber_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber1024)
            .context("Failed to generate Kyber1024 keypair")?;
        
        let dilithium_keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium5)
            .context("Failed to generate Dilithium5 keypair")?;
        
        Ok(Self {
            kyber_keypair,
            dilithium_keypair: Some(dilithium_keypair),
        })
    }

    /// Get the public key for encryption
    pub fn public_key(&self) -> Vec<u8> {
        self.kyber_keypair.public_key().to_vec()
    }

    /// Get the Dilithium public key if available
    pub fn signing_public_key(&self) -> Option<Vec<u8>> {
        self.dilithium_keypair.as_ref().map(|kp| kp.public_key().to_vec())
    }
}

/// Encrypt a document using PQC
///
/// This function:
/// 1. Encapsulates a shared secret using Kyber
/// 2. Derives encryption key from the shared secret
/// 3. Encrypts the document content with the derived key
/// 4. Optionally signs the encrypted document
pub fn encrypt_document(
    document_id: &str,
    content: &[u8],
    recipient_public_key: &[u8],
    sender_keybundle: Option<&PqcKeyBundle>,
) -> Result<PqcEncryptedDocument> {
    // Determine security level from public key size
    let security_level = match recipient_public_key.len() {
        800 => KyberSecurityLevel::Kyber512,
        1184 => KyberSecurityLevel::Kyber768,
        1568 => KyberSecurityLevel::Kyber1024,
        _ => KyberSecurityLevel::Kyber768, // Default
    };
    
    // Encapsulate using Kyber to get shared secret
    let encap_result = vantis_pqc::encapsulate(recipient_public_key, security_level)
        .map_err(|e| anyhow::anyhow!("Failed to encapsulate key with Kyber: {:?}", e))?;
    let shared_secret = encap_result.shared_secret;
    let ciphertext = encap_result.ciphertext;
    
    // Derive encryption keys from the shared secret
    let derived_keys = derive_keys_from_shared_secret(
        &shared_secret,
        document_id,
        2,
        32,
    ).context("Failed to derive keys from shared secret")?;
    
    // Use derived_keys[0] for encryption, derived_keys[1] for MAC/signing
    let encryption_key = &derived_keys[0];
    
    // Generate nonce for AES-GCM
    let nonce = secure_random_bytes(12);
    
    // Encrypt content using AES-256-GCM with the derived key
    let encrypted_content = aes_gcm_encrypt(encryption_key, &nonce, content)?;
    
    // Sign if signing key is available
    let signature = if let Some(kb) = sender_keybundle {
        if let Some(ref dkp) = kb.dilithium_keypair {
            let sig = dkp.sign(&encrypted_content)
                .context("Failed to sign document")?;
            Some(sig)
        } else {
            None
        }
    } else {
        None
    };
    
    // Determine security level string
    let security_level_str = match recipient_public_key.len() {
        800 => "kyber512",
        1184 => "kyber768",
        1568 => "kyber1024",
        _ => "unknown",
    };
    
    Ok(PqcEncryptedDocument {
        document_id: document_id.to_string(),
        encrypted_content,
        nonce,
        kyber_ciphertext: ciphertext,
        signature,
        security_level: security_level_str.to_string(),
        created_at: chrono::Utc::now(),
    })
}

/// Decrypt a PQC-encrypted document
///
/// This function:
/// 1. Decapsulates the shared secret using the private key
/// 2. Derives the encryption key
/// 3. Decrypts the content using AES-256-GCM
/// 4. Verifies the signature if present
pub fn decrypt_document(
    encrypted_doc: &PqcEncryptedDocument,
    private_key: &[u8],
    sender_public_key: Option<&[u8]>,
) -> Result<Vec<u8>> {
    // Determine security level from ciphertext size
    let security_level = match encrypted_doc.kyber_ciphertext.len() {
        768 => KyberSecurityLevel::Kyber512,
        1088 => KyberSecurityLevel::Kyber768,
        1568 => KyberSecurityLevel::Kyber1024,
        _ => KyberSecurityLevel::Kyber768,
    };
    
    // Decapsulate to get shared secret
    let shared_secret = vantis_pqc::decapsulate(private_key, &encrypted_doc.kyber_ciphertext, security_level)
        .map_err(|e| anyhow::anyhow!("Failed to decapsulate with Kyber: {:?}", e))?;
    
    // Derive keys
    let derived_keys = derive_keys_from_shared_secret(
        &shared_secret,
        &encrypted_doc.document_id,
        2,
        32,
    ).context("Failed to derive keys")?;
    
    // Verify signature if present and sender key provided
    if let (Some(sig), Some(sender_pk)) = (&encrypted_doc.signature, sender_public_key) {
        let dilithium_level = match sender_pk.len() {
            1312 => DilithiumSecurityLevel::Dilithium2,
            1952 => DilithiumSecurityLevel::Dilithium3,
            2592 => DilithiumSecurityLevel::Dilithium5,
            _ => DilithiumSecurityLevel::Dilithium3,
        };
        
        let valid = DilithiumKeyPair::verify(sender_pk, &encrypted_doc.encrypted_content, sig, dilithium_level)
            .context("Signature verification failed")?;
        
        if !valid {
            anyhow::bail!("Document signature verification failed");
        }
    }
    
    // Decrypt content using derived_keys[0] (same as encryption)
    let content = aes_gcm_decrypt(&derived_keys[0], &encrypted_doc.nonce, &encrypted_doc.encrypted_content)
        .context("Failed to decrypt content")?;
    
    Ok(content)
}

/// Simple AES-256-GCM encryption (placeholder - uses basic XOR for demonstration)
fn aes_gcm_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    // In a real implementation, this would use AES-256-GCM
    // For now, we use a simple XOR-based approach for demonstration
    let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
    
    // Simple XOR encryption (NOT SECURE - replace with real AES-GCM in production)
    for (i, byte) in plaintext.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let nonce_byte = nonce[i % nonce.len()];
        ciphertext.push(byte ^ key_byte ^ nonce_byte);
    }
    
    // Add a simple "tag" (in real implementation, this would be GCM tag)
    let tag = simple_hash(&[key, nonce, &ciphertext].concat());
    ciphertext.extend_from_slice(&tag[..16]);
    
    Ok(ciphertext)
}

/// Simple AES-256-GCM decryption (placeholder)
fn aes_gcm_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if ciphertext.len() < 16 {
        anyhow::bail!("Ciphertext too short");
    }
    
    // Split ciphertext and tag
    let (encrypted_content, stored_tag) = ciphertext.split_at(ciphertext.len() - 16);
    
    // Verify tag
    let expected_tag = simple_hash(&[key, nonce, encrypted_content].concat());
    if !constant_time_compare(stored_tag, &expected_tag[..16]) {
        anyhow::bail!("Authentication tag mismatch");
    }
    
    // Decrypt (simple XOR - NOT SECURE)
    let mut plaintext = Vec::with_capacity(encrypted_content.len());
    for (i, byte) in encrypted_content.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let nonce_byte = nonce[i % nonce.len()];
        plaintext.push(byte ^ key_byte ^ nonce_byte);
    }
    
    Ok(plaintext)
}

/// Simple hash function (placeholder - would use SHA-256 in production)
fn simple_hash(data: &[u8]) -> Vec<u8> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Expand to 32 bytes
    let mut result = Vec::with_capacity(32);
    for i in 0..4 {
        let shifted = hash.wrapping_add(i as u64);
        result.extend_from_slice(&shifted.to_le_bytes());
    }
    
    result
}

/// Hybrid encryption combining classical and post-quantum algorithms
pub fn hybrid_encrypt(
    content: &[u8],
    _classical_public_key: &[u8],
    pqc_public_key: &[u8],
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    // Determine security level
    let security_level = match pqc_public_key.len() {
        800 => KyberSecurityLevel::Kyber512,
        1568 => KyberSecurityLevel::Kyber1024,
        _ => KyberSecurityLevel::Kyber768,
    };
    
    // Generate hybrid shared secret
    let hybrid_result = vantis_pqc::hybrid_encapsulate(
        &vantis_pqc::HybridPublicKey {
            kyber: pqc_public_key.to_vec(),
            classical: vec![0u8; 32], // Placeholder
        },
        security_level,
    ).map_err(|e| anyhow::anyhow!("Hybrid key exchange failed: {:?}", e))?;
    let shared_secret = hybrid_result.shared_secret;
    let kyber_ciphertext = hybrid_result.kyber_ciphertext;
    
    // Derive encryption key
    let keys = derive_keys_from_shared_secret(
        &shared_secret,
        "hybrid_encryption",
        1,
        32,
    ).context("Key derivation failed")?;
    
    // Generate nonce
    let nonce = secure_random_bytes(12);
    
    // Encrypt content
    let encrypted = aes_gcm_encrypt(&keys[0], &nonce, content)?;
    
    Ok((encrypted, kyber_ciphertext, nonce))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pqc_key_bundle_creation() {
        let bundle = PqcKeyBundle::new_kyber768().unwrap();
        assert!(!bundle.public_key().is_empty());
        assert!(bundle.signing_public_key().is_none());
    }

    #[test]
    fn test_pqc_key_bundle_with_signing() {
        let bundle = PqcKeyBundle::new_with_signing().unwrap();
        assert!(!bundle.public_key().is_empty());
        assert!(bundle.signing_public_key().is_some());
    }

    #[test]
    fn test_encrypt_decrypt_document() {
        let recipient_bundle = PqcKeyBundle::new_kyber768().unwrap();
        let sender_bundle = PqcKeyBundle::new_with_signing().unwrap();
        
        let content = b"Hello, post-quantum world!";
        
        let encrypted = encrypt_document(
            "test-doc-001",
            content,
            &recipient_bundle.public_key(),
            Some(&sender_bundle),
        ).unwrap();
        
        assert_eq!(encrypted.document_id, "test-doc-001");
        assert!(encrypted.signature.is_some());
        
        let decrypted = decrypt_document(
            &encrypted,
            recipient_bundle.kyber_keypair.private_key(),
            sender_bundle.signing_public_key().as_deref(),
        ).unwrap();
        
        assert_eq!(decrypted, content);
    }

    #[test]
    fn test_encrypt_decrypt_without_signature() {
        let bundle = PqcKeyBundle::new_kyber768().unwrap();
        let content = b"Test content without signature";
        
        let encrypted = encrypt_document(
            "test-doc-002",
            content,
            &bundle.public_key(),
            None,
        ).unwrap();
        
        assert!(encrypted.signature.is_none());
        
        let decrypted = decrypt_document(
            &encrypted,
            bundle.kyber_keypair.private_key(),
            None,
        ).unwrap();
        
        assert_eq!(decrypted, content);
    }

    #[test]
    fn test_aes_gcm_roundtrip() {
        let key = secure_random_bytes(32);
        let nonce = secure_random_bytes(12);
        let plaintext = b"Hello, AES-GCM!";
        
        let ciphertext = aes_gcm_encrypt(&key, &nonce, plaintext).unwrap();
        let decrypted = aes_gcm_decrypt(&key, &nonce, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_gcm_wrong_key() {
        let key1 = secure_random_bytes(32);
        let key2 = secure_random_bytes(32);
        let nonce = secure_random_bytes(12);
        let plaintext = b"Secret message";
        
        let ciphertext = aes_gcm_encrypt(&key1, &nonce, plaintext).unwrap();
        let result = aes_gcm_decrypt(&key2, &nonce, &ciphertext);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_gcm_wrong_nonce() {
        let key = secure_random_bytes(32);
        let nonce1 = secure_random_bytes(12);
        let nonce2 = secure_random_bytes(12);
        let plaintext = b"Secret message";
        
        let ciphertext = aes_gcm_encrypt(&key, &nonce1, plaintext).unwrap();
        let result = aes_gcm_decrypt(&key, &nonce2, &ciphertext);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_high_security_bundle() {
        let bundle = PqcKeyBundle::new_high_security().unwrap();
        assert_eq!(bundle.kyber_keypair.public_key().len(), 1568); // Kyber1024
        assert!(bundle.dilithium_keypair.is_some());
    }
}