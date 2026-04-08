# Qleaner: 100 Deep Implementations, Fixes, & Architecture Improvements

> **📈 Progress Statistics**
> **Total Tasks:** 105 | **Done:** 31 | **Ongoing:** 1 | **Pending:** 73
> *Note: Agents must update these stats continuously as `[x]` / `[/]` / `[ ]` statuses are achieved.*

The current state of **Qleaner** is an MVP. While the integration between Tauri, Rust, and Svelte 5 is functioning, the application relies on synchronous looping, brute-force directory deletion, hardcoded generic paths, and a barebones UI loop.
Below are **100 required best implementations, fixes, and improvements** to transition Qleaner from a basic MVP into an enterprise-grade, high-performance system optimization tool.
---

## 🚨 Tier 1: High Priority (Architecture, Security, DevOps & CLI)
*Critical infrastructure, permissions, testing, and distribution pipelines.*
- **28.** **Windows Update Cache:** Add Windows `SoftwareDistribution/Download` cache cleaning (requires elevated privileges).
- [x] **37.** **Dry-Run Architecture:** Implement true dry-run scanning in Rust. Currently, [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301) just executes. Provide a simulation API to guarantee file counts.
- **53.** **Hover Context Menus:** Use `bits-ui` Dropdown to add right-click options to rows: "Open Folder Location", "Add to Ignore List", "View Properties".
- [x] **56.** **Sortable Columns:** Add clickable headers (Target, Category, Size) to sort the `cleanerStore.results` dynamically based on `$derived` state.
- [x] **76.** **Error Toast Notifications:** Instead of silently failing to delete an item, pipe exact Rust error messages up to a frontend `sonner` toast system.
- **77.** **Sudo Policy Enforcer:** If macOS requires Full Disk Access, implement a watcher that detects lacking permissions and redirects the user to `System Settings -> Privacy`.
- **78.** **Sandboxed IFrame Documentation:** Move help texts and privacy policies to a segregated iframe without standard script contexts.
- **85.** **E2E Tests (Playwright):** Integrate Playwright for Tauri E2E testing to simulate UI clicks automatically spawning mocked Tauri commands.
- **86.** **Component Tests (Vitest):** Add `vitest` for the layout logic, specifically testing the reactivity of `$derived(totalSelectedSize)`.
- **90.** **AppImage Builder (Linux):** Optimize Tauri config for modern Linux distributions with exact package exclusions.
- **94.** **Feature Flags Architecture:** Implement Cargo features (e.g., `#[cfg(feature = "dangerous-clean")]`) to gate highly destructive beta features.
- **98.** **Crash Reporting:** Integrate Sentry natively via `sentry-rust` and `sentry-javascript` to catch unhandled application panics remotely.
- **99.** **Update Auto-Updater:** Enable Tauri's built-in updater system (`plugin-updater`) so users get the latest optimization engines directly.
- **100.** **Architectural Readme:** Completely rewrite `README.md` introducing the dual Svelte/Rust architecture, contribution guidelines, PR templates, and local dev spin-up instructions.
- **107.** ** [BACKLOG] Browser Playwright Execution:** Establish an automated flow to run the browser dynamically and close it alongside strict required logging messages and comments.
- **109.** ** [BACKLOG] Universal Target Builds:** Scale the Linux actions explicitly for Ubuntu/Debian `.deb`, Fedora `.rpm`, Arch `.pacman` distributions alongside Apple notarization and Windows code signing.
- **110.** ** [BACKLOG] Production Telemetry:** Evaluate privacy-respecting mechanisms for telemetry logging and crash dumps to monitor application stability natively.
*Based on the competitive landscape observed in `docs/vison.html` mapping 30 top-tier disk utilities (e.g., CleanMyMac, BleachBit, DaisyDisk).*
- **118.** **Malware & Security Analysis (Malware):** Add baseline YARA scanning or static malware signature detection against cache payloads looking for common miners or rogue binaries (inspired by `Advanced SystemCare`).

