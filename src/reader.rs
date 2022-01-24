use clap::ArgMatches;
use nix::sys::statvfs::statvfs;
use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

use crate::procfields::ProcFields;
use crate::stats::Stats;

pub struct Reader;

impl Reader {
    pub fn read(args: &ArgMatches) -> (Vec<Stats>, usize) {
        let file = match File::open("/proc/mounts") {
            Ok(f) => f,
            Err(e) => {
                println!("Error: Could not open /proc/mounts - {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(&file);

        let mut stats: Vec<Stats> = Vec::new();
        let mut max_width = 0;

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    let statvfs = match statvfs(fields[ProcFields::Mountpoint.downcast()]) {
                        Ok(s) => s,
                        Err(_) => continue, // i.e.: no permissions to read
                    };
                    let size_disk = statvfs.blocks() as u64 * statvfs.block_size() as u64;
                    let available_disk =
                        statvfs.blocks_available() as u64 * statvfs.block_size() as u64;

                    let total_inodes = statvfs.files() as u64;
                    let available_inodes = statvfs.files_available() as u64;

                    if args.is_present("debug") {
                        println!(
                            "{} blocks: {} size: {} available: {}",
                            fields[ProcFields::Filesystem.downcast()],
                            statvfs.block_size(),
                            size_disk,
                            available_disk
                        );
                    }
                    let s = Stats::new(
                        fields[ProcFields::Filesystem.downcast()],
                        size_disk,
                        available_disk,
                        fields[ProcFields::Mountpoint.downcast()],
                        total_inodes,
                        available_inodes,
                    );

                    max_width = cmp::max(max_width, s.filesystem.len());
                    stats.push(s);
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        stats.sort();
        (stats, max_width)
    }
}
