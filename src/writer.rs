use byte_unit::Byte;
use colored::*;

use crate::stats::Stats;
use crate::util::bargraph;
pub struct Writer;

impl Writer {
    fn iec_representation(input: u64) -> String {
        Byte::from_bytes(input as u128)
            .get_appropriate_unit(false)
            .format(0)
            .replace(" ", "")
    }

    pub fn write(stats: Vec<Stats>, max_width: usize) {
        println!(
            "{:width$} {:>5} {:>5} {:>5} {:>6} {:20} {}",
            "Filesystem".yellow(),
            "Size".yellow(),
            "Used".yellow(),
            "Avail".yellow(),
            "Use%".yellow(),
            "",
            "Mounted on".yellow(),
            width = max_width
        );

        for stat in stats {
            if Writer::is_relevant(&stat) {
                Writer::write_stat(stat, max_width);
            }
        }
    }

    fn write_stat(stat: Stats, max_width: usize) {
        let fs = if stat.is_network() {
            stat.filesystem.cyan()
        } else {
            stat.filesystem.normal()
        };
        let percent = if stat.percent.is_nan() {
            "     -".to_string()
        } else {
            format!("{:>5.0}%", stat.percent)
        };
        print!(
            "{:width$} {:>5} {:>5} {:>5} {} {:20} ",
            fs,
            Writer::iec_representation(stat.size),
            Writer::iec_representation(stat.used),
            Writer::iec_representation(stat.avail),
            percent,
            bargraph(stat.percent),
            width = max_width
        );
        if stat.mount.contains("/boot") || stat.mount == "/" || stat.mount.contains("/home") {
            println!("{}", stat.mount.blue());
        } else if stat.mount.contains("/var/log") {
            println!("{}", stat.mount.white());
        } else if stat.mount.contains("/mnt") {
            println!("{}", stat.mount.green());
        } else {
            println!("{}", stat.mount);
        }
    }

    fn is_relevant(stat: &Stats) -> bool {
        stat.size > 0
    }
}
