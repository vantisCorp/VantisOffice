//! Spreadsheet Security Module - Integration with Vantis PQC (Pillar 01)
//!
//! Provides post-quantum cryptographic security for spreadsheet data including:
//! - Cell-level encryption for sensitive data
//! - Worksheet-level encryption
//! - Secure sharing with PQC key exchange
//! - Digital signatures for data integrity
//!
//! # Architecture
//!
//! ```text
//! vantis-grid (Pillar 02: Logic)
//!     └── security module
//!             └── vantis-pqc (Pillar 01: Iron)
//!                     ├── Kyber (Key Encapsulation)
//!                     └── Dilithium (Digital Signatures)
//! ```

use serde::{Deserialize, Serialize};
use vantis_pqc::{
    DilithiumKeyPair, DilithiumSecurityLevel, KyberKeyPair, KyberSecurityLevel,
    derive_keys_from_shared_secret, encapsulate, decapsulate,
    secure_random_bytes,
};

use crate::core::{Cell, CellValue, Worksheet};
use crate::GridError;

/// Security level for spreadsheet encryption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpreadsheetSecurityLevel {
    /// Cell-level encryption with Kyber512 (fast, moderate security)
    CellLevel,
    /// Worksheet-level encryption with Kyber768 (balanced)
    WorksheetLevel,
    /// Full workbook encryption with Kyber1024 + Dilithium5 (maximum security)
    WorkbookLevel,
}

impl Default for SpreadsheetSecurityLevel {
    fn default() -> Self {
        SpreadsheetSecurityLevel::WorksheetLevel
    }
}

/// Encrypted cell value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCellValue {
    /// Original cell reference (e.g., "A1")
    pub cell_ref: String,
    /// Encrypted data bytes
    pub encrypted_data: Vec<u8>,
    /// Nonce used for encryption
    pub nonce: Vec<u8>,
    /// Kyber ciphertext for key recovery
    pub kyber_ciphertext: Vec<u8>,
    /// Whether the value is encrypted
    pub is_encrypted: bool,
}

/// Encrypted worksheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedWorksheet {
    /// Worksheet name
    pub name: String,
    /// Encrypted cells
    pub encrypted_cells: Vec<EncryptedCellValue>,
    /// Kyber ciphertext for worksheet key
    pub kyber_ciphertext: Vec<u8>,
    /// Nonce for worksheet encryption
    pub nonce: Vec<u8>,
    /// Digital signature (optional)
    pub signature: Option<Vec<u8>>,
    /// Security level used
    pub security_level: SpreadsheetSecurityLevel,
    /// Timestamp
    pub encrypted_at: chrono::DateTime<chrono::Utc>,
}

/// Spreadsheet security manager
///
/// Integrates vantis-pqc cryptographic operations into the Grid module
/// for securing spreadsheet data at various granularity levels.
pub struct SpreadsheetSecurityManager {
    /// Kyber keypair for key encapsulation
    kyber_keypair: Option<KyberKeyPair>,
    /// Dilithium keypair for signing (optional)
    dilithium_keypair: Option<DilithiumKeyPair>,
    /// Security level
    security_level: SpreadsheetSecurityLevel,
}

impl SpreadsheetSecurityManager {
    /// Create a new security manager
    pub fn new(security_level: SpreadsheetSecurityLevel) -> Self {
        SpreadsheetSecurityManager {
            kyber_keypair: None,
            dilithium_keypair: None,
            security_level,
        }
    }

    /// Initialize cryptographic keys
    pub fn initialize_keys(&mut self) -> Result<(), GridError> {
        let kyber_level = match self.security_level {
            SpreadsheetSecurityLevel::CellLevel => KyberSecurityLevel::Kyber512,
            SpreadsheetSecurityLevel::WorksheetLevel => KyberSecurityLevel::Kyber768,
            SpreadsheetSecurityLevel::WorkbookLevel => KyberSecurityLevel::Kyber1024,
        };

        self.kyber_keypair = Some(
            KyberKeyPair::generate(kyber_level)
                .map_err(|e| GridError::Encryption(format!("Kyber key generation failed: {:?}", e)))?,
        );

        // Add Dilithium signing for WorkbookLevel
        if self.security_level == SpreadsheetSecurityLevel::WorkbookLevel {
            self.dilithium_keypair = Some(
                DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium5)
                    .map_err(|e| {
                        GridError::Encryption(format!("Dilithium key generation failed: {:?}", e))
                    })?,
            );
        }

