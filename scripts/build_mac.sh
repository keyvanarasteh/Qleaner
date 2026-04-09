#!/usr/bin/env bash
# Qleaner Production Build Pipeline - MAC (Cross-Compile or Native)
set -e
set -o pipefail

APP_NAME="Qleaner"
TIMESTAMP=$(date +"%Y%m%d_%H%M")
BUILD_ID="MAC_${TIMESTAMP}"
PLATFORM="x86_64-apple-darwin" # fallback to native if on mac, or handle cross compilation if on linux

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="$PROJECT_ROOT/src-tauri"
OUTPUT_DIR="$PROJECT_ROOT/output/mac_${TIMESTAMP}"
LOG_FILE="$OUTPUT_DIR/build.log"

BOT_TOKEN='7957182560:AAFgUd1EZNUglClnV9WBme3WNgUtstjJHsU'
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

log "${BLUE}Starting Build for ${APP_NAME} (Mac)${NC}"
send_telegram_msg "🚀 <b>${APP_NAME} Mac Build Started</b>%0A%0A💻 <b>Platform:</b> macOS%0A🕒 <b>Time:</b> $(date)"

cd "$PROJECT_ROOT"
bun install >> "$LOG_FILE" 2>&1
bun run build >> "$LOG_FILE" 2>&1

# If running on macOS, build natively. If on Linux, cross compilation via osxcross is complex, so we assume native mac or warn.
if [[ "$OSTYPE" == "darwin"* ]]; then
    log "Native macOS detected. Building Tauri App..."
    bun run tauri build --target aarch64-apple-darwin >> "$LOG_FILE" 2>&1
    cp src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/*.dmg "$OUTPUT_DIR/" 2>/dev/null || true
    cp src-tauri/target/aarch64-apple-darwin/release/bundle/macos/*.app "$OUTPUT_DIR/" -r 2>/dev/null || true
else
    log "${YELLOW}Warning:${NC} Building Mac target from a non-Mac host. Cargo may fail unless osxcross is configured."
    bun run tauri build --target $PLATFORM >> "$LOG_FILE" 2>&1
    cp src-tauri/target/$PLATFORM/release/bundle/dmg/*.dmg "$OUTPUT_DIR/" 2>/dev/null || true
fi

# Rename artifacts to include timestamp
find "$OUTPUT_DIR" -maxdepth 1 -type f -name "*.dmg" -exec sh -c 'mv "$0" "${0%.dmg}_'"$TIMESTAMP"'.dmg"' {} \;

log "${GREEN}Build complete!${NC}"
send_telegram_msg "✅ <b>${APP_NAME} Mac Build Complete!</b>%0A%0A📁 <b>Output Dir:</b> <code>$OUTPUT_DIR</code>"
