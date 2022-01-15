use crate::colorizer::Colorizer;

pub struct Bar;

impl Bar {
    fn bar_disk(percent_disk: f64) -> String {
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
        let chars = "■■■■■■■■■■■■■■■■■■■■";
        let used_end = compute_used_end(percent_disk, chars);

        let bar1 = Colorizer::colorize_disk_used(chars[..used_end].to_string(), percent_disk);
        let bar2 = Colorizer::colorize_disk_free(chars[used_end..].to_string());
        format!("{}{}", bar1, bar2)
    }
    pub fn new_disk(percent_disk: f64) -> String {
        Bar::bar_disk(percent_disk)
    }
}
