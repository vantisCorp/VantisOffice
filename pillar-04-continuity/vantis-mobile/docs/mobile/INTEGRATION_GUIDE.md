# VantisMobile FFI Integration Guide

This guide explains how to integrate the VantisMobile FFI bindings into iOS and Android applications for secure, end-to-end encrypted communication with VantisOffice desktop.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [iOS Integration](#ios-integration)
- [Android Integration](#android-integration)
- [Encryption Protocol](#encryption-protocol)
- [Message Flow](#message-flow)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)

## Overview

VantisMobile FFI provides Rust-based cryptographic primitives for secure mobile-desktop communication:

- **X25519 Key Exchange**: Diffie-Hellman key exchange for secure session establishment
- **ChaCha20-Poly1305 AEAD**: Authenticated encryption with associated data
- **JSON Protocol**: Cross-platform message format
- **WebSocket Tunnel**: Real-time bidirectional communication

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   iOS/Android   │         │  WebSocket       │         │  VantisOffice   │
│   Mobile App    │◄────────►│  Tunnel Server   │◄────────►│   Desktop App   │
│                 │         │                  │         │                 │
│  ┌───────────┐  │         │                  │         │  ┌───────────┐  │
│  │ VantisFFI │  │         │                  │         │  │ VantisFFI │  │
│  └───────────┘  │         │                  │         │  └───────────┘  │
└─────────────────┘         └──────────────────┘         └─────────────────┘
       │                                                          │
       │ X25519 Key Exchange                                      │
       │ ChaCha20-Poly1305 Encryption                             │
       └──────────────────────────────────────────────────────────┘
```

## iOS Integration

### Prerequisites

- Xcode 15.0 or later
- iOS 14.0 or later deployment target
- Swift 5.9 or later

### Setup

1. **Add VantisMobile to your SPM Package.swift:**

```swift
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "YourApp",
    platforms: [.iOS(.v14)],
    products: [
        .library(name: "YourApp", targets: ["YourApp"]),
    ],
    dependencies: [
        .package(path: "./VantisMobile")
    ],
    targets: [
        .target(
            name: "YourApp",
            dependencies: ["VantisMobile"],
            linkerSettings: [
                .unsafeFlags(["-lvantismobile"])
            ]
        ),
    ]
)
```

2. **Build the native library:**

```bash
cd VantisMobile
./build-ios.sh
```

This creates:
- `target/ios/VantisMobile.xcframework` - XCFramework for Xcode
- `target/ios/universal/libvantismobile.a` - Universal static library
- `target/ios/universal/include/vantis_mobile.h` - C headers

3. **Import in your Swift code:**

```swift
import VantisMobileFFI
```

### Usage Example

```swift
import Foundation
import VantisMobileFFI

class SecureCommunicationService {
    private var keyPair: VantisMobileFFI.KeyPair?
    private var encryptor: VantisMobileFFI.Encryptor?
    
    init() {
        // Initialize FFI library
        _ = VantisMobileFFI.shared
        
        // Generate X25519 key pair
        self.keyPair = VantisMobileFFI.KeyPair()
    }
    
    // Get our public key for key exchange
    var publicKeyBase64: String {
        return keyPair?.publicKeyBase64 ?? ""
    }
    
    // Set up encryption with peer's public key
    func setupEncryption(peerPublicKeyBase64: String) -> Bool {
        // In production, derive shared secret using X25519
        // For now, use pre-configured shared secret
        let sharedSecret = "your-shared-secret-base64"
        guard let enc = VantisMobileFFI.Encryptor(sharedSecretBase64: sharedSecret) else {
            return false
        }
        self.encryptor = enc
        return true
    }
    
    // Encrypt a message
    func encryptMessage(_ plaintext: Data) -> String? {
        guard let encryptor = encryptor else {
            return nil
        }
        return encryptor.encrypt(plaintext)
    }
    
    // Decrypt a message
    func decryptMessage(_ encryptedJson: String) -> Data? {
        guard let encryptor = encryptor else {
            return nil
        }
        return encryptor.decrypt(encryptedJson)
    }
}
```

### Swift API Reference

#### VantisMobileFFI (Singleton)

```swift
VantisMobileFFI.shared
```
- Singleton instance for initializing the FFI library

#### KeyPair

```swift
let keyPair = VantisMobileFFI.KeyPair()
let publicKey = keyPair.publicKeyBase64
```
- Generates X25519 key pair
- `publicKeyBase64`: String - Public key in Base64 encoding
- `free?`: (() -> Void)? - Cleanup method

#### Encryptor

```swift
let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: keyBase64)
let encrypted = encryptor.encrypt(plaintextData)
let decrypted = encryptor.decrypt(encryptedJsonString)
```
- Creates ChaCha20-Poly1305 encryptor
- `encrypt(_ data: Data) -> String?`: Encrypt data to JSON string
- `decrypt(_ json: String) -> Data?`: Decrypt JSON string to data

## Android Integration

### Prerequisites

- Android Studio Hedgehog (2023.1.1) or later
- Android SDK API level 21+ (Android 5.0+)
- Kotlin 1.9.0 or later
- NDK r25c or later (optional, for building native lib)

### Setup

1. **Add the Kotlin wrapper to your project:**

Copy `VantisMobileFFI.kt` to:
```
app/src/main/java/com/vantiscorp/vantismobile/ffi/VantisMobileFFI.kt
```

2. **Add the native library:**

```bash
cd VantisMobile
./build-android.sh
```

This creates libraries in `target/android/` for all architectures:
- arm64-v8a
- armeabi-v7a
- x86
- x86_64

3. **Update your build.gradle:**

```kotlin
android {
    sourceSets {
        main {
            jniLibs.srcDirs += ["../../target/android/vantismobile/jni"]
        }
    }
}
```

4. **Import in your Kotlin code:**

```kotlin
import com.vantiscorp.vantismobile.ffi.VantisMobileFFI
```

### Usage Example

```kotlin
import com.vantiscorp.vantismobile.ffi.VantisMobileFFI
import java.util.Base64

class SecureCommunicationService {
    private var keyPair: VantisMobileFFI.VantisKeyPair? = null
    private var encryptor: VantisMobileFFI.VantisEncryptor? = null
    
    init {
        // Initialize FFI library
        System.loadLibrary("vantismobile")
        
        // Generate X25519 key pair
        try {
            keyPair = VantisMobileFFI.VantisKeyPair()
        } catch (e: Exception) {
            println("Failed to initialize encryption: ${e.message}")
        }
    }
    
    // Get our public key for key exchange
    val publicKeyBase64: String?
        get() = keyPair?.publicKeyBase64
    
    // Set up encryption with peer's public key
    fun setupEncryption(peerPublicKeyBase64: String): Boolean {
        return try {
            // In production, derive shared secret using X25519
            // For now, use pre-configured shared secret
            val sharedSecret = "your-shared-secret-base64"
            encryptor = VantisMobileFFI.VantisEncryptor(sharedSecret)
            true
        } catch (e: Exception) {
            println("Failed to setup encryption: ${e.message}")
            false
        }
    }
    
    // Encrypt a message
    fun encryptMessage(plaintext: ByteArray): String? {
        return encryptor?.encrypt(plaintext)
    }
    
    // Decrypt a message
    fun decryptMessage(encryptedJson: String): ByteArray? {
        return encryptor?.decrypt(encryptedJson)
    }
}
```

### Kotlin API Reference

#### VantisKeyPair

```kotlin
val keyPair = VantisMobileFFI.VantisKeyPair()
val publicKey = keyPair.publicKeyBase64
```
- Generates X25519 key pair
- `publicKeyBase64`: String? - Public key in Base64 encoding
- `free()`: Unit - Cleanup method

#### VantisEncryptor

```kotlin
val encryptor = VantisMobileFFI.VantisEncryptor(sharedSecretBase64)
val encrypted = encryptor.encrypt(plaintextBytes)
val decrypted = encryptor.decrypt(encryptedJsonString)
```
- Creates ChaCha20-Poly1305 encryptor
- `encrypt(plaintext: ByteArray): String?`: Encrypt data to JSON string
- `decrypt(json: String): ByteArray?`: Decrypt JSON string to bytes

## Encryption Protocol

### Message Format

All encrypted messages use JSON format:

```json
{
  "nonce": "base64-encoded-nonce",
  "ciphertext": "base64-encoded-ciphertext",
  "tag": "base64-encoded-auth-tag"
}
```

### Encryption Process

1. **Key Exchange:**
   - Mobile generates X25519 key pair
   - Sends public key via handshake message
   - Desktop responds with its public key
   - Both derive shared secret using X25519

2. **Encryption:**
   - Generate random 96-bit nonce
   - Encrypt message using ChaCha20-Poly1305
   - Serialize to JSON format

3. **Decryption:**
   - Parse JSON to extract nonce, ciphertext, tag
   - Decrypt using ChaCha20-Poly1305
   - Return plaintext

### Security Properties

- **Forward Secrecy**: Each session uses unique key exchange
- **Authentication**: Poly1305 AEAD ensures message integrity
- **Replay Protection**: Nonces prevent message replay attacks
- **Confidentiality**: ChaCha20 provides strong encryption

## Message Flow

### Connection Establishment

```
Mobile                    Desktop
  |                         |
  |--- Handshake ---------->|
  |  (public_key)           |
  |                         |
  |<-- KeyExchange ---------|
  |  (public_key)           |
  |                         |
  |--- SetupEncryption ---->|
  |  (derive shared secret) |
  |                         |
  |<-- SetupEncryption -----|
  |  (derive shared secret) |
  |                         |
  |--- Encrypted Msg ------>|
  |<-- Encrypted Msg -------|
```

### Message Types

- `handshake`: Initial connection with public key
- `key_exchange`: Peer public key response
- `ping`: Keepalive and latency measurement
- `pong`: Ping response
- `command`: Remote command execution
- `command_response`: Command result
- `sync_request`: Document sync request
- `sync_response`: Document sync data
- `notification`: Desktop notification

## Testing

### Unit Tests

Run the Rust library tests:

```bash
cd VantisMobile
cargo test --lib
```

Expected: 28 tests passing

### Integration Tests

#### iOS

```swift
import XCTest
import VantisMobileFFI

class EncryptionTests: XCTestCase {
    func testKeyPairGeneration() {
        let keyPair = VantisMobileFFI.KeyPair()
        XCTAssertNotNil(keyPair)
        XCTAssertFalse(keyPair.publicKeyBase64.isEmpty)
    }
    
    func testEncryptionDecryption() {
        let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: "test-key")
        let plaintext = "Hello, World!".data(using: .utf8)!
        
        let encrypted = encryptor.encrypt(plaintext)
        XCTAssertNotNil(encrypted)
        
        let decrypted = encryptor.decrypt(encrypted!)
        XCTAssertEqual(decrypted, plaintext)
    }
}
```

#### Android

```kotlin
import org.junit.Test
import com.vantiscorp.vantismobile.ffi.VantisMobileFFI

class EncryptionTests {
    @Test
    fun testKeyPairGeneration() {
        val keyPair = VantisMobileFFI.VantisKeyPair()
        assertNotNull(keyPair)
        assertFalse(keyPair.publicKeyBase64.isNullOrBlank())
    }
    
    @Test
    fun testEncryptionDecryption() {
        val encryptor = VantisMobileFFI.VantisEncryptor("test-key")
        val plaintext = "Hello, World!".toByteArray()
        
        val encrypted = encryptor.encrypt(plaintext)
        assertNotNull(encrypted)
        
        val decrypted = encryptor.decrypt(encrypted!!)
        assertArrayEquals(plaintext, decrypted)
    }
}
```

## Troubleshooting

### iOS Issues

**Problem**: Library not found during linking
```
ld: library not found for -lvantismobile
```

**Solution**:
1. Build the library: `./build-ios.sh`
2. Add linker settings in Package.swift:
```swift
.linkerSettings: [
    .unsafeFlags(["-Lvantis-mobile/target/ios/universal", "-lvantismobile"])
]
```

**Problem**: Crashes on initialization

**Solution**:
- Ensure the FFI library is built for correct architecture
- Check that the static library is in the expected location
- Verify the module map is correctly configured

### Android Issues

**Problem**: UnsatisfiedLinkError
```
java.lang.UnsatisfiedLinkError: No implementation found for ...
```

**Solution**:
1. Build the library: `./build-android.sh`
2. Copy libraries to `app/src/main/jniLibs/`
3. Ensure correct architecture libraries are present

**Problem**: JNI method not found

**Solution**:
- Verify the C header function signatures match Kotlin declarations
- Check that the library is loaded before use
- Use `javap` to verify JNI method names

### Encryption Issues

**Problem**: Decryption fails

**Solution**:
1. Verify both ends use the same shared secret
2. Check that the JSON format is correct
3. Ensure the encryption key is in Base64 format
4. Verify the nonce is unique per message

**Problem**: Performance issues

**Solution**:
- Reuse Encryptor instances instead of recreating
- Avoid unnecessary Base64 encoding/decoding
- Consider batch encryption for multiple messages

## Additional Resources

- [FFI API Reference](./API_REFERENCE.md)
- [Deployment Guide](../deployment/DEPLOYMENT.md)
- [Security Considerations](./SECURITY.md)
- [GitHub Repository](https://github.com/vantisCorp/VantisOffice)

## Support

For issues and questions:
- Create an issue on GitHub
- Check existing documentation
- Review test cases for examples