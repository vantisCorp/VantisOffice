#!/bin/bash
# VantisOffice Test Script

set -e

echo "🧪 Running VantisOffice Test Suite..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Run tests with coverage
echo -e "${BLUE}Running unit tests...${NC}"
cargo test --lib

echo -e "${BLUE}Running integration tests...${NC}"
cargo test --test '*'

echo -e "${BLUE}Running doc tests...${NC}"
cargo test --doc

# Run tests for each pillar
for pillar in pillar-*; do
    echo -e "${BLUE}Testing $pillar...${NC}"
    cd $pillar
    cargo test
    cd ..
done

echo -e "${GREEN}✅ All tests passed!${NC}"