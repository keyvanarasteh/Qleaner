# MacOS M-Series (Apple Silicon) Build Checklist for Qleaner

This document covers the complete workflow for compiling and signing the native macOS release of Qleaner (Tauri v2) on an Apple Silicon Mac Mini.

## 1. System Requirements & Prerequisites
- [x] Install Xcode Command Line Tools: `xcode-select --install`
- [x] Install Homebrew: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
- [x] Install Rust for Apple Silicon: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [x] Install Bun (Package Manager): `curl -fsSL https://bun.sh/install | bash`
- [x] Ensure macOS specific dependencies are met (no extra GTK like Linux is required as Tauri uses native WebKit).

## 2. Project Initialization
- [x] Clone repository: `git clone <repo_url> && cd Qleaner`
- [x] Install JS dependencies using Bun: `bun install`
- [x] Verify Svelte/Vite configs are functioning: `bun run dev` (Ensure it hot-reloads properly on the Mac).

## 3. Apple Developer & Security Certificates (Signing)
*Note: Without signing, the generated `.app` and `.dmg` will trigger the "App is damaged and cannot be opened" Gatekeeper error and requires users to run `xattr -cr /Applications/Qleaner.app`.*

- [ ] Obtain an **Apple Developer Program** license.
- [ ] Add the Developer ID Application Certificate into your macOS Keychain via Xcode.
- [ ] Configure `tauri.conf.json` -> `bundle.macOS`:
  - Enforce `entitlements.mac.plist` (if accessing restricted OS paths like Full Disk Access for scanning).
  - Set specific `signingIdentity` string.
- [ ] Setup Apple Notarization credentials in env variables (for `altool` / `notarytool`):
  - `APPLE_ID="..."`
  - `APPLE_PASSWORD="..."` (App-Specific Password)
  - `APPLE_TEAM_ID="..."`

## 4. Compiling & Production Builds
- [ ] Native Apple Silicon Target (M1/M2/M3):
  `rustup target add aarch64-apple-darwin`
- [ ] Universal Binary (Both Intel & Mac - Optional but recommended):
  `rustup target add x86_64-apple-darwin`
  `bun run tauri build --target universal-apple-darwin`
- [ ] Build the project for the native chipset:
  `bun run tauri build`
  
## 5. Artifact Verification
- [ ] The generated artifacts will exist under: `src-tauri/target/release/bundle/macos/` and `src-tauri/target/release/bundle/dmg/`
- [ ] Ensure the generated `.dmg` has the correct background image and icon sizes specific to macOS Finder configurations (Configured in `tauri.conf.json`).

## 6. Known MacOS Specific Edge Cases
- Deep system cleaning in macOS requires `Full Disk Access`. The Svelte app must prompt the user proactively with instructions to enable "Qleaner.app" under `System Settings -> Privacy & Security -> Full Disk Access`.
- Avoid executing raw shell scripts using `bash / rm` on macOS when cleaning; strictly use native Rust `tokio::fs` calls to prevent Permission/Sandbox faults under SIP (System Integrity Protection).
