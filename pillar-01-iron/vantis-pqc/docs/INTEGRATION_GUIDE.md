# Vantis PQC Integration Guide

## Overview

This guide explains how to integrate Vantis PQC (Post-Quantum Cryptography) into your applications for quantum-resistant security.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
vantis-pqc = { path = "../vantis-pqc" }
```

## Quick Start

### Key Encapsulation with Kyber

```rust
use vantis_pqc::{KyberKeyPair, KyberSecurityLevel, encapsulate, decapsulate};

// Generate a keypair (Kyber768 recommended for most uses)
let keypair = KyberKeyPair::generate(KyberSecurityLevel::Level2)?;

// Encapsulate a shared secret
let (shared_secret, ciphertext) = encapsulate(&keypair.public_key)?;

// Decapsulate to recover the shared secret
let recovered_secret = decapsulate(&keypair.private_key, &ciphertext.data)?;

assert_eq!(shared_secret, recovered_secret);
```

### Digital Signatures with Dilithium

```rust
use vantis_pqc::{DilithiumKeyPair, DilithiumSecurityLevel, sign, verify};

// Generate a signing keypair
let keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3)?;

// Sign a message
let message = b"Important document content";
let signature = sign(&keypair.private_key, message)?;

// Verify the signature
let is_valid = verify(&keypair.public_key, message, &signature.data)?;
assert!(is_valid);
```

### Hybrid Key Exchange

For maximum security, combine classical and post-quantum algorithms:

```rust
use vantis_pqc::{hybrid_key_exchange, HybridAlgorithm, KyberKeyPair, KyberSecurityLevel};

// Generate Kyber keypair
let kyber_kp = KyberKeyPair::generate(KyberSecurityLevel::Level2)?;

// Generate X25519 keypair (using your preferred library)
let x25519_pk = /* X25519 public key bytes */;

// Perform hybrid key exchange
let (shared_secret, ciphertext) = hybrid_key_exchange(
    &x25519_pk,
    &kyber_kp.public_key,
    HybridAlgorithm::X25519Kyber768,
)?;
```

## Security Levels

### Kyber (Key Encapsulation)

| Level | Algorithm | Public Key | Ciphertext | Security |
|-------|-----------|------------|------------|----------|
| 1 | Kyber512 | 800 bytes | 768 bytes | 128-bit |
| 2 | Kyber768 | 1184 bytes | 1088 bytes | 192-bit |
| 3 | Kyber1024 | 1568 bytes | 1568 bytes | 256-bit |

**Recommendation**: Use Kyber768 (Level2) for balanced security and performance.

### Dilithium (Digital Signatures)

| Level | Algorithm | Public Key | Private Key | Signature | Security |
|-------|-----------|------------|-------------|-----------|----------|
| 2 | Dilithium2 | 1312 bytes | 2560 bytes | 2420 bytes | 128-bit |
| 3 | Dilithium3 | 1952 bytes | 4032 bytes | 3309 bytes | 192-bit |
| 5 | Dilithium5 | 2592 bytes | 4896 bytes | 4627 bytes | 256-bit |

**Recommendation**: Use Dilithium3 (Level3) for most applications.

## Key Management

### Using the KeyManager

```rust
use vantis_pqc::{KeyManager, KeyStorage, KyberSecurityLevel};

// Create a key manager with memory storage
let storage = KeyStorage::Memory;
let manager = KeyManager::new(storage);

// Generate and store a key
let key_id = manager.generate_key("my-app-key", KyberSecurityLevel::Level2)?;

// Retrieve the key
let keypair = manager.get_key(&key_id)?;
```

### Key Rotation

```rust
use vantis_pqc::{KeyRotationManager, RotationPolicy};

// Create rotation manager with strict policy
let policy = RotationPolicy::strict();
let rotation_manager = KeyRotationManager::new(policy);

// Check if key needs rotation
if rotation_manager.should_rotate(&key_metadata)? {
    let new_key_id = rotation_manager.rotate_key(&key_id)?;
}
```

## Key Derivation

### Deriving Keys from Shared Secret

```rust
use vantis_pqc::derive_keys_from_shared_secret;

// After KEM, derive multiple keys from the shared secret
let keys = derive_keys_from_shared_secret(
    &shared_secret,
    "encryption-context",
    2,  // Number of keys to derive
    32, // Key length in bytes
)?;

