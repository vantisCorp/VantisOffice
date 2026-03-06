# Vantis Mobile Companion

## Overview

Vantis Mobile Companion is a mobile application for iOS and Android that provides secure remote access to VantisOffice documents and notifications through encrypted tunnels to your computer.

## Key Features

- **Secure Tunnel**: End-to-end encrypted connection to computer
- **Document Viewer**: View documents on mobile devices
- **Notification Access**: Receive and manage notifications
- **Remote Control**: Control VantisOffice remotely
- **Offline Mode**: Access documents offline
- **Biometric Auth**: Secure mobile authentication

## Architecture

```
vantis-mobile/
├── apps/
│   ├── ios/                   # iOS application
│   │   ├── VantisMobile/
│   │   │   ├── Models/        # Data models
│   │   │   ├── Views/         # UI views
│   │   │   ├── ViewModels/    # View models
│   │   │   ├── Services/      # Services
│   │   │   └── Networking/    # Network layer
│   │   └── VantisMobileTests/
│   └── android/               # Android application
│       ├── app/
│       │   ├── src/
│       │   │   ├── main/
│       │   │   │   ├── java/  # Java code
│       │   │   │   ├── res/   # Resources
│       │   │   │   └── assets/
│       │   │   └── test/
│       └── gradle/
├── shared/
│   ├── protocol/              # Communication protocol
│   ├── crypto/                # Cryptographic utilities
│   ├── models/                # Shared data models
│   └── utils/                 # Shared utilities
├── backend/
│   ├── tunnel/                # Tunnel server
│   ├── sync/                  # Sync service
│   ├── notification/          # Notification service
│   └── auth/                  # Authentication service
└── docs/
    ├── api/                   # API documentation
    └── deployment/            # Deployment guides
```

## Secure Tunnel

### Connection Establishment

```swift
// iOS - Swift
import VantisMobile

let tunnel = SecureTunnel()
let config = TunnelConfig(
    serverURL: URL(string: "https://tunnel.vantis.ai")!,
    deviceID: deviceID,
    encryptionKey: encryptionKey
)

tunnel.connect(config: config) { result in
    switch result {
    case .success:
        print("Connected to computer")
    case .failure(let error):
        print("Connection failed: \(error)")
    }
}
```

```kotlin
// Android - Kotlin
import com.vantis.mobile.SecureTunnel

val tunnel = SecureTunnel()
val config = TunnelConfig(
    serverUrl = "https://tunnel.vantis.ai",
    deviceId = deviceId,
    encryptionKey = encryptionKey
)

tunnel.connect(config) { result ->
    if (result.isSuccess) {
        println("Connected to computer")
    } else {
        println("Connection failed: ${result.exceptionOrNull()}")
    }
}
```

### End-to-End Encryption

```swift
// Key exchange
let keyExchange = KeyExchange()
let keyPair = keyExchange.generateKeyPair()

// Send public key to computer
tunnel.sendPublicKey(keyPair.publicKey)

// Receive computer's public key
tunnel.onPublicKeyReceived { publicKey in
    // Derive shared secret
    let sharedSecret = keyExchange.deriveSharedSecret(
        privateKey: keyPair.privateKey,
        publicKey: publicKey
    )
    
    // Use shared secret for encryption
    tunnel.setEncryptionKey(sharedSecret)
}
```

## Document Viewer

### Viewing Documents

```swift
// iOS
import VantisMobile

let viewer = DocumentViewer()

// Load document
viewer.loadDocument(
    documentID: "doc-123",
    format: .vdoc
) { result in
    switch result {
    case .success(let document):
        self.showDocument(document)
    case .failure(let error):
        self.showError(error)
    }
}
```

```kotlin
// Android
import com.vantis.mobile.DocumentViewer

val viewer = DocumentViewer()

// Load document
viewer.loadDocument(
    documentId = "doc-123",
    format = DocumentFormat.VDOC
) { result ->
    result.onSuccess { document ->
        showDocument(document)
    }.onFailure { error ->
        showError(error)
    }
}
```

### Document Operations

