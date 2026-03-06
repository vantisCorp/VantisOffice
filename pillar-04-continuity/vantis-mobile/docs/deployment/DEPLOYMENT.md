# VantisMobile Deployment Guide

This guide covers deployment procedures for VantisMobile FFI bindings to iOS and Android platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [iOS Deployment](#ios-deployment)
- [Android Deployment](#android-deployment)
- [CI/CD Integration](#cicd-integration)
- [Release Process](#release-process)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

**macOS (for iOS builds):**
- macOS Ventura (13.0) or later
- Xcode 15.0 or later
- Command Line Tools for Xcode
- Rust 1.75.0 or later

**Linux/macOS (for Android builds):**
- Rust 1.75.0 or later
- Android NDK r25c or later
- Android SDK API level 21+

**Cross-Platform:**
- Git
- Bash shell
- Basic build tools (make, cmake)

### Rust Toolchain Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add iOS targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Install cargo-ndk (optional, for Android builds)
cargo install cargo-ndk
```

## iOS Deployment

### Building the Library

**1. Clone the Repository:**

```bash
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice/pillar-04-continuity/vantis-mobile
```

**2. Run the iOS Build Script:**

```bash
chmod +x build-ios.sh
./build-ios.sh
```

**3. Build Output:**

The script creates the following in `target/ios/`:

```
target/ios/
├── arm64/
│   └── libvantismobile.a          # Static library for iOS devices
├── x86_64/
│   └── libvantismobile.a          # Static library for iOS simulators
├── universal/
│   ├── libvantismobile.a          # Universal binary (arm64 + x86_64)
│   └── include/
│       └── vantis_mobile.h        # C header file
└── VantisMobile.xcframework       # XCFramework for Xcode
```

### Integrating into iOS Project

#### Option 1: Swift Package Manager

**1. Add to Package.swift:**

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
                .unsafeFlags(["-Lvantis-mobile/target/ios/universal", "-lvantismobile"])
            ]
        ),
    ]
)
```

**2. Import in Swift:**

```swift
import VantisMobileFFI
```

#### Option 2: Manual Integration

**1. Copy XCFramework:**

```bash
cp -R target/ios/VantisMobile.xcframework /path/to/YourApp/Frameworks/
```

**2. Add to Xcode Project:**

- Drag `VantisMobile.xcframework` into your Xcode project
- Ensure it's added to "Frameworks, Libraries, and Embedded Content"

**3. Copy Swift Wrapper:**

```bash
cp apps/ios/VantisMobile/Sources/VantisMobileFFI.swift \
   /path/to/YourApp/Sources/
```

**4. Import in Swift:**

```swift
import VantisMobileFFI
```

### Code Signing

**For App Store Distribution:**

1. **Enable Signing in Xcode:**
   - Select your target
   - Go to "Signing & Capabilities"
   - Select your development team
   - Enable automatic signing

2. **Create Provisioning Profile:**
   - Go to Apple Developer Portal
   - Create App ID with bundle identifier
   - Create provisioning profile
   - Download and add to Xcode

**For TestFlight:**

```bash
# Archive for App Store
xcodebuild archive \
  -workspace YourApp.xcworkspace \
  -scheme YourApp \
  -archivePath build/YourApp.xcarchive \
  -configuration Release

# Export for App Store
xcodebuild -exportArchive \
  -archivePath build/YourApp.xcarchive \
  -exportPath build/Export \
  -exportOptionsPlist ExportOptions.plist
```

### App Store Submission

**1. Prepare Metadata:**

- App screenshots
- App description
- Keywords
- Privacy policy URL
- Support URL

**2. Build and Archive:**

```bash
xcodebuild archive \
  -workspace YourApp.xcworkspace \
  -scheme YourApp \
  -archivePath build/YourApp.xcarchive \
  -configuration Release \
  CODE_SIGN_IDENTITY="Apple Distribution: Your Team"
  PROVISIONING_PROFILE_SPECIFIER="Your App Store Profile"
```

**3. Upload to App Store Connect:**

```bash
xcodebuild -exportArchive \
  -archivePath build/YourApp.xcarchive \
  -exportPath build/Export \
  -exportOptionsPlist ExportOptions.plist \
  -uploadToAppStore
```

**4. Submit for Review:**

- Log in to App Store Connect
- Go to "My Apps"
- Select your app
- Click "Prepare for Submission"
- Fill in all required information
- Submit for review

## Android Deployment

### Building the Library

**1. Clone the Repository:**

```bash
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice/pillar-04-continuity/vantis-mobile
```

**2. Set Android NDK Path:**

```bash
# Set NDK path
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653

# Or add to ~/.bashrc or ~/.zshrc
echo 'export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653' >> ~/.bashrc
source ~/.bashrc
```

**3. Run the Android Build Script:**

```bash
chmod +x build-android.sh
./build-android.sh
```

**4. Build Output:**

The script creates the following in `target/android/`:

```
target/android/
├── arm64-v8a/
│   └── libvantismobile.a          # Static library for ARM64 devices
├── armeabi-v7a/
│   └── libvantismobile.a          # Static library for ARMv7 devices
├── x86/
│   └── libvantismobile.a          # Static library for x86 emulators
├── x86_64/
│   └── libvantismobile.a          # Static library for x86_64 emulators
├── jni/
│   └── include/
│       └── vantis_mobile.h        # C header file
└── vantismobile/
    └── jni/
        ├── Android.mk             # Makefile for NDK build
        ├── Application.mk         # NDK application config
        ├── arm64-v8a/
        │   └── libvantismobile.a
        ├── armeabi-v7a/
        │   └── libvantismobile.a
        ├── x86/
        │   └── libvantismobile.a
        └── x86_64/
            └── libvantismobile.a
```

### Integrating into Android Project

#### Option 1: Gradle Integration

**1. Copy Kotlin Wrapper:**

```bash
mkdir -p app/src/main/java/com/vantiscorp/vantismobile/ffi
cp apps/android/VantisMobile/ffi/src/main/java/com/vantiscorp/vantismobile/ffi/VantisMobileFFI.kt \
   app/src/main/java/com/vantiscorp/vantismobile/ffi/
```

**2. Copy Native Libraries:**

```bash
mkdir -p app/src/main/jniLibs

# Copy all architectures
cp -R target/android/vantismobile/jni/* app/src/main/jniLibs/
```

**3. Update build.gradle (app module):**

```kotlin
android {
    // ... existing configuration ...

    sourceSets {
        main {
            jniLibs.srcDirs += ["src/main/jniLibs"]
        }
    }

    // Optional: Exclude unused architectures to reduce APK size
    packagingOptions {
        jniLibs {
            useLegacyPackaging = false
        }
    }
}
```

**4. Update build.gradle (project level):**

```kotlin
buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.1.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.0")
    }
}
```

#### Option 2: AAR Library

**1. Create AAR Module:**

```bash
# Create library module
mkdir -p vantismobile-lib/src/main
cd vantismobile-lib

# Create build.gradle
cat > build.gradle << 'EOF'
plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.vantiscorp.vantismobile"
    compileSdk = 34

    defaultConfig {
        minSdk = 21
        targetSdk = 34
    }

    sourceSets {
        main {
            jniLibs.srcDirs = ["jniLibs"]
        }
    }
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib:1.9.0")
}
EOF

# Copy libraries
cp -R ../../target/android/vantismobile/jni/* src/main/jniLibs/
mkdir -p src/main/java/com/vantiscorp/vantismobile/ffi
cp ../../apps/android/VantisMobile/ffi/src/main/java/com/vantiscorp/vantismobile/ffi/VantisMobileFFI.kt \
   src/main/java/com/vantiscorp/vantismobile/ffi/
```

**2. Add to Project:**

```kotlin
// In settings.gradle
include(":vantismobile-lib")
project(":vantismobile-lib").projectDir = file("vantismobile-lib")

// In app/build.gradle
dependencies {
    implementation(project(":vantismobile-lib"))
}
```

### Code Signing

**1. Generate Keystore:**

```bash
keytool -genkeypair \
  -alias vantismobile \
  -keyalg RSA \
  -keysize 4096 \
  -validity 10000 \
  -keystore vantismobile-release.keystore \
  -storepass your-store-password \
  -keypass your-key-password
```

**2. Configure signing in build.gradle:**

```kotlin
android {
    signingConfigs {
        create("release") {
            storeFile = file("../vantismobile-release.keystore")
            storePassword = System.getenv("KEYSTORE_PASSWORD")
            keyAlias = "vantismobile"
            keyPassword = System.getenv("KEY_PASSWORD")
        }
    }

    buildTypes {
        release {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
}
```

**3. Build Release APK:**

```bash
./gradlew assembleRelease
```

### Google Play Store Submission

**1. Create Play Console Account:**

- Sign up for Google Play Console
- Pay registration fee ($25)
- Create developer account

**2. Create App Listing:**

- Go to Play Console
- Create new app
- Fill in app details (name, description, screenshots, etc.)
- Set content rating
- Set pricing and distribution

**3. Upload Release APK:**

```bash
# Build release bundle
./gradlew bundleRelease

# Upload to Play Console
# Go to "Release management" -> "App releases"
# Click "Create new release"
# Upload bundle from app/build/outputs/bundle/release/
```

**4. Rollout to Production:**

- Choose rollout percentage (e.g., 10% staged rollout)
- Submit for review
- Wait for approval (usually 1-3 days)
- Increase rollout gradually

## CI/CD Integration

### GitHub Actions

**iOS CI Workflow:**

```yaml
name: iOS CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v4

      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          targets: aarch64-apple-ios, x86_64-apple-ios

      - name: Build iOS Library
        run: |
          cd pillar-04-continuity/vantis-mobile
          ./build-ios.sh

      - name: Test
        run: |
          cd pillar-04-continuity/vantis-mobile
          cargo test --lib

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ios-framework
          path: pillar-04-continuity/vantis-mobile/target/ios/VantisMobile.xcframework
```

**Android CI Workflow:**

```yaml
name: Android CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          targets: aarch64-linux-android, armv7-linux-androideabi, i686-linux-android, x86_64-linux-android

      - name: Set up NDK
        uses: nttld/setup-ndk@v1
        with:
          ndk-version: r25c

      - name: Build Android Library
        run: |
          cd pillar-04-continuity/vantis-mobile
          ./build-android.sh

      - name: Test
        run: |
          cd pillar-04-continuity/vantis-mobile
          cargo test --lib

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: android-libs
          path: pillar-04-continuity/vantis-mobile/target/android/
```

### Automated Testing

**Unit Tests:**

```bash
# Run all tests
cargo test --lib

# Run specific test
cargo test --lib test_encrypt_decrypt

# Run with output
cargo test --lib -- --nocapture
```

**Integration Tests:**

```swift
// iOS XCTest
import XCTest
@testable import VantisMobileFFI

class EncryptionTests: XCTestCase {
    func testEndToEndEncryption() async throws {
        let keyPair = VantisMobileFFI.KeyPair()
        let encryptor = VantisMobileFFI.Encryptor(sharedSecretBase64: "test-key")
        
        let plaintext = "Test message".data(using: .utf8)!
        let encrypted = encryptor.encrypt(plaintext)
        let decrypted = encryptor.decrypt(encrypted!)
        
        XCTAssertEqual(decrypted, plaintext)
    }
}
```

```kotlin
// Android Unit Test
import org.junit.Test
import com.vantiscorp.vantismobile.ffi.VantisMobileFFI

class EncryptionTests {
    @Test
    fun testEndToEndEncryption() {
        val keyPair = VantisMobileFFI.VantisKeyPair()
        val encryptor = VantisMobileFFI.VantisEncryptor("test-key")
        
        val plaintext = "Test message".toByteArray()
        val encrypted = encryptor.encrypt(plaintext)
        val decrypted = encryptor.decrypt(encrypted!!)
        
        assertArrayEquals(plaintext, decrypted)
    }
}
```

## Release Process

### Versioning

**Semantic Versioning:**

```
MAJOR.MINOR.PATCH

Example: 1.0.0

- MAJOR: Incompatible API changes
- MINOR: Backwards-compatible functionality additions
- PATCH: Backwards-compatible bug fixes
```

**Updating Version:**

```bash
# Update version in Cargo.toml
sed -i 's/^version = ".*"/version = "1.0.0"/' Cargo.toml

# Update version in Swift wrapper
sed -i 's/VantisMobile version: .*/VantisMobile version: 1.0.0/' \
  apps/ios/VantisMobile/Sources/VantisMobileFFI.swift

# Update version in Kotlin wrapper
sed -i 's/VantisMobile version: .*/VantisMobile version: 1.0.0/' \
  apps/android/VantisMobile/ffi/src/main/java/com/vantiscorp/vantismobile/ffi/VantisMobileFFI.kt
```

### Release Checklist

- [ ] Update version numbers in all files
- [ ] Update CHANGELOG.md
- [ ] Run all tests (cargo test, iOS, Android)
- [ ] Build iOS XCFramework
- [ ] Build Android libraries
- [ ] Update documentation
- [ ] Create release notes
- [ ] Create git tag
- [ ] Push to GitHub
- [ ] Build release artifacts
- [ ] Test release artifacts
- [ ] Publish to package managers (if applicable)

### Creating a Release

**1. Tag the Release:**

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

**2. Build Release Artifacts:**

```bash
# iOS
./build-ios.sh

# Android
./build-android.sh

# Package artifacts
mkdir -p release/v1.0.0
cp -R target/ios/VantisMobile.xcframework release/v1.0.0/
cp -R target/android/vantismobile release/v1.0.0/

# Create checksums
cd release/v1.0.0
shasum -a 256 VantisMobile.xcframework/* > SHA256SUMS
shasum -a 256 vantismobile/*/* >> SHA256SUMS
cd ../..
```

**3. Create GitHub Release:**

```bash
gh release create v1.0.0 \
  --title "VantisMobile v1.0.0" \
  --notes "Release notes..." \
  release/v1.0.0/*
```

## Troubleshooting

### iOS Build Issues

**Issue: Library not found during linking**

```
ld: library not found for -lvantismobile
```

**Solution:**
1. Verify library was built: `ls -la target/ios/universal/`
2. Check linker settings in Package.swift
3. Ensure library path is correct
4. Clean and rebuild: `xcodebuild clean`

**Issue: Architecture mismatch**

```
building for iOS Simulator, but linking in object file built for iOS
```

**Solution:**
1. Build universal binary: `./build-ios.sh`
2. Use correct XCFramework for target
3. Check architecture: `lipo -info target/ios/universal/libvantismobile.a`

### Android Build Issues

**Issue: UnsatisfiedLinkError**

```
java.lang.UnsatisfiedLinkError: No implementation found for ...
```

**Solution:**
1. Verify library was built: `ls -la target/android/`
2. Check correct ABI directories exist
3. Ensure library is in jniLibs folder
4. Clean and rebuild: `./gradlew clean`

**Issue: NDK not found**

```
Error: ANDROID_NDK_HOME not set
```

**Solution:**
```bash
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
```

### Runtime Issues

**Issue: Crashes on initialization**

**Solution:**
1. Verify library is loaded before use
2. Check architecture compatibility
3. Enable logging for more details
4. Test with minimal example

**Issue: Encryption/decryption failures**

**Solution:**
1. Verify shared secret is correct
2. Check Base64 encoding is valid
3. Ensure both ends use same key
4. Test with known values

## Additional Resources

- [Integration Guide](../mobile/INTEGRATION_GUIDE.md)
- [API Reference](../api/API_REFERENCE.md)
- [Security Considerations](../mobile/SECURITY.md)
- [GitHub Repository](https://github.com/vantisCorp/VantisOffice)

## Support

For deployment issues:
- Check documentation
- Review troubleshooting section
- Create GitHub issue with details
- Contact support: support@vantiscorp.io