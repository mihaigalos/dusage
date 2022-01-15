use byte_unit::Byte;
use clap::ArgMatches;
use colored::*;

use crate::bar::Bar;
use crate::colorizer::Colorizer;
use crate::stats::Stats;
pub struct Writer;

impl Writer {
    fn iec_representation(input: u64) -> String {
        Byte::from_bytes(input as u128)
            .get_appropriate_unit(false)
            .format(0)
            .replace(" ", "")
    }

    pub fn write(stats: Vec<Stats>, max_width: usize, args: ArgMatches) {
        if args.is_present("inodes") {
            Writer::write_inodes(stats, max_width);
        } else {
            Writer::write_disks(stats, max_width);
        }
    }
    pub fn write_disks(stats: Vec<Stats>, max_width: usize) {
        println!(
            "{:width$} {:>5} {:>5} {:>5} {:>6} {:>20} {}",
            "Filesystem".yellow(),
            "Size".yellow(),
            "Used".yellow(),
            "Avail".yellow(),
            "Use%".yellow(),
            "Disk / INodes".yellow(),
            "Mounted on".yellow(),
            width = max_width
        );
        for stat in stats {
            if Writer::is_relevant(&stat) {
                Writer::write_disk_stat(stat, max_width);
            }
        }
    }
    pub fn write_inodes(stats: Vec<Stats>, max_width: usize) {
        println!(
            "{:width$} {:>10} {:>10} {:>10} {:>6} {:>20} {}",
            "Filesystem".yellow(),
            "INodes".yellow(),
            "IUsed".yellow(),
            "IFree".yellow(),
            "IUse%".yellow(),
            "Disk / INodes".yellow(),
            "Mounted on".yellow(),
            width = max_width
        );
        for stat in stats {
            if Writer::is_relevant(&stat) {
                Writer::write_inodes_stat(stat, max_width);
            }
        }
    }

    fn write_disk_stat(stat: Stats, max_width: usize) {
        let percent_disk = if stat.percent_disk.is_nan() {
            "     -".to_string()
        } else {
            format!("{:>5.0}%", stat.percent_disk)
        };
        print!(
            "{:width$} {:>5} {:>5} {:>5} {} {:20} ",
            Colorizer::colorize_filesystem(stat.filesystem.clone(), stat.is_network()),
            Writer::iec_representation(stat.size_disk),
            Writer::iec_representation(stat.used_disk),
            Writer::iec_representation(stat.available_disk),
            percent_disk,
            Bar::new_disk(stat.percent_disk),
            width = max_width
        );
        println!("{}", Colorizer::colorize_mountpoint(stat.mount));
    }

    fn write_inodes_stat(stat: Stats, max_width: usize) {
        let percent_inodes = format!("{:>5.0}%", stat.percent_inodes);
        print!(
            "{:width$} {:>10} {:>10} {:>10} {} {:20} ",
            Colorizer::colorize_filesystem(stat.filesystem.clone(), stat.is_network()),
            stat.total_inodes,
            stat.used_inodes,
            stat.available_inodes,
            percent_inodes,
            Bar::new_disk(stat.percent_disk),
            width = max_width
        );
        println!("{}", Colorizer::colorize_mountpoint(stat.mount));
    }

    fn is_relevant(stat: &Stats) -> bool {
        stat.size_disk > 0
    }
}
