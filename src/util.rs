use colored::*;

pub fn bargraph(mut percent: f64) -> String {
    if percent.is_nan() {
        percent = 0.0;
    }
    let chars = "■■■■■■■■■■■■■■■■■■■■";
    let s1 = (percent / 10.0).round() as usize * 2;
    let s2 = 20 - s1;
    let bar1 = chars.chars().take(s1).collect::<String>();
    let bar1 = if percent > 90.0 {
        bar1.red()
    } else if percent > 75.0 {
        bar1.yellow()
    } else {
        bar1.green()
    };
    let bar2 = chars.chars().take(s2).collect::<String>().white().dimmed();
    format!("{}{}", bar1, bar2)
}
