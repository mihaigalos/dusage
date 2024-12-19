use clap::ArgMatches;
use nix::sys::statvfs::statvfs;
use std::cmp;
use std::process;
use std::process::Command;

use crate::procfields::ProcFields;
use crate::stats::Stats;

pub struct Reader;

impl Reader {
    pub fn read(args: &ArgMatches) -> (Vec<Stats>, usize) {
        let mount_info = if cfg!(target_os = "macos") {
            // on macOs, use mount command
            match Command::new("mount").output() {
                Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
                Err(e) => {
                    println!("Error: Could not execute mount command - {e}");
                    process::exit(1);
                }
            }
        } else {
            // on Linux, read from /proc/mounts
            match std::fs::read_to_string("/proc/mounts") {
                Ok(contents) => contents,
                Err(e) => {
                    println!("Error: Could not open /proc/mounts - {e}");
                    process::exit(1);
                }
            }
        };

        let mut stats: Vec<Stats> = Vec::new();
        let mut max_width = 0;

        for line in mount_info.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 2 {
                continue;
            }

            let (filesystem, mountpoint) = if cfg!(target_os = "macos") {
                // macOS mount output format: device_name on mount_point (options)
                if !line.contains(" on ") {
                    continue;
                }
                let parts: Vec<&str> = line.split(" on ").collect();
                if parts.len() != 2 {
                    continue;
                }
                let mount_parts: Vec<&str> = parts[1].split(" (").collect();
                if mount_parts.is_empty() {
                    continue;
                }
                (parts[0], mount_parts[0])
            } else {
                // Linux format from /proc/mounts
                (
                    fields[ProcFields::Filesystem.upcast()],
                    fields[ProcFields::Mountpoint.upcast()],
                )
            };

            let statvfs = match statvfs(mountpoint) {
                Ok(s) => s,
                Err(_) => continue, // i.e.: no permissions to read
            };

            if statvfs.blocks() == 0 {
                continue;
            }

            let s = Stats::new(filesystem, mountpoint, statvfs, args);

            max_width = cmp::max(max_width, s.filesystem.len());
            stats.push(s);
        }
        stats.sort();
        (stats, max_width)
    }
}
