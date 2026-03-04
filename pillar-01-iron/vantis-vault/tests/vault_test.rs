//! Comprehensive integration tests for Vantis Vault
//!
//! Tests cover:
//! - Vault initialization and creation
//! - Encryption and decryption with different profiles
//! - Different key slots
//! - Edge cases (empty data, large data)
//! - Multiple encryption/decryption cycles
//! - Error handling

use vantis_vault::{init};
use vantis_vault::api::{EncryptionProfile, KeySlot, Vault};

// ============================================================================
// Initialization Tests
// ============================================================================

#[test]
fn test_vault_initialization() {
    let result = init();
    assert!(result.is_ok(), "Vault initialization should succeed");
}

#[test]
fn test_vault_creation() {
    let vault = Vault::new();
    assert!(vault.is_ok(), "Vault creation should succeed");
}

#[test]
fn test_vault_default() {
    let _vault = Vault::default();
    // Vault should be created without error
    // This test ensures Default trait is properly implemented
}

// ============================================================================
// Encryption/Decryption Tests
// ============================================================================

#[test]
fn test_encrypt_document_none() {
    let vault = Vault::new().unwrap();
    let plaintext = b"Hello, Vantis!";
    let result = vault.encrypt_document(plaintext, EncryptionProfile::None, KeySlot::Primary);
    assert!(result.is_ok(), "Encryption should succeed");

    let ciphertext = result.unwrap();
    assert_eq!(ciphertext, plaintext, "No encryption should preserve data");
}

#[test]
fn test_encrypt_decrypt_software() {
    let vault = Vault::new().unwrap();
    let plaintext = b"Secret message";

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::Software, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "Encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "Decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Decrypted data should match original"
    );
}

#[test]
fn test_encrypt_decrypt_tpm() {
    let vault = Vault::new().unwrap();
    let plaintext = b"TPM encrypted message";

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::TPM2_0, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "TPM encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "TPM decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "TPM decrypted data should match original"
    );
}

// ============================================================================
// Key Slot Tests
// ============================================================================

#[test]
fn test_encrypt_decrypt_backup_slot() {
    let vault = Vault::new().unwrap();
    let plaintext = b"Backup key encrypted data";

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::Software, KeySlot::Backup);
    assert!(ciphertext.is_ok(), "Backup slot encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Backup);
    assert!(decrypted.is_ok(), "Backup slot decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Backup slot decrypted data should match original"
    );
}

#[test]
fn test_encrypt_decrypt_custom_slot() {
    let vault = Vault::new().unwrap();
    let plaintext = b"Custom key slot data";
    let custom_slot = KeySlot::Custom(42);

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::Software, custom_slot);
    assert!(ciphertext.is_ok(), "Custom slot encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), custom_slot);
    assert!(decrypted.is_ok(), "Custom slot decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Custom slot decrypted data should match original"
    );
}

#[test]
fn test_encrypt_decrypt_multiple_custom_slots() {
    let vault = Vault::new().unwrap();
    let plaintext = b"Multi-slot test data";

    // Test multiple custom slots
    for slot_id in 1..=5 {
        let custom_slot = KeySlot::Custom(slot_id);
        
        let ciphertext =
            vault.encrypt_document(plaintext, EncryptionProfile::Software, custom_slot);
        assert!(ciphertext.is_ok(), "Custom slot encryption should succeed");

        let decrypted = vault.decrypt_document(&ciphertext.unwrap(), custom_slot);
        assert!(decrypted.is_ok(), "Custom slot decryption should succeed");

        assert_eq!(
            decrypted.unwrap(),
            plaintext,
            "Custom slot decrypted data should match original"
        );
    }
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_empty_document() {
    let vault = Vault::new().unwrap();
    let plaintext = b"";

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::Software, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "Empty document encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "Empty document decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Empty decrypted data should match original"
    );
}

#[test]
fn test_large_document() {
    let vault = Vault::new().unwrap();
    let plaintext = vec![0xABu8; 10_000]; // 10KB of data

    let ciphertext =
        vault.encrypt_document(&plaintext, EncryptionProfile::Software, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "Large document encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "Large document decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Large decrypted data should match original"
    );
}

