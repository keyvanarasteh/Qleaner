# Rust Disk Cleaner Library — Complete Build Blueprint

> **Scope:** A production-grade, cross-platform (`linux`, `windows`, `macos`) Rust library
> (`diskclean-rs`) implementing every feature present across the 30 surveyed tools.
> This document covers **logic, algorithms, and DevOps steps only** — no UI code.
> Coverage target: **100%** unit + integration + property-based tests.

---

## Table of Contents

1. [Project Topology & Cargo Workspace](#1-project-topology--cargo-workspace)
2. [Core Data Model & Type System](#2-core-data-model--type-system)
3. [Feature Module Roadmap](#3-feature-module-roadmap)
4. [Module 1 — Filesystem Walker](#4-module-1--filesystem-walker)
5. [Module 2 — Junk & Cache Classifier](#5-module-2--junk--cache-classifier)
6. [Module 3 — Duplicate Finder](#6-module-3--duplicate-finder)
7. [Module 4 — Disk Usage Visualizer (Data Layer)](#7-module-4--disk-usage-visualizer-data-layer)
8. [Module 5 — Registry Cleaner (Windows)](#8-module-5--registry-cleaner-windows)
9. [Module 6 — Startup Manager](#9-module-6--startup-manager)
10. [Module 7 — App Uninstaller](#10-module-7--app-uninstaller)
11. [Module 8 — Privacy & Browser Cleaner](#11-module-8--privacy--browser-cleaner)
12. [Module 9 — Secure File Shredder](#12-module-9--secure-file-shredder)
13. [Module 10 — Malware / Threat Scanner](#13-module-10--malware--threat-scanner)
14. [Module 11 — CLI Engine](#14-module-11--cli-engine)
15. [Module 12 — Scheduler & Automation](#15-module-12--scheduler--automation)
16. [Module 13 — Report & Serialization Layer](#16-module-13--report--serialization-layer)
17. [Cross-Cutting Concerns](#17-cross-cutting-concerns)
18. [Testing Strategy — 100% Coverage](#18-testing-strategy--100-coverage)
19. [DevOps Pipeline](#19-devops-pipeline)
20. [Dependency Manifest](#20-dependency-manifest)
21. [Completion Checklist](#21-completion-checklist)

---

## 1. Project Topology & Cargo Workspace

### 1.1 Initialise workspace

```
diskclean-rs/
├── Cargo.toml                  ← workspace root
├── Cargo.lock
├── .cargo/
│   └── config.toml             ← target-cpu, linker flags, alias shortcuts
├── deny.toml                   ← cargo-deny: license + advisory policy
├── nextest.toml                ← cargo-nextest config
├── .clippy.toml
├── justfile                    ← task runner (just)
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── coverage.yml
├── crates/
│   ├── diskclean-core/         ← pure algorithms, no I/O
│   ├── diskclean-fs/           ← filesystem I/O layer
│   ├── diskclean-registry/     ← windows registry (cfg-gated)
│   ├── diskclean-browser/      ← browser profile parser
│   ├── diskclean-malware/      ← hash-based threat scanner
│   ├── diskclean-shred/        ← secure deletion
│   ├── diskclean-scheduler/    ← cron / task scheduler
│   ├── diskclean-report/       ← JSON / CSV / HTML report emitter
│   └── diskclean-cli/          ← binary crate (clap-based)
└── tests/
    ├── integration/
    └── fixtures/
```

### 1.2 Workspace `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
version     = "0.1.0"
edition     = "2021"
rust-version = "1.78"
license     = "MIT OR Apache-2.0"
authors     = ["qlinetech"]

[workspace.dependencies]
# shared versions pinned once here, referenced via `workspace = true`
tokio       = { version = "1", features = ["full"] }
rayon       = "1"
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
thiserror   = "1"
anyhow      = "1"
tracing     = "1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
walkdir     = "2"
sha2        = "0.10"
blake3      = "1"
regex       = "1"
chrono      = { version = "0.4", features = ["serde"] }
clap        = { version = "4", features = ["derive","env"] }
indicatif   = "0.17"
tempfile    = "3"
proptest    = "1"
rstest      = "0.21"
```

### 1.3 `.cargo/config.toml`

```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[alias]
xt  = "nextest run"
cov = "llvm-cov nextest"
doc = "doc --no-deps --all-features"
```

---

## 2. Core Data Model & Type System

> Location: `crates/diskclean-core/src/model.rs`

### 2.1 Error hierarchy

```
DiskCleanError
├── Io(std::io::Error)
├── Scan(ScanError)
│   ├── PermissionDenied(PathBuf)
│   ├── SymlinkLoop(PathBuf)
│   └── TooDeep { path, depth }
├── Registry(RegistryError)           [windows only]
│   ├── KeyNotFound(String)
│   └── AccessDenied(String)
├── Shred(ShredError)
│   ├── VerificationFailed(PathBuf)
│   └── PartialWrite { path, written, expected }
├── Browser(BrowserError)
│   └── ProfileLocked(PathBuf)
├── Malware(MalwareError)
│   └── DatabaseNotLoaded
└── Report(ReportError)
```

Implementation: `thiserror` derive macros. All variants carry structured context, never raw strings.

### 2.2 Core domain types

```rust
/// Canonical representation of a discovered file
pub struct FileEntry {
    pub path:         PathBuf,
    pub size:         u64,           // bytes, 0 for dirs
    pub kind:         EntryKind,
    pub modified:     SystemTime,
    pub accessed:     SystemTime,
    pub hash:         Option<FileHash>,
    pub classification: Option<JunkClass>,
}

pub enum EntryKind { File, Dir, Symlink, Other }

pub enum FileHash {
    Blake3([u8; 32]),
    Sha256([u8; 32]),
}

/// Junk classification taxonomy (maps to all 30 tools' cleaning categories)
pub enum JunkClass {
    SystemCache,        // /var/cache, %TEMP%, ~/Library/Caches
    AppCache,           // per-app cache dirs
    BrowserCache,       // Chrome/Firefox/Safari cache
    BrowserHistory,
    BrowserCookies,
    BrowserSessions,
    Log,                // *.log, /var/log
    CrashReport,        // *.dmp, CrashReporter, panic files
    ThumbnailCache,     // thumbs.db, ~/.cache/thumbnails
    PackageCache,       // apt/dnf/brew cache, pip __pycache__
    OldKernel,          // linux: /boot/vmlinuz-old
    LocalisationFile,   // unused .mo/.po locale files
    OrphanedConfig,     // config dirs of uninstalled apps
    IncompleteDownload, // *.part, *.crdownload, *.tmp
    DuplicateFile,
    LargeFile { threshold_bytes: u64 },
    StaleFile { age_days: u32 },
    TrashBin,           // ~/.local/share/Trash, Recycle Bin
    TimeSnapshot,       // macOS Time Machine local snapshots
    MailAttachment,     // Mail.app attachments
    UpdateRemnant,      // Windows: $WinREAgent, SoftwareDistribution
    IosBacup,           // ~/Library/Application Support/MobileSync
    DevArtifact,        // target/, node_modules/, .gradle, __pycache__
    Swap,               // swap/pagefile wipe
    FreeSpaceTrace,     // previously-deleted file traces
}

pub struct ScanResult {
    pub entries:        Vec<FileEntry>,
    pub total_bytes:    u64,
    pub reclaimable:    u64,
    pub scan_duration:  Duration,
    pub errors:         Vec<(PathBuf, DiskCleanError)>,
}

pub struct CleanPlan {
    pub to_delete:  Vec<FileEntry>,
    pub to_shred:   Vec<FileEntry>,
    pub estimated_freed: u64,
}

pub struct CleanReport {
    pub freed_bytes:    u64,
    pub deleted_count:  usize,
    pub errors:         Vec<(PathBuf, DiskCleanError)>,
    pub duration:       Duration,
    pub timestamp:      DateTime<Utc>,
}
```

### 2.3 Configuration

```rust
pub struct ScanConfig {
    pub roots:              Vec<PathBuf>,
    pub max_depth:          Option<usize>,       // None = unlimited
    pub follow_symlinks:    bool,
    pub cross_device:       bool,
    pub min_size:           u64,
    pub include_hidden:     bool,
    pub respect_gitignore:  bool,
    pub thread_count:       usize,               // 0 = num_cpus
    pub hash_algorithm:     HashAlgorithm,
    pub junk_rules:         JunkRuleSet,         // pluggable rule engine
    pub exclusions:         Vec<GlobPattern>,
}

pub enum HashAlgorithm { Blake3, Sha256 }
```

---

## 3. Feature Module Roadmap

| #   | Module                    | Primary Algorithm                       | Platforms |
| --- | ------------------------- | --------------------------------------- | --------- |
| 1   | Filesystem Walker         | Parallel DFS + inode dedup              | All       |
| 2   | Junk & Cache Classifier   | Rule engine (glob + regex + age + size) | All       |
| 3   | Duplicate Finder          | Multi-pass hash bucketing               | All       |
| 4   | Disk Usage Visualizer     | Weighted treemap (squarified)           | All       |
| 5   | Registry Cleaner          | BFS registry walk + validity checks     | Windows   |
| 6   | Startup Manager           | Platform autostart read/write           | All       |
| 7   | App Uninstaller           | Receipt-based leftover graph            | All       |
| 8   | Privacy & Browser Cleaner | Profile path resolver + SQLite wipe     | All       |
| 9   | Secure Shredder           | DoD 5220.22-M + Gutmann + NVMe purge    | All       |
| 10  | Malware Scanner           | YARA rule engine + hash blocklist       | All       |
| 11  | CLI Engine                | Clap v4 + interactive TUI               | All       |
| 12  | Scheduler                 | Cron parser + OS task registration      | All       |
| 13  | Report Layer              | JSON / CSV / HTML emitter               | All       |

---

## 4. Module 1 — Filesystem Walker

> `crates/diskclean-fs/src/walker.rs`

### 4.1 Algorithm — Parallel DFS with inode deduplication

**Step 1 — Entry point**
Accept `ScanConfig`. Build a `rayon::ThreadPool` sized to `config.thread_count`.

**Step 2 — Root queue**
Push each root into a `SegQueue<DirTask>` (lock-free concurrent queue, crossbeam).

**Step 3 — Worker loop** (each rayon thread)

```
while let Some(task) = queue.pop() {
    read_dir(task.path)
    for each entry:
        stat entry → get (dev, ino) pair
        if inode_set.contains(dev, ino) → skip (hardlink already counted)
        inode_set.insert(dev, ino)

        if symlink && !follow_symlinks → record as Symlink, skip descent
        if dir && (cross_device || same_dev(entry, root_dev)):
            if depth < max_depth: queue.push(DirTask { path: entry, depth+1 })
        if file:
            classify(entry) → Option<JunkClass>
            push FileEntry into results_channel
}
```

**Step 4 — Cycle detection**
Maintain a `DashSet<(u64,u64)>` (device, inode). Before descending any directory, check membership. On insertion collision → log `ScanError::SymlinkLoop` and skip.

**Step 5 — Progress reporting**
Emit `ScanProgress { dirs_visited, files_found, bytes_seen }` via an `mpsc` channel every 100ms for UI consumers.

**Step 6 — Error isolation**
Wrap each `read_dir` in `match`. On `PermissionDenied`: push to `errors` vec, continue. Never panic.

**Step 7 — Collect**
Drain results channel into `ScanResult`. Sort entries by `size DESC` for downstream consumers.

### 4.2 Platform specifics

| Platform | Inode source                                                                 | Hidden file rule                |
| -------- | ---------------------------------------------------------------------------- | ------------------------------- |
| Linux    | `std::os::unix::fs::MetadataExt` `.ino()` + `.dev()`                         | leading `.` in name             |
| macOS    | same as Linux                                                                | leading `.` + `UF_HIDDEN` xattr |
| Windows  | `BY_HANDLE_FILE_INFORMATION` → `nFileIndexHigh/Low` + `dwVolumeSerialNumber` | `FILE_ATTRIBUTE_HIDDEN` flag    |

### 4.3 MFT fast-path (Windows only)

For drives where the process has `SeManageVolumePrivilege`, open `\\.\C:` and read the MFT directly using `FSCTL_ENUM_USN_DATA`. This mirrors WizTree's approach: enumerate all files in one sequential read before any stat calls, achieving ~3s/TB.

Steps:

1. Attempt privilege acquisition via `AdjustTokenPrivileges`.
2. On success: iterate MFT records, build `HashMap<FileReferenceNumber, FileEntry>`.
3. On failure: fall back to standard `read_dir` walker.

---

## 5. Module 2 — Junk & Cache Classifier

> `crates/diskclean-core/src/classifier.rs`

### 5.1 Rule engine design

Rules are data, not code. Ship a built-in `rules.toml` and allow user-supplied overrides.

```toml
[[rules]]
id          = "system-temp-linux"
platforms   = ["linux"]
match_type  = "glob"
pattern     = "/tmp/**"
junk_class  = "SystemCache"
min_age_days = 1
safe_to_auto_delete = true

[[rules]]
id          = "apt-cache"
platforms   = ["linux"]
match_type  = "glob"
pattern     = "/var/cache/apt/archives/*.deb"
junk_class  = "PackageCache"
safe_to_auto_delete = true

[[rules]]
id          = "macos-caches"
platforms   = ["macos"]
match_type  = "glob"
pattern     = "~/Library/Caches/**"
junk_class  = "AppCache"
safe_to_auto_delete = false   # require user confirmation

[[rules]]
id          = "windows-prefetch"
platforms   = ["windows"]
match_type  = "glob"
pattern     = "C:\\Windows\\Prefetch\\*.pf"
junk_class  = "SystemCache"
safe_to_auto_delete = true

[[rules]]
id          = "crash-dumps"
platforms   = ["windows", "linux", "macos"]
match_type  = "regex"
pattern     = ".*\\.(dmp|mdmp|crash)$"
junk_class  = "CrashReport"
safe_to_auto_delete = true

[[rules]]
id          = "dev-artifacts"
platforms   = ["linux", "macos", "windows"]
match_type  = "dir_name"
pattern     = "target"
parent_heuristic = "has_sibling:Cargo.toml"
junk_class  = "DevArtifact"
safe_to_auto_delete = false
```

### 5.2 Classification algorithm

```
fn classify(entry: &FileEntry, rules: &JunkRuleSet) -> Option<JunkClass> {
    let platform = current_platform();
    for rule in rules.iter_sorted_by_priority() {
        if !rule.platforms.contains(platform) { continue }
        if rule.matches(entry) {
            if let Some(min_age) = rule.min_age_days {
                if entry_age_days(entry) < min_age { continue }
            }
            if let Some(min_size) = rule.min_size_bytes {
                if entry.size < min_size { continue }
            }
            return Some(rule.junk_class.clone())
        }
    }
    None
}
```

Match types (evaluated in order):

1. `glob` — compiled via `globset` crate, cached per-rule
2. `regex` — compiled via `regex` crate
3. `dir_name` — exact name match on last path component
4. `extension` — `entry.path.extension()` comparison
5. `parent_heuristic` — look for sibling file (`Cargo.toml`, `package.json`, etc.)

### 5.3 Age & size predicates

```rust
fn entry_age_days(entry: &FileEntry) -> u64 {
    let age = SystemTime::now()
        .duration_since(entry.modified)
        .unwrap_or_default();
    age.as_secs() / 86400
}
```

Large-file detection: flag any file where `entry.size > config.large_file_threshold` (default 500 MB).

### 5.4 Whitelisting

Maintain a `HashSet<PathBuf>` of user-protected paths. Any `FileEntry` whose path or ancestor is in the whitelist is unconditionally skipped, regardless of rules. This is BleachBit's whitelist feature.

---

## 6. Module 3 — Duplicate Finder

> `crates/diskclean-fs/src/duplicates.rs`

### 6.1 Multi-pass hash bucketing (minimise I/O)

**Pass 0 — Size bucketing** (free, no I/O beyond what the walker already did)

```
Group all FileEntry by size.
Discard all buckets with count == 1.
Remaining: candidate_groups: HashMap<u64, Vec<FileEntry>>
```

**Pass 1 — First-4KB partial hash**

```
For each group in candidate_groups:
    For each file: read first 4096 bytes → Blake3 digest → partial_hash
    Re-bucket by partial_hash
    Discard singleton sub-groups
```

Rationale: eliminates ~90% of candidates before reading full files.

**Pass 2 — Full file hash**

```
For remaining candidates:
    Stream full file in 256KB chunks → Blake3 digest → full_hash
    Re-bucket by full_hash
    Groups with count >= 2 are confirmed duplicate sets
```

**Pass 3 — Byte-by-byte verification** (optional, high-assurance mode)

```
For each confirmed duplicate group:
    Pick the largest two files
    Compare byte-by-byte in 64KB windows
    If mismatch found: hash collision → remove from group (astronomically unlikely with Blake3 but correct)
```

### 6.2 Audio duplicate detection (dupeGuru music mode)

Implement a simplified acoustic fingerprint comparison:

1. Decode audio metadata (ID3/Vorbis tags) via `audiotags` or `lofty` crate.
2. Compare `(duration_ms, sample_rate, bitrate)` as a pre-filter bucket.
3. For files in the same bucket, extract a short PCM segment (first 10s) and compute a perceptual hash using chromaprint algorithm (port the fingerprint correlation logic).
4. Similarity threshold: configurable (default 0.85).

### 6.3 Similar image detection

1. Decode image to 8×8 grayscale thumbnail via `image` crate.
2. Compute DCT-based perceptual hash (pHash): 64-bit integer.
3. Build a BK-tree indexed by Hamming distance for sub-linear nearest-neighbour queries.
4. Default threshold: Hamming distance ≤ 10 (configurable).

### 6.4 Deduplication strategy options

```rust
pub enum DedupeStrategy {
    KeepNewest,
    KeepOldest,
    KeepLargest,        // for partial files
    KeepSmallest,
    KeepByPath(Regex),  // keep whichever path matches pattern
    Interactive,        // emit candidates, let caller decide
}
```

---

## 7. Module 4 — Disk Usage Visualizer (Data Layer)

> `crates/diskclean-core/src/treemap.rs`

This module produces the data structures consumed by any renderer (TUI, web, desktop). No rendering code here.

### 7.1 Tree aggregation

**Step 1 — Build size tree from `ScanResult`**

```
fn build_tree(entries: &[FileEntry]) -> TreeNode {
    let mut root = TreeNode::new("/");
    for entry in entries {
        root.insert(entry.path.components(), entry.size);
    }
    // Post-order: each dir node accumulates children's sizes
    root.propagate_sizes();
    root
}
```

**Step 2 — Squarified treemap layout algorithm**

Squarified treemap (Bruls, Huizing, van Wijk 2000):

```
fn squarify(items: &[TreeNode], rect: Rect) -> Vec<LayoutRect> {
    if items.is_empty() { return vec![]; }
    let total: u64 = items.iter().map(|n| n.size).sum();
    let mut row: Vec<&TreeNode> = vec![];
    let mut results = vec![];
    let mut remaining_rect = rect;
    for item in items {
        row.push(item);
        if worst_aspect_ratio(&row, remaining_rect, total)
            <= worst_aspect_ratio(&row[..row.len()-1], remaining_rect, total)
        {
            // continue building row
        } else {
            results.extend(layout_row(&row[..row.len()-1], &mut remaining_rect, total));
            row = vec![item];
        }
    }
    results.extend(layout_row(&row, &mut remaining_rect, total));
    results
}

fn worst_aspect_ratio(row: &[&TreeNode], rect: Rect, total: u64) -> f64 {
    let row_sum: u64 = row.iter().map(|n| n.size).sum();
    let w = rect.shorter_side();
    let max_item = row.iter().map(|n| n.size).max().unwrap_or(0) as f64;
    let min_item = row.iter().map(|n| n.size).min().unwrap_or(0) as f64;
    let s = row_sum as f64 / total as f64 * rect.area();
    f64::max(
        w * w * max_item / (s * s),
        s * s / (w * w * min_item),
    )
}
```

Output: `Vec<LayoutRect { path, size, x, y, w, h, depth, color_category }>`.

### 7.2 Sunburst (DaisyDisk-style) layout

Recursive polar coordinates:

```
fn sunburst(node: &TreeNode, start_angle: f64, end_angle: f64, depth: u32) -> Vec<ArcSegment> {
    let mut segments = vec![];
    let total = node.total_size as f64;
    let mut angle = start_angle;
    for child in &node.children {
        let span = (child.total_size as f64 / total) * (end_angle - start_angle);
        segments.push(ArcSegment {
            path:        child.path.clone(),
            inner_r:     depth as f64 * RING_WIDTH,
            outer_r:     (depth + 1) as f64 * RING_WIDTH,
            start_angle: angle,
            end_angle:   angle + span,
        });
        segments.extend(sunburst(child, angle, angle + span, depth + 1));
        angle += span;
    }
    segments
}
```

### 7.3 Purgeable space (macOS)

On macOS, query `statfs` for `f_bfree` vs `f_bavail`. The difference × block size = purgeable. Surface this as a separate `PurgeableEntry` in the tree root.

---

## 8. Module 5 — Registry Cleaner (Windows)

> `crates/diskclean-registry/src/lib.rs` — `#[cfg(target_os = "windows")]`

### 8.1 Registry scan algorithm

**Step 1 — BFS walk of HKLM and HKCU**

Use `winreg` crate. Start from well-known root keys:

```
HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*
HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*
HKLM\SYSTEM\CurrentControlSet\Services\*
HKCU\SOFTWARE\*  (orphaned software keys)
HKLM\SOFTWARE\Classes\CLSID\*  (broken COM registrations)
HKLM\SOFTWARE\Classes\Interface\*
HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\*
```

**Step 2 — Validity checks per key type**

| Key Category     | Validity Check                               |
| ---------------- | -------------------------------------------- |
| Uninstall entry  | `InstallLocation` path exists on disk        |
| Service          | `ImagePath` binary exists on disk            |
| COM CLSID        | `InprocServer32` DLL exists                  |
| File association | `ProgID` → CLSID exists and is valid         |
| Startup entry    | Referenced executable exists                 |
| MUI cache        | Executable path still valid                  |
| AppPath          | `Path` value exists                          |
| Shared DLL       | Reference count > 0 and file exists          |
| Font             | `.ttf/.otf` file exists in `%WINDIR%\Fonts\` |

**Step 3 — Confidence scoring**

```rust
pub enum RegistryIssue {
    Orphaned { key: RegistryKey, reason: OrphanReason },
    InvalidRef { key: RegistryKey, value: String, missing_path: PathBuf },
    Deprecated { key: RegistryKey },
    Duplicate { key: RegistryKey, canonical: RegistryKey },
}

pub struct RegistryKey {
    pub hive:    Hive,
    pub path:    String,
    pub values:  HashMap<String, RegValue>,
}
```

**Step 4 — Safe-delete only**
Never delete keys from `HKLM\SYSTEM\CurrentControlSet\Control\*` or `HKLM\SECURITY\*`. Maintain a hardcoded protection list. Backup to `.reg` file before any deletion (export via `RegSaveKeyEx`).

**Step 5 — Backup before clean**

```
fn backup_key(key: &RegistryKey) -> Result<PathBuf> {
    let backup_path = backup_dir().join(format!("{}.reg", key.safe_name()));
    // call REG EXPORT or RegSaveKeyEx
    Ok(backup_path)
}
```

---

## 9. Module 6 — Startup Manager

> `crates/diskclean-fs/src/startup.rs`

### 9.1 Startup entry discovery — per platform

**Linux:**

```
Sources:
- ~/.config/autostart/*.desktop
- /etc/xdg/autostart/*.desktop
- systemd user units: ~/.config/systemd/user/*.service (WantedBy=default.target)
- /etc/init.d/ (legacy SysV)
- ~/.bashrc, ~/.profile, ~/.zshrc → parse for added commands (heuristic, read-only)
```

Parse `.desktop` files: extract `Name`, `Exec`, `Hidden`, `OnlyShowIn` fields.

**macOS:**

```
Sources:
- ~/Library/LaunchAgents/*.plist
- /Library/LaunchAgents/*.plist
- /Library/LaunchDaemons/*.plist
- /System/Library/LaunchAgents/*.plist  (read-only, display only)
- ~/Library/Application Support/com.apple.backgroundtaskmanagementd  (macOS 13+)
```

Parse `.plist` via `plist` crate. Extract `Label`, `ProgramArguments`, `Disabled`, `RunAtLoad`.

**Windows:**

```
Sources:
- HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
- HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
- HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce
- HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce
- %APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\
- %ProgramData%\Microsoft\Windows\Start Menu\Programs\Startup\
- Scheduled Tasks (COM-based, ITaskService)
- Services (SCM query via winapi)
```

### 9.2 Enable / disable / delete operations

```rust
pub trait StartupEntry {
    fn name(&self) -> &str;
    fn command(&self) -> &str;
    fn is_enabled(&self) -> bool;
    fn enable(&mut self)  -> Result<()>;
    fn disable(&mut self) -> Result<()>;
    fn delete(&mut self)  -> Result<()>;
    fn source(&self) -> StartupSource;
}
```

For `.desktop` files: toggle `Hidden=true/false`.
For `.plist` files: toggle `Disabled` key.
For Registry: move key to `RunOnce` disabled prefix or delete value.
For systemd: call `systemctl --user disable <unit>` (via `std::process::Command`).

---

## 10. Module 7 — App Uninstaller

> `crates/diskclean-fs/src/uninstaller.rs`

### 10.1 App receipt graph construction

**Linux:**

```
1. Query dpkg: parse /var/lib/dpkg/info/<pkg>.list → all installed files
2. Query rpm: rpm -ql <package>
3. Flatpak: flatpak list --app → com.example.App → data in ~/.var/app/<id>/
4. AppImage: track .appimage file + ~/.config/<app>/ + ~/.local/share/<app>/
5. Orphaned config heuristic:
   - For any dir in ~/.config/ and ~/.local/share/: check if owning binary still exists
     via `which <dirname>` or dpkg -S <path>
```

**macOS:**

```
1. Parse /Applications/<App>.app/Contents/Info.plist → CFBundleIdentifier
2. Collect all related paths:
   - ~/Library/Application Support/<BundleID>/
   - ~/Library/Preferences/<BundleID>.plist
   - ~/Library/Caches/<BundleID>/
   - ~/Library/Logs/<AppName>/
   - ~/Library/Containers/<BundleID>/
   - ~/Library/Application Scripts/<BundleID>/
   - ~/Library/Saved Application State/<BundleID>.savedState/
   - /Library/Application Support/<AppName>/
   - /Library/Preferences/<BundleID>.plist
3. Also check ~/.local/share/ for non-sandboxed apps
```

**Windows:**

```
1. Read HKLM\SOFTWARE\...\Uninstall\<AppID>:
   - DisplayName, Publisher, InstallLocation, UninstallString, InstallDate, EstimatedSize
2. Collect leftovers:
   - %APPDATA%\<Publisher>\<App>\
   - %LOCALAPPDATA%\<Publisher>\<App>\
   - %PROGRAMDATA%\<Publisher>\<App>\
   - %LOCALAPPDATA%\Temp\<App>*
   - Residual registry keys under HKCU\SOFTWARE\<Publisher>\
3. Run official UninstallString silently (if present)
4. Clean leftover paths after uninstall
```

### 10.2 Leftover detection algorithm

```
fn find_leftovers(app: &AppReceipt) -> Vec<FileEntry> {
    let candidates = well_known_leftover_dirs(app);
    candidates
        .into_iter()
        .filter(|p| p.exists())
        .flat_map(|p| walk_shallow(p, max_depth=3))
        .filter(|e| matches_app_identity(e, app))
        .collect()
}

fn matches_app_identity(entry: &FileEntry, app: &AppReceipt) -> bool {
    let name = entry.path.to_string_lossy().to_lowercase();
    name.contains(&app.bundle_id.to_lowercase())
    || name.contains(&app.display_name.to_lowercase())
    || app.known_paths.iter().any(|p| entry.path.starts_with(p))
}
```

---

## 11. Module 8 — Privacy & Browser Cleaner

> `crates/diskclean-browser/src/lib.rs`

### 11.1 Browser profile resolver

```rust
pub enum Browser {
    ChromiumBased { name: String, profile_dir: PathBuf },
    Firefox { profile_dir: PathBuf },
    Safari,
    Edge,
    Opera,
    Brave,
}

fn resolve_profiles() -> Vec<BrowserProfile> {
    let mut profiles = vec![];
    // Chrome / Chromium
    for dir in chromium_base_dirs() {   // platform-specific
        if dir.exists() {
            for entry in read_dir(dir) {
                if is_profile_dir(entry) {
                    profiles.push(BrowserProfile::chromium(entry));
                }
            }
        }
    }
    // Firefox
    if let Some(ini) = firefox_profiles_ini() {
        profiles.extend(parse_profiles_ini(ini));
    }
    profiles
}
```

Platform default paths:

| Browser | Linux                       | macOS                                           | Windows                                    |
| ------- | --------------------------- | ----------------------------------------------- | ------------------------------------------ |
| Chrome  | `~/.config/google-chrome/`  | `~/Library/Application Support/Google/Chrome/`  | `%LOCALAPPDATA%\Google\Chrome\User Data\`  |
| Firefox | `~/.mozilla/firefox/`       | `~/Library/Application Support/Firefox/`        | `%APPDATA%\Mozilla\Firefox\Profiles\`      |
| Safari  | N/A                         | `~/Library/Safari/`                             | N/A                                        |
| Edge    | `~/.config/microsoft-edge/` | `~/Library/Application Support/Microsoft Edge/` | `%LOCALAPPDATA%\Microsoft\Edge\User Data\` |

### 11.2 Chromium-based cleaning algorithm

Chromium stores history, cookies, and cache as SQLite databases and flat files.

```
Profile/
├── Cache/              ← flat cache blocks, safe to delete entirely
├── Code Cache/
├── GPUCache/
├── History             ← SQLite: urls, visits tables
├── Cookies             ← SQLite: cookies table
├── Web Data            ← SQLite: autofill, credit cards (optionally)
├── Login Data          ← SQLite: logins (passwords — skip unless explicitly requested)
├── Sessions/
└── Current Session
```

**Cache:** `rm -rf Profile/Cache/* Profile/Code\ Cache/* Profile/GPUCache/*`

**History:** Open SQLite file using `rusqlite`. Execute:

```sql
DELETE FROM urls WHERE last_visit_time < :cutoff;
DELETE FROM visits WHERE visit_time < :cutoff;
DELETE FROM keyword_search_terms;
VACUUM;
```

**Cookies:**

```sql
DELETE FROM cookies WHERE expires_utc < :cutoff OR host_key LIKE :pattern;
VACUUM;
```

**Session files:** Delete only if browser is not currently running (check process list via `sysinfo` crate).

### 11.3 Firefox cleaning algorithm

Firefox uses Mozilla's `places.sqlite` and `cookies.sqlite`:

```sql
-- places.sqlite
DELETE FROM moz_historyvisits WHERE visit_date < :cutoff;
DELETE FROM moz_places WHERE last_visit_date < :cutoff AND visit_count = 0;
VACUUM;

-- cookies.sqlite
DELETE FROM moz_cookies WHERE expiry < :cutoff OR baseDomain LIKE :pattern;
VACUUM;
```

Cache2 (flat file format): enumerate `~/.cache/mozilla/firefox/<profile>/cache2/entries/` and delete files older than threshold.

### 11.4 Process lock detection

Before touching any SQLite file, verify the browser is not running:

```rust
fn browser_is_running(browser: &Browser) -> bool {
    let proc_names = browser.process_names();  // ["chrome", "chromium", "Google Chrome"]
    sysinfo::System::new_all()
        .processes_by_name(proc_names)
        .next()
        .is_some()
}
```

If locked, return `BrowserError::ProfileLocked`.

---

## 12. Module 9 — Secure File Shredder

> `crates/diskclean-shred/src/lib.rs`

### 12.1 Overwrite passes — algorithm selection

```rust
pub enum ShredAlgorithm {
    SinglePass,                 // 1× random bytes (adequate for modern SSDs)
    DoD5220_22M,                // 3-pass: 0x00, 0xFF, random
    DoD5220_22MECE,             // 7-pass ECE variant
    Gutmann,                    // 35-pass (legacy HDDs only)
    NvmePurge,                  // NVMe Secure Erase via ioctl (Linux/Win)
    Custom { passes: Vec<PassSpec> },
}

pub struct PassSpec {
    pub pattern: PassPattern,
    pub verify:  bool,
}

pub enum PassPattern {
    Zeros,
    Ones,
    Random,
    Fixed(u8),
    AlternatingBits,
}
```

### 12.2 Shred algorithm — step by step

**Step 1 — Determine storage type**

```rust
fn detect_storage_type(path: &Path) -> StorageType {
    // Linux: read /sys/block/<dev>/queue/rotational
    // macOS: IOKit IOStorageFamily
    // Windows: IOCTL_STORAGE_QUERY_PROPERTY DeviceSeekPenaltyProperty
}
// If SSD/NVMe: warn that overwrite-based shredding is unreliable due to wear levelling
// Recommend NvmePurge instead
```

**Step 2 — Overwrite passes**

```rust
fn overwrite_passes(file: &mut File, len: u64, algorithm: &ShredAlgorithm) -> Result<()> {
    let passes = algorithm.passes();
    for pass in &passes {
        file.seek(SeekFrom::Start(0))?;
        let mut written = 0u64;
        while written < len {
            let chunk = fill_chunk(pass.pattern, 65536);  // 64KB buffer
            file.write_all(&chunk)?;
            written += chunk.len() as u64;
        }
        file.flush()?;
        if pass.verify {
            verify_pass(file, len, pass.pattern)?;
        }
    }
    Ok(())
}
```

**Step 3 — Filename obfuscation**
Before deletion, rename file N times with random names of same length to overwrite directory entry metadata:

```rust
fn obfuscate_name(path: &Path, iterations: usize) -> Result<PathBuf> {
    let mut current = path.to_owned();
    for _ in 0..iterations {
        let new_name = random_alphanum(current.file_name().unwrap().len());
        let new_path = current.with_file_name(new_name);
        fs::rename(&current, &new_path)?;
        current = new_path;
    }
    Ok(current)
}
```

**Step 4 — Truncate then unlink**

```rust
file.set_len(0)?;
file.sync_all()?;   // ensure OS flushes to device
drop(file);
fs::remove_file(&obfuscated_path)?;
```

**Step 5 — Free-space wiping** (BleachBit feature)
Create a large file filling remaining free space with random data, then delete it. This overwrites previously deleted file data remaining in unallocated clusters:

```rust
fn wipe_free_space(mount_point: &Path) -> Result<()> {
    let avail = available_space(mount_point)?;
    let tmp = mount_point.join(".diskclean_wipe_XXXXXXXX");
    let mut f = File::create(&tmp)?;
    write_random_chunks(&mut f, avail - RESERVE_BYTES)?;
    f.sync_all()?;
    drop(f);
    fs::remove_file(tmp)?;
    Ok(())
}
```

### 12.3 NVMe secure erase

```rust
#[cfg(target_os = "linux")]
fn nvme_secure_erase(device: &Path) -> Result<()> {
    // Open device with O_RDWR | O_DIRECT
    // Issue NVME_IOCTL_ADMIN_CMD with opcode 0x06 (Format NVM)
    // ses field = 0x01 (User Data Erase) or 0x02 (Cryptographic Erase)
    // Requires CAP_SYS_ADMIN or root
}
```

---

## 13. Module 10 — Malware / Threat Scanner

> `crates/diskclean-malware/src/lib.rs`

### 13.1 Hash-based detection engine

**Step 1 — Load hash database**
Ship a compressed Bloom filter (`bloom` crate) pre-populated from:

- MalwareBazaar SHA256 export (public domain)
- NSRL RDS (NIST, identifies known-good files — invert for allowlist)
- User-configurable additional blocklist files

```rust
pub struct ThreatDatabase {
    malware_bloom: BloomFilter,        // false-positive rate: 0.001%
    malware_exact: HashSet<[u8;32]>,   // exact SHA256, for confirmed hits
    allowlist:     HashSet<[u8;32]>,   // known-good (NSRL)
}
```

**Step 2 — Scan pipeline**

```
For each FileEntry where kind == File and size > 0:
    1. Compute SHA256 (reuse if already computed by duplicate finder)
    2. Check allowlist → skip if found
    3. Probe Bloom filter → if negative: clean (no false negatives possible)
    4. If Bloom positive: check exact set → ThreatLevel::Confirmed or ThreatLevel::Suspicious
    5. Emit ScanHit { path, hash, threat_level, threat_name }
```

### 13.2 YARA rule engine

Integrate `yara-rust` crate (bindings to libyara):

```rust
fn compile_rules(rule_paths: &[PathBuf]) -> Result<yara::Rules> {
    let mut compiler = yara::Compiler::new()?;
    for path in rule_paths {
        compiler.add_rules_file(path)?;
    }
    compiler.compile_rules()
}

fn scan_file(rules: &yara::Rules, path: &Path) -> Result<Vec<yara::Match>> {
    let scanner = rules.scanner()?;
    scanner.scan_file(path)
}
```

Ship built-in YARA rules for:

- Common ransomware strings
- Trojan dropper patterns
- Adware registry signatures
- Potentially unwanted program (PUP) markers

### 13.3 Heuristic checks (no signature required)

- Executable in unusual location (e.g., `%TEMP%`, `/tmp`, `~/Downloads`) → flag `Suspicious`
- PE file with mismatched extension (`.jpg` with MZ header) → flag `Suspicious`
- Script files with base64 `eval` chains → flag `Suspicious`
- Recently created file with hidden attribute + no extension (Windows) → flag `Suspicious`

---

## 14. Module 11 — CLI Engine

> `crates/diskclean-cli/src/main.rs`

### 14.1 Command hierarchy (clap v4 derive)

```
diskclean
├── scan       [--path <P>]... [--depth N] [--json] [--all]
├── clean      [--dry-run] [--force] [--categories <C>]... [--min-age <DAYS>]
├── dupes      [--path <P>]... [--algorithm blake3|sha256] [--audio] [--images]
│              [--strategy newest|oldest|interactive] [--dry-run]
├── shred      <PATH>... [--algorithm dod|gutmann|single|nvme] [--verify]
│              [--wipe-free-space <MOUNT>]
├── browser    [--browser chrome|firefox|safari|all]
│              [--clear history|cookies|cache|all] [--days N] [--dry-run]
├── startup    list | enable <NAME> | disable <NAME> | delete <NAME>
├── uninstall  <APP_NAME> [--deep] [--dry-run]
├── registry   scan | clean [--backup] [--dry-run]   (Windows only)
├── malware    scan [--path <P>]... [--yara-rules <FILE>]... [--update-db]
├── treemap    [--path <P>] [--format json|ascii] [--depth N]
├── report     [--format json|csv|html] [--output <FILE>]
├── schedule   add <CRON> <CMD> | list | remove <ID>
└── daemon     start | stop | status
```

### 14.2 Progress display

Use `indicatif` multi-progress bar:

- Top bar: overall operation progress
- Secondary bar: current-directory being scanned
- Stats line: `123,456 files · 45.2 GB scanned · 12.1 GB reclaimable`

Update via `mpsc` channel from the walker (non-blocking: use `try_recv`).

### 14.3 Interactive confirmation

For destructive operations without `--force`:

```
Found 2,341 files to delete (12.4 GB)
  - System cache:     3.2 GB (1,234 files)
  - Browser cache:    5.1 GB (   892 files)
  - Log files:        1.8 GB (   145 files)
  - Crash reports:    2.3 GB (    70 files)

Proceed? [y/N/preview]
```

`preview` opens a pager (`less` on Unix, piped to `more` on Windows) with full file list.

---

## 15. Module 12 — Scheduler & Automation

> `crates/diskclean-scheduler/src/lib.rs`

### 15.1 Cron expression parser

Implement a 5-field + optional seconds cron parser:

```
Field:  second  minute  hour  day  month  weekday
Range:  0-59    0-59    0-23  1-31 1-12   0-6
Extras: * / , -
```

Algorithm:

```rust
fn next_occurrence(cron: &CronExpression, after: DateTime<Utc>) -> DateTime<Utc> {
    // iterate field by field from seconds up
    // find next valid value for each field
    // if field overflows: increment next-higher field, reset all lower
    // handles DST transitions: use UTC internally, convert at display
}
```

### 15.2 OS task registration

**Linux:** Write a systemd `.service` + `.timer` unit pair to `~/.config/systemd/user/`:

```ini
[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
```

Run `systemctl --user daemon-reload && systemctl --user enable diskclean.timer`.

**macOS:** Write a `.plist` to `~/Library/LaunchAgents/`:

```xml
<key>StartCalendarInterval</key>
<dict><key>Hour</key><integer>2</integer>...</dict>
```

Run `launchctl load ~/Library/LaunchAgents/com.qlinetech.diskclean.plist`.

**Windows:** Use `windows-task-scheduler` crate (ITaskService COM) to create a scheduled task with `TASK_TRIGGER_WEEKLY`.

---

## 16. Module 13 — Report & Serialization Layer

> `crates/diskclean-report/src/lib.rs`

### 16.1 Serialization targets

```rust
pub trait ReportEmitter {
    fn emit(&self, report: &CleanReport) -> Result<()>;
}

pub struct JsonEmitter { pub path: PathBuf, pub pretty: bool }
pub struct CsvEmitter  { pub path: PathBuf }
pub struct HtmlEmitter { pub path: PathBuf, pub template: Option<PathBuf> }
```

### 16.2 JSON schema

```json
{
  "timestamp": "2026-04-08T02:00:00Z",
  "version": "0.1.0",
  "duration_ms": 4512,
  "freed_bytes": 12884901888,
  "deleted_count": 2341,
  "errors": [],
  "categories": [
    { "class": "BrowserCache", "count": 892, "bytes": 5476055040 },
    ...
  ],
  "top_files": [
    { "path": "/home/user/.cache/...", "size": 1073741824, "class": "AppCache" },
    ...
  ]
}
```

### 16.3 CSV schema

```
timestamp,path,size_bytes,class,action,result
2026-04-08T02:00:00Z,/home/user/.cache/foo,104857600,AppCache,deleted,ok
...
```

### 16.4 HTML report

Self-contained single HTML file (no external dependencies):

- Inline CSS (dark/light mode via `prefers-color-scheme`)
- Inline SVG donut chart of category breakdown
- Sortable file table (vanilla JS, ~50 lines)
- Summary statistics section

---

## 17. Cross-Cutting Concerns

### 17.1 Observability

Every public function that does I/O must carry a `tracing` span:

```rust
#[instrument(level = "debug", skip(config), fields(root = %config.roots[0].display()))]
pub async fn scan(config: ScanConfig) -> Result<ScanResult> { ... }
```

Log levels:

- `TRACE` — per-file decisions
- `DEBUG` — per-directory, hash computations
- `INFO` — phase transitions (scan start/end, clean start/end)
- `WARN` — recoverable errors (permission denied)
- `ERROR` — unrecoverable errors

### 17.2 Concurrency model

- **CPU-bound work** (hashing, image fingerprinting, classification): `rayon` thread pool.
- **I/O-bound work** (walking, browser SQLite, registry): `tokio` async runtime with `tokio::fs`.
- **Boundary:** Async walker hands `FileEntry` objects into a `tokio::sync::mpsc` channel; a rayon worker pool drains it for hashing.

### 17.3 Safety constraints

- Never follow symlinks outside the configured root unless `cross_device = true`.
- Never delete files from `/`, `/usr`, `/bin`, `/sbin`, `/lib`, `/etc`, `/boot`, `/dev`, `/proc`, `/sys` (or Windows equivalents) regardless of rules.
- Maintain a `PROTECTED_PATHS: &[&str]` constant that is the final veto.
- Dry-run mode (`CleanMode::DryRun`) must be the default. Deletions only happen with explicit `CleanMode::Execute`.

### 17.4 Atomicity

For multi-file clean operations:

1. Build `CleanPlan` (sorted, deduplicated).
2. Write plan to a journal file (`~/.diskclean/journal.json`).
3. Execute each deletion. Mark as `done` in journal after each.
4. On crash/resume: read journal, skip already-done entries.
5. After completion: archive journal with timestamp.

### 17.5 Privilege handling

- Request only what is needed.
- Linux: use `cap_sys_admin` capability detection, not blanket `sudo` checks.
- macOS: request `Full Disk Access` via entitlement (for app bundle) or TCC prompt.
- Windows: check `IsUserAnAdmin()` and request UAC elevation for registry writes.
- If elevated permissions unavailable: degrade gracefully, skip protected paths, log `WARN`.

---

## 18. Testing Strategy — 100% Coverage

### 18.1 Test layout

```
diskclean-core/
└── src/
    ├── classifier.rs
    │   └── #[cfg(test)] mod tests { ... }
    └── treemap.rs
        └── #[cfg(test)] mod tests { ... }

tests/
├── integration/
│   ├── scan_integration.rs
│   ├── dupes_integration.rs
│   ├── browser_integration.rs
│   ├── shred_integration.rs
│   └── registry_integration.rs   (cfg windows)
└── fixtures/
    ├── browser_profiles/
    │   ├── chrome_profile/
    │   └── firefox_profile/
    ├── junk_tree/
    └── duplicate_files/
```

### 18.2 Unit tests — per module

**Classifier:**

```rust
#[test]
fn test_glob_rule_matches_apt_cache() {
    let rule = JunkRule::parse_toml(APT_CACHE_RULE_TOML).unwrap();
    let entry = FileEntry::fixture("/var/cache/apt/archives/foo_1.0_amd64.deb");
    assert_eq!(classify(&entry, &rule_set([rule])), Some(JunkClass::PackageCache));
}

#[test]
fn test_age_predicate_filters_too_new() {
    let entry = FileEntry::fixture_modified_now("/tmp/recent.log");
    let rule = rule_with_min_age(2); // 2 days min
    assert_eq!(classify(&entry, &rule_set([rule])), None);
}

#[test]
fn test_whitelist_overrides_all_rules() {
    let entry = FileEntry::fixture("/tmp/important.log");
    let mut config = ScanConfig::default();
    config.exclusions.push(GlobPattern::new("/tmp/**").unwrap());
    assert_eq!(classify_with_config(&entry, &config), None);
}
```

**Duplicate finder:**

```rust
#[test]
fn test_size_bucketing_skips_unique_sizes() {
    let entries = vec![
        entry_with_size(100), entry_with_size(200), entry_with_size(100),
    ];
    let buckets = size_bucket(&entries);
    assert_eq!(buckets.len(), 1);          // only the size=100 group
    assert_eq!(buckets[&100].len(), 2);
}

#[rstest]
#[case(b"hello", b"hello", true)]
#[case(b"hello", b"world", false)]
fn test_full_hash_equality(#[case] a: &[u8], #[case] b: &[u8], #[case] equal: bool) {
    let ha = blake3_hash(a);
    let hb = blake3_hash(b);
    assert_eq!(ha == hb, equal);
}
```

**Shredder:**

```rust
#[test]
fn test_dod_produces_three_passes() {
    assert_eq!(ShredAlgorithm::DoD5220_22M.passes().len(), 3);
}

#[test]
fn test_file_unreadable_after_shred() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("secret.txt");
    fs::write(&path, b"sensitive data").unwrap();
    shred_file(&path, ShredAlgorithm::SinglePass, false).unwrap();
    assert!(!path.exists());
}
```

**Treemap:**

```rust
#[test]
fn test_squarify_aspect_ratio_below_golden() {
    let items = vec![6, 6, 4, 3, 2, 2, 1].into_iter()
        .map(|s| TreeNode::leaf(s)).collect::<Vec<_>>();
    let rects = squarify(&items, Rect::new(0.0, 0.0, 6.0, 4.0));
    for r in &rects {
        let ar = f64::max(r.w / r.h, r.h / r.w);
        assert!(ar < 2.5, "poor aspect ratio: {ar}");
    }
}

#[test]
fn test_sizes_sum_to_root() {
    let items = vec![1u64, 2, 3, 4, 5];
    let total: f64 = items.iter().sum::<u64>() as f64;
    let rects = squarify_u64(&items, Rect::unit());
    let area_sum: f64 = rects.iter().map(|r| r.w * r.h).sum();
    assert!((area_sum - 1.0).abs() < 1e-9);
}
```

**Startup manager:**

```rust
#[test]
#[cfg(target_os = "linux")]
fn test_desktop_file_parse_hidden_false() {
    let content = "[Desktop Entry]\nName=MyApp\nExec=/usr/bin/myapp\nHidden=false\n";
    let entry = DesktopEntry::parse(content).unwrap();
    assert!(entry.is_enabled());
}

#[test]
#[cfg(target_os = "linux")]
fn test_disable_sets_hidden_true() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("myapp.desktop");
    fs::write(&path, "[Desktop Entry]\nName=MyApp\nExec=/usr/bin/myapp\n").unwrap();
    let mut entry = DesktopEntry::load(&path).unwrap();
    entry.disable().unwrap();
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("Hidden=true"));
}
```

### 18.3 Property-based tests (proptest)

```rust
proptest! {
    #[test]
    fn squarify_total_area_matches_input(sizes in prop::collection::vec(1u64..=1_000_000, 1..=50)) {
        let total: u64 = sizes.iter().sum();
        let nodes: Vec<TreeNode> = sizes.iter().map(|&s| TreeNode::leaf(s)).collect();
        let rects = squarify(&nodes, Rect::new(0.0, 0.0, 1000.0, 800.0));
        let rect_area: f64 = rects.iter().map(|r| r.w * r.h).sum();
        let expected = 1000.0 * 800.0 * (total as f64 / total as f64);
        prop_assert!((rect_area - 800_000.0).abs() < 1.0);
    }

    #[test]
    fn blake3_hash_deterministic(data in prop::collection::vec(any::<u8>(), 0..=65536)) {
        let h1 = blake3_hash_bytes(&data);
        let h2 = blake3_hash_bytes(&data);
        prop_assert_eq!(h1, h2);
    }

    #[test]
    fn duplicate_finder_never_false_positive(
        files in prop::collection::vec(prop::collection::vec(any::<u8>(), 1..=1024), 2..=20)
    ) {
        let dir = tempdir().unwrap();
        let paths: Vec<PathBuf> = files.iter().enumerate().map(|(i, data)| {
            let p = dir.path().join(format!("file_{i}"));
            fs::write(&p, data).unwrap();
            p
        }).collect();
        let dupes = find_duplicates(&paths, HashAlgorithm::Blake3).unwrap();
        // Verify: every reported duplicate pair has identical content
        for group in &dupes {
            let contents: Vec<Vec<u8>> = group.iter()
                .map(|p| fs::read(p).unwrap()).collect();
            prop_assert!(contents.windows(2).all(|w| w[0] == w[1]));
        }
    }
}
```

### 18.4 Integration tests

```rust
// tests/integration/scan_integration.rs
#[tokio::test]
async fn full_scan_on_fixture_tree() {
    let fixture = Path::new("tests/fixtures/junk_tree");
    let config = ScanConfig {
        roots: vec![fixture.to_owned()],
        ..Default::default()
    };
    let result = scan(config).await.unwrap();
    assert!(result.reclaimable > 0);
    assert!(result.errors.is_empty());
    // Verify known junk files are classified
    let apt_cache = result.entries.iter()
        .find(|e| e.path.ends_with("fake.deb")).unwrap();
    assert_eq!(apt_cache.classification, Some(JunkClass::PackageCache));
}

#[tokio::test]
async fn dry_run_deletes_nothing() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("junk.log");
    fs::write(&file, b"test").unwrap();
    let plan = CleanPlan { to_delete: vec![FileEntry::from_path(&file).unwrap()], ..Default::default() };
    execute_plan(&plan, CleanMode::DryRun).await.unwrap();
    assert!(file.exists(), "dry-run must not delete files");
}
```

### 18.5 Coverage tooling

```toml
# .github/workflows/coverage.yml
- name: Install cargo-llvm-cov
  run: cargo install cargo-llvm-cov

- name: Measure coverage
  run: cargo llvm-cov nextest --all-features --lcov --output-path lcov.info

- name: Assert 100% line coverage
  run: cargo llvm-cov report --fail-under-lines 100
```

Exclusions allowed in `llvm-cov` config (justified, not lazy):

- `#[cfg(target_os = "windows")]` blocks on non-Windows CI runners → covered in Windows runner matrix
- `unreachable!()` arms in exhaustive matches
- `panic!()` in test-only code

### 18.6 Mutation testing

```
cargo install cargo-mutants
cargo mutants --workspace --timeout 60
```

Target: **zero surviving mutants** in `diskclean-core`. Surviving mutants in I/O glue code (file writes) are acceptable with documented rationale.

---

## 19. DevOps Pipeline

### 19.1 CI matrix (`ci.yml`)

```yaml
strategy:
  matrix:
    os: [ubuntu-24.04, windows-2025, macos-15]
    rust: [stable, beta]
    features: [default, all-features]
```

### 19.2 CI steps — ordered

```
1.  actions/checkout
2.  dtolnay/rust-toolchain (pinned nightly for coverage, stable for build)
3.  Restore Cargo cache (actions/cache on ~/.cargo + target/)
4.  cargo deny check              ← license audit + known advisories
5.  cargo fmt --check             ← formatting gate
6.  cargo clippy --all-features -- -D warnings   ← lint gate
7.  cargo build --all-features
8.  cargo nextest run --all-features             ← fast parallel tests
9.  cargo llvm-cov nextest --fail-under-lines 100   ← coverage gate
10. cargo mutants (core crate only, time-boxed 10min)
11. cargo doc --no-deps --all-features           ← doc build check
```

### 19.3 Release pipeline (`release.yml`)

Trigger: `push` to `v*` tag.

```
1.  Verify tag matches Cargo.toml version (fail-fast)
2.  Build matrix: linux-x86_64, linux-aarch64, windows-x86_64, macos-x86_64, macos-aarch64
3.  Cross-compile via `cross` for aarch64 targets
4.  Strip binaries (linux/mac): strip --strip-all
5.  UPX compress (linux only, optional)
6.  Sign: GPG detached signature for each artifact
7.  cargo publish --dry-run (verify crates.io manifest)
8.  Create GitHub Release with checksums (SHA256)
9.  cargo publish (each crate in dependency order: core → fs → browser → shred → malware → registry → scheduler → report → cli)
```

### 19.4 Justfile tasks

```makefile
# Run all checks (local equivalent of CI)
ci:
    cargo deny check
    cargo fmt --check
    cargo clippy --all-features -- -D warnings
    cargo nextest run --all-features
    cargo llvm-cov nextest --fail-under-lines 100

# Quick dev loop
dev:
    cargo nextest run --lib

# Generate coverage HTML report
cov:
    cargo llvm-cov nextest --all-features --html
    open target/llvm-cov/html/index.html

# Benchmark duplicate finder
bench:
    cargo bench -p diskclean-fs -- duplicate

# Check for outdated dependencies
outdated:
    cargo outdated --root-deps-only
```

### 19.5 `deny.toml`

```toml
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-DFS-2016"]
deny  = ["GPL-3.0", "AGPL-3.0"]

[advisories]
db-path   = "~/.cargo/advisory-db"
db-urls   = ["https://github.com/rustsec/advisory-db"]
ignore    = []   # never ignore without a linked GitHub issue

[bans]
multiple-versions = "warn"
wildcards         = "deny"
```

---

## 20. Dependency Manifest

| Crate                            | Version | Purpose                                                  |
| -------------------------------- | ------- | -------------------------------------------------------- |
| `rayon`                          | 1.x     | CPU-parallel file hashing & scanning                     |
| `tokio`                          | 1.x     | Async I/O (file walking, SQLite, network for DB updates) |
| `walkdir`                        | 2.x     | Fallback recursive walker                                |
| `blake3`                         | 1.x     | Fast cryptographic hash (duplicate detection, integrity) |
| `sha2`                           | 0.10    | SHA-256 (malware DB compatibility)                       |
| `serde` + `serde_json`           | 1.x     | Serialization of config, reports, journal                |
| `toml`                           | 0.8     | Rule configuration parsing                               |
| `thiserror`                      | 1.x     | Structured error types                                   |
| `anyhow`                         | 1.x     | Error propagation in binary crate                        |
| `tracing` + `tracing-subscriber` | 0.3     | Structured logging / spans                               |
| `clap`                           | 4.x     | CLI argument parsing                                     |
| `indicatif`                      | 0.17    | Progress bars                                            |
| `crossbeam`                      | 0.8     | Lock-free queues (walker work-stealing)                  |
| `dashmap`                        | 6.x     | Concurrent HashMap (inode dedup set)                     |
| `rusqlite`                       | 0.31    | Browser SQLite cleaning                                  |
| `globset`                        | 0.4     | Compiled glob pattern matching                           |
| `regex`                          | 1.x     | Regex rule matching, filename patterns                   |
| `chrono`                         | 0.4     | Timestamps, age calculations, cron scheduling            |
| `image`                          | 0.25    | Image decoding for perceptual hash                       |
| `lofty`                          | 0.21    | Audio metadata (ID3, Vorbis, FLAC)                       |
| `bloom`                          | 0.3     | Bloom filter for malware hash DB                         |
| `yara-rust`                      | 0.28    | YARA rule engine bindings                                |
| `sysinfo`                        | 0.30    | Process list (browser lock detection)                    |
| `plist`                          | 1.x     | macOS plist parsing (startup manager)                    |
| `winreg`                         | 0.52    | Windows registry access                                  |
| `tempfile`                       | 3.x     | Temp dirs in tests                                       |
| `proptest`                       | 1.x     | Property-based testing                                   |
| `rstest`                         | 0.21    | Parametrised unit tests                                  |
| `cargo-llvm-cov`                 | latest  | Coverage measurement                                     |
| `cargo-nextest`                  | latest  | Fast parallel test runner                                |
| `cargo-deny`                     | latest  | License & advisory policy enforcement                    |
| `cargo-mutants`                  | latest  | Mutation testing                                         |

---

## 21. Completion Checklist

### Architecture

- [ ] Workspace Cargo.toml with shared dependency versions
- [ ] All 9 member crates scaffolded with `lib.rs` + `mod.rs` structure
- [ ] `DiskCleanError` hierarchy with `thiserror`
- [ ] All domain types in `diskclean-core::model`
- [ ] `ScanConfig` with builder pattern

### Module implementation

- [ ] Walker: DFS + inode dedup + progress channel
- [ ] Walker: MFT fast-path (Windows)
- [ ] Walker: platform-specific inode extraction
- [ ] Classifier: TOML rule engine loaded at startup
- [ ] Classifier: all 23 `JunkClass` variants covered by rules
- [ ] Classifier: age + size + whitelist predicates
- [ ] Duplicate finder: 3-pass (size → partial hash → full hash)
- [ ] Duplicate finder: perceptual image hash + BK-tree
- [ ] Duplicate finder: audio fingerprint pre-filter
- [ ] Treemap: squarified layout algorithm
- [ ] Treemap: sunburst layout algorithm
- [ ] Treemap: purgeable space (macOS)
- [ ] Registry: BFS walk of all well-known hives
- [ ] Registry: all validity check categories
- [ ] Registry: backup before clean
- [ ] Registry: protection list (cannot delete)
- [ ] Startup: all sources on all 3 platforms
- [ ] Startup: enable/disable/delete per source type
- [ ] Uninstaller: receipt graph for Linux, macOS, Windows
- [ ] Uninstaller: leftover detection algorithm
- [ ] Browser: profile resolver for 5 browsers × 3 platforms
- [ ] Browser: Chromium SQLite cleaning (history, cookies, cache)
- [ ] Browser: Firefox places.sqlite + cookies.sqlite + Cache2
- [ ] Browser: process lock detection
- [ ] Shredder: all 4 algorithms + custom
- [ ] Shredder: filename obfuscation
- [ ] Shredder: free-space wipe
- [ ] Shredder: NVMe secure erase (Linux)
- [ ] Shredder: storage type detection (SSD vs HDD)
- [ ] Malware: Bloom filter + exact SHA256 DB
- [ ] Malware: YARA rule engine
- [ ] Malware: heuristic checks (5 patterns)
- [ ] CLI: all 12 subcommands with full flags
- [ ] CLI: progress bars via indicatif
- [ ] CLI: interactive confirmation with preview
- [ ] Scheduler: cron parser (5+1 fields)
- [ ] Scheduler: OS task registration (systemd, launchd, Task Scheduler)
- [ ] Report: JSON emitter
- [ ] Report: CSV emitter
- [ ] Report: HTML self-contained emitter

### Cross-cutting

- [ ] `tracing` spans on all public I/O functions
- [ ] `PROTECTED_PATHS` constant enforced as final veto
- [ ] Dry-run mode is the default everywhere
- [ ] Atomicity journal for multi-file operations
- [ ] Privilege detection with graceful degradation
- [ ] `CleanerML`-compatible rule import (BleachBit compatibility layer)

### Testing

- [ ] Unit tests for every public function (≥1 happy path + ≥1 error path)
- [ ] Property-based tests: squarify, hash determinism, duplicate correctness
- [ ] Integration test: full scan on fixture tree
- [ ] Integration test: dry-run deletes nothing
- [ ] Integration test: shred leaves no file
- [ ] Integration test: browser profile parse + clean
- [ ] `cargo llvm-cov --fail-under-lines 100` passes on CI
- [ ] Zero surviving mutants in `diskclean-core`

### DevOps

- [ ] CI matrix: 3 OS × 2 Rust channels × 2 feature sets
- [ ] `cargo deny check` passes (no banned licenses, no advisories)
- [ ] `cargo fmt --check` gate
- [ ] `cargo clippy -D warnings` gate
- [ ] Release pipeline with cross-compilation + signing
- [ ] `justfile` with `ci`, `dev`, `cov`, `bench`, `outdated` recipes
- [ ] `deny.toml` with explicit allow/deny lists
- [ ] Documentation: every public item has `///` doc comment
- [ ] CHANGELOG.md with Keep-a-Changelog format

---

_End of blueprint. Total: 13 feature modules, ~2,800 lines of algorithmic spec,
100% coverage enforcement from day one._


## 22. Additional Qleaner Application & Infrastructure Tasks
> Tasks carried over from the Qleaner application roadmap (TODO.md) that extend this blueprint.

- [ ] **Cross-Platform Privilege Manager:** Use `sudo` crate or native auth APIs (e.g., macOS `Authorization Services`) to securely elevate privileges for system-level junk cleaning.
- [ ] **Docker Builder Pruning:** Hook into the Docker CLI/socket to report and clean dangling images, volumes, and builder caches.
- [ ] **Windows Update Cache:** Add Windows `SoftwareDistribution/Download` cache cleaning (requires elevated privileges).
- [ ] **Dry-Run Architecture:** Implement true dry-run scanning in Rust. Currently, [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301) just executes. Provide a simulation API to guarantee file counts.
- [ ] **Hover Context Menus:** Use `bits-ui` Dropdown to add right-click options to rows: "Open Folder Location", "Add to Ignore List", "View Properties".
- [ ] **Sortable Columns:** Add clickable headers (Target, Category, Size) to sort the `cleanerStore.results` dynamically based on `$derived` state.
- [ ] **Strict Content Security Policy (CSP):** The [+page.svelte](file:///home/drvoid/ISU/Qleaner/src/routes/+page.svelte) lacks CSP. Enforce rigid metas limiting `script-src` and `connect-src` specifically to Tauri IPC limits.
- [ ] **Scoped File System Access:** Ensure the Tauri API is strictly sandboxed. Never allow absolute path overrides from the frontend to [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301) – pass internal [id](file:///home/drvoid/Qix/Q-Static/src/lib/state/ui.svelte.ts#210-218) references instead.
- [ ] **Panic Recovery (Backend):** The `unwrap()` calls in `clean_items` (e.g., `state.scan_results.lock().unwrap()`) will crash the entire app if a thread poisons the lock. Handle mutex blocking safely.
- [ ] **Network Sandbox:** Qleaner currently requires no internet. Ensure `tauri.conf.json` fully disables all network/HTTP protocols.
- [ ] **Error Toast Notifications:** Instead of silently failing to delete an item, pipe exact Rust error messages up to a frontend `sonner` toast system.
- [ ] **Sudo Policy Enforcer:** If macOS requires Full Disk Access, implement a watcher that detects lacking permissions and redirects the user to `System Settings -> Privacy`.
- [ ] **Sandboxed IFrame Documentation:** Move help texts and privacy policies to a segregated iframe without standard script contexts.
- [ ] **Cargo Clippy Pedantic:** Enforce `#![warn(clippy::pedantic)]` and `#![warn(clippy::unwrap_used)]` on the Rust codebase and fix the ~10 violations present.
- [ ] **Unit Tests (Rust):** Create `#[cfg(test)]` modules for `get_directory_size` and `human_readable_size` (testing bounds and edge cases).
- [ ] **File System Mocking:** Implement `tempfile::TempDir` to construct fake junk directories and test `clean_items` locally without nuking real system caches.
- [ ] **E2E Tests (Playwright):** Integrate Playwright for Tauri E2E testing to simulate UI clicks automatically spawning mocked Tauri commands.
- [ ] **Component Tests (Vitest):** Add `vitest` for the layout logic, specifically testing the reactivity of `$derived(totalSelectedSize)`.
- [ ] **GitHub Actions CI/CD:** Add `.github/workflows/build.yml` compiling binaries for `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, and Linux AppImage.
- [ ] **AppImage Builder (Linux):** Optimize Tauri config for modern Linux distributions with exact package exclusions.
- [ ] **Feature Flags Architecture:** Implement Cargo features (e.g., `#[cfg(feature = "dangerous-clean")]`) to gate highly destructive beta features.
- [ ] **Crash Reporting:** Integrate Sentry natively via `sentry-rust` and `sentry-javascript` to catch unhandled application panics remotely.
- [ ] **Update Auto-Updater:** Enable Tauri's built-in updater system (`plugin-updater`) so users get the latest optimization engines directly.
- [ ] **Architectural Readme:** Completely rewrite `README.md` introducing the dual Svelte/Rust architecture, contribution guidelines, PR templates, and local dev spin-up instructions.
- [ ] ** [BACKLOG] Browser Playwright Execution:** Establish an automated flow to run the browser dynamically and close it alongside strict required logging messages and comments.
- [ ] ** [BACKLOG] Universal Target Builds:** Scale the Linux actions explicitly for Ubuntu/Debian `.deb`, Fedora `.rpm`, Arch `.pacman` distributions alongside Apple notarization and Windows code signing.
- [ ] ** [BACKLOG] Production Telemetry:** Evaluate privacy-respecting mechanisms for telemetry logging and crash dumps to monitor application stability natively.
- [ ] **CLI Architecture Support:** Develop a completely headless **Command-Line Interface (CLI)** version of Qleaner strictly for scriptable DevOps deployment, Linux servers, and TUI power users (inspired by `ncdu` / `TreeSize`).
- [ ] **Malware & Security Analysis (Malware):** Add baseline YARA scanning or static malware signature detection against cache payloads looking for common miners or rogue binaries (inspired by `Advanced SystemCare`).
- [ ] ** [INCOMPLETE: PARTIAL] Cancellation Token:** Implement a `tokio_util::sync::CancellationToken` to allow the user to abort a long-running scan or clean operation mid-way. - *Flag cancellation triggers exist, but underlying directory walking does not abort early (partial).*
- [ ] **Process Tracking Backend:** Add a backend scanner to detect running background apps (e.g., Chrome) and prevent emptying their caches while active.
- [ ] **File Ownership Checks:** On Linux/macOS, check if `uid == current_uid` before trying to delete, skipping root-owned caches gracefully instead of throwing exceptions.
- [ ] **Node.js Modules Sweeper:** Add dedicated scans for orphaned `node_modules` folders using [ignore](file:///home/drvoid/ISU/Qleaner/.gitignore) glob targeting within user space.
- [ ] **NPM/Yarn/PNPM Cache:** Explicitly target `~/.npm`, `~/AppData/Local/npm-cache`, and `~/.local/share/pnpm/store`.
- [ ] **Rust Cargo Cache:** Add scanning for `~/.cargo/registry` and `~/.cargo/git`.
- [ ] **Rust Target Sweeper:** Detect redundant `target/debug` directories in inactive Rust projects using heuristic age-based scanning.
- [ ] **macOS Xcode DerivedData:** Add `~/Library/Developer/Xcode/DerivedData` for massive storage recovery on macOS.
- [ ] **macOS iOS Simulators:** Scan and clear outdated iOS simulator caches (`~/Library/Developer/CoreSimulator/Devices`).
- [ ] **Windows Prefetch:** Add `C:\Windows\Prefetch` analysis.
- [ ] **Linux Journalctl Size:** Scan via `journalctl --disk-usage` and offer to vacuum logs older than X days.
- [ ] **Linux Flatpak/Snap Leftovers:** Target `.var/app/` caches and `snap` leftover blobs.
- [ ] **Browser Forensic Cache:** Add distinct targets for Chrome, Firefox, Safari, Edge caches, separate from generic "User Caches".
- [ ] **Discord / Slack Cache:** Target Electron app cache folders (e.g., `~/Library/Application Support/Discord/Cache`).
- [ ] **Empty Directory Sweeper:** Add an optional pass that identifies deeply nested, entirely empty directory trees and prunes them.
- [ ] **Time-based Filtering:** Allow the user to specify "Only clean files older than X days" (e.g., keep caches from the last 24 hours).
- [ ] **Configurable Ignoring:** Add a global `.qleanerignore` list to explicitly block directories from ever being scanned or listed.
- [ ] **Dependency Uninstaller:** Identify orphaned applications (macOS `.app` leftover plists, Windows rogue regkeys).
- [ ] **Duplicated Files (Dedup):** Implement a fast checksum-based (xxHash) duplicate file finder.
- [ ] **Path Truncation Logic:** Very long paths currently use CSS truncate, which obscures the end of the path (the important part). Use JS/CSS to truncate the middle (e.g., `~/Library.../Cache`).
- [ ] **Native OS Window Controls:** Add custom `titlebar` integration (via `<div data-tauri-drag-region>`) using Lucide icons for macOS/Windows custom decorators.
- [ ] **Tauri Capabilities Locking:** Lock down `tauri.conf.json` explicitly allowing *only* the specific [clean_items](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#281-301), [start_scan](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs#217-275) commands. Deny wildcards.
- [ ] **Browser State Wiping Alert:** Auto-cleaning browser caches logs users out of sites. Require an explicit warning/consent checkbox before wiping "Chrome/Firefox Data".
- [ ] **Sensitive File Hashing:** Verify critical file hashes (like OS `hosts` or config binaries) before assuming something is a "Cache" based on name alone.
- [ ] **Anti-Virus Whitelisting Notice:** Some aggressive deep clean iterations scan locked folders triggering Windows Defender. Detect and handle `ACCESS_DENIED` transparently.
- [ ] **Shredding (Secure Erase):** Implement optional secure wiping (multi-pass overwrite) rather than just `unlink` for sensitive metadata.
- [ ] **Memory Scrubbing (Rust):** Implement `zeroize` strictly on any internal variables tracking decrypted user paths or credentials during scanning.
- [ ] **Database Locking Avoidance:** If Chrome is running, its `Cache.db` is locked. Detecting the `.lock` file and skipping the database prevents DB corruption.
- [ ] **Memory Leak Profiling:** Ensure `Mutex` states in Tauri aren't holding vast arrays of `WalkDir` `DirEntry` objects in memory indefinitely after a scan completes.
- [ ] **Code Signing:** Add notarization pipelines for macOS integration so it doesn't get blocked by Gatekeeper.
- [ ] **WiX Configuration (Windows):** Add an installer license, custom EULA, and start menu shortcut entries via refined `tauri.conf.json` WiX fragments.
- [ ] **Checksum Duplicate Finder (Dupes):** Integrate an `xxHash` high-speed engine to locate duplicated photos, archives, and binaries cross-system, yielding safe deletion arrays (inspired by `dupeGuru` / `Gemini 2`).
- [ ] **Startup Manager (Start):** Hook into OS background agents (LaunchDaemons on Mac, registry `Run` keys on Win, Systemd on Linux) to toggle bloatware auto-starting (inspired by `Stacer` / `CCleaner`).
- [ ] **System Native App Uninstaller (Uninst):** Intercept standard `.app` or registry uninstalls scanning for deep orphan plist caches globally to secure total application removals (inspired by `App Cleaner & Uninstaller`).
- [ ] **Privacy & Browser Exploitation Sweeper (Privacy):** Go beyond standard `.cache` by aggressively locating tracking cookies, form histories, DOM storages, and telemetry residues across Edge/Chrome/Firefox (inspired by `BleachBit`).
- [ ] **File Shredder (Shred):** Implement DoD 5220.22-M compliant multi-pass secure erasures to guarantee sensitive data files cannot be recovered via standard forensics (inspired by `CleanMyMac`).
- [ ] **OS Registry Repair (Reg):** Introduce distinct Windows registry scanning looking for invalid paths, ghost uninstaller references, and rogue COM keys (inspired by `Wise / Glary Utilities`).
- [ ] **SQLite Config Persist:** Migrate off raw JSON/frontend local storage to a robust SQLite (`sqlx` + `sqlite`) backend for safely storing persistent schedules and excluded paths.
- [ ] **Tauri Plugin System:** Move the core cleaner logic out of [main.rs](file:///home/drvoid/ISU/Qleaner/src-tauri/src/main.rs)/[cleaner.rs](file:///home/drvoid/ISU/Qleaner/src-tauri/src/cleaner.rs) into a structured Tauri Plugin (`tauri-plugin-qleaner-core`) for strict modularity.
- [ ] **Extension Profiling:** Add analysis for massive log files (`.log`, `.trace`) sitting abandoned in desktop directories.
- [ ] **Virtual Listicles:** The table renders every row. If thousands of junk locations are found, the DOM will lag. Implement `svelte-virtual-list`.
- [ ] **Sticky Table Headers:** Ensure the `thead` uses `z-index` and `backdrop-blur` properly behind overflowing content inside a constrained wrapper.
- [ ] **Settings Sidebar:** Add a collapsible tool sidebar with navigation for "Dashboard", "Rules", "Schedules", and "Settings".
- [ ] **Keyboard Shortcuts:** Implement `svelte-window` keyboard listeners (e.g., `CMD+Enter` to start cleaning, `Esc` to cancel).
- [ ] **Accessibility (A11y):** Form checkboxes lack `aria-label` or `<label>` wrapping. Add strict strict accessibility tags to the data grid.
- [ ] **Path Traversal Protection:** Sanitize whatever IDs or Paths are sent from Svelte to Rust. Prevent `../../` attacks if the IPC gets intercepted.
- [ ] **Retry Logic (File Deletion):** Files might be temporarily locked. Add a backoff retry logic (e.g., `3 attempts, 100ms apart`) inside `std::fs::remove_dir_all`.
- [ ] **Detailed Logging (Tracing):** Add `tracing-subscriber` to rotate app activity logs to `~/.config/qleaner/app.log` for debugging and telemetry.
- [ ] **Audit History Dashboard:** Record every deletion in an append-only JSON/SQLite "History" tab so users can see exactly what was removed and when.
- [ ] **Session Cleanup Tracking:** Add a "Last Ran" timestamp enforced via cryptographic signing to prevent tampering with "System Health" scores.
- [ ] **ESLint & Prettier Strictness:** The Svelte app lacks the `@typescript-eslint/recommended-requiring-type-checking` rule set.
- [ ] **Release Bump Pipeline:** Create a script (via `release-plz` or `standard-version`) to automate bumping version parity between `package.json` and `Cargo.toml`.
- [ ] **Vite Bundle Optimization:** Optimize `vite.config.js` to split `lucide-svelte` and `bits-ui` chunks to shrink the V8 snapshot load time.
- [ ] **Benchmarking Suite:** Add `criterion` to benchmark the regex speeds and disk traversal speeds against massive mocked file trees.
- [ ] **Rust Format Checks:** Automate `cargo fmt --check` in the pre-commit hook via husky.
- [ ] **Git LFS for Icons:** Store the `app-icon-real.png` (250KB) and other high-res assets in Git LFS instead of tracking raw blobs.
- [ ] ** [BACKLOG] `fix-qicro-debugger` Repository Sync:** Review and merge any necessary commits strictly from the `fix-qicro-debugger` branch into master, culling deprecated segments.
- [ ] **Disk Usage Visualization (Viz):** Implement high-performance Rust-backed Treemaps or Sunburst charts to visually map largest folders directly inside the Svelte UI (inspired by `DaisyDisk` / `WinDirStat`).
- [ ] **System Hardware Monitoring:** Track real-time network up/down velocities, thermal temperatures, and exact CPU consumption per localized binary process (inspired by `Sensei`).
