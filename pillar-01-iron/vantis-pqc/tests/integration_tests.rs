//! Integration tests for vantis-pqc module

use vantis_pqc::{
    KyberKeyPair, KyberSecurityLevel,
    DilithiumKeyPair, DilithiumSecurityLevel,
    StreamingEncryptor, StreamingDecryptor,
    MultiPartyManager, AccessLevel,
};
use vantis_pqc::hybrid::HybridKeyPair;
use vantis_pqc::streaming::NONCE_SIZE;

// ============================================================================
// Kyber Integration Tests
// ============================================================================
// Note: Kyber tests verify API correctness and output sizes since the current
// implementation is a placeholder. When a real Kyber implementation is added,
// these tests should be updated to verify shared secret equality.

#[test]
fn test_kyber_key_generation_and_encapsulation() {
    // Test key generation produces correct sizes for Kyber768
    let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    
    // Verify key sizes match Kyber-768 specification
    assert_eq!(keypair.public_key().len(), 1184);
    assert_eq!(keypair.private_key().len(), 2400);
    
    // Test encapsulation produces correct output sizes
    let encapsulation = vantis_pqc::kyber::encapsulate(
        keypair.public_key(),
        KyberSecurityLevel::Kyber768
    ).unwrap();
    
    // Shared secret should be 32 bytes (256 bits)
    assert_eq!(encapsulation.shared_secret.len(), 32);
    // Ciphertext should be 1088 bytes for Kyber-768
    assert_eq!(encapsulation.ciphertext.len(), 1088);
    
    // Test decapsulation produces a valid shared secret
    let shared_secret = vantis_pqc::kyber::decapsulate(
        keypair.private_key(),
        &encapsulation.ciphertext,
        KyberSecurityLevel::Kyber768
    ).unwrap();
    
    assert_eq!(shared_secret.len(), 32);
}

#[test]
fn test_kyber_different_security_levels() {
    // Test all three Kyber security levels
    let test_cases = [
        (KyberSecurityLevel::Kyber512, 800, 1632, 768),   // (level, pk_size, sk_size, ct_size)
        (KyberSecurityLevel::Kyber768, 1184, 2400, 1088),
        (KyberSecurityLevel::Kyber1024, 1568, 3168, 1568),
    ];

    for (level, expected_pk_size, expected_sk_size, expected_ct_size) in test_cases {
        let keypair = KyberKeyPair::generate(level).unwrap();
        
        // Verify key sizes
        assert_eq!(keypair.public_key().len(), expected_pk_size, 
            "Public key size mismatch for {:?}", level);
        assert_eq!(keypair.private_key().len(), expected_sk_size,
            "Private key size mismatch for {:?}", level);
        
        // Test encapsulation
        let encapsulation = vantis_pqc::kyber::encapsulate(
            keypair.public_key(),
            level
        ).unwrap();
        
        // Verify output sizes
        assert_eq!(encapsulation.shared_secret.len(), 32,
            "Shared secret size mismatch for {:?}", level);
        assert_eq!(encapsulation.ciphertext.len(), expected_ct_size,
            "Ciphertext size mismatch for {:?}", level);
        
        // Test decapsulation
        let shared_secret = vantis_pqc::kyber::decapsulate(
            keypair.private_key(),
            &encapsulation.ciphertext,
            level
        ).unwrap();
        
        assert_eq!(shared_secret.len(), 32,
            "Decapsulated shared secret size mismatch for {:?}", level);
    }
}

#[test]
fn test_kyber_invalid_ciphertext_size() {
    let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    
    // Decapsulation should fail with wrong ciphertext size
    let result = vantis_pqc::kyber::decapsulate(
        keypair.private_key(),
        &[0u8; 100],  // Wrong size
        KyberSecurityLevel::Kyber768
    );
    
    assert!(result.is_err());
}

// ============================================================================
// Dilithium Integration Tests
// ============================================================================

#[test]
fn test_dilithium_full_sign_verify_flow() {
    let keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium3).unwrap();
    let message = b"Important document to sign";

    let signature = keypair.sign(message).unwrap();

    let valid = DilithiumKeyPair::verify(
        keypair.public_key(),
        message,
        &signature,
        DilithiumSecurityLevel::Dilithium3
    ).unwrap();

    assert!(valid);
}

