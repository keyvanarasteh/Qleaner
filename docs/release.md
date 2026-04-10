# Qleaner Release Pipeline Documentation

The Qleaner repository utilizes a fully automated, cross-platform CI/CD pipeline powered by GitHub Actions and Tauri. 
This automated workflow compiles binaries for Linux, Windows, and macOS, auto-generates changelogs, and publishes the GitHub Release natively.

## 🚀 How It Works

The canonical release workflow is defined in `.github/workflows/release.yml` (The older `build.yml` was deprecated). When triggered, it executes on a matrix of 4 distinct jobs:
1. `ubuntu-22.04` (Linux builds: `.AppImage`, `.deb`, `.rpm`)
2. `windows-latest` (Windows builds: `.exe`, `.msi`)
3. `macos-latest` with `--target aarch64-apple-darwin` (macOS Apple Silicon: `.dmg`)
4. `macos-latest` with `--target x86_64-apple-darwin` (macOS Intel: `.dmg`)

For each job, the workflow:
- Installs necessary OS-level dependencies (e.g., `libwebkit2gtk-4.1-dev` on Linux).
- Sets up `Bun` for frontend dependencies.
- Sets up Rust via `dtolnay/rust-toolchain`.
- Generates a Markdown changelog pulling the `git log` since the previous release tag.
- Executes `tauri-action` to build the platform assets and merge them automatically into a single, cohesive GitHub Release.

## 🔨 How to Trigger a New Release

To trigger a new production release, you simply bump the version numbers and push a standard `v*` tag.

### Step-by-Step Command Guide
1. **Bump the Application Versions:**
   Ensure versions match in both your `Cargo.toml` and `tauri.conf.json`.
   ```bash
   # Update tauri configuration
   sed -i 's/"version": "X.Y.Z"/"version": "X.Y.NEXT"/' src-tauri/tauri.conf.json
   
   # Update cargo configuration
   sed -i 's/^version = "X.Y.Z"/version = "X.Y.NEXT"/' src-tauri/Cargo.toml
   ```

2. **Commit the Bump:**
   ```bash
   git add src-tauri/tauri.conf.json src-tauri/Cargo.toml
   git commit -m "chore: bump version to X.Y.NEXT"
   git push origin main
   ```

3. **Create and Push the Tag:**
   The workflow is configured to listen specifically for tags starting with `v` (e.g., `v1.0.1`).
   > *Note:* We use `v__VERSION__` standardization. Do NOT use `app-v*` prefixes.
   ```bash
   git tag vX.Y.NEXT
   git push origin vX.Y.NEXT
   ```
   *Alternative:* You can also trigger the release manually via **workflow_dispatch** in the GitHub "Actions" tab by targeting the `main` branch.

## 👀 Checking the Results

As soon as the tag is pushed to remote, GitHub Actions will spin up the `Release` workflow.

1. Navigate to the [GitHub Actions tab](https://github.com/keyvanarasteh/Qleaner/actions).
2. Click on the running `Release` workflow to monitor the live terminal output for all 4 matrix jobs.
3. Building Tauri applications cross-platform typically takes between 10 to 15 minutes.
4. Once completed, navigate to the [Releases page](https://github.com/keyvanarasteh/Qleaner/releases). You will see the new officially published release (non-draft) containing:
    - Auto-generated Git changelogs.
    - Table explaining the platforms.
    - 9 binary assets ready for download.

### ⚠️ Common Troubleshooting

- **Missing Apple Code Signing:** Currently, the `APPLE_*` environment variables in `release.yml` are commented out to prevent failing builds due to missing GitHub Action Secrets. Thus, macOS binaries are compiled **unsigned**. Users will have to `Right-click -> Open` the `.dmg` to bypass the initial macOS Gatekeeper check. If you add Apple Dev secrets to GitHub, simply uncomment these lines in `release.yml`.
- **Dependencies Missing:** If the pipeline fails during the `cargo build` phase with "missing dependencies," ensure `src-tauri/Cargo.toml` possesses every dependency natively used by your logic. CI runs clean, deterministic builds without pre-cached local registries. (e.g. Ensure you actually included `tokio-util`, `sqlx`, etc).
