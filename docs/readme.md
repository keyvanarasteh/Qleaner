# Qleaner Documentation Index

Welcome to the central repository for Qleaner's backend-driven architecture and module constraints.

## Table of Contents

1. [Core Architecture](./architecture.md)
2. [Internationalization (i18n)](./i18n.md)
3. [macOS Specific Detectors](./macos_detectors.md)
4. [Native Desktop Integration](./native_desktop.md)
5. [Visual Market Analysis](./vison.html)

## Implemented Modules Coverage & Gap Analysis

As Qleaner develops from an MVP into an enterprise utility, various modules have been integrated natively in Rust but currently exhibit UX or capability gaps preventing full usage by the end user.

| Module | Rust Backend Coverage | Svelte Frontend Coverage | Missing Gaps to Fully Functional |
|--------|-----------------------|--------------------------|---------------------------------|
| **Database Persist (SQLite)** | **High**: `sqlx` maps async SQLite configurations natively. | **Low**: The `/schedules` and `/settings` routes lack functional forms reading/writing DB rules actively. | Need UX CRUD layouts for executing SQLite schema models. |
| **Audit Logs & Telemetry** | **High**: `tracing-subscriber` rotating log boundaries & HMAC-SHA256 signatures persist natively. | **High**: Rendered dynamically mirroring the database table via modern grid layouts. | None. Full Stack coverage achieved. |
| **File Shredder** | **High**: Implements comprehensive fallbacks mimicking DoD 5220.22-M overrides safely locally. | **Low**: The UI explicitly lacks a toggle letting the user choose between secure shredding vs generic unlinking operations natively. | Need a 'Security Level' preference toggle inside the global Settings panel. |
| **System Hardware Monitoring** | **Medium**: Extracts Memory, active CPU constraints, and CPU Thermal sensors explicitly utilizing `sysinfo`. | **Medium**: Binds smoothly utilizing `@number-flow` animations over the dashboard dynamically. | Lacks network up/down metrics mapping `sysinfo::Networks` and realtime IO monitoring over Disk structures. |
| **Privacy Forensics & Extensions** | **High**: Evaluates precise tracking paths safely securely without corrupting environments. Sweeps Extension Telemetry. | **Low**: Groups results statically alongside generic caches inside the table without explicitly filtering dangerous tracking forms distinctly. | Requires explicit privacy indicator filters or distinct UX tab sorting algorithms to display severity constraints transparently. |
