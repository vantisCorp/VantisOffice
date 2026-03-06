# VantisMobile - Android Application

The official Android companion app for VantisOffice, providing seamless access to your documents, notifications, and remote workspace features.

## Features

### Core Functionality
- **Secure Tunnel**: WebSocket-based encrypted connection to VantisOffice desktop
- **Document Management**: View, search, and manage documents from anywhere
- **Real-time Notifications**: Push notifications for important events
- **Biometric Authentication**: Fingerprint / Face recognition for secure access
- **Device Discovery**: Automatic discovery of VantisOffice instances on local network
- **QR Code Scanning**: Easy device pairing via QR codes

### UI/UX
- Modern Material 3 design with Jetpack Compose
- Dark mode support
- Bottom navigation for easy access
- Smooth animations and transitions
- Accessibility support

## Architecture

### Components

#### Models
- `Device.kt`: Device information, types, and pairing state
- `Document.kt`: Document metadata, types, and filtering
- `Notification.kt`: Notification system with priorities and grouping
- `Connection.kt`: Connection status, quality, and discovery

#### Services
- `SecureTunnelService.kt`: WebSocket communication with encrypted messaging
- `BiometricAuthService.kt`: Fingerprint / Face authentication wrapper

#### UI
- `MainActivity.kt`: Main activity entry point
- `MainApp.kt`: Bottom navigation container
- `HomeScreen.kt`: Dashboard with connection status and quick actions
- `DocumentsScreen.kt`: Document list with search and filters
- `NotificationsScreen.kt`: Notification center with grouping
- `SettingsScreen.kt`: App settings and configuration

#### Theme
- `Color.kt`: Material 3 color scheme
- `Typography.kt`: Typography definitions
- `Theme.kt`: Dark/Light theme configuration

## Requirements

- Android 7.0 (API 24) or higher
- Android Studio Hedgehog or later
- Kotlin 1.9.20 or higher
- Java 17

## Building

### Using Android Studio
1. Clone the repository
2. Open the project in Android Studio
3. Sync Gradle files
4. Select target device (Emulator or physical device)
5. Build and Run

### Using Command Line
```bash
# Build debug APK
./gradlew assembleDebug

# Build release APK
./gradlew assembleRelease

# Install on connected device
./gradlew installDebug
```

### Running Tests
```bash
# Run unit tests
./gradlew test

# Run instrumentation tests
./gradlew connectedAndroidTest
```

## Permissions

The app requires the following permissions:
- **INTERNET**: For WebSocket communication
- **ACCESS_NETWORK_STATE**: For network status detection
- **CAMERA**: For QR code scanning during device pairing
- **USE_BIOMETRIC**: For fingerprint/face authentication
- **POST_NOTIFICATIONS**: For push notifications
- **VIBRATE**: For notification feedback

All permissions are optional and only requested when needed.

## Security

- End-to-end encryption using ChaCha20-Poly1305
- X25519 key exchange for secure pairing
- Biometric authentication for sensitive operations
- All data transmitted through encrypted WebSocket tunnel
- Secure storage of credentials using Android Keystore

## Configuration

### Build Variants

The app supports two build variants:
- **debug**: Development build with logging and debugging features
- **release**: Production build with ProGuard/R8 obfuscation

### Dependencies

Key dependencies include:
- Jetpack Compose (UI)
- Material 3 (Design)
- Kotlin Coroutines (Async)
- Kotlin Serialization (JSON)
- Java-WebSocket (WebSocket)
- Biometric (Authentication)
- CameraX (QR Scanning)

## Future Enhancements

- [ ] FFI bindings for vantis-mobile Rust library
- [ ] Document preview and editing capabilities
- [ ] Offline mode with local caching using Room
- [ ] Multiple workspace support
- [ ] File upload/download with progress tracking
- [ ] Wear OS companion app
- [ ] Widget support
- [ ] Auto-sync with WorkManager

## Contributing

This is a private project for VantisOffice. Contributions are managed by the VantisCorp team.

## License

Copyright © 2025 VantisCorp. All rights reserved.