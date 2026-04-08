#!/usr/bin/env bash
# Qleaner Local Development Environment - WINDOWS (Cross-compilation dev from Linux host allowed, but native is better)

APP_NAME="Qleaner"
if command -v cygpath &> /dev/null; then
    # In cygwin/msys2
    PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
else
    if [ -d "/home/drvoid/ISU/Qleaner" ]; then
        PROJECT_ROOT="/home/drvoid/ISU/Qleaner"
    else
        PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    fi
fi

# ANSI Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' 

echo -e "${YELLOW}==============================================${NC}"
echo -e "${GREEN}      Qleaner Development Server (Windows)   ${NC}"
echo -e "${YELLOW}==============================================${NC}"

cd "$PROJECT_ROOT"

# Ensure dependencies
if [ ! -d "node_modules" ] || [ ! -d ".svelte-kit" ]; then
    echo -e "${BLUE}Dependencies missing. Running bun install...${NC}"
    bun install
fi

export RUST_LOG="info"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "${YELLOW}Warning:${NC} You are running run_dev_win.sh on a Linux host."
    echo -e "Attempting to cross-compile for Windows..."
    bun tauri dev --target x86_64-pc-windows-gnu
else
    echo -e "${GREEN}Deploying local hot-reloading environment...${NC}"
    bun tauri dev
fi