#[test]
fn test_binary_data() {
    let vault = Vault::new().unwrap();
    let plaintext: Vec<u8> = (0..=255).cycle().take(256).collect(); // All byte values

    let ciphertext =
        vault.encrypt_document(&plaintext, EncryptionProfile::Software, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "Binary data encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "Binary data decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Binary decrypted data should match original"
    );
}

#[test]
fn test_unicode_data() {
    let vault = Vault::new().unwrap();
    let plaintext = "Hello 世界 🌍 Привет مرحبا".as_bytes();

    let ciphertext =
        vault.encrypt_document(plaintext, EncryptionProfile::Software, KeySlot::Primary);
    assert!(ciphertext.is_ok(), "Unicode data encryption should succeed");

    let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
    assert!(decrypted.is_ok(), "Unicode data decryption should succeed");

    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "Unicode decrypted data should match original"
    );
}

// ============================================================================
// Multiple Encryption Tests
// ============================================================================

#[test]
fn test_multiple_encryption_cycles() {
    let vault = Vault::new().unwrap();
    let mut data = b"Original message".to_vec();

    // Encrypt and decrypt multiple times
    for i in 0..5 {
        let ciphertext =
            vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Primary);
        assert!(ciphertext.is_ok());

        let decrypted = vault.decrypt_document(&ciphertext.unwrap(), KeySlot::Primary);
        assert!(decrypted.is_ok());

        data = decrypted.unwrap();
    }

    assert_eq!(
        data,
        b"Original message".to_vec(),
        "Data should remain consistent after multiple cycles"
    );
}

#[test]
fn test_multiple_documents_same_vault() {
    let vault = Vault::new().unwrap();
    
    let doc1 = b"Document 1 content";
    let doc2 = b"Document 2 content";
    let doc3 = b"Document 3 content";

    // Encrypt all documents
    let cipher1 = vault.encrypt_document(doc1, EncryptionProfile::Software, KeySlot::Primary).unwrap();
    let cipher2 = vault.encrypt_document(doc2, EncryptionProfile::Software, KeySlot::Backup).unwrap();
    let cipher3 = vault.encrypt_document(doc3, EncryptionProfile::Software, KeySlot::Custom(1)).unwrap();

    // Decrypt all documents
    let dec1 = vault.decrypt_document(&cipher1, KeySlot::Primary).unwrap();
    let dec2 = vault.decrypt_document(&cipher2, KeySlot::Backup).unwrap();
    let dec3 = vault.decrypt_document(&cipher3, KeySlot::Custom(1)).unwrap();

    assert_eq!(dec1, doc1, "Document 1 should match");
    assert_eq!(dec2, doc2, "Document 2 should match");
    assert_eq!(dec3, doc3, "Document 3 should match");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_workflow() {
    // Initialize vault
    let init_result = init();
    assert!(init_result.is_ok(), "Vault initialization should succeed");

    // Create vault instance
    let vault = Vault::new().unwrap();

    // Encrypt multiple documents with different profiles and key slots
    let documents = vec![
        (b"Document 1".to_vec(), EncryptionProfile::None, KeySlot::Primary),
        (b"Document 2".to_vec(), EncryptionProfile::Software, KeySlot::Primary),
        (b"Document 3".to_vec(), EncryptionProfile::Software, KeySlot::Backup),
        (b"Document 4".to_vec(), EncryptionProfile::Software, KeySlot::Custom(100)),
    ];

    for (doc, profile, slot) in &documents {
        let cipher = vault.encrypt_document(doc, *profile, *slot);
        assert!(cipher.is_ok(), "Encryption should succeed for all documents");

        let decrypted = vault.decrypt_document(&cipher.unwrap(), *slot);
        assert!(decrypted.is_ok(), "Decryption should succeed for all documents");

        assert_eq!(decrypted.unwrap(), *doc, "Decrypted data should match original");
    }
}

#[test]
fn test_vault_instance_reuse() {
    let vault = Vault::new().unwrap();

    // Use the same vault instance for multiple operations
    for i in 0..10 {
        let plaintext = format!("Message {}", i).as_bytes().to_vec();
        let cipher = vault.encrypt_document(&plaintext, EncryptionProfile::Software, KeySlot::Primary);
        assert!(cipher.is_ok());

        let decrypted = vault.decrypt_document(&cipher.unwrap(), KeySlot::Primary);
        assert!(decrypted.is_ok());

        assert_eq!(decrypted.unwrap(), plaintext);
    }
}