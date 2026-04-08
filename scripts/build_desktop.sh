#!/usr/bin/env bash
# Local Developer Desktop Build Pipeline (Fastest, no optimizations)

APP_NAME="Qleaner"
TAURI_DIR="$(pwd)/../src-tauri"
if [ -d "/home/drvoid/ISU/Qleaner" ]; then
    PROJECT_ROOT="/home/drvoid/ISU/Qleaner"
else
    PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi
TAURI_DIR="$PROJECT_ROOT/src-tauri"
OUTPUT_DIR="$PROJECT_ROOT/output/desktop"

# ANSI Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}==============================================${NC}"
echo -e "${GREEN}      Qleaner Desktop Build Pipeline         ${NC}"
echo -e "${YELLOW}==============================================${NC}"
echo -e "${BLUE}Notice:${NC} This script is for fast local development."
echo -e "${BLUE}Notice:${NC} It skips LTO, optimization flags, and artifact uploads."

mkdir -p "$OUTPUT_DIR"

echo -e "\n${BLUE}[1/4] Checking Dependencies...${NC}"
if ! command -v bun &> /dev/null; then
    echo -e "${RED}bun is required but not installed. Aborting.${NC}"
    exit 1
fi
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}cargo is required but not installed. Aborting.${NC}"
    exit 1
fi

echo -e "\n${BLUE}[2/4] Building Frontend ($PROJECT_ROOT)...${NC}"
cd "$PROJECT_ROOT" || exit 1
bun install
bun run build

echo -e "\n${BLUE}[3/4] Building Tauri App ($TAURI_DIR)...${NC}"
cd "$TAURI_DIR" || exit 1
cargo tauri build

echo -e "\n${BLUE}[4/4] Copying Artifacts...${NC}"
cp target/release/qleaner "$OUTPUT_DIR/" 2>/dev/null || echo -e "${YELLOW}Could not find binary. Check if build succeeded.${NC}"

echo -e "\n${GREEN}Build completed successfully!${NC}"
echo -e "Outputs saved to ${YELLOW}$OUTPUT_DIR${NC}"
