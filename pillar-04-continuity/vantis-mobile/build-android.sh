#!/bin/bash

# Build script for Android
# Builds for multiple Android architectures using NDK

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Build configurations
BUILD_DIR="target/android"
LIB_NAME="libvantismobile"

# Check for Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    if [ -d "$HOME/Android/Sdk/ndk" ]; then
        # Find the latest NDK version
        ANDROID_NDK_HOME=$(ls -d $HOME/Android/Sdk/ndk/*/ | tail -1)
        echo -e "${YELLOW}Using NDK at: $ANDROID_NDK_HOME${NC}"
    else
        echo -e "${RED}Error: ANDROID_NDK_HOME not set and NDK not found${NC}"
        exit 1
    fi
fi

# Android architectures to build for
ANDROID_ARCHS=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")
ANDROID_ABI=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

echo -e "${GREEN}Building VantisMobile for Android${NC}"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Setup NDK toolchain
NDK_TOOLCHAIN="$BUILD_DIR/toolchain"
$ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py \
    --arch arm64 \
    --api 21 \
    --install-dir "$NDK_TOOLCHAIN" \
    --force 2>/dev/null || true

# Build for each architecture
for i in "${!ANDROID_ARCHS[@]}"; do
    ARCH="${ANDROID_ARCHS[$i]}"
    ABI="${ANDROID_ABI[$i]}"
    
    echo -e "${YELLOW}Building for $ARCH ($ABI)...${NC}"
    
    ARCH_DIR="$BUILD_DIR/$ABI"
    mkdir -p "$ARCH_DIR"
    
    # Set up environment
    case $ARCH in
        aarch64-linux-android)
            TARGET="aarch64-linux-android"
            ;;
        armv7-linux-androideabi)
            TARGET="armv7-linux-androideabi"
            ;;
        i686-linux-android)
            TARGET="i686-linux-android"
            ;;
        x86_64-linux-android)
            TARGET="x86_64-linux-android"
            ;;
    esac
    
    # Build using cargo-ndk or cargo with target
    if command -v cargo-ndk &> /dev/null; then
        cargo ndk --platform 21 --target $TARGET build --release --lib
    else
        cargo build --release --target $TARGET --lib 2>/dev/null || {
            echo -e "${YELLOW}Note: Run 'rustup target add $TARGET' first${NC}"
            rustup target add $TARGET
            cargo build --release --target $TARGET --lib
        }
    fi
    
    # Copy library
    if [ -f "target/$TARGET/release/libvantismobile.a" ]; then
        cp "target/$TARGET/release/libvantismobile.a" "$ARCH_DIR/"
        echo -e "${GREEN}✓ Built for $ARCH ($ABI)${NC}"
    else
        echo -e "${RED}✗ Failed to build for $ARCH ($ABI)${NC}"
    fi
done

# Create JNI header directory
JNI_DIR="$BUILD_DIR/jni"
mkdir -p "$JNI_DIR/include"
cp include/vantis_mobile.h "$JNI_DIR/include/"

# Create Android AAR structure
AAR_DIR="$BUILD_DIR/vantismobile"
mkdir -p "$AAR_DIR/jni"
for ABI in "${ANDROID_ABI[@]}"; do
    if [ -f "$BUILD_DIR/$ABI/libvantismobile.a" ]; then
        mkdir -p "$AAR_DIR/jni/$ABI"
        cp "$BUILD_DIR/$ABI/libvantismobile.a" "$AAR_DIR/jni/$ABI/"
    fi
done

# Create Android.mk
cat > "$AAR_DIR/jni/Android.mk" << 'EOF'
LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)
LOCAL_MODULE := vantismobile
LOCAL_SRC_FILES := $(TARGET_ARCH_ABI)/libvantismobile.a
include $(PREBUILT_STATIC_LIBRARY)
EOF

# Create Application.mk
cat > "$AAR_DIR/jni/Application.mk" << 'EOF'
APP_ABI := arm64-v8a armeabi-v7a x86 x86_64
APP_PLATFORM := android-21
APP_STL := c++_static
EOF

echo -e "${GREEN}✓ Android AAR structure created${NC}"

# Create Kotlin JNI wrapper example
mkdir -p "$BUILD_DIR/kotlin"
cat > "$BUILD_DIR/kotlin/VantisMobile.kt" << 'EOF'
package com.vantiscorp.vantismobile

/**
 * Kotlin JNI wrapper for VantisMobile native library
 */
object VantisMobile {
    
    init {
        System.loadLibrary("vantismobile")
    }
    
    // Native methods
    external fun initialize(): Int
    external fun cleanup()
    external fun getVersion(): String
    
    // KeyPair operations
    external fun keypairGenerate(): Long
    external fun keypairFree(keypair: Long)
    external fun keypairPublicKeyHex(keypair: Long): String
    
    // Encryption operations
    external fun encryptorCreate(sharedSecret: ByteArray): Long
    external fun encryptorFree(encryptor: Long)
    external fun encrypt(encryptor: Long, plaintext: ByteArray): ByteArray
    external fun decrypt(encryptor: Long, ciphertext: ByteArray): ByteArray
    
    // Device info
    external fun deviceInfoCreate(
        name: String,
        deviceType: Int,
        osVersion: String,
        appVersion: String
    ): Long
    external fun deviceInfoFree(info: Long)
    external fun deviceInfoToJson(info: Long): String
}
EOF

echo -e "${GREEN}✓ Kotlin JNI wrapper created${NC}"

# Print summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Output directory: $BUILD_DIR"
echo "JNI headers: $JNI_DIR/include/"
echo "AAR structure: $AAR_DIR/"
echo "Kotlin wrapper: $BUILD_DIR/kotlin/VantisMobile.kt"
echo ""
echo -e "${GREEN}Build completed successfully!${NC}"
echo ""
echo -e "${YELLOW}Note: To build for all architectures, install targets:${NC}"
echo "  rustup target add aarch64-linux-android"
echo "  rustup target add armv7-linux-androideabi"
echo "  rustup target add i686-linux-android"
echo "  rustup target add x86_64-linux-android"
echo ""
echo -e "${YELLOW}Or use cargo-ndk for easier Android builds:${NC}"
echo "  cargo install cargo-ndk"