//! Property-based tests for vantis-pqc module
//!
//! Uses proptest to verify cryptographic invariants and edge cases.

use proptest::prelude::*;
use vantis_pqc::{
    KyberKeyPair, KyberSecurityLevel,
    DilithiumKeyPair, DilithiumSecurityLevel,
    StreamingEncryptor, StreamingDecryptor,
};
use vantis_pqc::streaming::NONCE_SIZE;

// ============================================================================
// Property Strategies
// ============================================================================

/// Strategy for generating arbitrary byte vectors
fn arb_bytes(max_size: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(any::<u8>(), 0..max_size)
}

/// Strategy for generating non-empty byte vectors
fn arb_non_empty_bytes(min_size: usize, max_size: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(any::<u8>(), min_size..max_size)
}

/// Strategy for Kyber security levels
fn arb_kyber_level() -> impl Strategy<Value = KyberSecurityLevel> {
    prop_oneof![
        Just(KyberSecurityLevel::Kyber512),
        Just(KyberSecurityLevel::Kyber768),
        Just(KyberSecurityLevel::Kyber1024),
    ]
}

/// Strategy for Dilithium security levels
fn arb_dilithium_level() -> impl Strategy<Value = DilithiumSecurityLevel> {
    prop_oneof![
        Just(DilithiumSecurityLevel::Dilithium2),
        Just(DilithiumSecurityLevel::Dilithium3),
        Just(DilithiumSecurityLevel::Dilithium5),
    ]
}

// ============================================================================
// Key Generation Properties
// ============================================================================

proptest! {
    /// Test that key generation always produces keys of correct size
    #[test]
    fn prop_kyber_key_size_correct(level in arb_kyber_level()) {
        let keypair = KyberKeyPair::generate(level).unwrap();
        
        let (expected_pk, expected_sk) = match level {
            KyberSecurityLevel::Kyber512 => (800, 1632),
            KyberSecurityLevel::Kyber768 => (1184, 2400),
            KyberSecurityLevel::Kyber1024 => (1568, 3168),
        };
        
        prop_assert_eq!(keypair.public_key().len(), expected_pk);
        prop_assert_eq!(keypair.private_key().len(), expected_sk);
    }
    
    /// Test that different key generations produce different keys
    #[test]
    fn prop_kyber_keys_are_unique(level in arb_kyber_level()) {
        let keypair1 = KyberKeyPair::generate(level).unwrap();
        let keypair2 = KyberKeyPair::generate(level).unwrap();
        
        prop_assert_ne!(keypair1.public_key(), keypair2.public_key());
        prop_assert_ne!(keypair1.private_key(), keypair2.private_key());
    }
    
    /// Test that Dilithium key generation produces correct sizes
    #[test]
    fn prop_dilithium_key_size_correct(level in arb_dilithium_level()) {
        let keypair = DilithiumKeyPair::generate(level).unwrap();
        
        // ML-DSA standard key sizes (per FIPS 204)
        let (expected_pk, expected_sk) = match level {
            DilithiumSecurityLevel::Dilithium2 => (1312, 2528),
            DilithiumSecurityLevel::Dilithium3 => (1952, 4000),
            DilithiumSecurityLevel::Dilithium5 => (2592, 4864),
        };
        
        prop_assert_eq!(keypair.public_key().len(), expected_pk);
        prop_assert_eq!(keypair.private_key().len(), expected_sk);
    }
    
    /// Test that Dilithium keys are unique
    #[test]
    fn prop_dilithium_keys_are_unique(level in arb_dilithium_level()) {
        let keypair1 = DilithiumKeyPair::generate(level).unwrap();
        let keypair2 = DilithiumKeyPair::generate(level).unwrap();
        
        prop_assert_ne!(keypair1.public_key(), keypair2.public_key());
        prop_assert_ne!(keypair1.private_key(), keypair2.private_key());
    }
}

// ============================================================================
// Encapsulation Properties
// ============================================================================

