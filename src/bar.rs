use crate::colorizer::Colorizer;
use colored::*;

pub struct Bar;

impl Bar {
    fn compute_bar_units(mut percent: f64, bar_unit: String, total_chars: usize) -> usize {
        if percent.is_nan() {
            percent = 0.0;
        }
        let parts_used = (percent.round() * total_chars as f64 / 100.0).ceil() as usize;
        parts_used
    }
    fn compute_used_end(mut percent_disk: f64, chars: &str) -> usize {
        if percent_disk.is_nan() {
            percent_disk = 0.0;
        }
        let one_char_length_in_bytes = chars.chars().take(1).last().unwrap().len_utf8();
        let total_chars = chars.len() as f64 / one_char_length_in_bytes as f64;
        let parts_used = (percent_disk.round() * total_chars / 100.0).ceil() as usize;
        let used_end = parts_used * one_char_length_in_bytes;
        used_end
    }
    fn bar_disk(percent_disk: f64) -> String {
        let chars = "■■■■■■■■■■■■■■■■■■■■";
        let used_end = Bar::compute_used_end(percent_disk, chars);

        let bar_used = Colorizer::colorize_disk_used(chars[..used_end].to_string(), percent_disk);
        let bar_free = Colorizer::colorize_disk_free(chars[used_end..].to_string());

        format!("{}{}", bar_used, bar_free)
    }
    fn mixin_inodes(percent_inodes: f64, input: String) -> String {
        let length_of_formatting = 5;
        let used_end = Bar::compute_used_end(percent_inodes, &input) + length_of_formatting;
        let bar_used =
            Colorizer::colorize_inodes_used(input[..used_end].to_string(), percent_inodes);
        let bar_free = input[used_end..].to_string();

        format!("{}{}", bar_used, bar_free)
    }

    pub fn new_disk(percent_disk: f64, percent_inodes: f64) -> String {
        let bar_length = 20;
        let mut result = "".to_string();
        let bar_unit = "■";
        let count_inode_units =
            Bar::compute_bar_units(percent_inodes, bar_unit.to_string(), bar_length);
        let count_disk_units =
            Bar::compute_bar_units(percent_disk, bar_unit.to_string(), bar_length);

        for i in 0..bar_length {
            if i < count_disk_units {
                result = format!("{}{}", result, bar_unit.green());
            } else {
                result = format!("{}{}", result, bar_unit);
            }
            if i < count_inode_units {
                result = format!("{}", result.on_blue());
            } else {
                result = format!("{}", result);
            }
        }
        result
    }
}
