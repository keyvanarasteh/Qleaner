# Qleaner 🧹

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white) ![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=FFFFFF)

> A blazing fast, asynchronous, cross-platform system junk and cache optimization platform. 
Built on **Tauri V2, Rust, and Svelte 5**.

## Features & Documentation
Qleaner is designed modularly. Read up on our implementation documentation inside the `./docs/` repository:
- [Core Application Architecture](./docs/architecture.md)
- [MacOS Advanced Detector Heuristics](./docs/macos_detectors.md)
- [Internationalization (i18n) Guide](./docs/i18n.md)
- [Tauri Native Integrations](./docs/native_desktop.md)

## Tech Stack
* **Backend:** Rust, Tokio (Async FS), rayon (Parallel I/O).
* **Frontend:** Svelte 5, TailwindCSS v4, Bits-UI, Number-flow.
* **Packaging:** MSI (Windows), DMG (macOS), deb/AppImage (Linux).

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- Node.js & Tauri native dependencies matching your OS.

### Spin up Locally
```bash
git clone https://github.com/keyvanarasteh/qleaner
cd qleaner
bun install
bun run tauri dev
```

## Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md) for how to get started!

## License
Dual-licensed under [MIT](./LICENSE-MIT) and [Apache 2.0](./LICENSE-APACHE).
