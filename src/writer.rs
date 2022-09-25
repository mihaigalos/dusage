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
            .get_appropriate_unit(true)
            .format(1)
            .replace(' ', "")
            .replace('i', "")
            .replace('B', "")
    }

    pub fn write(stats: Vec<Stats>, mut max_width: usize, args: ArgMatches) {
        let min_width = 12;
        if max_width < min_width {
            max_width = min_width;
        }
        if args.is_present("inodes") {
            Writer::write_inodes(stats, max_width, args);
        } else {
            Writer::write_disks(stats, max_width, args);
        }
    }
    pub fn write_disks(stats: Vec<Stats>, max_width: usize, args: ArgMatches) {
        println!(
            "{:width$} {:>8} {:>8} {:>8} {:>6} {:>20} {}",
            "Filesystem".yellow().bold(),
            "Size".yellow().bold(),
            "Used".yellow().bold(),
            "Avail".yellow().bold(),
            "Use%".yellow().bold(),
            "Disk / INodes".yellow().bold(),
            "Mounted on".yellow().bold(),
            width = max_width
        );
        let is_copy_friendly = args.is_present("copy-friendly");
        for stat in stats {
            if Writer::is_relevant(&stat) {
                Writer::write_disk_stat(stat, max_width, is_copy_friendly);
            }
        }
    }
    pub fn write_inodes(stats: Vec<Stats>, max_width: usize, args: ArgMatches) {
        println!(
            "{:width$} {:>10} {:>10} {:>10} {:>6} {:>20} {}",
            "Filesystem".yellow().bold(),
            "INodes".yellow().bold(),
            "IUsed".yellow().bold(),
            "IFree".yellow().bold(),
            "IUse%".yellow().bold(),
            "Disk / INodes".yellow().bold(),
            "Mounted on".yellow().bold(),
            width = max_width
        );
        let is_copy_friendly = args.is_present("copy-friendly");
        for stat in stats {
            if Writer::is_relevant(&stat) {
                Writer::write_inodes_stat(stat, max_width, is_copy_friendly);
            }
        }
    }

    fn write_disk_stat(stat: Stats, max_width: usize, is_copy_friendly: bool) {
        let percent_disk = if stat.percent_disk.is_nan() {
            "     -".to_string()
        } else {
            format!("{:>5.0}%", stat.percent_disk)
        };
        print!(
            "{:width$} {:>8} {:>8} {:>8} {} {:20} ",
            Colorizer::colorize_filesystem(stat.filesystem.clone(), stat.is_network()),
            Writer::iec_representation(stat.size_disk),
            Writer::iec_representation(stat.used_disk),
            Writer::iec_representation(stat.available_disk),
            percent_disk,
            Bar::new_disk(stat.percent_disk, stat.percent_inodes, is_copy_friendly),
            width = max_width
        );
        println!("{}", Colorizer::colorize_mountpoint(stat.mount));
    }
    fn write_inodes_stat(stat: Stats, max_width: usize, is_copy_friendly: bool) {
        let percent_inodes = if stat.percent_inodes.is_nan() {
            "     -".to_string()
        } else {
            format!("{:>5.0}%", stat.percent_inodes)
        };
        print!(
            "{:width$} {:>10} {:>10} {:>10} {} {:20} ",
            Colorizer::colorize_filesystem(stat.filesystem.clone(), stat.is_network()),
            stat.total_inodes,
            stat.used_inodes,
            stat.available_inodes,
            percent_inodes,
            Bar::new_disk(stat.percent_disk, stat.percent_inodes, is_copy_friendly),
            width = max_width
        );
        println!("{}", Colorizer::colorize_mountpoint(stat.mount));
    }

    fn is_relevant(stat: &Stats) -> bool {
        stat.size_disk > 0
    }
}

#[test]
fn test_when_typical() {
    assert_eq!(Writer::iec_representation(486126166016), "452.7G");
}
