#!/bin/bash
# VantisOffice Development Script

set -e

echo "🚀 Starting VantisOffice Development Environment..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if we're in watch mode
WATCH_MODE=${1:-"false"}

if [ "$WATCH_MODE" = "watch" ]; then
    echo -e "${YELLOW}Running in watch mode...${NC}"
    cargo watch -x 'run'
else
    echo -e "${BLUE}Running development build...${NC}"
    cargo build
    
    echo -e "${GREEN}✅ Development build completed!${NC}"
    echo "💡 Use './scripts/dev.sh watch' for hot-reload mode"
fi