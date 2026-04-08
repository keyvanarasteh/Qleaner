use std::path::Path;
use ignore::WalkBuilder;

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

pub fn get_directory_size(path: &Path) -> u64 {
    let total_size = std::sync::atomic::AtomicU64::new(0);
    WalkBuilder::new(path)
        .standard_filters(false)
        .follow_links(false) // Task 16: Symlink Safeties
        .threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4))
        .build_parallel()
        .run(|| {
            Box::new(|result| {
                if let Ok(entry) = result {
                    if let Ok(metadata) = std::fs::symlink_metadata(entry.path()) {
                        if metadata.is_file() {
                            total_size.fetch_add(metadata.len(), std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });
    total_size.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_human_readable_size() {
        assert_eq!(human_readable_size(0), "0 B");
        assert_eq!(human_readable_size(500), "500 B");
        assert_eq!(human_readable_size(1024), "1.0 KB");
        assert_eq!(human_readable_size(1536), "1.5 KB");
        assert_eq!(human_readable_size(1048576), "1.0 MB");
        assert_eq!(human_readable_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_get_directory_size() {
        let dir = tempdir().unwrap();
        let path1 = dir.path().join("file1.txt");
        let path2 = dir.path().join("file2.txt");

        fs::write(&path1, "Hello").unwrap(); // 5 bytes
        fs::write(&path2, "World!").unwrap(); // 6 bytes

        let size = get_directory_size(dir.path());
        assert_eq!(size, 11);
    }
}
