use std::path::Path;
use ignore::WalkBuilder;

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

pub fn get_directory_size(path: &Path) -> u64 {
    let total_size = std::sync::atomic::AtomicU64::new(0);
    WalkBuilder::new(path)
        .standard_filters(false)
        .follow_links(false) // Task 16: Symlink Safeties
        .threads(std::thread::available_parallelism().map(std::num::NonZero::get).unwrap_or(4))
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

        let size = get_directory_size(dir.path());
        assert_eq!(size, 16); // 5 + 6 + 5 = 16 bytes

        // Empty directory
        let empty_dir = tempdir().unwrap();
        assert_eq!(get_directory_size(empty_dir.path()), 0);
    }
}
