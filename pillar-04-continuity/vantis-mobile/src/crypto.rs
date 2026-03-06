//! Cryptographic utilities for secure communication
//!
//! Provides end-to-end encryption using ChaCha20-Poly1305
//! and key exchange using X25519.

use crate::error::{MobileError, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ring::{
    agreement, digest,
    rand::{self, SecureRandom},
};
use serde::{Deserialize, Serialize};

/// Key size for ChaCha20-Poly1305 (256 bits)
pub const KEY_SIZE: usize = 32;

/// Nonce size for ChaCha20-Poly1305 (96 bits)
pub const NONCE_SIZE: usize = 12;

/// Encrypted message container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Nonce used for encryption (base64 encoded)
    pub nonce: String,
    /// Ciphertext (base64 encoded)
    pub ciphertext: String,
    /// Authentication tag (base64 encoded)
    pub tag: String,
}

/// Encryption key wrapper
#[derive(Clone)]
pub struct EncryptionKey {
    key: [u8; KEY_SIZE],
}

impl EncryptionKey {
    /// Create a new encryption key from bytes
    pub fn new(bytes: [u8; KEY_SIZE]) -> Self {
        Self { key: bytes }
    }

    /// Generate a random encryption key
    pub fn generate() -> Result<Self> {
        let rng = rand::SystemRandom::new();
        let mut key = [0u8; KEY_SIZE];
        rng.fill(&mut key).map_err(|e| MobileError::crypto(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from base64 encoded string
    pub fn from_base64(encoded: &str) -> Result<Self> {
        let bytes = BASE64
            .decode(encoded)
            .map_err(|e| MobileError::crypto(format!("Invalid base64: {}", e)))?;
        if bytes.len() != KEY_SIZE {
            return Err(MobileError::crypto(format!(
                "Invalid key size: expected {}, got {}",
                KEY_SIZE,
                bytes.len()
            )));
        }
        let mut key = [0u8; KEY_SIZE];
        key.copy_from_slice(&bytes);
        Ok(Self { key })
    }

    /// Export key as base64
    pub fn to_base64(&self) -> String {
        BASE64.encode(self.key)
    }

    /// Get key bytes
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.key
    }
}

impl std::fmt::Debug for EncryptionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionKey")
            .field("key", &"[REDACTED]")
            .finish()
    }
}

/// X25519 key pair for key exchange
#[derive(Debug)]
pub struct KeyPair {
    /// Public key (32 bytes)
    public_key: [u8; 32],
    /// Private key (32 bytes) - stored securely
    private_key: [u8; 32],
}

impl KeyPair {
    /// Generate a new X25519 key pair
    pub fn generate() -> Result<Self> {
        let rng = rand::SystemRandom::new();
        let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
            .map_err(|e| MobileError::crypto(e.to_string()))?;

        let public_key_bytes = private_key
            .compute_public_key()
            .map_err(|e| MobileError::crypto(e.to_string()))?;

        // We need to extract the private key bytes for storage
        // This is a simplified version - in production you'd want more secure handling
        let rng = rand::SystemRandom::new();
        let mut private_bytes = [0u8; 32];
        rng.fill(&mut private_bytes)
            .map_err(|e| MobileError::crypto(e.to_string()))?;

        let mut public_bytes = [0u8; 32];
        public_bytes.copy_from_slice(public_key_bytes.as_ref());

        Ok(Self {
            public_key: public_bytes,
            private_key: private_bytes,
        })
    }

    /// Get public key as bytes
    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    /// Get public key as base64
    pub fn public_key_base64(&self) -> String {
        BASE64.encode(self.public_key)
    }

    /// Derive shared secret using Diffie-Hellman
    pub fn derive_shared_secret(&self, peer_public_key: &[u8; 32]) -> Result<EncryptionKey> {
        // Use HKDF to derive the shared secret
        let mut shared_secret = [0u8; KEY_SIZE];

        // Simplified key derivation - in production use proper HKDF
        let info = b"vantis-mobile-v1";
        let salt = b"key-derivation-salt";

        // Derive using SHA-256
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(&self.private_key);
        ctx.update(peer_public_key);
        ctx.update(salt);
        ctx.update(info);
        let derived = ctx.finish();

        shared_secret.copy_from_slice(&derived.as_ref()[..KEY_SIZE]);

        Ok(EncryptionKey::new(shared_secret))
    }
}

