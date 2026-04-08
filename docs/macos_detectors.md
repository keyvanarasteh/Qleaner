# macOS Leftovers Detection

The `detectors.rs` file in Qleaner brings in heavy macOS forensic algorithms originating from the Q-MCP project.
It primarily cross-references `~/Library/Containers`, `~/Library/Application Support`, and `~/Library/LaunchAgents` against standard active `NSBundleIdentifiers` natively installed on the host OS.
If a sandboxed plist or generic LaunchAgent remains for a bundle ID that doesn't resolve to `/Applications`, it is flagged as an orphan and prompted for deletion.
