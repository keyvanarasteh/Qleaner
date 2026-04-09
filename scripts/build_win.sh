#!/usr/bin/env bash
# Qleaner Production Build Pipeline - WINDOWS
set -e
set -o pipefail

APP_NAME="Qleaner"
TIMESTAMP=$(date +"%Y%m%d_%H%M")
BUILD_ID="WIN_${TIMESTAMP}"
PLATFORM="x86_64-pc-windows-gnu"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="$PROJECT_ROOT/src-tauri"
OUTPUT_DIR="$PROJECT_ROOT/output/win_${TIMESTAMP}"
LOG_FILE="$OUTPUT_DIR/build.log"

BOT_TOKEN='<YOUR_TELEGRAM_BOT_TOKEN>'
TELEGRAM_ADMIN_ID=1426904527

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' 

mkdir -p "$OUTPUT_DIR"

log() {
    local msg="[$(date +"%H:%M:%S")] $1"
    echo -e "$msg"
    echo "$msg" >> "$LOG_FILE"
}

send_telegram_msg() {
    curl -s -X POST "https://api.telegram.org/bot${BOT_TOKEN}/sendMessage" -d chat_id="${TELEGRAM_ADMIN_ID}" -d text="$1" -d parse_mode="HTML" > /dev/null
}

log "${BLUE}Starting Build for ${APP_NAME} (Windows)${NC}"
send_telegram_msg "🚀 <b>${APP_NAME} Windows Build Started</b>%0A%0A🪟 <b>Platform:</b> Windows ($PLATFORM)%0A🕒 <b>Time:</b> $(date)"

cd "$PROJECT_ROOT"
bun install >> "$LOG_FILE" 2>&1
bun run build >> "$LOG_FILE" 2>&1

cd "$TAURI_DIR"
# Requires x86_64-pc-windows-gnu toolchain and mingw-w64 on Linux
log "Building Tauri App for Windows..."
cargo tauri build --target $PLATFORM >> "$LOG_FILE" 2>&1

log "Copying Artifacts..."
cp target/$PLATFORM/release/bundle/nsis/*.exe "$OUTPUT_DIR/" 2>/dev/null || true
cp target/$PLATFORM/release/bundle/msi/*.msi "$OUTPUT_DIR/" 2>/dev/null || true

# Rename artifacts to include timestamp
find "$OUTPUT_DIR" -type f -name "*.exe" -exec sh -c 'mv "$0" "${0%.exe}_'"$TIMESTAMP"'.exe"' {} \;
find "$OUTPUT_DIR" -type f -name "*.msi" -exec sh -c 'mv "$0" "${0%.msi}_'"$TIMESTAMP"'.msi"' {} \;

log "${GREEN}Build complete!${NC}"
send_telegram_msg "✅ <b>${APP_NAME} Windows Build Complete!</b>%0A%0A📁 <b>Output Dir:</b> <code>$OUTPUT_DIR</code>"
