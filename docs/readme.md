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
| **Database Persist (SQLite)** | **High**: `sqlx` maps async SQLite configurations natively. | **High**: The `/schedules` and `/settings` routes actively parse explicit CRUD structures tracking sweeping layouts. | None. Full Stack coverage achieved. |
| **Audit Logs & Telemetry** | **High**: `tracing-subscriber` rotating log boundaries & HMAC-SHA256 signatures persist natively. | **High**: Rendered dynamically mirroring the database table via modern grid layouts. | None. Full Stack coverage achieved. |
| **File Shredder** | **High**: Implements comprehensive fallbacks mimicking DoD 5220.22-M overrides safely locally. | **High**: Explict 'Security Level' preference toggle maps DoD overrides safely inside global Settings parameters. | None. Full Stack coverage achieved. |
| **System Hardware Monitoring** | **High**: Extracts Memory, active CPU constraints, and explicit `sysinfo::Networks` limits. | **High**: Seamlessly bounds live network upload/download thresholds over visual layouts mapping `@number-flow` logic dynamically. | None. Full Stack coverage achieved. |
| **Privacy Forensics & Extensions** | **High**: Evaluates precise tracking paths safely securely without corrupting environments. Sweeps Extension Telemetry. | **High**: Evaluates threat signatures targeting Privacy classifications dynamically rendering explicit severity tokens globally over metrics grids. | None. Full Stack coverage achieved. |
