# Qleaner Core Architecture

Qleaner operates on a strict **Backend-Driven Async** model.
- **Tauri V2 Shell**: Manages native Window bindings and AppHandle state tracking.
- **Rust Modular Backend**: Separates traversal bounds into `scanner.rs`, memory structures into `models.rs`, and distinct evaluation constraints in `detectors.rs`.
- **Svelte 5 Frontend**: Binds strictly via `$state` to UI components utilizing the Bits-UI integration to decouple layout logic from native OS logic.
