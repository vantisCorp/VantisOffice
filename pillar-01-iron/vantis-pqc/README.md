# 🔐 Vantis PQC - Post-Quantum Cryptography

Part of the **VantisOffice** suite | Pillar 01: Iron (Security Foundation)

## Overview

Vantis PQC provides post-quantum cryptographic primitives for VantisOffice, ensuring long-term security against quantum computing threats.

## Features

- **Kyber KEM**: Key Encapsulation Mechanism (NIST PQC standard)
- **Dilithium Signatures**: Digital signature scheme (NIST PQC standard)
- **Hybrid Cryptography**: X25519 + Kyber for transitional security
- **Key Management**: Comprehensive key lifecycle management
- **Key Derivation**: HKDF, PBKDF2, Argon2 support
- **Key Rotation**: Automatic key rotation policies
- **FFI Bindings**: C, Swift (iOS), and Kotlin (Android) bindings

## Architecture

```
vantis-pqc/
├── src/           # Core PQC implementation
├── tests/         # Comprehensive test suite
├── benches/       # Performance benchmarks
├── bindings/      # FFI bindings (C, Swift, Kotlin)
├── include/       # C header files
├── examples/      # Usage examples
└── docs/          # Documentation
```

## Usage

```rust
use vantis_pqc::{KyberKem, DilithiumSigner, HybridEncryption};

// Key Encapsulation
let (public_key, secret_key) = KyberKem::keypair();
let (ciphertext, shared_secret) = KyberKem::encapsulate(&public_key);
let decrypted_secret = KyberKem::decapsulate(&secret_key, &ciphertext);

// Digital Signatures
let signer = DilithiumSigner::new();
let signature = signer.sign(message);
assert!(signer.verify(message, &signature));

// Hybrid Encryption (X25519 + Kyber)
let hybrid = HybridEncryption::new();
let encrypted = hybrid.encrypt(plaintext);
let decrypted = hybrid.decrypt(&encrypted);
```

## Security Standards

- NIST Post-Quantum Cryptography Standards (FIPS 203, FIPS 204)
- Hybrid approach for transitional security
- Constant-time implementations where applicable
- Memory-safe Rust implementation

## License

Proprietary - See [LICENSE](../../LICENSE) for details.