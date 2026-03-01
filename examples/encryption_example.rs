//! Encryption example using Vantis Vault

use anyhow::Result;
use vantis_vault::{Vault, EncryptionProfile, KeySlot};

fn main() -> Result<()> {
    println!("🔐 Demonstrating Vantis Vault encryption...");
    println!();
    
    // Create a vault instance
    let vault = Vault::new()?;
    println!("✅ Vault created");
    println!();
    
    // Original data
    let plaintext = b"Secret message from VantisOffice!";
    println!("📄 Original data: {:?}", String::from_utf8_lossy(plaintext));
    println!();
    
    // Encrypt with software encryption
    println!("🔒 Encrypting with software encryption...");
    let ciphertext = vault.encrypt_document(
        plaintext,
        EncryptionProfile::Software,
        KeySlot::Primary,
    )?;
    println!("✅ Encrypted data length: {} bytes", ciphertext.len());
    println!();
    
    // Decrypt
    println!("🔓 Decrypting...");
    let decrypted = vault.decrypt_document(&ciphertext, KeySlot::Primary)?;
    println!("✅ Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));
    println!();
    
    // Verify
    if decrypted == plaintext {
        println!("✅ Encryption/Decryption successful!");
    } else {
        println!("❌ Encryption/Decryption failed!");
    }
    
    Ok(())
}
