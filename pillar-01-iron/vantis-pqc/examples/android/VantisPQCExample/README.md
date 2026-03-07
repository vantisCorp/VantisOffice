# VantisPQC Android Example

This example demonstrates how to use the VantisPQC Post-Quantum Cryptography module in an Android application.

## Overview

The example showcases:
- Key generation (Kyber and Dilithium)
- Key encapsulation/decapsulation
- Digital signatures
- Streaming encryption for large files
- Multi-party encryption for group collaboration

## Requirements

- Android 8.0+ (API level 26)
- Kotlin 1.9+
- Android Studio Hedgehog (2023.1.1) or later

## Setup

### Gradle Dependency

Add the JitPack repository to your root `build.gradle`:

```groovy
allprojects {
    repositories {
        ...
        maven { url 'https://jitpack.io' }
    }
}
```

Add the dependency to your app's `build.gradle`:

```groovy
dependencies {
    implementation 'com.github.vantisCorp:VantisOffice:v0.1.0'
}
```

### Kotlin DSL (build.gradle.kts)

```kotlin
dependencies {
    implementation("com.github.vantisCorp:VantisOffice:v0.1.0")
}
```

## Usage Examples

### Key Generation

```kotlin
import com.vantis.pqc.*

// Generate Kyber key pair for key exchange
val kyberKeyPair = KyberKeyPair.generate(KyberSecurityLevel.KYBER_768)

// Generate Dilithium key pair for signatures
val dilithiumKeyPair = DilithiumKeyPair.generate(DilithiumSecurityLevel.DILITHIUM_3)
```

### Key Encapsulation

```kotlin
// Encapsulate a shared secret
val encapsulation = Kyber.encapsulate(
    publicKey = kyberKeyPair.publicKey,
    securityLevel = KyberSecurityLevel.KYBER_768
)

// Decapsulate on the recipient side
val sharedSecret = Kyber.decapsulate(
    privateKey = kyberKeyPair.privateKey,
    ciphertext = encapsulation.ciphertext,
    securityLevel = KyberSecurityLevel.KYBER_768
)
```

### Digital Signatures

```kotlin
val message = "Important document to sign".toByteArray()

// Sign
val signature = dilithiumKeyPair.sign(message)

// Verify
val isValid = DilithiumKeyPair.verify(
    publicKey = dilithiumKeyPair.publicKey,
    message = message,
    signature = signature,
    securityLevel = DilithiumSecurityLevel.DILITHIUM_3
)
```

### Streaming Encryption

```kotlin
// Encrypt large file
val encryptor = StreamingEncryptor(
    key = encryptionKey,
    nonce = nonce,
    chunkSize = 64 * 1024
)

val encryptedData = encryptor.encryptChunk(
    plaintext = fileData,
    isFinal = true
)

// Decrypt
val decryptor = StreamingDecryptor(key = encryptionKey, nonce = nonce)
val decryptedData = decryptor.decryptChunk(ciphertext = encryptedData)
```

## Architecture

The Android example follows MVVM architecture with Jetpack Compose:

```
VantisPQCExample/
├── app/
│   ├── src/main/java/com/vantis/pqc/example/
│   │   ├── MainActivity.kt
│   │   ├── ui/
│   │   │   ├── theme/
│   │   │   ├── screens/
│   │   │   └── components/
│   │   ├── viewmodel/
│   │   │   ├── KeyExchangeViewModel.kt
│   │   │   └── SignatureViewModel.kt
│   │   └── service/
│   │       └── VantisPQCService.kt
│   └── build.gradle.kts
└── gradle/
    └── libs.versions.toml
```

## Security Considerations

1. **Key Storage**: Use Android Keystore for secure key storage
2. **Memory Safety**: Keys are zeroized when no longer needed
3. **Biometric Protection**: Consider adding BiometricPrompt for key access

## Building the Native Library

Since VantisPQC is written in Rust, you'll need to build it for Android targets:

```bash
# Install Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Build using cargo-ndk
cargo install cargo-ndk
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ./jniLibs build --release
```

## Integration with Existing Android Crypto

VantisPQC can be used alongside Android's built-in cryptography:

```kotlin
// Use Android Keystore for key storage
val keyStore = KeyStore.getInstance("AndroidKeyStore")
keyStore.load(null)

// Store VantisPQC keys in Keystore
val keyPair = KyberKeyPair.generate(KyberSecurityLevel.KYBER_768)
// Store securely using your preferred method
```

## License

MIT License - See LICENSE file for details.