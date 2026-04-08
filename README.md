# Qleaner 🧹

Qleaner is a blazing fast, cross-platform system utility and disk cleaner built with true memory-safe operations.

## Architecture & Tech Stack

This project is built utilizing a fundamentally dual-layer composition:

### 1. The Core (Backend)
- **Framework:** Rust + Tauri v2
- **Concurrency:** Fully asynchronous traversals scaling off multi-threading via `tokio` and `jwalk`/`ignore`.
- **System Monitoring:** Implements `sysinfo` to explicitly lock execution paths of currently running background applications (i.e. guarding active Chrome browser profiles from internal wipeouts).
- **Graceful Abortion:** Maps `tokio_util::sync::CancellationToken` downwards universally into the directory recursive size calculators, allowing the frontend to kill execution immediately.

### 2. The Interactive Shell (Frontend)
- **Framework:** Svelte 5 + SvelteKit
- **Aesthetics:** Styled with TailwindCSS v4 natively embracing glassmorphism and obsidian depth UI.
- **Component Primitives:** Mapped entirely around pure `bits-ui` APIs supporting custom hover menus and reactive dropdown tracking models directly into tables.

## Quick Spin-Up (Local Developer Environment)

To boot up the unified engine locally:

1. Guarantee standard dependencies for Linux (or your respective OS):
```bash
sudo apt-get install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```
2. Spawn the frontend + backend daemon wrapper:
```bash
bun install
bun tauri dev
```

## Compilation Feature Gates

To completely access deep system routes outside the standard safety profiles (such as `/var/root` or `Windows/System` directories), you must compile utilizing explicit bypass gates:

```bash
cargo build --features dangerous-clean
```

## Contributing

We thrive off Open Source collaborations.

### PR Protocol Structure
When issuing a Merge Request to `main`, observe these structural standards:
1. Ensure your history stays linear. Squashing your commits prior to review decreases the friction natively.
2. Ensure you have respected the Svelte 5 syntax directives (runes `$state`, `$derived`).
3. If targeting Backend functionality, guarantee zero `unsafe` variables exist across the scope.

---

*Engineered natively to unify memory-safe logic directly inside high-caliber interactive environments.*
