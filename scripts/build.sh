#!/bin/bash
# VantisOffice Build Script

set -e

echo "🏗️  Building VantisOffice..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check for required tools
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed"
        exit 1
    fi
}

echo -e "${BLUE}Checking dependencies...${NC}"
check_tool cargo
check_tool rustc
check_tool cmake
check_tool git

# Build release
echo -e "${BLUE}Building Pillar I: System Foundations${NC}"
cd pillar-01-iron
cargo build --release
cd ..

echo -e "${BLUE}Building Pillar II: Productivity Applications${NC}"
cd pillar-02-logic
cargo build --release
cd ..

echo -e "${BLUE}Building Pillar III: Ecosystem & Collaboration${NC}"
cd pillar-03-sync
cargo build --release
cd ..

echo -e "${BLUE}Building Pillar IV: Critical Tools${NC}"
cd pillar-04-continuity
cargo build --release
cd ..

echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo "📦 Binaries are in: target/release/"