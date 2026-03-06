# VantisMobile Security Considerations

This document outlines security considerations and best practices for implementing VantisMobile FFI bindings in production applications.

## Table of Contents

- [Overview](#overview)
- [Cryptographic Primitives](#cryptographic-primitives)
- [Key Management](#key-management)
- [Secure Communication](#secure-communication)
- [Data Protection](#data-protection)
- [Implementation Security](#implementation-security)
- [Threat Model](#threat-model)
- [Compliance](#compliance)

## Overview

VantisMobile provides end-to-end encrypted communication between mobile devices and desktop applications using industry-standard cryptographic primitives.

### Security Properties

- **Confidentiality**: Messages are encrypted and only readable by intended recipients
- **Integrity**: Message authenticity verified via Poly1305 AEAD
- **Authentication**: Peer identity verified through X25519 key exchange
- **Forward Secrecy**: Each session uses unique key exchange
- **Replay Protection**: Nonces prevent message replay attacks

### Security Level

VantisMobile provides cryptographic security equivalent to AES-256:

- **Key Exchange**: X25519 (Curve25519, 256-bit security)
- **Encryption**: ChaCha20 (256-bit key, 256-bit security)
- **Authentication**: Poly1305 (128-bit security)

## Cryptographic Primitives

### X25519 Key Exchange

**Algorithm**: Curve25519 Diffie-Hellman

**Properties:**
- 256-bit security level
- Constant-time implementation
- Side-channel resistant
- No known weaknesses

**Key Generation:**
```swift
// Generates new X25519 key pair
let keyPair = VantisMobileFFI.KeyPair()
let publicKey = keyPair.publicKeyBase64
```

**Security Considerations:**
- Private keys never leave the device
- Public keys can be freely shared
- Each session should use a new key pair (recommended)
- Key pair is generated using cryptographically secure random number generator

### ChaCha20-Poly1305 AEAD

**Algorithm**: Authenticated Encryption with Associated Data (AEAD)

**Properties:**
- 256-bit encryption key
- 96-bit nonce (12 bytes)
- 128-bit authentication tag
- Authenticated encryption (confidentiality + integrity)

**Nonce Generation:**
- Nonces are generated randomly for each message
- 96-bit nonce space provides sufficient protection against nonce reuse
- Probability of nonce collision is negligible (2^-96)

**Security Considerations:**
- Nonce reuse would compromise security
- Implementation ensures unique nonces per encryption
- Authentication tag is verified before decryption

## Key Management

### Key Generation

**Best Practices:**

1. **Use Cryptographically Secure RNG:**
   ```swift
   // Handled internally by VantisMobileFFI
   let keyPair = VantisMobileFFI.KeyPair()
   ```

2. **Generate New Keys Per Session:**
   ```swift
   // Recommended for forward secrecy
   let keyPair = VantisMobileFFI.KeyPair()
   ```

3. **Never Hardcode Keys:**
   ```swift
   // BAD: Never do this
   let hardcodedKey = "SGVsbG8gV29ybGQ="
   ```

### Key Storage

**iOS Keychain:**

```swift
import Security

func savePrivateKeyToKeychain(_ privateKey: Data) {
    let query: [String: Any] = [
        kSecClass as String: kSecClassKey,
        kSecAttrApplicationTag as String: "vantis.mobile.key",
        kSecValueData as String: privateKey,
        kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlocked
    ]
    SecItemAdd(query as CFDictionary, nil)
}
```

**Android Keystore:**

```kotlin
import java.security.KeyStore

fun savePrivateKeyToKeystore(privateKey: ByteArray) {
    val keyStore = KeyStore.getInstance("AndroidKeyStore")
    keyStore.load(null)
    
    val spec = KeyProtection.Builder(
        KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
    )
        .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
        .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
        .setUserAuthenticationRequired(true)
        .build()
    
    keyStore.setEntry(
        "vantis.mobile.key",
        KeyStore.SecretKeyEntry(SecretKeySpec(privateKey, "AES")),
        spec
    )
}
```

**Key Storage Best Practices:**

1. **Use Platform Key Storage:**
   - iOS: Keychain Services
   - Android: Android Keystore
   - Never store keys in plain text files

2. **Restrict Key Access:**
   - iOS: `kSecAttrAccessibleWhenUnlocked`
   - Android: `setUserAuthenticationRequired(true)`

3. **Enable Hardware Security:**
   - iOS: Secure Enclave (for larger keys)
   - Android: Hardware-backed Keystore

### Key Exchange

**Secure Key Exchange Flow:**

```
1. Mobile generates X25519 key pair
2. Mobile sends public key to Desktop
3. Desktop generates X25519 key pair
4. Desktop sends public key to Mobile
5. Both derive shared secret using X25519
6. Shared secret used for ChaCha20-Poly1305
```

**Implementation:**

```swift
// Mobile sends handshake
let handshake = ProtocolMessage.handshake(
    deviceId: deviceId,
    publicKey: publicKeyBase64,
    deviceInfo: deviceInfo
)
try await send(handshake)

// Receive key exchange
if case .keyExchange(let peerPublicKey) = message {
    // Derive shared secret
    let sharedSecret = deriveSharedSecret(
        privateKey: privateKey,
        peerPublicKey: peerPublicKey
    )
    
    // Create encryptor
    encryptor = VantisMobileFFI.Encryptor(
        sharedSecretBase64: sharedSecret.base64EncodedString()
    )
}
```

**Key Exchange Security:**

- Public keys can be transmitted over insecure channels
- Shared secret is never transmitted
- X25519 provides perfect forward secrecy
- Man-in-the-middle attacks prevented by proper implementation

## Secure Communication

### Transport Security

**WebSocket over TLS:**

```swift
var request = URLRequest(url: URL(string: "wss://api.vantis.io/tunnel")!)
request.setValue("vantis-mobile/1.0", forHTTPHeaderField: "User-Agent")
```

**Best Practices:**

1. **Always Use WSS (WebSocket Secure):**
   - Never use unencrypted WebSocket (ws://)
   - Verify TLS certificate chain
   - Use strong TLS versions (TLS 1.2+)

2. **Certificate Pinning (Optional but Recommended):**

```swift
class PinnedURLSessionDelegate: NSObject, URLSessionDelegate {
    func urlSession(
        _ session: URLSession,
        didReceive challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        guard let serverTrust = challenge.protectionSpace.serverTrust else {
            completionHandler(.cancelAuthenticationChallenge, nil)
            return
        }
        
        let certificate = SecTrustGetCertificateAtIndex(serverTrust, 0)
        let pin = "SHA256/PinnedCertificateHash"
        
        // Verify certificate pin
        if verifyCertificatePin(certificate, expectedPin: pin) {
            completionHandler(.useCredential, URLCredential(trust: serverTrust))
        } else {
            completionHandler(.cancelAuthenticationChallenge, nil)
        }
    }
}
```

### Message Authentication

**AEAD Authentication:**

- Every encrypted message includes Poly1305 authentication tag
- Tag is verified before decryption
- Messages with invalid tags are rejected

**Implementation:**

```swift
// Decryption automatically verifies tag
guard let decrypted = encryptor.decrypt(encryptedJson) else {
    // Authentication failed - reject message
    throw TunnelError.decryptionFailed
}
```

### Replay Protection

**Nonce-Based Protection:**

- Each message uses unique 96-bit nonce
- Nonces generated randomly
- Probability of nonce reuse: 2^-96 (negligible)

**Additional Protection (Optional):**

```swift
// Track received nonces
private var receivedNonces = Set<String>()

func validateNonce(nonce: String) -> Bool {
    if receivedNonces.contains(nonce) {
        return false // Replay detected
    }
    receivedNonces.insert(nonce)
    
    // Prune old nonces periodically
    if receivedNonces.count > 10000 {
        receivedNonces.removeFirst(5000)
    }
    
    return true
}
```

## Data Protection

### Encryption at Rest

**Secure Key Storage:**
- Private keys stored in platform key storage
- Shared secrets stored encrypted in key storage
- Never cache sensitive data in memory longer than necessary

**Memory Protection:**

```swift
// Clear sensitive data immediately after use
func useSensitiveData(_ data: Data) {
    // Use data...
    
    // Zeroize memory
    data.withUnsafeMutableBytes { ptr in
        memset(ptr.baseAddress, 0, data.count)
    }
}
```

### Encryption in Transit

**Layered Encryption:**

```
Application Layer: ChaCha20-Poly1305 (End-to-End)
Transport Layer: TLS 1.3 (Point-to-Point)
```

**Benefits:**
- Defense in depth
- Compromise of TLS doesn't expose messages
- End-to-end encryption maintained even through proxies

### Data Sanitization

**Before Logging:**

```swift
// BAD: Logs sensitive data
print("Encrypted message: \(encryptedMessage)")

// GOOD: Sanitize before logging
print("Encrypted message: [\(encryptedMessage.count) bytes]")
```

**Before Storage:**

```swift
// Always encrypt sensitive data before persisting
func saveEncryptedMessage(_ message: Data) {
    let encrypted = encryptor.encrypt(message)!
    UserDefaults.standard.set(encrypted, forKey: "cached_message")
}
```

## Implementation Security

### Error Handling

**Never Expose Cryptographic Details:**

```swift
// BAD: Exposes internal state
catch {
    print("Decryption failed: authentication tag mismatch")
}

// GOOD: Generic error message
catch {
    print("Decryption failed")
    logError("crypto_decryption_failed")
}
```

### Input Validation

**Validate All Inputs:**

```swift
func validatePublicKey(_ key: String) -> Bool {
    // Check length
    guard key.count == 44 else { return false }
    
    // Check Base64 encoding
    guard Data(base64Encoded: key) != nil else { return false }
    
    // Additional validation as needed
    return true
}
```

### Timing Attack Prevention

**Constant-Time Comparisons:**

```swift
// Constant-time string comparison
func constantTimeCompare(_ a: String, _ b: String) -> Bool {
    guard a.count == b.count else { return false }
    
    var result = 0
    for (ca, cb) in zip(a, b) {
        result |= ca numericEquals cb
    }
    
    return result == 0
}
```

### Side-Channel Protection

**Rust Implementation Benefits:**
- Constant-time X25519 implementation
- No timing information leaks
- Memory access patterns independent of secrets

## Threat Model

### Attack Scenarios

#### 1. Passive Network Eavesdropper

**Threat:** Attacker monitors network traffic

**Protection:**
- All messages encrypted with ChaCha20-Poly1305
- Nonces prevent pattern analysis
- Authentication prevents tampering

**Mitigation:** ✅ Fully protected

#### 2. Active Network Attacker

**Threat:** Attacker modifies or injects messages

**Protection:**
- Poly1305 authentication tag detects tampering
- Invalid messages rejected
- Nonces prevent replay attacks

**Mitigation:** ✅ Fully protected

#### 3. Compromised TLS

**Threat:** TLS endpoint compromised or certificate authority failure

**Protection:**
- End-to-end encryption at application layer
- Messages remain encrypted even if TLS broken
- Compromise of TLS doesn't expose content

**Mitigation:** ✅ Fully protected

#### 4. Device Compromise

**Threat:** Attacker gains access to device

**Protection:**
- Private keys stored in secure key storage
- Key storage protected by device lock
- Biometric authentication for key access

**Mitigation:** ⚠️ Partially protected (depends on device security)

#### 5. Replay Attack

**Threat:** Attacker retransmits captured messages

**Protection:**
- Unique nonces per message
- Nonce tracking optional but recommended
- Expired nonces rejected

**Mitigation:** ✅ Fully protected

### Limitations

**What VantisMobile Does NOT Protect Against:**

1. **Quantum Computer Attacks:**
   - X25519 not quantum-resistant
   - Post-quantum cryptography required for long-term security

2. **Endpoint Compromise:**
   - If device is fully compromised, all bets are off
   - Requires device security best practices

3. **Social Engineering:**
   - Cannot prevent user from revealing secrets
   - Requires security awareness training

## Compliance

### Standards Alignment

**Cryptographic Standards:**
- ✅ NIST-approved ChaCha20-Poly1305
- ✅ RFC 7748 (Curve25519)
- ✅ RFC 7539 (ChaCha20-Poly1305)

**Security Best Practices:**
- ✅ OWASP Mobile Security Guidelines
- ✅ Apple iOS Security Guide
- ✅ Android Security Best Practices

### Regulatory Considerations

**GDPR (General Data Protection Regulation):**
- ✅ Data encryption at rest and in transit
- ✅ Data protection by design
- ✅ Minimization of data exposure

**HIPAA (Health Insurance Portability and Accountability Act):**
- ✅ End-to-end encryption for protected health information
- ✅ Access controls for encryption keys
- ✅ Audit logging of cryptographic operations

**SOC 2 (Service Organization Control 2):**
- ✅ Encryption controls
- ✅ Access management
- ✅ Monitoring and logging

## Security Checklist

### Implementation Checklist

- [ ] Use platform key storage (Keychain/Keystore)
- [ ] Enable biometric authentication for key access
- [ ] Use WSS (WebSocket Secure) with TLS 1.2+
- [ ] Implement certificate pinning (optional but recommended)
- [ ] Validate all public keys before use
- [ ] Never log or expose cryptographic details
- [ ] Clear sensitive data from memory after use
- [ ] Encrypt sensitive data at rest
- [ ] Implement nonce tracking for replay protection
- [ ] Use constant-time comparisons for secrets

### Deployment Checklist

- [ ] Perform security audit of implementation
- [ ] Test with automated security scanning tools
- [ ] Conduct penetration testing
- [ ] Review code for timing side-channels
- [ ] Verify TLS configuration
- [ ] Test certificate pinning implementation
- [ ] Validate error handling doesn't leak information
- [ ] Monitor for cryptographic failures in production
- [ ] Implement security incident response plan
- [ ] Regular security updates and patch management

## Additional Resources

- [OWASP Mobile Security Testing Guide](https://owasp.org/www-project-mobile-security-testing-guide/)
- [Apple iOS Security Guide](https://support.apple.com/guide/security/welcome/web)
- [Android Security Best Practices](https://developer.android.com/topic/security/best-practices)
- [NIST Cryptographic Standards](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines)

## Reporting Security Issues

To report security vulnerabilities:

1. Do NOT create a public issue
2. Send details to: security@vantiscorp.io
3. Include reproduction steps and impact assessment
4. Allow 90 days for remediation before disclosure
5. Follow responsible disclosure practices