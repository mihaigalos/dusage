use std::cmp::Ordering;

use crate::filesystem::Filesystem;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size_disk: u64,
    pub used_disk: u64,
    pub available_disk: u64,
    pub percent_disk: f64,
    pub mount: String,
    pos: usize,
    pub percent_inodes: f64,
}

impl Stats {
    pub fn new(fs: &str, size_disk: u64, available_disk: u64, mount: &str) -> Stats {
        let used_disk = size_disk - available_disk;
        let percent_disk = used_disk as f64 / size_disk as f64;
        let pos = grouped_pos_by_length(fs);
        Stats {
            filesystem: fs.to_string(),
            size_disk: size_disk,
            available_disk: available_disk,
            used_disk: used_disk,
            percent_disk: 100.0 * percent_disk,
            mount: mount.to_string(),
            pos: pos,
            percent_inodes: 0.0,
        }
    }

    pub fn is_network(&self) -> bool {
        self.pos == Filesystem::Network as usize
    }
}

fn grouped_pos_by_length(fs: &str) -> usize {
    let mut result = Filesystem::new(fs) as usize;
    if fs.starts_with("/dev") {
        result = result + fs.len();
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
