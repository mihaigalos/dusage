use crate::colorizer::Colorizer;

pub struct Bar;

impl Bar {
    pub fn new_disk(percent_disk: f64, percent_inodes: f64) -> String {
        let bar_length = 20;
        let bar_unit = "■";
        let count_inode_units = Bar::compute_bar_units(percent_inodes, bar_length);
        let count_disk_units = Bar::compute_bar_units(percent_disk, bar_length);
        Colorizer::colorize_bar(
            bar_length,
            bar_unit,
            count_disk_units,
            count_inode_units,
            percent_disk,
            percent_inodes,
        )
    }
    fn compute_bar_units(mut percent: f64, total_chars: usize) -> usize {
        if percent.is_nan() {
            percent = 0.0;
        }
        let parts_used = (percent.round() * total_chars as f64 / 100.0).ceil() as usize;
        parts_used
    }
}