proptest! {
    /// Test that encapsulation produces correct output sizes
    #[test]
    fn prop_encapsulation_sizes(level in arb_kyber_level()) {
        let keypair = KyberKeyPair::generate(level).unwrap();
        let result = vantis_pqc::kyber::encapsulate(keypair.public_key(), level).unwrap();
        
        let expected_ct = match level {
            KyberSecurityLevel::Kyber512 => 768,
            KyberSecurityLevel::Kyber768 => 1088,
            KyberSecurityLevel::Kyber1024 => 1568,
        };
        
        prop_assert_eq!(result.shared_secret.len(), 32);
        prop_assert_eq!(result.ciphertext.len(), expected_ct);
    }
    
    /// Test that decapsulation rejects wrong ciphertext sizes
    #[test]
    fn prop_decapsulation_rejects_wrong_size(
        level in arb_kyber_level(),
        wrong_size in 1usize..2000
    ) {
        let keypair = KyberKeyPair::generate(level).unwrap();
        
        // Skip if wrong_size happens to match expected size
        let expected_ct = match level {
            KyberSecurityLevel::Kyber512 => 768,
            KyberSecurityLevel::Kyber768 => 1088,
            KyberSecurityLevel::Kyber1024 => 1568,
        };
        prop_assume!(wrong_size != expected_ct);
        
        let wrong_ciphertext = vec![0u8; wrong_size];
        let result = vantis_pqc::kyber::decapsulate(
            keypair.private_key(),
            &wrong_ciphertext,
            level
        );
        
        prop_assert!(result.is_err());
    }
}

// ============================================================================
// Signature Properties
// ============================================================================

proptest! {
    /// Test that signature size is correct for each security level
    #[test]
    fn prop_signature_size_correct(
        level in arb_dilithium_level(),
        message in arb_non_empty_bytes(1, 1024)
    ) {
        let keypair = DilithiumKeyPair::generate(level).unwrap();
        let signature = keypair.sign(&message).unwrap();
        
        let expected_sig_size = match level {
            DilithiumSecurityLevel::Dilithium2 => 2420,
            DilithiumSecurityLevel::Dilithium3 => 3293,
            DilithiumSecurityLevel::Dilithium5 => 4595,
        };
        
        prop_assert_eq!(signature.len(), expected_sig_size);
    }
    
    /// Test that different messages produce different signatures
    #[test]
    fn prop_signatures_differ_for_different_messages(
        message1 in arb_non_empty_bytes(32, 128),
        message2 in arb_non_empty_bytes(32, 128)
    ) {
        prop_assume!(message1 != message2);
        
        let keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium3).unwrap();
        let sig1 = keypair.sign(&message1).unwrap();
        let sig2 = keypair.sign(&message2).unwrap();
        
        prop_assert_ne!(sig1, sig2);
    }
    
    /// Test that verification accepts valid signatures
    #[test]
    fn prop_verification_accepts_valid(
        level in arb_dilithium_level(),
        message in arb_non_empty_bytes(1, 1024)
    ) {
        let keypair = DilithiumKeyPair::generate(level).unwrap();
        let signature = keypair.sign(&message).unwrap();
        
        let valid = DilithiumKeyPair::verify(
            keypair.public_key(),
            &message,
            &signature,
            level
        ).unwrap();
        
        prop_assert!(valid);
    }
}

// ============================================================================
// Streaming Encryption Properties
// ============================================================================

proptest! {
    /// Test streaming encryption with various data sizes
    #[test]
    fn prop_streaming_encrypt_decrypt_roundtrip(
        data in arb_bytes(1024 * 100) // Up to 100KB
    ) {
        let key = vantis_pqc::secure_memory::secure_random_bytes(32);
        let nonce = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
        
        let mut encryptor = StreamingEncryptor::new(&key, &nonce, Some(64 * 1024)).unwrap();
        let encrypted = encryptor.encrypt_chunk(&data, true).unwrap();
        
        let mut decryptor = StreamingDecryptor::new(&key, &nonce).unwrap();
        let decrypted = decryptor.decrypt_chunk(&encrypted).unwrap();
        
        prop_assert_eq!(data, decrypted);
    }
    
    /// Test that encryption produces larger output (due to auth tag)
    #[test]
    fn prop_encryption_adds_overhead(
        data in arb_non_empty_bytes(1, 1024 * 10)
    ) {
        let key = vantis_pqc::secure_memory::secure_random_bytes(32);
        let nonce = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
        
        let mut encryptor = StreamingEncryptor::new(&key, &nonce, None).unwrap();
        let encrypted = encryptor.encrypt_chunk(&data, true).unwrap();
        
        // ChaCha20-Poly1305 adds 16-byte auth tag
        prop_assert!(encrypted.len() > data.len());
        prop_assert_eq!(encrypted.len(), data.len() + 16);
    }
    
    /// Test that wrong key fails decryption
    #[test]
    fn prop_wrong_key_fails_decryption(
        data in arb_non_empty_bytes(32, 1024)
    ) {
        let key1 = vantis_pqc::secure_memory::secure_random_bytes(32);
        let key2 = vantis_pqc::secure_memory::secure_random_bytes(32);
        prop_assume!(key1 != key2);
        
        let nonce = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
        
        let mut encryptor = StreamingEncryptor::new(&key1, &nonce, None).unwrap();
        let encrypted = encryptor.encrypt_chunk(&data, true).unwrap();
        
        let mut decryptor = StreamingDecryptor::new(&key2, &nonce).unwrap();
        let result = decryptor.decrypt_chunk(&encrypted);
        
        prop_assert!(result.is_err());
    }
    
    /// Test that wrong nonce fails decryption
    #[test]
    fn prop_wrong_nonce_fails_decryption(
        data in arb_non_empty_bytes(32, 1024)
    ) {
        let key = vantis_pqc::secure_memory::secure_random_bytes(32);
        let nonce1 = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
        let nonce2 = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
        prop_assume!(nonce1 != nonce2);
        
        let mut encryptor = StreamingEncryptor::new(&key, &nonce1, None).unwrap();
        let encrypted = encryptor.encrypt_chunk(&data, true).unwrap();
        
        let mut decryptor = StreamingDecryptor::new(&key, &nonce2).unwrap();
        let result = decryptor.decrypt_chunk(&encrypted);
        
        prop_assert!(result.is_err());
    }
}

