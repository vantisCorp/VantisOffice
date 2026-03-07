# VantisPQC iOS Example

This example demonstrates how to use the VantisPQC Post-Quantum Cryptography module in an iOS application.

## Overview

The example showcases:
- Key generation (Kyber and Dilithium)
- Key encapsulation/decapsulation
- Digital signatures
- Streaming encryption for large files
- Multi-party encryption for group collaboration

## Requirements

- iOS 15.0+
- Xcode 14+
- Swift 5.7+

## Setup

### Option 1: Swift Package Manager

Add the following to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/vantisCorp/VantisOffice.git", from: "0.1.0")
]
```

### Option 2: Xcode

1. File → Add Packages...
2. Enter repository URL: `https://github.com/vantisCorp/VantisOffice.git`
3. Select the `vantis-pqc` module

## Usage Examples

### Key Generation

```swift
import VantisPQC

// Generate Kyber key pair for key exchange
let kyberKeyPair = try KyberKeyPair.generate(securityLevel: .kyber768)

// Generate Dilithium key pair for signatures
let dilithiumKeyPair = try DilithiumKeyPair.generate(securityLevel: .dilithium3)
```

### Key Encapsulation

```swift
// Encapsulate a shared secret
let encapsulation = try Kyber.encapsulate(
    publicKey: kyberKeyPair.publicKey,
    securityLevel: .kyber768
)

// Decapsulate on the recipient side
let sharedSecret = try Kyber.decapsulate(
    privateKey: kyberKeyPair.privateKey,
    ciphertext: encapsulation.ciphertext,
    securityLevel: .kyber768
)
```

### Digital Signatures

```swift
let message = "Important document to sign".data(using: .utf8)!

// Sign
let signature = try dilithiumKeyPair.sign(message: message)

// Verify
let isValid = try DilithiumKeyPair.verify(
    publicKey: dilithiumKeyPair.publicKey,
    message: message,
    signature: signature,
    securityLevel: .dilithium3
)
```

### Streaming Encryption

```swift
// Encrypt large file
let encryptor = try StreamingEncryptor(
    key: encryptionKey,
    nonce: nonce,
    chunkSize: 64 * 1024
)

let encryptedData = try encryptor.encryptChunk(
    plaintext: fileData,
    isFinal: true
)

// Decrypt
let decryptor = try StreamingDecryptor(key: encryptionKey, nonce: nonce)
let decryptedData = try decryptor.decryptChunk(ciphertext: encryptedData)
```

## Architecture

The iOS example follows MVVM architecture:

```
VantisPQCExample/
├── App/
│   ├── VantisPQCExampleApp.swift
│   └── ContentView.swift
├── ViewModels/
│   ├── KeyExchangeViewModel.swift
│   └── SignatureViewModel.swift
├── Services/
│   └── VantisPQCService.swift
└── Models/
    └── CryptoModels.swift
```

## Security Considerations

1. **Key Storage**: Use iOS Keychain for secure key storage
2. **Memory Safety**: Keys are automatically zeroized when deallocated
3. **Biometric Protection**: Consider adding Face ID/Touch ID for key access

## Building the FFI Layer

Since VantisPQC is written in Rust, you'll need to build it as a static library:

```bash
# Install targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# Build for device
cargo build --release --target aarch64-apple-ios

# Build for simulator
cargo build --release --target aarch64-apple-ios-sim
```

## License

MIT License - See LICENSE file for details.