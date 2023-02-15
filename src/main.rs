pub mod colors;

use std::{thread, time};
use crate::colors::*;

struct LoadingBar {
    progress: i32,
    title: String
}


fn progress_loading_bar(loading_bar: &mut LoadingBar, amount: i32) {
    loading_bar.progress += amount
}

fn show_loading_bar(loading_bar: &mut LoadingBar) {
    match loading_bar.progress {
        100 => println!("\r{} |{}{}{}| (Done {}✓{})", loading_bar.title, COLOR_GREEN, "█".repeat(10), COLOR_RESET, COLOR_GREEN, COLOR_RESET),
        _ => println!("\r{} |{}{}{}{}{}| ({}%)\x1B[1A", loading_bar.title, COLOR_GREEN, "█".repeat((loading_bar.progress / 10) as usize), COLOR_RED, "█".repeat((10 - (loading_bar.progress / 10)) as usize), COLOR_RESET, loading_bar.progress),
    }
}
fn main() {
    let mut progress_indicator = LoadingBar {
        progress: 0,
        title: "Loading".to_string()
    };

    while progress_indicator.progress < 100 {
        progress_loading_bar(&mut progress_indicator, 10);
        show_loading_bar(&mut progress_indicator);
        thread::sleep(time::Duration::from_millis(1000));
    }
    println!("");
}