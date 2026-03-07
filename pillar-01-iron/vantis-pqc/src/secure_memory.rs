//! Secure memory handling utilities
//!
//! Provides functions for secure memory allocation and zeroization.

use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroize;

/// Generate cryptographically secure random bytes
pub fn secure_random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Securely zero memory
pub fn secure_zero(data: &mut [u8]) {
    data.zeroize();
}

/// Constant-time comparison
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Secure memory container
#[derive(Zeroize)]
pub struct SecureBytes(Vec<u8>);

impl SecureBytes {
    /// Create new secure bytes
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Generate random secure bytes
    pub fn random(len: usize) -> Self {
        Self(secure_random_bytes(len))
    }

    /// Get reference to inner bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Drop for SecureBytes {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let bytes1 = secure_random_bytes(32);
        let bytes2 = secure_random_bytes(32);
        assert_eq!(bytes1.len(), 32);
        assert_ne!(bytes1, bytes2); // Very unlikely to be equal
    }

    #[test]
    fn test_constant_time_compare() {
        let a = b"hello world";
        let b = b"hello world";
        let c = b"hello earth";

        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
        assert!(!constant_time_compare(a, b"short"));
    }

    #[test]
    fn test_secure_bytes() {
        let bytes = SecureBytes::random(32);
        assert_eq!(bytes.len(), 32);
        assert!(!bytes.is_empty());
    }
}