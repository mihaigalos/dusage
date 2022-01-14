use colored::*;
pub struct Colorizer;

impl Colorizer {
    pub fn colorize_from_filesystem(input: String, is_network: bool) -> ColoredString {
        match is_network {
            true => input.cyan(),
            false => input.normal(),
        }
    }
    pub fn colorize_from_mountpoint(input: String) -> ColoredString {
        match input.as_ref() {
            "/" | "/boot" => input.blue(),
            _ => {
                if input.contains("/home") {
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
}
