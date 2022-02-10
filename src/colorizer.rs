use colored::*;
pub struct Colorizer;

impl Colorizer {
    pub fn colorize_filesystem(input: String, is_network: bool) -> ColoredString {
        match is_network {
            true => input.cyan(),
            false => input.normal(),
        }
    }
    pub fn colorize_mountpoint(input: String) -> ColoredString {
        match input.as_ref() {
            "/" => input.blue(),
            _ => {
                if input.contains("/boot") {
                    input.blue()
                } else if input.contains("/home") {
                    input.blue()
                } else if input.contains("/var/log") {
                    input.white()
                } else if input.contains("/mnt") {
                    input.green()
                } else {
                    ColoredString::from(&input[..])
                }
            }
        }
    }
    pub fn colorize_disk_used(input: String, percent_disk: f64) -> ColoredString {
        if percent_disk > 90.0 {
            input.red()
        } else if percent_disk > 75.0 {
            input.yellow()
        } else {
            input.green()
        }
    }
    pub fn colorize_disk_free(input: String) -> ColoredString {
        input.white().dimmed()
    }
    pub fn colorize_inodes_used(input: String, percent_disk: f64) -> ColoredString {
        if percent_disk > 90.0 {
            format!("{}", input).on_magenta()
        } else {
            format!("{}", input).on_blue()
        }
    }
    pub fn colorize_bar(
        bar_length: usize,
        bar_unit: &str,
        bar_unit_empty: &str,
        is_copy_friendly: bool,
        count_disk_units: usize,
        count_inode_units: usize,
        percent_disk: f64,
        percent_inodes: f64,
    ) -> String {
        let mut result = "".to_string();
        let background = match is_copy_friendly {
            true => bar_unit_empty.white(),
            false => bar_unit.white().dimmed(),
        };
        for i in 0..bar_length {
            if i < count_disk_units {
                result = format!(
                    "{}{}",
                    result,
                    Colorizer::colorize_disk_used(bar_unit.to_string(), percent_disk)
                );
            } else {
                result = format!("{}{}", result, background);
            }
            if i < count_inode_units {
                result = format!(
                    "{}",
                    Colorizer::colorize_inodes_used(result, percent_inodes)
                );
            }
        }
        result
    }
}