let encryption_key = &keys[0];
let mac_key = &keys[1];
```

### Password-Based Key Derivation

```rust
use vantis_pqc::{Argon2Kdf, Argon2Config};

// Configure Argon2id for password hashing
let config = Argon2Config {
    memory_cost: 64 * 1024,  // 64 MB
    time_cost: 3,             // 3 iterations
    parallelism: 4,           // 4 threads
    output_length: 32,
};

let derived_key = Argon2Kdf::derive(
    password,
    salt,
    config,
)?;
```

## Secure Memory Handling

```rust
use vantis_pqc::{SecureBox, SecureVec, secure_zero};

// Store sensitive data securely
let mut secret_key: SecureVec<u8> = SecureVec::from(key_bytes);

// The key will be zeroized when dropped
// Manual zeroing is also available
secure_zero(&mut buffer);
```

## FFI Integration

For C/C++ integration, use the FFI functions:

```c
#include <vantis_pqc.h>

// Generate Kyber keypair
uint8_t public_key[1184];
uint8_t private_key[2400];
size_t pk_len = sizeof(public_key);
size_t sk_len = sizeof(private_key);

int result = pqc_kyber_generate_keypair(
    2,  // Kyber768
    public_key, &pk_len,
    private_key, &sk_len
);

if (result != 0) {
    // Handle error
}
```

## Integration with Vantis Vault

For document encryption in Vantis Vault:

```rust
use vantis_vault::crypto::{PqcKeyBundle, encrypt_document, decrypt_document};

// Create a key bundle
let bundle = PqcKeyBundle::new_with_signing()?;

// Encrypt a document
let encrypted = encrypt_document(
    "doc-001",
    document_content,
    bundle.public_key(),
    Some(&bundle),
)?;

// Decrypt the document
let decrypted = decrypt_document(
    &encrypted,
    &bundle.kyber_keypair.private_key,
    bundle.signing_public_key(),
)?;
```

## Performance Considerations

### Batch Operations

For high-throughput scenarios, use batch operations:

```rust
use vantis_pqc::{BatchKyberGenerator, BatchEncapsulator};

// Generate multiple keys in parallel
let generator = BatchKyberGenerator::new(KyberSecurityLevel::Level2);
let keypairs = generator.generate_parallel(100)?;

// Batch encapsulation
let encapsulator = BatchEncapsulator::new(public_keys);
let results = encapsulator.encapsulate_batch_parallel(messages)?;
```

### Benchmarking

Run benchmarks to measure performance:

```bash
cargo bench
```

## Error Handling

```rust
use vantis_pqc::{PQCError, Result};

match encapsulate(&public_key) {
    Ok((shared_secret, ciphertext)) => {
        // Success
    }
    Err(PQCError::InvalidPublicKey) => {
        // Handle invalid public key
    }
    Err(PQCError::EncapsulationFailed(msg)) => {
        // Handle encapsulation failure
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Best Practices

1. **Use Kyber768 and Dilithium3** for most applications (NIST Level 3 security)
2. **Implement key rotation** with appropriate policies
3. **Use hybrid mode** during the transition period
4. **Store keys securely** using HSM or secure enclaves
5. **Zeroize sensitive data** immediately after use
6. **Validate all inputs** before cryptographic operations
7. **Keep dependencies updated** for security patches

## Migration from Classical Cryptography

### From RSA/ECC to PQC

1. **Assess current key usage**: Identify all RSA/ECC keys
2. **Generate PQC keypairs**: Create equivalent Kyber/Dilithium keys
3. **Implement hybrid mode**: Support both classical and PQC
4. **Migrate data**: Re-encrypt with PQC keys
5. **Retire classical keys**: After full migration

### Key Size Comparison

| Algorithm | Key Size | Signature Size |
|-----------|----------|----------------|
| RSA-2048 | 256 bytes | 256 bytes |
| ECDSA P-256 | 64 bytes | 64 bytes |
| Kyber768 | 1184 bytes | N/A |
| Dilithium3 | 1952 bytes | 3309 bytes |

## Troubleshooting

### Common Issues

1. **"Invalid public key"**: Check key size matches security level
2. **"Decapsulation failed"**: Ensure ciphertext matches the key's security level
3. **"Signature verification failed"**: Verify message and signature integrity

### Getting Help

- Documentation: `/docs` directory
- Issues: GitHub issue tracker
- Security issues: security@vantis.corp