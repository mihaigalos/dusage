use byte_unit::Byte;
use colored::*;

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

    pub fn write(stats: Vec<Stats>, max_width: usize) {
        println!(
            "{:width$} {:>5} {:>5} {:>5} {:>6} {:>20} {}",
            "Filesystem".yellow(),
            "Size".yellow(),
            "Used".yellow(),
            "Avail".yellow(),
            "Use%".yellow(),
            "Disk / iNodes".yellow(),
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
        let percent = if stat.percent.is_nan() {
            "     -".to_string()
        } else {
            format!("{:>5.0}%", stat.percent)
        };
        print!(
            "{:width$} {:>5} {:>5} {:>5} {} {:20} ",
            Colorizer::colorize_filesystem(stat.filesystem.clone(), stat.is_network()),
            Writer::iec_representation(stat.size),
            Writer::iec_representation(stat.used),
            Writer::iec_representation(stat.avail),
            percent,
            Writer::bargraph(stat.percent),
            width = max_width
        );
        println!("{}", Colorizer::colorize_mountpoint(stat.mount));
    }

    fn is_relevant(stat: &Stats) -> bool {
        stat.size > 0
    }

    fn bargraph(mut percent: f64) -> String {
        if percent.is_nan() {
            percent = 0.0;
        }
        let chars = "■■■■■■■■■■■■■■■■■■■■";
        let one_char_length_in_bytes = chars.chars().take(1).last().unwrap().len_utf8();

        let parts_used = (percent / 10.0).round() as usize * 2;
        let used_end = parts_used * one_char_length_in_bytes;

        let bar1 = Colorizer::colorize_disk_used(chars[..used_end].to_string(), percent);
        let bar2 = Colorizer::colorize_disk_free(chars[used_end..].to_string());
        format!("{}{}", bar1, bar2)
    }
}
