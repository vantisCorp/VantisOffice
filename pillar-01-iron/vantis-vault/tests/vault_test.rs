//! Integration tests for Vantis Vault

use vantis_vault::{init, EncryptionProfile, KeySlot, Vault};

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
