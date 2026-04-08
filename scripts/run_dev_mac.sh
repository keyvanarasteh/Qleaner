#!/usr/bin/env bash
# Qleaner Local Development Environment - MAC

APP_NAME="Qleaner"
if [ -d "/home/drvoid/ISU/Qleaner" ]; then
    PROJECT_ROOT="/home/drvoid/ISU/Qleaner"
else
    PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi

# ANSI Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' 

echo -e "${YELLOW}==============================================${NC}"
echo -e "${GREEN}      Qleaner Development Server (Mac)       ${NC}"
echo -e "${YELLOW}==============================================${NC}"

cd "$PROJECT_ROOT"

# Ensure dependencies
if [ ! -d "node_modules" ] || [ ! -d ".svelte-kit" ]; then
    echo -e "${BLUE}Dependencies missing. Running bun install...${NC}"
    bun install
fi

# Run Tauri Dev Server
export RUST_LOG="info"

echo -e "${GREEN}Deploying local hot-reloading environment...${NC}"
bun tauri dev
