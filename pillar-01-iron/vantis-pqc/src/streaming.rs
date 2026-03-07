//! Streaming encryption for large files
//!
//! This module provides memory-efficient streaming encryption using ChaCha20-Poly1305
//! for processing large files without loading entire content into memory.

use crate::error::{PQCError, Result};
use std::io::{Read, Write};

/// Default chunk size for streaming (64KB)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// Maximum chunk size (1MB)
pub const MAX_CHUNK_SIZE: usize = 1024 * 1024;

/// Minimum chunk size (1KB)
pub const MIN_CHUNK_SIZE: usize = 1024;

/// Nonce size for ChaCha20-Poly1305
pub const NONCE_SIZE: usize = 12;

/// Authentication tag size
pub const TAG_SIZE: usize = 16;

/// Key size for ChaCha20-Poly1305
pub const KEY_SIZE: usize = 32;

/// Streaming encryptor for large data
pub struct StreamingEncryptor {
    /// Encryption key
    key: [u8; KEY_SIZE],
    /// Base nonce for chunk nonces
    base_nonce: [u8; NONCE_SIZE],
    /// Chunk counter
    chunk_counter: u64,
    /// Chunk size
    chunk_size: usize,
    /// Finalized flag
    finalized: bool,
}

impl StreamingEncryptor {
    /// Create a new streaming encryptor
    pub fn new(key: &[u8], nonce: &[u8], chunk_size: Option<usize>) -> Result<Self> {
        if key.len() != KEY_SIZE {
            return Err(PQCError::InvalidKeySize(key.len()));
        }
        if nonce.len() != NONCE_SIZE {
            return Err(PQCError::InvalidKeyMaterial(format!(
                "Nonce must be {} bytes",
                NONCE_SIZE
            )));
        }

        let chunk = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        if chunk < MIN_CHUNK_SIZE || chunk > MAX_CHUNK_SIZE {
            return Err(PQCError::InvalidKeyMaterial(format!(
                "Chunk size must be between {} and {} bytes",
                MIN_CHUNK_SIZE, MAX_CHUNK_SIZE
            )));
        }

        let mut key_arr = [0u8; KEY_SIZE];
        let mut nonce_arr = [0u8; NONCE_SIZE];
        key_arr.copy_from_slice(key);
        nonce_arr.copy_from_slice(nonce);

        Ok(Self {
            key: key_arr,
            base_nonce: nonce_arr,
            chunk_counter: 0,
            chunk_size: chunk,
            finalized: false,
        })
    }

    /// Encrypt a chunk of data
    pub fn encrypt_chunk(&mut self, plaintext: &[u8], is_final: bool) -> Result<Vec<u8>> {
        if self.finalized {
            return Err(PQCError::EncryptionFailed(
                "Encryptor already finalized".to_string(),
            ));
        }

        if is_final {
            self.finalized = true;
        }

        let nonce = self.get_chunk_nonce();
        let ciphertext = chacha20_poly1305_encrypt(&self.key, &nonce, plaintext)?;
        self.chunk_counter += 1;
        Ok(ciphertext)
    }

    /// Get the current chunk counter
    pub fn chunk_counter(&self) -> u64 {
        self.chunk_counter
    }

    /// Get the chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Generate nonce for current chunk
    fn get_chunk_nonce(&self) -> [u8; NONCE_SIZE] {
        let mut nonce = self.base_nonce;
        let counter_bytes = self.chunk_counter.to_le_bytes();
        
        // XOR counter into nonce
        for (i, byte) in counter_bytes.iter().enumerate() {
            nonce[i % NONCE_SIZE] ^= byte;
        }
        
        nonce
    }
}

/// Streaming decryptor for large data
pub struct StreamingDecryptor {
    /// Decryption key
    key: [u8; KEY_SIZE],
    /// Base nonce for chunk nonces
    base_nonce: [u8; NONCE_SIZE],
    /// Chunk counter
    chunk_counter: u64,
}

