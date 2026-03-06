# VantisMobile FFI API Reference

Complete API reference for the VantisMobile Foreign Function Interface (FFI) bindings.

## Table of Contents

- [Overview](#overview)
- [C API](#c-api)
- [Swift API](#swift-api)
- [Kotlin API](#kotlin-api)
- [Error Handling](#error-handling)
- [Data Types](#data-types)

## Overview

The VantisMobile FFI provides a C-compatible interface to Rust cryptographic primitives, with language-specific wrappers for Swift (iOS) and Kotlin (Android).

**Core Components:**
- KeyPair: X25519 key pair generation
- Encryptor: ChaCha20-Poly1305 AEAD encryption
- DeviceInfo: Device metadata management

## C API

### Version Information

```c
const char* vantis_version(void);
```
Returns the version string of the VantisMobile library.

**Returns:** Const char pointer to version string (e.g., "0.1.0")

**Example:**
```c
const char* version = vantis_version();
printf("VantisMobile version: %s\n", version);
```

### Initialization

```c
int vantis_init(void);
```
Initializes the VantisMobile library. Must be called before any other functions.

**Returns:** 0 on success, non-zero on failure

**Example:**
```c
if (vantis_init() != 0) {
    fprintf(stderr, "Failed to initialize VantisMobile\n");
    return 1;
}
```

### Cleanup

```c
void vantis_cleanup(void);
```
Cleans up resources used by the VantisMobile library. Should be called when done.

**Example:**
```c
vantis_cleanup();
```

### KeyPair Operations

#### Generate KeyPair

```c
vantis_keypair_t* vantis_keypair_generate(void);
```
Generates a new X25519 key pair.

**Returns:** Pointer to vantis_keypair_t struct, or NULL on failure

**Example:**
```c
vantis_keypair_t* keypair = vantis_keypair_generate();
if (!keypair) {
    fprintf(stderr, "Failed to generate keypair\n");
    return 1;
}
```

#### Get Public Key

```c
char* vantis_keypair_public_key_base64(const vantis_keypair_t* keypair);
```
Returns the public key in Base64 encoding.

**Parameters:**
- `keypair`: Pointer to keypair struct

**Returns:** Newly allocated Base64 string (caller must free), or NULL on failure

**Example:**
```c
char* public_key = vantis_keypair_public_key_base64(keypair);
if (public_key) {
    printf("Public key: %s\n", public_key);
    free(public_key);
}
```

#### Free KeyPair

```c
void vantis_keypair_free(vantis_keypair_t* keypair);
```
Frees a keypair struct and its resources.

**Parameters:**
- `keypair`: Pointer to keypair struct (can be NULL)

**Example:**
```c
vantis_keypair_free(keypair);
```

### Encryptor Operations

#### Create Encryptor

```c
vantis_encryptor_t* vantis_encryptor_create(const char* shared_secret_base64);
```
Creates a new encryptor with the given shared secret.

**Parameters:**
- `shared_secret_base64`: Shared secret in Base64 encoding

**Returns:** Pointer to vantis_encryptor_t struct, or NULL on failure

**Example:**
```c
vantis_encryptor_t* encryptor = vantis_encryptor_create("SGVsbG8gV29ybGQ=");
if (!encryptor) {
    fprintf(stderr, "Failed to create encryptor\n");
    return 1;
}
```

#### Encrypt

```c
char* vantis_encrypt(vantis_encryptor_t* encryptor, const char* plaintext, size_t len);
```
Encrypts plaintext data and returns JSON string with nonce, ciphertext, and tag.

**Parameters:**
- `encryptor`: Pointer to encryptor struct
- `plaintext`: Pointer to plaintext data
- `len`: Length of plaintext data

**Returns:** Newly allocated JSON string (caller must free), or NULL on failure

**JSON Format:**
```json
{
  "nonce": "base64-encoded-nonce",
  "ciphertext": "base64-encoded-ciphertext",
  "tag": "base64-encoded-auth-tag"
}
```

**Example:**
```c
const char* message = "Hello, World!";
char* encrypted = vantis_encrypt(encryptor, message, strlen(message));
if (encrypted) {
    printf("Encrypted: %s\n", encrypted);
    free(encrypted);
}
```

#### Decrypt

```c
char* vantis_decrypt(vantis_encryptor_t* encryptor, const char* encrypted_json);
```
Decrypts an encrypted JSON message and returns plaintext.

**Parameters:**
- `encryptor`: Pointer to encryptor struct
- `encrypted_json`: JSON string containing encrypted data

**Returns:** Newly allocated plaintext string (caller must free), or NULL on failure

**Example:**
```c
char* decrypted = vantis_decrypt(encryptor, encrypted);
if (decrypted) {
    printf("Decrypted: %s\n", decrypted);
    free(decrypted);
}
```

#### Free Encryptor

```c
void vantis_encryptor_free(vantis_encryptor_t* encryptor);
```
Frees an encryptor struct and its resources.

**Parameters:**
- `encryptor`: Pointer to encryptor struct (can be NULL)

**Example:**
```c
vantis_encryptor_free(encryptor);
```

### DeviceInfo Operations

#### Create DeviceInfo

```c
vantis_device_info_t* vantis_device_info_create(
    int device_type,
    const char* name,
    const char* os_version,
    const char* app_version
);
```
Creates a device info struct with the given parameters.

**Parameters:**
- `device_type`: Device type enum value (0=Ios, 1=Android, 2=Desktop, 3=Laptop, 4=Tablet)
- `name`: Device name string
- `os_version`: OS version string
- `app_version`: App version string

**Returns:** Pointer to vantis_device_info_t struct, or NULL on failure

**Example:**
```c
vantis_device_info_t* info = vantis_device_info_create(
    1,  // Android
    "Samsung Galaxy",
    "13.0",
    "1.0.0"
);
```

#### Get Device Info JSON

```c
char* vantis_device_info_to_json(const vantis_device_info_t* info);
```
Returns the device info as a JSON string.

**Parameters:**
- `info`: Pointer to device info struct

**Returns:** Newly allocated JSON string (caller must free), or NULL on failure

**Example:**
```c
char* json = vantis_device_info_to_json(info);
if (json) {
    printf("Device info: %s\n", json);
    free(json);
}
```

#### Free DeviceInfo

```c
void vantis_device_info_free(vantis_device_info_t* info);
```
Frees a device info struct and its resources.

**Parameters:**
- `info`: Pointer to device info struct (can be NULL)

**Example:**
```c
vantis_device_info_free(info);
```

## Swift API

### VantisMobileFFI (Singleton)

```swift
let ffi = VantisMobileFFI.shared
```
Singleton instance for library initialization.

**Properties:**
- `version`: String - Library version

**Methods:**
```swift
let ffi = VantisMobileFFI.shared
let version = ffi.version
```

### KeyPair Class

```swift
class KeyPair {
    public let publicKeyBase64: String
    public var free: (() -> Void)?
    
    public init()
    public func free()
}
```

**Example:**
```swift
let keyPair = VantisMobileFFI.KeyPair()
let publicKey = keyPair.publicKeyBase64

// When done
keyPair.free?()
```

### Encryptor Class

```swift
class Encryptor {
    public init?(sharedSecretBase64: String)
    
    public func encrypt(_ data: Data) -> String?
    public func decrypt(_ json: String) -> Data?
}
```

**Parameters:**
- `sharedSecretBase64`: Shared secret in Base64 encoding

**Methods:**
- `encrypt(_ data: Data) -> String?`: Encrypts data and returns JSON string
- `decrypt(_ json: String) -> Data?`: Decrypts JSON string and returns data

**Example:**
```swift
guard let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: "key") else {
    return
}

let plaintext = "Hello, World!".data(using: .utf8)!
let encrypted = encryptor.encrypt(plaintext)
let decrypted = encryptor.decrypt(encrypted!)
```

## Kotlin API

### VantisKeyPair Class

```kotlin
class VantisKeyPair {
    val publicKeyBase64: String?
    
    init()
    fun free()
}
```

**Example:**
```kotlin
val keyPair = VantisMobileFFI.VantisKeyPair()
val publicKey = keyPair.publicKeyBase64

// When done
keyPair.free()
```

### VantisEncryptor Class

```kotlin
class VantisEncryptor {
    constructor(sharedSecretBase64: String)
    
    fun encrypt(plaintext: ByteArray): String?
    fun decrypt(json: String): ByteArray?
}
```

**Parameters:**
- `sharedSecretBase64`: Shared secret in Base64 encoding

**Methods:**
- `encrypt(plaintext: ByteArray): String?`: Encrypts data and returns JSON string
- `decrypt(json: String): ByteArray?`: Decrypts JSON string and returns bytes

**Example:**
```kotlin
val encryptor = VantisMobileFFI.VantisEncryptor("key")
val plaintext = "Hello, World!".toByteArray()
val encrypted = encryptor.encrypt(plaintext)
val decrypted = encryptor.decrypt(encrypted!!)
```

### VantisDeviceInfo Class

```kotlin
class VantisDeviceInfo {
    val toJson: String?
    
    constructor(deviceType: VantisDeviceType, name: String, osVersion: String, appVersion: String)
    fun free()
}
```

**Parameters:**
- `deviceType`: Device type enum (IOS, ANDROID, DESKTOP, LAPTOP, TABLET)
- `name`: Device name
- `osVersion`: OS version
- `appVersion`: App version

**Example:**
```kotlin
val info = VantisMobileFFI.VantisDeviceInfo(
    deviceType = VantisMobileFFI.VantisDeviceType.ANDROID,
    name = "Samsung Galaxy",
    osVersion = "13.0",
    appVersion = "1.0.0"
)
val json = info.toJson
info.free()
```

### VantisDeviceType Enum

```kotlin
enum class VantisDeviceType(val value: Int) {
    IOS(0),
    ANDROID(1),
    DESKTOP(2),
    LAPTOP(3),
    TABLET(4)
}
```

## Error Handling

### C API Errors

All C API functions return NULL or 0 on failure. Common causes:

- `vantis_keypair_generate()` - NULL: Memory allocation failure
- `vantis_encryptor_create()` - NULL: Invalid Base64 string
- `vantis_encrypt()` - NULL: Encryption failure
- `vantis_decrypt()` - NULL: Decryption failure, invalid JSON, or authentication failure

**Best Practices:**
```c
vantis_keypair_t* keypair = vantis_keypair_generate();
if (!keypair) {
    // Handle error
    fprintf(stderr, "Failed to generate keypair\n");
    return 1;
}
```

### Swift Errors

Swift API uses Optional return values for error handling:

```swift
guard let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: "key") else {
    // Handle error
    print("Failed to create encryptor")
    return
}

guard let encrypted = encryptor.encrypt(plaintext) else {
    // Handle error
    print("Failed to encrypt")
    return
}
```

### Kotlin Errors

Kotlin API uses nullable return types for error handling:

```kotlin
val encryptor = VantisMobileFFI.VantisEncryptor("key") ?: run {
    // Handle error
    println("Failed to create encryptor")
    return
}

val encrypted = encryptor.encrypt(plaintext) ?: run {
    // Handle error
    println("Failed to encrypt")
    return
}
```

## Data Types

### C Structs

```c
typedef struct vantis_keypair_t vantis_keypair_t;
typedef struct vantis_encryptor_t vantis_encryptor_t;
typedef struct vantis_device_info_t vantis_device_info_t;
```

All structs are opaque pointers managed by the Rust library.

### JSON Formats

#### Encrypted Message

```json
{
  "nonce": "base64-encoded-nonce",
  "ciphertext": "base64-encoded-ciphertext",
  "tag": "base64-encoded-auth-tag"
}
```

**Fields:**
- `nonce`: 12-byte nonce in Base64 (96 bits)
- `ciphertext`: Encrypted data in Base64
- `tag`: Poly1305 authentication tag in Base64 (16 bytes)

#### Device Info

```json
{
  "device_type": "android",
  "name": "Samsung Galaxy",
  "os_version": "13.0",
  "app_version": "1.0.0"
}
```

### Device Type Mapping

| Value | Type    | Description              |
|-------|---------|--------------------------|
| 0     | iOS     | iOS device               |
| 1     | Android | Android device           |
| 2     | Desktop | Desktop computer         |
| 3     | Laptop  | Laptop computer          |
| 4     | Tablet  | Tablet device            |

### Encryption Parameters

- **Algorithm**: ChaCha20-Poly1305 AEAD
- **Nonce Size**: 12 bytes (96 bits)
- **Key Size**: 32 bytes (256 bits)
- **Tag Size**: 16 bytes (128 bits)
- **Key Exchange**: X25519

## Memory Management

### C API

All returned strings must be freed by the caller:

```c
char* str = vantis_keypair_public_key_base64(keypair);
// Use str...
free(str);
```

### Swift API

Swift wrapper handles automatic memory management:

```swift
let keyPair = VantisMobileFFI.KeyPair()
// Automatic cleanup when object is deallocated
```

Explicit cleanup:
```swift
keyPair.free?()
```

### Kotlin API

Kotlin wrapper requires explicit cleanup:

```kotlin
val keyPair = VantisMobileFFI.VantisKeyPair()
// Use keyPair...
keyPair.free()
```

## Thread Safety

- All API functions are thread-safe
- Multiple encryptors can be used concurrently
- KeyPair operations are thread-safe after creation

## Performance Considerations

- Reuse Encryptor instances instead of recreating
- Generate nonce only once per encryption (handled internally)
- Use Base64 encoding for network transmission
- Batch operations for multiple messages

## Security Best Practices

1. **Key Management**: Never expose private keys
2. **Nonce Reuse**: Nonces are generated randomly and never reused
3. **Memory Security**: Zeroize sensitive data when done
4. **Error Handling**: Always check return values
5. **Version Compatibility**: Verify library version on init

## Additional Resources

- [Integration Guide](../mobile/INTEGRATION_GUIDE.md)
- [Security Considerations](../mobile/SECURITY.md)
- [Deployment Guide](../deployment/DEPLOYMENT.md)