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
                    let statvfs = match statvfs(fields[ProcFields::Mountpoint.upcast()]) {
                        Ok(s) => s,
                        Err(_) => continue, // i.e.: no permissions to read
                    };

                    let s = Stats::new(
                        fields[ProcFields::Filesystem.upcast()],
                        fields[ProcFields::Mountpoint.upcast()],
                        statvfs,
                        args,
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