```swift
// iOS
// Open document in Vantis Writer
viewer.openInApp(document: document, app: .writer)

// Download for offline access
viewer.downloadDocument(document: document) { progress in
    print("Download progress: \(progress)%")
}

// Share document
viewer.shareDocument(document: document)
```

```kotlin
// Android
// Open document in Vantis Writer
viewer.openInApp(document, AppType.WRITER)

// Download for offline access
viewer.downloadDocument(document) { progress ->
    println("Download progress: $progress%")
}

// Share document
viewer.shareDocument(document)
```

## Notification System

### Receiving Notifications

```swift
// iOS
import VantisMobile

let notificationManager = NotificationManager()

// Request permission
notificationManager.requestPermission { granted in
    if granted {
        // Register for notifications
        notificationManager.register()
    }
}

// Handle incoming notifications
notificationManager.onNotificationReceived { notification in
    self.showNotification(notification)
}
```

```kotlin
// Android
import com.vantis.mobile.NotificationManager

val notificationManager = NotificationManager()

// Request permission
if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS)
    == PackageManager.PERMISSION_GRANTED) {
    
    // Register for notifications
    notificationManager.register()
}

// Handle incoming notifications
notificationManager.onNotificationReceived { notification ->
    showNotification(notification)
}
```

### Notification Types

```swift
// Document updates
NotificationType.documentUpdated(documentID: "doc-123")

// Collaboration alerts
NotificationType.collaborationRequest(from: "Alice", documentID: "doc-123")

// System notifications
NotificationType.system(message: "Backup completed")

// Calendar events
NotificationType.calendarEvent(eventID: "event-456")
```

### Notification Actions

```swift
// iOS
let action = NotificationAction(
    identifier: "approve",
    title: "Approve"
)

let category = NotificationCategory(
    identifier: "collaboration",
    actions: [action]
)

NotificationManager.registerCategory(category)
```

## Remote Control

### Controlling Applications

```swift
// iOS
import VantisMobile

let controller = RemoteController()

// Open application
controller.openApp(.writer)

// Execute command
controller.executeCommand(
    command: "save",
    parameters: ["documentID": "doc-123"]
)

// Get application state
controller.getState(for: .writer) { state in
    print("App state: \(state)")
}
```

```kotlin
// Android
import com.vantis.mobile.RemoteController

val controller = RemoteController()

// Open application
controller.openApp(AppType.WRITER)

// Execute command
controller.executeCommand(
    command = "save",
    parameters = mapOf("documentID" to "doc-123")
)

// Get application state
controller.getState(AppType.WRITER) { state ->
    println("App state: $state")
}
```

## Offline Mode

### Caching Documents

```swift
// iOS
let cacheManager = CacheManager()

// Cache document for offline access
cacheManager.cacheDocument(document: document) {
    print("Document cached")
}

// Check cache status
let isCached = cacheManager.isDocumentCached(documentID: "doc-123")

// Get cached document
if isCached {
    let cached = cacheManager.getCachedDocument(documentID: "doc-123")
    showDocument(cached)
}
```

```kotlin
// Android
val cacheManager = CacheManager()

// Cache document for offline access
cacheManager.cacheDocument(document) {
    println("Document cached")
}

// Check cache status
val isCached = cacheManager.isDocumentCached("doc-123")

// Get cached document
if (isCached) {
    val cached = cacheManager.getCachedDocument("doc-123")
    showDocument(cached)
}
```

### Sync on Reconnect

```swift
// iOS
tunnel.onReconnect {
    // Sync changes
    SyncManager.syncChanges()
    
    // Clear cache if needed
    CacheManager.clearExpiredCache()
}
```

## Biometric Authentication

```swift
// iOS
import LocalAuthentication

let authManager = BiometricAuthManager()

// Authenticate with Face ID
authManager.authenticate(reason: "Access documents") { result in
    switch result {
    case .success:
        print("Authenticated")
        // Grant access
    case .failure(let error):
        print("Authentication failed: \(error)")
        // Deny access
    }
}
```

