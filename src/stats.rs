use crate::filesystem::Filesystem;
use clap::ArgMatches;
use nix::sys::statvfs::Statvfs;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size_disk: u64,
    pub used_disk: u64,
    pub available_disk: u64,
    pub percent_disk: f64,
    pub mount: String,
    pos: usize,
    pub total_inodes: u64,
    pub used_inodes: u64,
    pub available_inodes: u64,
    pub percent_inodes: f64,
}

impl Stats {
    pub fn new(fs: &str, mount: &str, statvfs: Statvfs, args: &ArgMatches) -> Stats {
        let size_disk = u64::from(statvfs.blocks()) * u64::from(statvfs.fragment_size());
        let fragment_size = statvfs.fragment_size();
        let blocks = statvfs.blocks();
        let available_disk = u64::from(statvfs.blocks_available()) * statvfs.fragment_size();
        let free_disk = u64::from(statvfs.blocks_free()) * statvfs.fragment_size();

        let total_inodes = u64::from(statvfs.files());
        let available_inodes = u64::from(statvfs.files_available());

        let used_disk = size_disk - free_disk;
        let percent_disk = used_disk as f32 / size_disk as f32;
        let pos = grouped_pos_by_length(fs);

        let used_inodes = total_inodes - available_inodes;
        let percent_inodes = used_inodes as f32 / total_inodes as f32;

        if args.get_flag("debug") {
            if !args.get_flag("inodes") {
                println!(
                    "{} blocks: {} fragment_size: {} size: {} free: {} available: {}",
                    fs, blocks, fragment_size, size_disk, free_disk, available_disk
                );
            } else {
                println!(
                    "{fs} total_inodes: {total_inodes} iused: {used_inodes} ifree: {available_inodes} iused%: {percent_inodes}"
                );
            }
        }
        Stats {
            filesystem: fs.to_string(),
            size_disk,
            used_disk,
            available_disk,
            percent_disk: 100.0 * percent_disk as f64,
            mount: mount.to_string(),
            pos,
            total_inodes,
            used_inodes,
            available_inodes,
            percent_inodes: 100.0 * percent_inodes as f64,
        }
    }

    pub fn is_network(&self) -> bool {
        self.pos == Filesystem::Network as usize
    }
}

fn grouped_pos_by_length(fs: &str) -> usize {
    let mut result = Filesystem::new(fs) as usize;
    if fs.starts_with("/dev") {
        result += fs.len();
    }
    result
}

impl Ord for Stats {
    fn cmp(&self, other: &Stats) -> Ordering {
        let cmp = self.pos.cmp(&other.pos);
        if cmp != Ordering::Equal {
            return cmp;
        }

        self.filesystem.cmp(&other.filesystem)
    }
}

impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Stats) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stats {
    fn eq(&self, other: &Stats) -> bool {
        self.filesystem == other.filesystem
    }
}

impl Eq for Stats {}
