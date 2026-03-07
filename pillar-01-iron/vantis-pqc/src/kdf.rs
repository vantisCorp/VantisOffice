//! Key Derivation Functions
//!
//! Provides HKDF-based key derivation using SHA3-256.

use crate::error::{PQCError, Result};
use sha3::{Digest, Sha3_256};

/// Derive a key from input key material
pub fn derive_key(ikm: &[u8], info: &[u8], len: usize) -> Result<Vec<u8>> {
    if len > 64 {
        return Err(PQCError::KeyDerivationFailed(
            "Output length too large (max 64)".to_string(),
        ));
    }

    // HKDF-Extract
    let mut prk = Sha3_256::new();
    prk.update(ikm);
    let prk_result = prk.finalize();

    // HKDF-Expand
    let mut okm = Sha3_256::new();
    okm.update(&prk_result);
    okm.update(info);
    okm.update(&[1u8]); // Counter
    let result = okm.finalize();

    Ok(result[..len].to_vec())
}

/// Derive multiple keys from a single input
pub fn derive_keys(ikm: &[u8], info: &[u8], lengths: &[usize]) -> Result<Vec<Vec<u8>>> {
    let mut keys = Vec::with_capacity(lengths.len());

    for (i, &len) in lengths.iter().enumerate() {
        let key_info = format!("{}-{:02}", String::from_utf8_lossy(info), i);
        let key = derive_key(ikm, key_info.as_bytes(), len)?;
        keys.push(key);
    }

    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let ikm = b"input key material";
        let info = b"context info";
        let key = derive_key(ikm, info, 32).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_derive_key_deterministic() {
        let ikm = b"input key material";
        let info = b"context info";
        let key1 = derive_key(ikm, info, 32).unwrap();
        let key2 = derive_key(ikm, info, 32).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_keys() {
        let ikm = b"input key material";
        let info = b"context";
        let keys = derive_keys(ikm, info, &[32, 16, 32]).unwrap();
        assert_eq!(keys.len(), 3);
        assert_eq!(keys[0].len(), 32);
        assert_eq!(keys[1].len(), 16);
        assert_eq!(keys[2].len(), 32);
    }
}