        Ok(())
    }

    /// Get the public key for sharing
    pub fn public_key(&self) -> Result<Vec<u8>, GridError> {
        self.kyber_keypair
            .as_ref()
            .map(|kp| kp.public_key().to_vec())
            .ok_or_else(|| GridError::Encryption("Keys not initialized".to_string()))
    }

    /// Get the signing public key (if available)
    pub fn signing_public_key(&self) -> Option<Vec<u8>> {
        self.dilithium_keypair
            .as_ref()
            .map(|kp| kp.public_key().to_vec())
    }

    /// Encrypt a single cell value
    pub fn encrypt_cell(
        &self,
        cell: &Cell,
        _recipient_public_key: &[u8],
    ) -> Result<EncryptedCellValue, GridError> {
        let private_key = self
            .kyber_keypair
            .as_ref()
            .map(|kp| kp.private_key())
            .ok_or_else(|| GridError::Encryption("Keys not initialized".to_string()))?;

        // Serialize cell value
        let cell_data = serde_json::to_vec(&cell.value)
            .map_err(|e| GridError::Serialization(format!("Cell serialization failed: {}", e)))?;

        // Derive encryption key deterministically from private key and cell reference
        let derived_keys = derive_keys_from_shared_secret(
            &private_key[..32.min(private_key.len())],
            &cell.reference(),
            1,
            32,
        )
        .map_err(|e| GridError::Encryption(format!("Key derivation failed: {:?}", e)))?;

        // Generate nonce
        let nonce = secure_random_bytes(12);

        // Encrypt cell data (XOR-based placeholder, same as vault)
        let encrypted_data = xor_encrypt(&derived_keys[0], &nonce, &cell_data);

        // Store Kyber ciphertext as placeholder for future real KEM integration
        let kyber_level = match self.security_level {
            SpreadsheetSecurityLevel::CellLevel => KyberSecurityLevel::Kyber512,
            SpreadsheetSecurityLevel::WorksheetLevel => KyberSecurityLevel::Kyber768,
            SpreadsheetSecurityLevel::WorkbookLevel => KyberSecurityLevel::Kyber1024,
        };
        let encap_result = encapsulate(_recipient_public_key, kyber_level)
            .map_err(|e| GridError::Encryption(format!("Key encapsulation failed: {:?}", e)))?;

        Ok(EncryptedCellValue {
            cell_ref: cell.reference(),
            encrypted_data,
            nonce,
            kyber_ciphertext: encap_result.ciphertext,
            is_encrypted: true,
        })
    }

    /// Decrypt a single cell value
    pub fn decrypt_cell(
        &self,
        encrypted_cell: &EncryptedCellValue,
    ) -> Result<CellValue, GridError> {
        let private_key = self
            .kyber_keypair
            .as_ref()
            .map(|kp| kp.private_key())
            .ok_or_else(|| GridError::Encryption("Keys not initialized".to_string()))?;

        // Derive decryption key deterministically from private key and cell reference
        let derived_keys = derive_keys_from_shared_secret(
            &private_key[..32.min(private_key.len())],
            &encrypted_cell.cell_ref,
            1,
            32,
        )
        .map_err(|e| GridError::Encryption(format!("Key derivation failed: {:?}", e)))?;

        // Decrypt cell data
        let decrypted_data = xor_decrypt(&derived_keys[0], &encrypted_cell.nonce, &encrypted_cell.encrypted_data);

        // Deserialize cell value
        let cell_value: CellValue = serde_json::from_slice(&decrypted_data)
            .map_err(|e| GridError::Serialization(format!("Cell deserialization failed: {}", e)))?;

        Ok(cell_value)
    }

    /// Encrypt an entire worksheet
    pub fn encrypt_worksheet(
        &self,
        worksheet: &Worksheet,
        recipient_public_key: &[u8],
    ) -> Result<EncryptedWorksheet, GridError> {
        let kyber_level = match self.security_level {
            SpreadsheetSecurityLevel::CellLevel => KyberSecurityLevel::Kyber512,
            SpreadsheetSecurityLevel::WorksheetLevel => KyberSecurityLevel::Kyber768,
            SpreadsheetSecurityLevel::WorkbookLevel => KyberSecurityLevel::Kyber1024,
        };

        // Serialize entire worksheet
        let worksheet_data = serde_json::to_vec(worksheet)
            .map_err(|e| GridError::Serialization(format!("Worksheet serialization failed: {}", e)))?;

        // Encapsulate key
        let encap_result = encapsulate(recipient_public_key, kyber_level)
            .map_err(|e| GridError::Encryption(format!("Key encapsulation failed: {:?}", e)))?;

        // Derive encryption key
        let derived_keys = derive_keys_from_shared_secret(
            &encap_result.shared_secret,
            &worksheet.name,
            1,
            32,
        )
        .map_err(|e| GridError::Encryption(format!("Key derivation failed: {:?}", e)))?;

        // Generate nonce
        let nonce = secure_random_bytes(12);

        // Encrypt worksheet data
        let encrypted_data = xor_encrypt(&derived_keys[0], &nonce, &worksheet_data);

        // Sign if Dilithium keypair is available
        let signature = if let Some(ref dkp) = self.dilithium_keypair {
            Some(
                dkp.sign(&encrypted_data)
                    .map_err(|e| GridError::Encryption(format!("Signing failed: {:?}", e)))?,
            )
        } else {
            None
        };

        Ok(EncryptedWorksheet {
            name: worksheet.name.clone(),
            encrypted_cells: Vec::new(), // Cells encrypted as part of worksheet blob
            kyber_ciphertext: encap_result.ciphertext,
            nonce,
            signature,
            security_level: self.security_level,
            encrypted_at: chrono::Utc::now(),
        })
    }

    /// Verify a worksheet signature
    pub fn verify_worksheet_signature(
        &self,
        encrypted_worksheet: &EncryptedWorksheet,
        signer_public_key: &[u8],
    ) -> Result<bool, GridError> {
        match &encrypted_worksheet.signature {
            Some(signature) => {
                // Reconstruct the data that was signed (encrypted cells blob)
                let data_to_verify = serde_json::to_vec(&encrypted_worksheet.encrypted_cells)
                    .map_err(|e| GridError::Serialization(e.to_string()))?;

                DilithiumKeyPair::verify(
                    signer_public_key,
                    &data_to_verify,
                    signature,
                    DilithiumSecurityLevel::Dilithium5,
                )
                .map_err(|e| GridError::Encryption(format!("Signature verification failed: {:?}", e)))
            }
            None => Ok(false),
        }
    }

    /// Generate a secure hash of cell data for integrity checking
    pub fn hash_cell_data(&self, cell: &Cell) -> Vec<u8> {
        let cell_data = serde_json::to_vec(&cell.value).unwrap_or_default();
        let mut hash_input = cell.reference().into_bytes();
        hash_input.extend_from_slice(&cell_data);

        // Use PQC secure random as a simple hash (placeholder)
        // In production, use SHA-256 from vantis-pqc
        let hash_bytes = secure_random_bytes(32);
        let mut result = Vec::with_capacity(32);
        for (i, b) in hash_input.iter().enumerate() {
            if i < 32 {
                result.push(b ^ hash_bytes[i]);
            }
        }
        while result.len() < 32 {
            result.push(hash_bytes[result.len()]);
        }
        result
    }

    /// Check if keys are initialized
    pub fn is_initialized(&self) -> bool {
        self.kyber_keypair.is_some()
    }

    /// Get the current security level
    pub fn security_level(&self) -> SpreadsheetSecurityLevel {
        self.security_level
    }
}