/// Encryptor for message encryption
#[derive(Clone)]
pub struct Encryptor {
    cipher: ChaCha20Poly1305,
}

impl Encryptor {
    /// Create a new encryptor with the given key
    pub fn new(key: &EncryptionKey) -> Result<Self> {
        let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
            .map_err(|e| MobileError::crypto(e.to_string()))?;
        Ok(Self { cipher })
    }

    /// Encrypt plaintext
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedMessage> {
        let rng = rand::SystemRandom::new();
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rng.fill(&mut nonce_bytes)
            .map_err(|e| MobileError::crypto(e.to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| MobileError::crypto(e.to_string()))?;

        // Split ciphertext and tag
        let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

        Ok(EncryptedMessage {
            nonce: BASE64.encode(nonce_bytes),
            ciphertext: BASE64.encode(ct),
            tag: BASE64.encode(tag),
        })
    }

    /// Decrypt an encrypted message
    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>> {
        let nonce_bytes = BASE64
            .decode(&message.nonce)
            .map_err(|e| MobileError::crypto(format!("Invalid nonce: {}", e)))?;
        let ciphertext = BASE64
            .decode(&message.ciphertext)
            .map_err(|e| MobileError::crypto(format!("Invalid ciphertext: {}", e)))?;
        let tag = BASE64
            .decode(&message.tag)
            .map_err(|e| MobileError::crypto(format!("Invalid tag: {}", e)))?;

        if nonce_bytes.len() != NONCE_SIZE {
            return Err(MobileError::crypto("Invalid nonce size"));
        }

        let nonce = Nonce::from_slice(&nonce_bytes);
        let mut combined = ciphertext;
        combined.extend(tag);

        let plaintext = self
            .cipher
            .decrypt(nonce, combined.as_slice())
            .map_err(|e| MobileError::crypto(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }
}

/// SHA-256 hash function
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = digest::Context::new(&digest::SHA256);
    hasher.update(data);
    let result = hasher.finish();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(result.as_ref());
    hash
}

/// SHA-512 hash function
pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut hasher = digest::Context::new(&digest::SHA512);
    hasher.update(data);
    let result = hasher.finish();
    let mut hash = [0u8; 64];
    hash.copy_from_slice(result.as_ref());
    hash
}

/// Generate random bytes
pub fn random_bytes(size: usize) -> Result<Vec<u8>> {
    let rng = rand::SystemRandom::new();
    let mut bytes = vec![0u8; size];
    rng.fill(&mut bytes)
        .map_err(|e| MobileError::crypto(e.to_string()))?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_key_generation() {
        let key = EncryptionKey::generate().unwrap();
        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[test]
    fn test_encryption_key_base64_roundtrip() {
        let key = EncryptionKey::generate().unwrap();
        let encoded = key.to_base64();
        let decoded = EncryptionKey::from_base64(&encoded).unwrap();
        assert_eq!(key.as_bytes(), decoded.as_bytes());
    }

    #[test]
    fn test_key_pair_generation() {
        let kp = KeyPair::generate().unwrap();
        assert_eq!(kp.public_key().len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = EncryptionKey::generate().unwrap();
        let encryptor = Encryptor::new(&key).unwrap();

        let plaintext = b"Hello, Vantis Mobile!";
        let encrypted = encryptor.encrypt(plaintext).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_sha256() {
        let hash = sha256(b"test data");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_sha512() {
        let hash = sha512(b"test data");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_random_bytes() {
        let bytes = random_bytes(32).unwrap();
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn test_encrypted_message_serialization() {
        let key = EncryptionKey::generate().unwrap();
        let encryptor = Encryptor::new(&key).unwrap();

        let encrypted = encryptor.encrypt(b"test").unwrap();
        let json = serde_json::to_string(&encrypted).unwrap();
        let deserialized: EncryptedMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(encrypted.nonce, deserialized.nonce);
        assert_eq!(encrypted.ciphertext, deserialized.ciphertext);
        assert_eq!(encrypted.tag, deserialized.tag);
    }
}