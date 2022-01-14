use std::cmp::Ordering;

use crate::filesystem::Filesystem;

#[derive(Debug)]
pub struct Stats {
    pub filesystem: String,
    pub size: u64,
    pub used: u64,
    pub avail: u64,
    pub percent: f64,
    pub mount: String,
    pos: usize,
}

impl Stats {
    pub fn new(fs: &str, size: u64, avail: u64, mount: &str) -> Stats {
        let used = size - avail;
        let percent = used as f64 / size as f64;
        let pos = grouped_pos_by_length(fs);
        Stats {
            filesystem: fs.to_string(),
            size: size,
            avail: avail,
            used: used,
            percent: 100.0 * percent,
            mount: mount.to_string(),
            pos: pos,
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