---
## ⚡ Tier 2: Medium Priority (Core Mechanics, Scanners & Competitor Vision)
*Detection heuristics, deep system integrations, and multi-OS sweeps.*
- **6.** ** [INCOMPLETE: PARTIAL] Cancellation Token:** Implement a `tokio_util::sync::CancellationToken` to allow the user to abort a long-running scan or clean operation mid-way. - *Flag cancellation triggers exist, but underlying directory walking does not abort early (partial).*
- **14.** **Process Tracking Backend:** Add a backend scanner to detect running background apps (e.g., Chrome) and prevent emptying their caches while active.
- **17.** **File Ownership Checks:** On Linux/macOS, check if `uid == current_uid` before trying to delete, skipping root-owned caches gracefully instead of throwing exceptions.
- **21.** **Node.js Modules Sweeper:** Add dedicated scans for orphaned `node_modules` folders using [ignore](file:///home/drvoid/ISU/Qleaner/.gitignore) glob targeting within user space.
- **22.** **NPM/Yarn/PNPM Cache:** Explicitly target `~/.npm`, `~/AppData/Local/npm-cache`, and `~/.local/share/pnpm/store`.
- **23.** **Rust Cargo Cache:** Add scanning for `~/.cargo/registry` and `~/.cargo/git`.
- **24.** **Rust Target Sweeper:** Detect redundant `target/debug` directories in inactive Rust projects using heuristic age-based scanning.
- **26.** **macOS Xcode DerivedData:** Add `~/Library/Developer/Xcode/DerivedData` for massive storage recovery on macOS.
- **27.** **macOS iOS Simulators:** Scan and clear outdated iOS simulator caches (`~/Library/Developer/CoreSimulator/Devices`).
- **29.** **Windows Prefetch:** Add `C:\Windows\Prefetch` analysis.
- **30.** **Linux Journalctl Size:** Scan via `journalctl --disk-usage` and offer to vacuum logs older than X days.
- **31.** **Linux Flatpak/Snap Leftovers:** Target `.var/app/` caches and `snap` leftover blobs.
- **32.** **Browser Forensic Cache:** Add distinct targets for Chrome, Firefox, Safari, Edge caches, separate from generic "User Caches".
- **33.** **Discord / Slack Cache:** Target Electron app cache folders (e.g., `~/Library/Application Support/Discord/Cache`).
- **34.** **Empty Directory Sweeper:** Add an optional pass that identifies deeply nested, entirely empty directory trees and prunes them.
- **36.** **Time-based Filtering:** Allow the user to specify "Only clean files older than X days" (e.g., keep caches from the last 24 hours).
- **38.** **Configurable Ignoring:** Add a global `.qleanerignore` list to explicitly block directories from ever being scanned or listed.
- **39.** **Dependency Uninstaller:** Identify orphaned applications (macOS `.app` leftover plists, Windows rogue regkeys).
- **40.** **Duplicated Files (Dedup):** Implement a fast checksum-based (xxHash) duplicate file finder.
- **54.** **Path Truncation Logic:** Very long paths currently use CSS truncate, which obscures the end of the path (the important part). Use JS/CSS to truncate the middle (e.g., `~/Library.../Cache`).
- **58.** **Native OS Window Controls:** Add custom `titlebar` integration (via `<div data-tauri-drag-region>`) using Lucide icons for macOS/Windows custom decorators.
- **62.** **Tauri Capabilities Locking:** Lock down `tauri.conf.json` explicitly allowing *only* the specific [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301), [start_scan](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#217-275) commands. Deny wildcards.
- **64.** **Browser State Wiping Alert:** Auto-cleaning browser caches logs users out of sites. Require an explicit warning/consent checkbox before wiping "Chrome/Firefox Data".
- **66.** **Sensitive File Hashing:** Verify critical file hashes (like OS `hosts` or config binaries) before assuming something is a "Cache" based on name alone.
- **71.** **Anti-Virus Whitelisting Notice:** Some aggressive deep clean iterations scan locked folders triggering Windows Defender. Detect and handle `ACCESS_DENIED` transparently.
- **73.** **Shredding (Secure Erase):** Implement optional secure wiping (multi-pass overwrite) rather than just `unlink` for sensitive metadata.
- **74.** **Memory Scrubbing (Rust):** Implement `zeroize` strictly on any internal variables tracking decrypted user paths or credentials during scanning.
- **75.** **Database Locking Avoidance:** If Chrome is running, its `Cache.db` is locked. Detecting the `.lock` file and skipping the database prevents DB corruption.
- **80.** **Memory Leak Profiling:** Ensure `Mutex` states in Tauri aren't holding vast arrays of `WalkDir` `DirEntry` objects in memory indefinitely after a scan completes.
- **89.** **Code Signing:** Add notarization pipelines for macOS integration so it doesn't get blocked by Gatekeeper.
- **91.** **WiX Configuration (Windows):** Add an installer license, custom EULA, and start menu shortcut entries via refined `tauri.conf.json` WiX fragments.
- **113.** **Checksum Duplicate Finder (Dupes):** Integrate an `xxHash` high-speed engine to locate duplicated photos, archives, and binaries cross-system, yielding safe deletion arrays (inspired by `dupeGuru` / `Gemini 2`).
- **114.** **Startup Manager (Start):** Hook into OS background agents (LaunchDaemons on Mac, registry `Run` keys on Win, Systemd on Linux) to toggle bloatware auto-starting (inspired by `Stacer` / `CCleaner`).
- **115.** **System Native App Uninstaller (Uninst):** Intercept standard `.app` or registry uninstalls scanning for deep orphan plist caches globally to secure total application removals (inspired by `App Cleaner & Uninstaller`).
- **116.** **Privacy & Browser Exploitation Sweeper (Privacy):** Go beyond standard `.cache` by aggressively locating tracking cookies, form histories, DOM storages, and telemetry residues across Edge/Chrome/Firefox (inspired by `BleachBit`).
- **117.** **File Shredder (Shred):** Implement DoD 5220.22-M compliant multi-pass secure erasures to guarantee sensitive data files cannot be recovered via standard forensics (inspired by `CleanMyMac`).
- **119.** **OS Registry Repair (Reg):** Introduce distinct Windows registry scanning looking for invalid paths, ghost uninstaller references, and rogue COM keys (inspired by `Wise / Glary Utilities`).

---
## 🎨 Tier 3: Low Priority (UI/UX, Polish & Minor Tooling)
*Interface enhancements, quality of life, and visual feedback metrics.*
- **19.** **SQLite Config Persist:** Migrate off raw JSON/frontend local storage to a robust SQLite (`sqlx` + `sqlite`) backend for safely storing persistent schedules and excluded paths.
- **20.** **Tauri Plugin System:** Move the core cleaner logic out of [main.rs](file:///home/drvoid/ISU/Qleaner/src-tauri/src/main.rs)/[cleaner.rs](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs) into a structured Tauri Plugin (`tauri-plugin-qleaner-core`) for strict modularity.
- **35.** **Extension Profiling:** Add analysis for massive log files (`.log`, `.trace`) sitting abandoned in desktop directories.
- **42.** **Virtual Listicles:** The table renders every row. If thousands of junk locations are found, the DOM will lag. Implement `svelte-virtual-list`.
- **55.** **Sticky Table Headers:** Ensure the `thead` uses `z-index` and `backdrop-blur` properly behind overflowing content inside a constrained wrapper.
- [x] **57.** **Settings Sidebar:** Add a collapsible tool sidebar with navigation for "Dashboard", "Rules", "Schedules", and "Settings".
- **59.** **Keyboard Shortcuts:** Implement `svelte-window` keyboard listeners (e.g., `CMD+Enter` to start cleaning, `Esc` to cancel).
- **60.** **Accessibility (A11y):** Form checkboxes lack `aria-label` or `<label>` wrapping. Add strict strict accessibility tags to the data grid.
- **65.** **Path Traversal Protection:** Sanitize whatever IDs or Paths are sent from Svelte to Rust. Prevent `../../` attacks if the IPC gets intercepted.
- **68.** **Retry Logic (File Deletion):** Files might be temporarily locked. Add a backoff retry logic (e.g., `3 attempts, 100ms apart`) inside `std::fs::remove_dir_all`.
- **69.** **Detailed Logging (Tracing):** Add `tracing-subscriber` to rotate app activity logs to `~/.config/qleaner/app.log` for debugging and telemetry.
- **70.** **Audit History Dashboard:** Record every deletion in an append-only JSON/SQLite "History" tab so users can see exactly what was removed and when.
- **79.** **Session Cleanup Tracking:** Add a "Last Ran" timestamp enforced via cryptographic signing to prevent tampering with "System Health" scores.
- **82.** **ESLint & Prettier Strictness:** The Svelte app lacks the `@typescript-eslint/recommended-requiring-type-checking` rule set.
- **88.** **Release Bump Pipeline:** Create a script (via `release-plz` or `standard-version`) to automate bumping version parity between `package.json` and `Cargo.toml`.
- **92.** **Vite Bundle Optimization:** Optimize `vite.config.js` to split `lucide-svelte` and `bits-ui` chunks to shrink the V8 snapshot load time.
- **93.** **Benchmarking Suite:** Add `criterion` to benchmark the regex speeds and disk traversal speeds against massive mocked file trees.
- **95.** **Rust Format Checks:** Automate `cargo fmt --check` in the pre-commit hook via husky.
- **96.** **Git LFS for Icons:** Store the `app-icon-real.png` (250KB) and other high-res assets in Git LFS instead of tracking raw blobs.
- **108.** ** [BACKLOG] `fix-qicro-debugger` Repository Sync:** Review and merge any necessary commits strictly from the `fix-qicro-debugger` branch into master, culling deprecated segments.
- **112.** **Disk Usage Visualization (Viz):** Implement high-performance Rust-backed Treemaps or Sunburst charts to visually map largest folders directly inside the Svelte UI (inspired by `DaisyDisk` / `WinDirStat`).
- **120.** **System Hardware Monitoring:** Track real-time network up/down velocities, thermal temperatures, and exact CPU consumption per localized binary process (inspired by `Sensei`).

---
## ✅ Completed Tasks (Archive)
*Historical preservation of implemented milestones.*

- **121.** **[DONE] Linux Debugging and Build Fix:** Fixed Svelte 5 `<slot>` compilation errors, resolved `lucide-svelte` Github export changes, cleaned up TS errors in `vite.config.js`, and safely bypassed `sudo::escalate_if_needed()` for unblocked Tauri local dev server execution on Linux.

- **81.** **Cargo Clippy Pedantic:** Enforce `#![warn(clippy::pedantic)]` and `#![warn(clippy::unwrap_used)]` on the Rust codebase and fix the ~10 violations present.

- **83.** **Unit Tests (Rust):** Create `#[cfg(test)]` modules for `get_directory_size` and `human_readable_size` (testing bounds and edge cases).

- **84.** **File System Mocking:** Implement `tempfile::TempDir` to construct fake junk directories and test `clean_items` locally without nuking real system caches.

- **87.** **GitHub Actions CI/CD:** Add `.github/workflows/build.yml` compiling binaries for `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, and Linux AppImage.

- **61.** **Strict Content Security Policy (CSP):** The [+page.svelte](file:///home/drvoid/ISU/Qleaner/src/routes/+page.svelte) lacks CSP. Enforce rigid metas limiting `script-src` and `connect-src` specifically to Tauri IPC limits.

- **63.** **Scoped File System Access:** Ensure the Tauri API is strictly sandboxed. Never allow absolute path overrides from the frontend to [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301) – pass internal [id](file:///home/drvoid/Qix/Q-Static/src/lib/state/ui.svelte.ts#210-218) references instead.

- **67.** **Panic Recovery (Backend):** The `unwrap()` calls in `clean_items` (e.g., `state.scan_results.lock().unwrap()`) will crash the entire app if a thread poisons the lock. Handle mutex blocking safely.

- **72.** **Network Sandbox:** Qleaner currently requires no internet. Ensure `tauri.conf.json` fully disables all network/HTTP protocols.
- **111.** **CLI Architecture Support:** Develop a completely headless **Command-Line Interface (CLI)** version of Qleaner strictly for scriptable DevOps deployment, Linux servers, and TUI power users (inspired by `ncdu` / `TreeSize`).
- **25.** **Docker Builder Pruning:** Hook into the Docker CLI/socket to report and clean dangling images, volumes, and builder caches.
- **15.** **Cross-Platform Privilege Manager:** Use `sudo` crate or native auth APIs (e.g., macOS `Authorization Services`) to securely elevate privileges for system-level junk cleaning.
- **1.** ** [DONE] Remove Synchronous Blocking:** Replace `std::thread::sleep(150)` with `tokio::time::sleep` – the current implementation blocks the Tauri command pool. - *Replaced `std::thread::sleep` with `tokio::time::sleep` to unblock IPC thread.*
- **2.** ** [DONE] Asynchronous I/O:** Switch from standard `std::fs` to `tokio::fs` for non-blocking file deletion. - *Switched underlying functions to `tokio::fs` asynchronous operations.*
- **3.** ** [DONE] Rayon Multi-threading:** Use `rayon::WalkDir` or `ignore::WalkBuilder` for parallel size calculation across massive cache directories. - *Utilized `ignore` crossbeam parallel iterators.*
- **4.** ** [DONE] AppHandle Stream Events:** Shift from manual `app.emit` in a single thread to a crossbeam / mpsc channel streaming architecture for scalable progress reporting. - *Pushed events onto async buffers for streaming.*
- **5.** ** [DONE] Dynamic Disk Refresh:** Stop forcing a full `disks.refresh_list()` entirely blocking on [get_system_stats()](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#302-358) every tick. Cache disk infrastructure and only refresh usage. - *Reduced UI thrashing by fetching global sys info selectively.*
- **7.** ** [DONE] Granular Deletion:** Instead of `remove_dir_all` and recreating the directory, iterate over `.cache` contents and delete files individually to avoid breaking directory-level permissions or sticky bits. - *Iteration logic is present.*
- **8.** ** [DONE] Soft Deletion / Trash:** Implement an optional "Move to Trash" mechanism using the `trash` crate as a safety net before permanent deletion. - *Trash rollback added.*
- **9.** ** [DONE] Smart Sysinfo Polling:** `System::new_all()` is extremely expensive. Create a global persistent `OnceLock<Mutex<System>>` and only call `sys.refresh_cpu()` and `sys.refresh_memory()`. - *Cached state via Mutex.*
- **10.** ** [DONE] Batched Emissions:** Emitting Tauri events down to the frontend on every file/sub-directory will throttle the IPC bridge. Throttle emissions to max 60Hz. - *Throttled emissions down.*
- **11.** ** [DONE] Strict Type Safety:** Remove generic `Result<(), String>` returns and implement a custom `thiserror::Error` enum for structured Rust errors (e.g., `IoError`, `PermissionDenied`, `TauriError`). - *Integrated DataError / cleaner errors.*
- **12.** ** [DONE] File Size Caching:** Cache the sizes of large unmodified directories (like `node_modules` or `rust target`) between scans to drastically accelerate subsequent scans. - *Sizes cached locally.*
- **13.** ** [DONE] Exclude Locked Files:** Windows locked OS files will crash `remove_dir_all`. Implement "Skip-on-locked" generic fallback gracefully. - *Permissions denied skipped gracefully.*
- **16.** ** [DONE] Symlink Safeties:** Default `WalkDir` follows symlinks or can get stuck. Explicitly handle symlinks (`metadata.file_type().is_symlink()`) and do **not** traverse or delete them. - *Traversals ignore `is_symlink()` metadata flags.*
- **18.** ** [DONE] Custom Temporary Directory Fallbacks:** The backup string `C:\Windows\Temp` is bad practice. Rely entirely on `std::env::temp_dir()`. - *Hardcoded Temp strings fallback onto OS standard temps.*
- **41.** ** [DONE] Remove Checkbox Two-Way Binding Hacks:** `cleaner.results.forEach(r => r.selected = checked)` in inline markup breaks Svelte 5 rune reactivity deeply. Use granular component state propagation. - *Stores utilize granular class architectures now.*
- **43.** ** [DONE] Animated Numbers:** Use `@number-flow/svelte` or `svelte-motion` for the "Freed Space" and "System Memory" dials to smoothly count up rather than snapping instantly. - *Integrated `@number-flow/svelte` for dial animations.*
- **44.** ** [DONE] Streaming Progress Bar:** The current SVG/spin loader drops to 0 instantly. Build a linear progression bar component taking `current / total` into account. - *Bar calculates total sizing percentages live.*
- **45.** ** [DONE] Granular Expandable Rows:** The table only shows high-level (e.g., "User Caches"). Make rows expandable to show exactly which subdirectories take the most space. - *Implemented sub-folders display logic.*
- **46.** ** [DONE] Confirmation Modal Hierarchy:** Add a dangerous operation confirmation modal. Currently, clicking "Clean" immediately triggers deletion. - *Built bits-ui root modal confirmation dialogs.*
- **47.** ** [DONE] Svelte Runes Stores:** Migrate `cleanerStore` completely to a `class AppState { scan = $state() }` export pattern per the `Q-Static` standards. - *Switched all `$derived` into classes.*
- **48.** ** [DONE] Dynamic System Theming:** Reactively sync Tauri's native `appWindow.theme()` with the UI `data-theme` attribute (Dark/Light). - *Bound system OS preferences and exposed manual toggle in layout.*
- **49.** ** [DONE] Bento Grid Layout:** Transition the flat 3-card stats layout into a modern asymmetrical Bento grid displaying CPU waveform, Space gauge, and Last Cleaned metrics. - *Completed custom CSS-grid implementation.*
- **50.** ** [DONE] Svelte Transitions:** Add `in:fade` and `out:fly` on table items so when they are cleaned, they elegantly slide out of the list. - *Fade and fly transitions enabled natively.*
- **51.** ** [DONE] Responsive Data-Tables:** The existing `table` is rigid on small windows. Migrate to a flex-based or grid-based responsive row architecture. - *Table overflows flexibly alongside media query queries.*
- **52.** ** [DONE] Detailed Empty States:** The "optimal clean" state is static. Add a confetti animation or success badge with the exact date/time recorded. - *UX states handle empty layouts efficiently with banners.*
- **97.** ** [DONE] Internationalization (i18n):** Wrap raw strings ("Scanning...", "Target") using `paraglide` to prepare for multi-language rollouts. - *Wired paraglide-sveltekit and added English language base.*
- **101.** ** [DONE] Q-MCP Heuristics Sync:** Migrated advanced macOS formatting tools and sandbox container analysis functions directly from `Q-MCP/src-tauri/src/cleaner.rs` into our new modular OOP backend.
- **102.** ** [DONE] Open Source Polish:** Generated full `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, MIT/Apache2 dual licenses, and Linux `snapcraft.yaml` distribution settings.
- **103.** ** [DONE] Github Actions Workflows:** Configured deep CI/CD pipelines inside `.github/workflows/release.yml` mapping automated cross-platform deployments (Windows `.exe`, macOS `.dmg` Silicon/Intel, Linux `.AppImage`).
- **104.** ** [DONE] Kadir's UX PRs:** Integrated beautiful UX aesthetics, hover semantics, and Bento grids inspired by Kadir's Qapri implementations.
- **105.** ** [DONE] System Tray Menu:** Activated Tauri native tray daemon featuring background running capability and instant `Open` / `Clean Now` controls.
- **106.** ** [DONE] Frontend Feature Exhaustion:** Implemented `paraglide-sveltekit` i18n, custom layout OS themes, and full SvelteKit paths for `/feedback`, `/donate`, and `/about`.
- [x] **122.** **Theme Mode Toggle:** Implemented native system theme mode UI toggle mapping to OS values inside the structured Sidebar and Settings.
- [x] **123.** **Settings Permissions & Language Panel:** Display standard user permissions (File System, Network) and Paraglide Language selection natively inside the Preferences menu.
- [x] **124.** **OS Environment Status:** Track CPU mapping, architecture, and live memory state straight from Tauri plugins directly inside the Settings layout.
- [x] **125.** **Architecture Constraints Sync:** Mapped out QDebugger agent directives (Svelte/Tauri skills & workflows) explicitly inside AGENTS.md and GEMINI.md.
