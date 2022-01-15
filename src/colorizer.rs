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
            "/"  => input.blue(),
            _ => {
                if input.contains("/boot") {
                    input.blue()
                }
                else if input.contains("/home") {
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
}