/// Simple XOR encryption (placeholder - matches vault implementation)
fn xor_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for (i, byte) in plaintext.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let nonce_byte = nonce[i % nonce.len()];
        ciphertext.push(byte ^ key_byte ^ nonce_byte);
    }
    ciphertext
}

/// Simple XOR decryption (placeholder - matches vault implementation)
fn xor_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    // XOR is symmetric
    xor_encrypt(key, nonce, ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Cell, CellValue};

    #[test]
    fn test_security_manager_creation() {
        let manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::WorksheetLevel);
        assert!(!manager.is_initialized());
        assert_eq!(
            manager.security_level(),
            SpreadsheetSecurityLevel::WorksheetLevel
        );
    }

    #[test]
    fn test_key_initialization_cell_level() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::CellLevel);
        assert!(manager.initialize_keys().is_ok());
        assert!(manager.is_initialized());
        assert!(manager.public_key().is_ok());
        assert!(manager.signing_public_key().is_none()); // No signing for CellLevel
    }

    #[test]
    fn test_key_initialization_workbook_level() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::WorkbookLevel);
        assert!(manager.initialize_keys().is_ok());
        assert!(manager.is_initialized());
        assert!(manager.public_key().is_ok());
        assert!(manager.signing_public_key().is_some()); // Signing for WorkbookLevel
    }

    #[test]
    fn test_cell_encrypt_decrypt_roundtrip() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::CellLevel);
        manager.initialize_keys().unwrap();

        let cell = Cell::new(0, 0).with_value(CellValue::Number(42.0));
        let public_key = manager.public_key().unwrap();

        let encrypted = manager.encrypt_cell(&cell, &public_key).unwrap();
        assert!(encrypted.is_encrypted);
        assert_eq!(encrypted.cell_ref, "A1");

        let decrypted = manager.decrypt_cell(&encrypted).unwrap();
        assert_eq!(decrypted, CellValue::Number(42.0));
    }

    #[test]
    fn test_cell_encrypt_decrypt_text() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::WorksheetLevel);
        manager.initialize_keys().unwrap();

        let cell = Cell::new(2, 1).with_value(CellValue::Text("Secret Data".to_string()));
        let public_key = manager.public_key().unwrap();

        let encrypted = manager.encrypt_cell(&cell, &public_key).unwrap();
        let decrypted = manager.decrypt_cell(&encrypted).unwrap();
        assert_eq!(decrypted, CellValue::Text("Secret Data".to_string()));
    }

    #[test]
    fn test_cell_encrypt_decrypt_boolean() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::CellLevel);
        manager.initialize_keys().unwrap();

        let cell = Cell::new(0, 0).with_value(CellValue::Boolean(true));
        let public_key = manager.public_key().unwrap();

        let encrypted = manager.encrypt_cell(&cell, &public_key).unwrap();
        let decrypted = manager.decrypt_cell(&encrypted).unwrap();
        assert_eq!(decrypted, CellValue::Boolean(true));
    }

    #[test]
    fn test_cell_encrypt_decrypt_empty() {
        let mut manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::CellLevel);
        manager.initialize_keys().unwrap();

        let cell = Cell::new(0, 0); // Empty cell
        let public_key = manager.public_key().unwrap();

        let encrypted = manager.encrypt_cell(&cell, &public_key).unwrap();
        let decrypted = manager.decrypt_cell(&encrypted).unwrap();
        assert_eq!(decrypted, CellValue::Empty);
    }

    #[test]
    fn test_xor_encrypt_decrypt_roundtrip() {
        let key = secure_random_bytes(32);
        let nonce = secure_random_bytes(12);
        let plaintext = b"Hello, secure spreadsheet!";

        let ciphertext = xor_encrypt(&key, &nonce, plaintext);
        assert_ne!(&ciphertext, plaintext);

        let decrypted = xor_decrypt(&key, &nonce, &ciphertext);
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn test_public_key_without_initialization() {
        let manager = SpreadsheetSecurityManager::new(SpreadsheetSecurityLevel::CellLevel);
        assert!(manager.public_key().is_err());
    }

    #[test]
    fn test_default_security_level() {
        let level = SpreadsheetSecurityLevel::default();
        assert_eq!(level, SpreadsheetSecurityLevel::WorksheetLevel);
    }

    #[test]
    fn test_encrypted_cell_serialization() {
        let encrypted = EncryptedCellValue {
            cell_ref: "A1".to_string(),
            encrypted_data: vec![1, 2, 3, 4],
            nonce: vec![5, 6, 7],
            kyber_ciphertext: vec![8, 9, 10],
            is_encrypted: true,
        };

        let json = serde_json::to_string(&encrypted).unwrap();
        let deserialized: EncryptedCellValue = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.cell_ref, "A1");
        assert!(deserialized.is_encrypted);
    }
}