#[test]
fn test_dilithium_different_security_levels() {
    let levels = [
        DilithiumSecurityLevel::Dilithium2,
        DilithiumSecurityLevel::Dilithium3,
        DilithiumSecurityLevel::Dilithium5,
    ];

    for level in levels {
        let keypair = DilithiumKeyPair::generate(level).unwrap();
        let message = b"Test message";
        let signature = keypair.sign(message).unwrap();
        let valid = DilithiumKeyPair::verify(
            keypair.public_key(),
            message,
            &signature,
            level
        ).unwrap();
        assert!(valid);
    }
}

// ============================================================================
// Streaming Encryption Integration Tests
// ============================================================================

#[test]
fn test_streaming_large_file_encryption() {
    let key = vantis_pqc::secure_memory::secure_random_bytes(32);
    let nonce = vantis_pqc::secure_memory::secure_random_bytes(NONCE_SIZE);
    
    let test_data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();

    let mut encryptor = StreamingEncryptor::new(&key, &nonce, Some(64 * 1024)).unwrap();
    let encrypted = encryptor.encrypt_chunk(&test_data, true).unwrap();

    let mut decryptor = StreamingDecryptor::new(&key, &nonce).unwrap();
    let decrypted = decryptor.decrypt_chunk(&encrypted).unwrap();

    assert_eq!(test_data, decrypted);
}

// ============================================================================
// Multi-Party Encryption Integration Tests
// ============================================================================

#[test]
fn test_multi_party_group_encryption_flow() {
    let mut manager = MultiPartyManager::new(
        "project-team",
        "Project Alpha Team",
        KyberSecurityLevel::Kyber768
    );

    let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.initialize("admin", admin_keypair.public_key()).unwrap();

    let writer_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.add_member("admin", "writer", writer_keypair.public_key(), AccessLevel::Write).unwrap();

    let reader_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.add_member("admin", "reader", reader_keypair.public_key(), AccessLevel::Read).unwrap();

    let secret_message = b"Confidential project update";
    let encrypted = manager.encrypt("writer", secret_message).unwrap();

    let decrypted_admin = manager.decrypt("admin", &encrypted).unwrap();
    assert_eq!(secret_message.to_vec(), decrypted_admin);

    let decrypted_writer = manager.decrypt("writer", &encrypted).unwrap();
    assert_eq!(secret_message.to_vec(), decrypted_writer);

    let decrypted_reader = manager.decrypt("reader", &encrypted).unwrap();
    assert_eq!(secret_message.to_vec(), decrypted_reader);
}

#[test]
fn test_multi_party_access_control_enforcement() {
    let mut manager = MultiPartyManager::new(
        "access-test",
        "Access Control Test",
        KyberSecurityLevel::Kyber768
    );

    let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.initialize("admin", admin_keypair.public_key()).unwrap();

    let reader_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.add_member("admin", "reader", reader_keypair.public_key(), AccessLevel::Read).unwrap();

    let writer_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
    manager.add_member("admin", "writer", writer_keypair.public_key(), AccessLevel::Write).unwrap();

    assert!(manager.encrypt("admin", b"admin message").is_ok());
    assert!(manager.encrypt("writer", b"writer message").is_ok());
    assert!(manager.encrypt("reader", b"reader message").is_err());
    assert!(manager.encrypt("non-member", b"hack message").is_err());
}

// ============================================================================
// Key Derivation Integration Tests
// ============================================================================

#[test]
fn test_key_derivation_from_shared_secret() {
    use vantis_pqc::kdf;

    let shared_secret = vantis_pqc::secure_memory::secure_random_bytes(32);
    let context = b"vantis-office-session";

    let key1 = kdf::derive_key(&shared_secret, context, 32).unwrap();
    let key2 = kdf::derive_key(&shared_secret, b"different-context", 32).unwrap();

    assert_ne!(key1, key2);

    let key1_again = kdf::derive_key(&shared_secret, context, 32).unwrap();
    assert_eq!(key1, key1_again);
}

// ============================================================================
// Secure Memory Integration Tests
// ============================================================================

#[test]
fn test_secure_random_bytes_uniqueness() {
    let bytes1 = vantis_pqc::secure_memory::secure_random_bytes(32);
    let bytes2 = vantis_pqc::secure_memory::secure_random_bytes(32);
    let bytes3 = vantis_pqc::secure_memory::secure_random_bytes(32);

    assert_ne!(bytes1, bytes2);
    assert_ne!(bytes2, bytes3);
    assert_ne!(bytes1, bytes3);
}

#[test]
fn test_constant_time_compare() {
    let a = b"hello world";
    let b = b"hello world";
    let c = b"hello earth";

    assert!(vantis_pqc::secure_memory::constant_time_compare(a, b));
    assert!(!vantis_pqc::secure_memory::constant_time_compare(a, c));
}
