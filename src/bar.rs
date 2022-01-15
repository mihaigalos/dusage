use crate::colorizer::Colorizer;

pub struct Bar;

impl Bar {
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
        let bar = Bar::bar_disk(percent_disk);
        Bar::mixin_inodes(percent_inodes, bar)
    }
}
