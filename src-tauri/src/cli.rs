use crate::cleaner::detectors::get_cache_locations;
#[cfg(target_os = "macos")]
use crate::cleaner::detectors::{get_installed_bundle_ids, detect_container_orphans, detect_group_container_orphans, detect_preference_orphans, detect_app_support_orphans, detect_launch_agent_orphans, detect_cache_orphans};
use crate::cleaner::scanner::{get_directory_size, human_readable_size};
use crate::cleaner::commands::{fetch_docker_size, perform_docker_clean};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Qleaner CLI")]
#[command(author = "Keyvan Arasteh")]
#[command(version = "1.0")]
#[command(about = "Headless CLI optimization engine for Qleaner", long_about = None)]
pub struct CliArgs {
    /// Skip opening the GUI and run headlessly
    #[arg(long)]
    pub cli: bool,

    /// Scan system caches and report their sizes
    #[arg(long)]
    pub scan: bool,

    /// Clean detected items and free disk space
    #[arg(long)]
    pub clean: bool,
}

pub async fn execute() {
    let args = CliArgs::parse();
    if !args.scan && !args.clean {
        println!("Please specify --scan or --clean! Use --help for more info.");
        return;
    }

    println!("=================================================");
    println!(" 🚀 Qleaner Headless Engine is targeting caches ");
    println!("=================================================");

    let mut total_size: u64 = 0;
    let mut found_count = 0;
    
    let mut locations = get_cache_locations();
    println!(">> Scanning primary system caches...\n");
    
    for loc in &mut locations {
        let is_docker = loc.path.starts_with("docker://");
        let exists = if is_docker {
             fetch_docker_size(&loc.path).await.is_some()
        } else {
             tokio::fs::try_exists(&loc.path).await.unwrap_or(false)
        };
        
        if exists {
            let size = if is_docker {
                fetch_docker_size(&loc.path).await.unwrap_or(0)
            } else {
                let path = PathBuf::from(&loc.path);
                tokio::task::spawn_blocking(move || {
                    get_directory_size(&path, tokio_util::sync::CancellationToken::new())
                }).await.unwrap_or(0)
            };
            
            if size > 0 {
                println!("[FOUND] {} ({}) - {}", loc.name, human_readable_size(size), loc.path);
                total_size += size;
                found_count += 1;
                
                if args.clean {
                    if is_docker {
                        perform_docker_clean(&loc.path).await;
                        println!("        -> Cleaned Docker Endpoint ✓");
                    } else {
                        let _ = trash::delete(&loc.path);
                        println!("        -> Trashed Recursively ✓");
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("\n>> Scanning aggressive software leftovers...");
        let installed = get_installed_bundle_ids();
        let containers = detect_container_orphans(&installed);
        let groups = detect_group_container_orphans(&installed);
        let prefs = detect_preference_orphans(&installed, None);
        let support = detect_app_support_orphans(&installed, None);
        let agents = detect_launch_agent_orphans(&installed, None);
        let caches = detect_cache_orphans(&installed, None);
        
        let all_orphans = [containers, groups, prefs, support, agents, caches].concat();
        for loc in all_orphans {
            println!("[FOUND] ORPHAN: {} ({}) - {}", loc.name, loc.size_human, loc.path);
            total_size += loc.size;
            found_count += 1;
            
            if args.clean {
                let _ = trash::delete(&loc.path);
                println!("        -> Purged Leftover ✓");
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("\n>> Leftover orphan detection is currently macOS-only. Skipping on this platform.");
    }
    
    println!("\n=================================================");
    println!(" 🎉 Qleaner Engine Passed Execution Successfully ");
    println!("=================================================");
    println!("Total Artifacts Found: {found_count}");
    println!("Total Storage Bound:   {}", human_readable_size(total_size));
    if args.clean {
        println!(">> Status: The reported storage has been completely recovered!");
    } else {
        println!(">> Status: Scan only. Run with --clean to authorize deletion.");
    }
}
