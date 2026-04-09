use std::path::Path;
use ignore::WalkBuilder;
use tokio_util::sync::CancellationToken;

#[allow(clippy::cast_precision_loss)]
pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn get_directory_size(path: &Path, token: CancellationToken) -> u64 {
    let total_size = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    WalkBuilder::new(path)
        .standard_filters(false)
        .follow_links(false) // Task 16: Symlink Safeties
        .threads(2)
        .build_parallel()
        .run(|| {
            let thread_token = token.clone();
            let ts_clone = total_size.clone();
            Box::new(move |result| {
                if thread_token.is_cancelled() {
                    return ignore::WalkState::Quit;
                }
                if let Ok(entry) = result {
                    if let Ok(metadata) = std::fs::symlink_metadata(entry.path()) {
                        if metadata.is_file() {
                            ts_clone.fetch_add(metadata.len(), std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });
    total_size.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn find_deep_evictions(target: &str, token: CancellationToken) -> (u64, Vec<std::path::PathBuf>) {
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
    
    let total_size = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let found_paths = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    
    let is_node = target == "node_modules";
    let is_rust = target == "rust_target";
    
    let mut builder = WalkBuilder::new(home);
    builder.standard_filters(false);
    builder.follow_links(false);
    builder.threads(2); // Reduced to prevent nested thread pool explosion
    builder.filter_entry(|e| {
        if e.file_type().map_or(false, |ft| ft.is_dir()) {
            if let Some(name) = e.file_name().to_str() {
                if name.starts_with('.') && name != ".npm" { return false; }
                if name == "Library" || name == "AppData" || name == "Windows" || name == "System" || name == "Applications" || name == "Applications (Parallels)" { return false; }
            }
        }
        true
    });

    builder.build_parallel().run(|| {
        let thread_token = token.clone();
        let ts_clone = total_size.clone();
        let paths_clone = found_paths.clone();
        
        Box::new(move |result| {
            if thread_token.is_cancelled() {
                return ignore::WalkState::Quit;
            }
            if let Ok(entry) = result {
                if let Ok(metadata) = std::fs::symlink_metadata(entry.path()) {
                    if metadata.is_dir() {
                        let name = entry.file_name().to_string_lossy();
                        let mut do_skip_and_compute = false;
                        
                        if is_node && name == "node_modules" {
                            do_skip_and_compute = true;
                        } else if is_rust && name == "target" {
                            let toml = entry.path().parent().unwrap_or_else(|| entry.path()).join("Cargo.toml");
                            if toml.exists() {
                                do_skip_and_compute = true;
                            }
                        }
                        
                        if do_skip_and_compute {
                            let path = entry.path().to_path_buf();
                            if let Ok(mut lock) = paths_clone.lock() {
                                lock.push(path.clone());
                            }
                            let size = get_directory_size(&path, thread_token.clone());
                            ts_clone.fetch_add(size, std::sync::atomic::Ordering::Relaxed);
                            return ignore::WalkState::Skip;
                        }
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });
    
    let final_paths = found_paths.lock().unwrap_or_else(|e| e.into_inner()).clone();
    (total_size.load(std::sync::atomic::Ordering::Relaxed), final_paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_human_readable_size() {
        assert_eq!(human_readable_size(0), "0 B");
        assert_eq!(human_readable_size(1023), "1023 B");
        assert_eq!(human_readable_size(1024), "1.0 KB");
        assert_eq!(human_readable_size(1536), "1.5 KB");
        assert_eq!(human_readable_size(1_048_576), "1.0 MB");
        assert_eq!(human_readable_size(1_073_741_824), "1.0 GB");
        assert_eq!(human_readable_size(1_099_511_627_776), "1.0 TB");
        assert_eq!(human_readable_size(1_125_899_906_842_624), "1.0 PB");

        // Edge case: Extremely large numbers don't exceed the units boundary
        assert_eq!(human_readable_size(std::u64::MAX), "16384.0 PB");
    }

    #[test]
    fn test_get_directory_size() {
        let dir = tempdir().unwrap();
        let path1 = dir.path().join("file1.txt");
        let path2 = dir.path().join("file2.txt");

        fs::write(&path1, "Hello").unwrap(); // 5 bytes
        fs::write(&path2, "World!").unwrap(); // 6 bytes

        // Nested directory
        let nested_dir = dir.path().join("nested");
        fs::create_dir(&nested_dir).unwrap();
        let path3 = nested_dir.join("file3.txt");
        fs::write(&path3, "12345").unwrap(); // 5 bytes

        let token = CancellationToken::new();
        let size = get_directory_size(dir.path(), token.clone());
        assert_eq!(size, 16); // 5 + 6 + 5 = 16 bytes

        // Empty directory
        let empty_dir = tempdir().unwrap();
        assert_eq!(get_directory_size(empty_dir.path(), token), 0);
    }
}
