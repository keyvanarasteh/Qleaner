# Contributing to Qleaner

We love your input! We want to make contributing to this application as easy and transparent as possible.

## Pull Request Process
1. Ensure your code passes `cargo check`, `cargo test`, and `bun run check`.
2. Follow the architectural split: Core heuristics belong in `src-tauri/src/cleaner/detectors.rs`.
3. Adhere to Svelte 5 `$state` and asynchronous `tokio::fs` usage for any new features.
4. Provide unit tests within `#[cfg(test)]` modules when modifying traversals.

## Internationalization (i18n)
All UI labels are managed via `paraglide-js`. Please refer to `./docs/i18n.md` before making UI text changes!

## Development Setup
```bash
bun install
bun run dev
```
