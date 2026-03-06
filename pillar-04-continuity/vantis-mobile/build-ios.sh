#!/bin/bash

# Build script for iOS (Universal Binary)
# Builds for arm64 (iOS devices) and x86_64 (simulators)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Build configurations
BUILD_DIR="target/ios"
LIB_NAME="libvantismobile"

echo -e "${GREEN}Building VantisMobile for iOS (Universal Binary)${NC}"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/arm64"
mkdir -p "$BUILD_DIR/x86_64"
mkdir -p "$BUILD_DIR/universal"

# iOS architectures
IOS_ARCHS=("aarch64-apple-ios" "x86_64-apple-ios")

# Build for each architecture
for ARCH in "${IOS_ARCHS[@]}"; do
    echo -e "${YELLOW}Building for $ARCH...${NC}"
    
    if [[ "$ARCH" == "aarch64-apple-ios" ]]; then
        TARGET_DIR="$BUILD_DIR/arm64"
        CARGO_TARGET="aarch64-apple-ios"
    else
        TARGET_DIR="$BUILD_DIR/x86_64"
        CARGO_TARGET="x86_64-apple-ios"
    fi
    
    # Build static library
    cargo build --release --target "$CARGO_TARGET" --lib
    
    # Copy static library to build directory
    if [ -f "target/$CARGO_TARGET/release/libvantismobile.a" ]; then
        cp "target/$CARGO_TARGET/release/libvantismobile.a" "$TARGET_DIR/"
        echo -e "${GREEN}✓ Built for $ARCH${NC}"
    else
        echo -e "${RED}✗ Failed to build for $ARCH${NC}"
        exit 1
    fi
done

# Create universal binary using lipo
echo "Creating universal binary..."
lipo -create \
    "$BUILD_DIR/arm64/libvantismobile.a" \
    "$BUILD_DIR/x86_64/libvantismobile.a" \
    -output "$BUILD_DIR/universal/libvantismobile.a"

echo -e "${GREEN}✓ Universal binary created${NC}"

# Copy header files
echo "Copying header files..."
mkdir -p "$BUILD_DIR/universal/include"
cp include/vantis_mobile.h "$BUILD_DIR/universal/include/"

echo -e "${GREEN}✓ Header files copied${NC}"

# Create XCFramework
echo "Creating XCFramework..."
if command -v xcodebuild &> /dev/null; then
    xcodebuild -create-xcframework \
        -library "$BUILD_DIR/arm64/libvantismobile.a" \
        -headers "$BUILD_DIR/universal/include" \
        -library "$BUILD_DIR/x86_64/libvantismobile.a" \
        -headers "$BUILD_DIR/universal/include" \
        -output "$BUILD_DIR/VantisMobile.xcframework"
    
    echo -e "${GREEN}✓ XCFramework created${NC}"
else
    echo -e "${YELLOW}⚠ xcodebuild not found, skipping XCFramework creation${NC}"
fi

# Create Swift package
echo "Creating Swift package structure..."
SWIFT_PACKAGE_DIR="$BUILD_DIR/VantisMobile"
mkdir -p "$SWIFT_PACKAGE_DIR/Sources/VantisMobile"
mkdir -p "$SWIFT_PACKAGE_DIR/Headers"

# Copy library and headers
cp "$BUILD_DIR/universal/libvantismobile.a" "$SWIFT_PACKAGE_DIR/Sources/VantisMobile/"
cp "$BUILD_DIR/universal/include/vantis_mobile.h" "$SWIFT_PACKAGE_DIR/Headers/"

# Create module map
cat > "$SWIFT_PACKAGE_DIR/Headers/module.modulemap" << 'EOF'
module VantisMobile {
    header "vantis_mobile.h"
    link "vantismobile"
    export *
}
EOF

echo -e "${GREEN}✓ Swift package structure created${NC}"

# Print summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Library: $BUILD_DIR/universal/libvantismobile.a"
echo "Headers: $BUILD_DIR/universal/include/"
echo "XCFramework: $BUILD_DIR/VantisMobile.xcframework"
echo "Swift Package: $SWIFT_PACKAGE_DIR"
echo ""
echo -e "${GREEN}Build completed successfully!${NC}"