impl StreamingDecryptor {
    /// Create a new streaming decryptor
    pub fn new(key: &[u8], nonce: &[u8]) -> Result<Self> {
        if key.len() != KEY_SIZE {
            return Err(PQCError::InvalidKeySize(key.len()));
        }
        if nonce.len() != NONCE_SIZE {
            return Err(PQCError::InvalidKeyMaterial(format!(
                "Nonce must be {} bytes",
                NONCE_SIZE
            )));
        }

        let mut key_arr = [0u8; KEY_SIZE];
        let mut nonce_arr = [0u8; NONCE_SIZE];
        key_arr.copy_from_slice(key);
        nonce_arr.copy_from_slice(nonce);

        Ok(Self {
            key: key_arr,
            base_nonce: nonce_arr,
            chunk_counter: 0,
        })
    }

    /// Decrypt a chunk of data
    pub fn decrypt_chunk(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let nonce = self.get_chunk_nonce();
        let plaintext = chacha20_poly1305_decrypt(&self.key, &nonce, ciphertext)?;
        self.chunk_counter += 1;
        Ok(plaintext)
    }

    /// Get the current chunk counter
    pub fn chunk_counter(&self) -> u64 {
        self.chunk_counter
    }

    /// Generate nonce for current chunk
    fn get_chunk_nonce(&self) -> [u8; NONCE_SIZE] {
        let mut nonce = self.base_nonce;
        let counter_bytes = self.chunk_counter.to_le_bytes();
        
        for (i, byte) in counter_bytes.iter().enumerate() {
            nonce[i % NONCE_SIZE] ^= byte;
        }
        
        nonce
    }
}

/// Streaming header (prepended to encrypted stream)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreamingHeader {
    /// Magic bytes for format identification
    pub magic: [u8; 4],
    /// Format version
    pub version: u8,
    /// Chunk size used
    pub chunk_size: u32,
    /// Original nonce
    pub nonce: [u8; NONCE_SIZE],
    /// Total chunks (0 if unknown)
    pub total_chunks: u64,
}

impl StreamingHeader {
    /// Magic bytes for VantisOffice streaming format
    pub const MAGIC: [u8; 4] = *b"VPQC";
    
    /// Current version
    pub const VERSION: u8 = 1;

    /// Create a new streaming header
    pub fn new(chunk_size: usize, nonce: [u8; NONCE_SIZE], total_chunks: u64) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            chunk_size: chunk_size as u32,
            nonce,
            total_chunks,
        }
    }

    /// Serialize header to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| PQCError::SerializationError(e.to_string()))
    }

    /// Deserialize header from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| PQCError::DeserializationError(e.to_string()))
    }
}

/// ChaCha20-Poly1305 encrypt (placeholder)
fn chacha20_poly1305_encrypt(key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE], plaintext: &[u8]) -> Result<Vec<u8>> {
    // Placeholder: XOR encryption (NOT SECURE - use actual ChaCha20-Poly1305 in production)
    let mut ciphertext = Vec::with_capacity(plaintext.len() + TAG_SIZE);
    
    for (i, byte) in plaintext.iter().enumerate() {
        ciphertext.push(byte ^ key[i % KEY_SIZE] ^ nonce[i % NONCE_SIZE]);
    }
    
    // Add placeholder tag
    let tag = compute_tag(key, nonce, &ciphertext);
    ciphertext.extend_from_slice(&tag);
    
    Ok(ciphertext)
}

/// ChaCha20-Poly1305 decrypt (placeholder)
fn chacha20_poly1305_decrypt(key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if ciphertext.len() < TAG_SIZE {
        return Err(PQCError::DecryptionFailed("Ciphertext too short".to_string()));
    }

    let (encrypted, tag) = ciphertext.split_at(ciphertext.len() - TAG_SIZE);
    
    // Verify tag
    let expected_tag = compute_tag(key, nonce, encrypted);
    if !constant_time_compare(tag, &expected_tag) {
        return Err(PQCError::DecryptionFailed("Authentication failed".to_string()));
    }

    // Placeholder: XOR decryption
    let mut plaintext = Vec::with_capacity(encrypted.len());
    for (i, byte) in encrypted.iter().enumerate() {
        plaintext.push(byte ^ key[i % KEY_SIZE] ^ nonce[i % NONCE_SIZE]);
    }

    Ok(plaintext)
}

/// Compute authentication tag
fn compute_tag(key: &[u8], nonce: &[u8], data: &[u8]) -> [u8; TAG_SIZE] {
    use sha3::{Sha3_256, Digest};
    
    let mut hasher = Sha3_256::new();
    hasher.update(key);
    hasher.update(nonce);
    hasher.update(data);
    
    let hash = hasher.finalize();
    let mut tag = [0u8; TAG_SIZE];
    tag.copy_from_slice(&hash[..TAG_SIZE]);
    tag
}

