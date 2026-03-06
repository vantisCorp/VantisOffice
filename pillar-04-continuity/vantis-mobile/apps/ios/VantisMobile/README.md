# VantisMobile - iOS Application

The official iOS companion app for VantisOffice, providing seamless access to your documents, notifications, and remote workspace features.

## Features

### Core Functionality
- **Secure Tunnel**: WebSocket-based encrypted connection to VantisOffice desktop
- **Document Management**: View, search, and manage documents from anywhere
- **Real-time Notifications**: Push notifications for important events
- **Biometric Authentication**: Face ID / Touch ID for secure access
- **Device Discovery**: Automatic discovery of VantisOffice instances on local network

### UI/UX
- Native iOS design with SwiftUI
- Dark mode support
- Tab-based navigation (Home, Documents, Notifications, Settings)
- Smooth animations and transitions
- Accessibility support

## Architecture

### Components

#### Models
- `Device.swift`: Device information and type detection
- `Document.swift`: Document metadata and types
- `Notification.swift`: Notification system with priorities
- `Connection.swift`: Connection status and info

#### Services
- `SecureTunnelService.swift`: WebSocket communication protocol
- `BiometricAuthService.swift`: Face ID / Touch ID authentication

#### Views
- `ContentView.swift`: Main TabView container
- `HomeView.swift`: Dashboard with connection status and quick actions
- `DocumentsView.swift`: Document list with search and filters
- `NotificationsView.swift`: Notification center with grouping
- `SettingsView.swift`: App settings and configuration
- `ConnectView.swift`: Server connection setup with QR scanner

## Requirements

- iOS 16.0+
- Xcode 15.0+
- Swift 5.9+

## Building

### Using Xcode
1. Open the project in Xcode
2. Select target device (iPhone Simulator or physical device)
3. Build and Run (⌘R)

### Using Swift Package Manager
```bash
cd VantisMobile
swift build
```

### Running Tests
```bash
swift test
```

## Permissions

The app requires the following permissions:
- **Camera**: For QR code scanning during device pairing
- **Face ID / Touch ID**: For biometric authentication
- **Local Network**: For discovering VantisOffice instances

All permissions are optional and only requested when needed.

## Security

- End-to-end encryption using ChaCha20-Poly1305
- X25519 key exchange for secure pairing
- Biometric authentication for sensitive operations
- All data transmitted through encrypted WebSocket tunnel

## Future Enhancements

- [ ] FFI bindings for vantis-mobile Rust library
- [ ] Document preview and editing capabilities
- [ ] Offline mode with local caching
- [ ] Multiple workspace support
- [ ] File upload/download
- [ ] Apple Watch companion app
- [ ] Spotlight integration
- [ ] Widget support

## Contributing

This is a private project for VantisOffice. Contributions are managed by the VantisCorp team.

## License

Copyright © 2025 VantisCorp. All rights reserved.