// ============================================================================
// Key Derivation Properties
// ============================================================================

proptest! {
    /// Test KDF produces consistent results
    #[test]
    fn prop_kdf_deterministic(
        secret in arb_non_empty_bytes(32, 64),
        context in arb_non_empty_bytes(8, 32)
    ) {
        use vantis_pqc::kdf;
        
        let key1 = kdf::derive_key(&secret, &context, 32).unwrap();
        let key2 = kdf::derive_key(&secret, &context, 32).unwrap();
        
        prop_assert_eq!(key1, key2);
    }
    
    /// Test KDF produces different keys for different contexts
    #[test]
    fn prop_kdf_different_contexts(
        secret in arb_non_empty_bytes(32, 64),
        context1 in arb_non_empty_bytes(8, 32),
        context2 in arb_non_empty_bytes(8, 32)
    ) {
        use vantis_pqc::kdf;
        
        prop_assume!(context1 != context2);
        
        let key1 = kdf::derive_key(&secret, &context1, 32).unwrap();
        let key2 = kdf::derive_key(&secret, &context2, 32).unwrap();
        
        prop_assert_ne!(key1, key2);
    }
    
    /// Test KDF produces different keys for different secrets
    #[test]
    fn prop_kdf_different_secrets(
        secret1 in arb_non_empty_bytes(32, 64),
        secret2 in arb_non_empty_bytes(32, 64),
        context in arb_non_empty_bytes(8, 32)
    ) {
        use vantis_pqc::kdf;
        
        prop_assume!(secret1 != secret2);
        
        let key1 = kdf::derive_key(&secret1, &context, 32).unwrap();
        let key2 = kdf::derive_key(&secret2, &context, 32).unwrap();
        
        prop_assert_ne!(key1, key2);
    }
    
    /// Test KDF output size (limited to 32 bytes due to SHA3-256 output size)
    #[test]
    fn prop_kdf_output_size(
        secret in arb_non_empty_bytes(16, 64),
        context in arb_non_empty_bytes(4, 32),
        size in 16usize..32
    ) {
        use vantis_pqc::kdf;
        
        let key = kdf::derive_key(&secret, &context, size).unwrap();
        prop_assert_eq!(key.len(), size);
    }
}

// ============================================================================
// Secure Memory Properties
// ============================================================================

proptest! {
    /// Test random bytes are unique
    #[test]
    fn prop_random_bytes_unique(size in 16usize..256) {
        let bytes1 = vantis_pqc::secure_memory::secure_random_bytes(size);
        let bytes2 = vantis_pqc::secure_memory::secure_random_bytes(size);
        
        prop_assert_ne!(bytes1, bytes2);
    }
    
    /// Test random bytes are correct size
    #[test]
    fn prop_random_bytes_size(size in 1usize..1024) {
        let bytes = vantis_pqc::secure_memory::secure_random_bytes(size);
        prop_assert_eq!(bytes.len(), size);
    }
    
    /// Test constant time compare is consistent
    #[test]
    fn prop_constant_time_compare_consistent(
        a in arb_non_empty_bytes(1, 256),
        b in arb_non_empty_bytes(1, 256)
    ) {
        let result1 = vantis_pqc::secure_memory::constant_time_compare(&a, &b);
        let result2 = vantis_pqc::secure_memory::constant_time_compare(&a, &b);
        
        prop_assert_eq!(result1, result2);
    }
    
    /// Test constant time compare with same bytes
    #[test]
    fn prop_constant_time_compare_same(bytes in arb_non_empty_bytes(1, 256)) {
        let result = vantis_pqc::secure_memory::constant_time_compare(&bytes, &bytes);
        prop_assert!(result);
    }
}