```kotlin
// Android
import androidx.biometric.BiometricPrompt

val authManager = BiometricAuthManager()

// Authenticate with fingerprint
authManager.authenticate(
    promptInfo = BiometricPrompt.PromptInfo.Builder()
        .setTitle("Access documents")
        .setSubtitle("Use your fingerprint to continue")
        .build()
) { result ->
    if (result.isSuccess) {
        println("Authenticated")
        // Grant access
    } else {
        println("Authentication failed")
        // Deny access
    }
}
```

## API Examples

### Initializing the App

```swift
// iOS
import VantisMobile

let app = VantisMobileApp()
app.initialize(
    config: AppConfig(
        serverURL: URL(string: "https://api.vantis.ai")!,
        deviceID: deviceID,
        encryptionKey: encryptionKey
    )
)
```

```kotlin
// Android
import com.vantis.mobile.VantisMobileApp

val app = VantisMobileApp()
app.initialize(
    AppConfig(
        serverUrl = "https://api.vantis.ai",
        deviceId = deviceId,
        encryptionKey = encryptionKey
    )
)
```

### Managing Documents

```swift
// List documents
let documents = app.documents.list()

// Search documents
let results = app.documents.search(query: "project")

// Download document
app.documents.download(documentID: "doc-123") { url in
    // Handle downloaded file
}
```

## Integration Points

- **Vantis Link**: Real-time sync
- **Vantis Vault**: Document encryption
- **Vantis Ark**: Mobile backup
- **Vantis Chronos**: Calendar notifications
- **All VantisOffice Apps**: Remote control

## Configuration

```toml
# mobile.toml
[tunnel]
server_url = "https://tunnel.vantis.ai"
reconnect_interval = 5
keepalive_interval = 30
encryption_algorithm = "chacha20-poly1305"

[documents]
cache_size_mb = 500
auto_cache = true
offline_mode = true

[notifications]
enabled = true
sound_enabled = true
vibration_enabled = true

[authentication]
biometric_enabled = true
pin_enabled = true
session_timeout = 3600
```

## Security Features

1. **E2EE Tunnel**: All communication encrypted
2. **Biometric Auth**: Secure mobile authentication
3. **Certificate Pinning**: Prevent MITM attacks
4. **Secure Storage**: Encrypted local storage
5. **Session Management**: Secure session handling
6. **Key Rotation**: Regular key rotation

## Performance Metrics

- **Connection Time**: <2s
- **Document Load**: <1s for average document
- **Sync Latency**: <500ms
- **Notification Delivery**: <1s
- **Cache Hit Rate**: >95%
- **Battery Impact**: <5% per day

## Supported Platforms

### iOS
- **Minimum Version**: iOS 15.0
- **Supported Devices**: iPhone 8 and later
- **Features**: Face ID, Touch ID, Push Notifications

### Android
- **Minimum Version**: Android 8.0 (API 26)
- **Supported Devices**: Wide range of Android devices
- **Features**: Fingerprint, Push Notifications

## Future Roadmap

- [ ] Apple Watch companion
- [ ] Wear OS companion
- [ ] Voice commands
- [ ] Augmented reality preview
- [ ] Advanced offline editing
- [ ] Multiple device support

## Build Requirements

### iOS
- Xcode 15+
- Swift 5.9+
- iOS 15.0 SDK
- CocoaPods

### Android
- Android Studio Hedgehog+
- Kotlin 1.9+
- Android 8.0 SDK
- Gradle 8.0+

### Shared
- Protocol Buffers
- OpenSSL

## FFI Bindings API

The `vantis-mobile` library provides C-compatible FFI bindings for integration with iOS (Swift) and Android (Kotlin/JNI) applications.

### Key Features

- **Cross-platform FFI**: C-compatible interface for mobile platforms
- **End-to-end encryption**: ChaCha20-Poly1305 with X25519 key exchange
- **JSON-based messages**: Encrypted messages serialized as JSON for easy handling
- **Safe wrappers**: Swift and Kotlin wrappers provide memory-safe, idiomatic APIs

### C Header API

