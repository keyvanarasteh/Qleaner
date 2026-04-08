#!/usr/bin/env bash
# Qleaner Production Build Pipeline - LINUX
set -e
set -o pipefail

# Configuration
APP_NAME="Qleaner"
ENVIRONMENT="production"
TIMESTAMP=$(date +"%Y%m%d_%H%M")
BUILD_ID="LINUX_${TIMESTAMP}"
PLATFORM="x86_64-unknown-linux-gnu"

# Directories
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TAURI_DIR="$PROJECT_ROOT/src-tauri"
OUTPUT_DIR="$PROJECT_ROOT/output/linux_${TIMESTAMP}"
LOG_FILE="$OUTPUT_DIR/build.log"

# Telegram Bot Token from user rules
BOT_TOKEN='7957182560:AAFgUd1EZNUglClnV9WBme3WNgUtstjJHsU'
TELEGRAM_ADMIN_ID=1426904527

# ANSI Colors
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
    local text="$1"
    curl -s -X POST "https://api.telegram.org/bot${BOT_TOKEN}/sendMessage" \
        -d chat_id="${TELEGRAM_ADMIN_ID}" \
        -d text="${text}" \
        -d parse_mode="HTML" > /dev/null
}

send_telegram_doc() {
    local filepath="$1"
    local caption="$2"
    curl -s -X POST "https://api.telegram.org/bot${BOT_TOKEN}/sendDocument" \
        -F chat_id="${TELEGRAM_ADMIN_ID}" \
        -F document="@${filepath}" \
        -F caption="${caption}" > /dev/null
}

log "${BLUE}Starting Build for ${APP_NAME} (Linux)${NC}"
send_telegram_msg "🚀 <b>${APP_NAME} Build Started</b>%0A%0A🖥 <b>Platform:</b> Linux%0A🕒 <b>Time:</b> $(date)"

# Use SCCACHE and MOLD for speed
export RUSTC_WRAPPER=$(which sccache 2>/dev/null || echo "")
export LDFLAGS="-fuse-ld=mold"
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

log "Building Frontend with Bun..."
cd "$PROJECT_ROOT"
bun install >> "$LOG_FILE" 2>&1
bun run build >> "$LOG_FILE" 2>&1

log "Building Tauri App (Release)..."
cd "$TAURI_DIR"
cargo tauri build >> "$LOG_FILE" 2>&1

log "Copying Artifacts..."
cp target/release/bundle/appimage/*.AppImage "$OUTPUT_DIR/" 2>/dev/null || true
cp target/release/bundle/deb/*.deb "$OUTPUT_DIR/" 2>/dev/null || true

# Rename artifacts to include timestamp
find "$OUTPUT_DIR" -type f -name "*.AppImage" -exec sh -c 'mv "$0" "${0%.AppImage}_'"$TIMESTAMP"'.AppImage"' {} \;
find "$OUTPUT_DIR" -type f -name "*.deb" -exec sh -c 'mv "$0" "${0%.deb}_'"$TIMESTAMP"'.deb"' {} \;

log "${GREEN}Build complete!${NC}"
send_telegram_msg "✅ <b>${APP_NAME} Linux Build Complete!</b>%0A%0A📁 <b>Output Dir:</b> <code>$OUTPUT_DIR</code>"

# Compress logs and artifacts if requested (skipping heavy files on tg, just sending log)
bzip2 -c "$LOG_FILE" > "$OUTPUT_DIR/build_log.bz2"
send_telegram_doc "$OUTPUT_DIR/build_log.bz2" "Build Logs"