/// Constant-time comparison
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Encrypt a stream of data
pub fn encrypt_stream<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    key: &[u8],
    chunk_size: Option<usize>,
) -> Result<u64> {
    let nonce = crate::secure_memory::secure_random_bytes(NONCE_SIZE);
    let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
    
    let mut encryptor = StreamingEncryptor::new(key, &nonce, Some(chunk_size))?;
    let mut total_chunks = 0u64;
    let mut buffer = vec![0u8; chunk_size];
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        let encrypted = encryptor.encrypt_chunk(&buffer[..bytes_read], true)?;
        writer.write_all(&(encrypted.len() as u64).to_le_bytes())?;
        writer.write_all(&encrypted)?;
        total_chunks += 1;
    }
    
    Ok(total_chunks)
}

/// Decrypt a stream of data
pub fn decrypt_stream<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    key: &[u8],
) -> Result<u64> {
    let mut decryptor = StreamingDecryptor::new(key, &crate::secure_memory::secure_random_bytes(NONCE_SIZE))?;
    let mut total_chunks = 0u64;
    
    loop {
        let mut len_bytes = [0u8; 8];
        match reader.read_exact(&mut len_bytes) {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        }
        
        let chunk_len = u64::from_le_bytes(len_bytes) as usize;
        let mut chunk = vec![0u8; chunk_len];
        reader.read_exact(&mut chunk)?;
        
        let decrypted = decryptor.decrypt_chunk(&chunk)?;
        writer.write_all(&decrypted)?;
        total_chunks += 1;
    }
    
    Ok(total_chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_encryptor_creation() {
        let key = crate::secure_memory::secure_random_bytes(KEY_SIZE);
        let nonce = crate::secure_memory::secure_random_bytes(NONCE_SIZE);
        let encryptor = StreamingEncryptor::new(&key, &nonce, None).unwrap();
        assert_eq!(encryptor.chunk_size(), DEFAULT_CHUNK_SIZE);
    }

    #[test]
    fn test_streaming_encryptor_invalid_key() {
        let key = vec![0u8; 16]; // Too short
        let nonce = crate::secure_memory::secure_random_bytes(NONCE_SIZE);
        let result = StreamingEncryptor::new(&key, &nonce, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_streaming_header() {
        let nonce = [1u8; NONCE_SIZE];
        let header = StreamingHeader::new(DEFAULT_CHUNK_SIZE, nonce, 10);
        assert_eq!(header.magic, StreamingHeader::MAGIC);
        assert_eq!(header.version, StreamingHeader::VERSION);
        
        let bytes = header.to_bytes().unwrap();
        let decoded = StreamingHeader::from_bytes(&bytes).unwrap();
        assert_eq!(header.nonce, decoded.nonce);
    }

    #[test]
    fn test_encrypt_decrypt_chunk() {
        let key = crate::secure_memory::secure_random_bytes(KEY_SIZE);
        let nonce = crate::secure_memory::secure_random_bytes(NONCE_SIZE);
        
        let mut encryptor = StreamingEncryptor::new(&key, &nonce, Some(MIN_CHUNK_SIZE)).unwrap();
        let plaintext = b"Hello, streaming world!";
        let ciphertext = encryptor.encrypt_chunk(plaintext, true).unwrap();
        
        let mut decryptor = StreamingDecryptor::new(&key, &nonce).unwrap();
        let decrypted = decryptor.decrypt_chunk(&ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_decryption_fails() {
        let key1 = crate::secure_memory::secure_random_bytes(KEY_SIZE);
        let key2 = crate::secure_memory::secure_random_bytes(KEY_SIZE);
        let nonce = crate::secure_memory::secure_random_bytes(NONCE_SIZE);
        
        let mut encryptor = StreamingEncryptor::new(&key1, &nonce, None).unwrap();
        let plaintext = b"Secret message";
        let ciphertext = encryptor.encrypt_chunk(plaintext, true).unwrap();
        
        let mut decryptor = StreamingDecryptor::new(&key2, &nonce).unwrap();
        let result = decryptor.decrypt_chunk(&ciphertext);
        assert!(result.is_err());
    }
}