```c
// Key Pair Operations
VantisKeyPair* vantis_keypair_generate(void);
void vantis_keypair_free(VantisKeyPair* keypair);
VantisResult vantis_keypair_public_key_base64(const VantisKeyPair* keypair, char* out, unsigned int out_len);

// Encryption Operations
VantisEncryptor* vantis_encryptor_create(const char* shared_secret_base64);
void vantis_encryptor_free(VantisEncryptor* encryptor);
int vantis_encrypt(VantisEncryptor* encryptor, const unsigned char* plaintext, unsigned int plaintext_len, char* json_out, unsigned int* json_out_len);
int vantis_decrypt(VantisEncryptor* encryptor, const char* json_encrypted, unsigned char* plaintext_out, unsigned int* plaintext_len_out);

// Device Info
typedef enum { VANTIS_DEVICE_IOS = 0, VANTIS_DEVICE_ANDROID = 1, VANTIS_DEVICE_DESKTOP = 2, VANTIS_DEVICE_LAPTOP = 3, VANTIS_DEVICE_TABLET = 4 } VantisDeviceType;
VantisDeviceInfo* vantis_device_info_create(const char* name, int device_type, const char* os_version, const char* app_version);
void vantis_device_info_free(VantisDeviceInfo* info);
VantisResult vantis_device_info_to_json(const VantisDeviceInfo* info, char* out, unsigned int out_len);

// Protocol Messages
VantisResult vantis_message_ping(char* out, unsigned int out_len);
VantisResult vantis_message_sync_request(unsigned long last_sync_timestamp, char* out, unsigned int out_len);
VantisResult vantis_message_notification(const char* title, const char* body, const char* notification_type, int priority, char* out, unsigned int out_len);
```

### Swift Usage

```swift
import VantisMobileFFI

// Generate key pair
let keyPair = VantisMobileFFI.KeyPair()!
let publicKey = keyPair.publicKeyBase64

// Create encryptor from base64 key
let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: "your-base64-key")!

// Encrypt data - returns JSON string
let plaintext = "Hello, World!".data(using: .utf8)!
let encryptedJSON = encryptor.encrypt(plaintext)

// Decrypt data
if let json = encryptedJSON {
    let decrypted = encryptor.decrypt(json)
    print(String(data: decrypted!, encoding: .utf8)!) // "Hello, World!"
}

// Create device info
let deviceInfo = VantisMobileFFI.DeviceInfo(
    name: "My iPhone",
    deviceType: .ios,
    osVersion: "17.0",
    appVersion: "1.0.0"
)
print(deviceInfo.json!)
```

### Kotlin Usage

```kotlin
import com.vantiscorp.vantismobile.ffi.*

// Generate key pair
val keyPair = VantisKeyPair()
val publicKey = keyPair.publicKeyBase64

// Create encryptor from base64 key
val encryptor = VantisEncryptor("your-base64-key")

// Encrypt data - returns JSON string
val plaintext = "Hello, World!".toByteArray()
val encryptedJSON = encryptor.encrypt(plaintext)

// Decrypt data
encryptedJSON?.let { json ->
    val decrypted = encryptor.decrypt(json)
    println(String(decrypted!!)) // "Hello, World!"
}

// Create device info
val deviceInfo = VantisDeviceInfo(
    name = "My Android",
    deviceType = VantisDeviceType.ANDROID,
    osVersion = "14",
    appVersion = "1.0.0"
)
println(deviceInfo.json)
```

### Build Scripts

The library includes build scripts for cross-compilation:

```bash
# Build iOS XCFramework
./build-ios.sh

# Build Android libraries (multiple ABIs)
./build-android.sh
```

### Error Handling

Both Swift and Kotlin wrappers provide typed error handling:

```swift
// Swift
do {
    let encryptor = try VantisMobileFFI.Encryptor(sharedSecretBase64: key)
} catch let error as VantisError {
    switch error {
    case .invalidData: print("Invalid key")
    case .encryptionError: print("Encryption failed")
    default: print("Unknown error")
    }
}
```

```kotlin
// Kotlin
try {
    val encryptor = VantisEncryptor(key)
} catch (e: VantisError) {
    when (e) {
        is VantisError.InvalidData -> println("Invalid key")
        is VantisError.EncryptionError -> println("Encryption failed")
        else -> println("Unknown error")
    }
}
```

---

**Part of VantisOffice Pillar IV - Critical Tools**