# Windows OS Build Checklist for Qleaner

This document outlines the workflow for compiling and distributing the native Windows (.msi / .nsis) release of Qleaner (Tauri v2).

## 1. System Requirements & Prerequisites
- [ ] Install **Visual Studio 2022 C++ Build Tools**: Make sure to check the "Desktop development with C++" workload (required for compiling the Rust backend).
- [ ] Install Rust for Windows targeting `x86_64-pc-windows-msvc`: Download from `https://rustup.rs/`.
- [ ] Install Bun (Package Manager) via PowerShell: `powershell -c "irm bun.sh/install.ps1 | iex"`
- [ ] Ensure **WebView2 Runtime** is installed (usually pre-installed on Windows 10/11 natively).

## 2. Project Initialization
- [ ] Clone repository: `git clone <repo_url> && cd Qleaner`
- [ ] Install JS dependencies using Bun: `bun install`
- [ ] Verify hot-reloading pipeline on Windows: `bun run dev` (Ensure firewall permits the Vite dev server port).

## 3. Tauri Bundle Tools Configuration
- [ ] Download and install the **WiX Toolset v3** (if generating `.msi` installers). Add the binary dir (e.g. `C:\Program Files (x86)\WiX Toolset v3.11\bin`) to your Windows `%PATH%`.
- [ ] If using the **NSIS** bundler framework (Often easier and compresses better for Tauri v2):
    Ensure the `tauri.conf.json` specifically arrays `"nsis"` and `"msi"` under the `bundle.targets` property.

## 4. Signing (Authenticode EV)
*Note: Without Code Signing, Windows SmartScreen will fiercely block the app and claim it is an "Unrecognized app from an unknown publisher".*

- [ ] Acquire an EV or Standard Code Signing Certificate (.pfx / .p12).
- [ ] Configure `signtool` executable path.
- [ ] Set environment variables before the build phase or utilize Github Actions / Azure Secrets to sign the release binaries using the certificate password.

## 5. Compiling & Production Builds
- [ ] Run the compilation command:
  `bun run tauri build`
  
## 6. Artifact Verification
- [ ] The generated installers will exist under: 
  - `src-tauri/target/release/bundle/msi/`
  - `src-tauri/target/release/bundle/nsis/`
- [ ] Verify the installer correctly places `Qleaner.exe` in `C:\Program Files\Qleaner` and successfully registers Windows Start Menu shortcuts and Registry keys.

## 7. Known Windows Specific Edge Cases
- Cleaning operations utilizing path traversals on Windows must securely handle `\\?\C:\` Long Path limitations. Rust handles this naturally, but Windows path structures are different from Unix.
- Certain Windows background locked files (e.g., Prefetch, active swap) cannot be deleted. The Rust backend must catch `std::io::ErrorKind::PermissionDenied` and log/skip locked files gracefully without throwing full application exceptions.
