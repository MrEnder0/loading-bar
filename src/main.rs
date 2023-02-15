pub mod colors;

use std::{thread, time};
use crate::colors::*;

struct LoadingBar {
    progress: i32,
    title: String
}

trait LoadingBarMethods {
    fn progress_loading_bar(&mut self, amount: i32);
    fn show_loading_bar(&mut self);
}

impl LoadingBarMethods for LoadingBar {
    fn progress_loading_bar(&mut self, amount: i32) {
        self.progress += amount
    }

    fn show_loading_bar(&mut self) {
        match self.progress {
            100 => println!("\r{} |{}{}{}| (Done {}✓{})", self.title, COLOR_GREEN, "█".repeat(10), COLOR_RESET, COLOR_GREEN, COLOR_RESET),
            _ => println!("\r{} |{}{}{}{}{}| ({}%)\x1B[1A", self.title, COLOR_GREEN, "█".repeat((self.progress / 10) as usize), COLOR_RED, "█".repeat((10 - (self.progress / 10)) as usize), COLOR_RESET, self.progress),
        }
    }
}
fn main() {
    let mut some_data_progress = LoadingBar{ progress: 0, title: "Loading".to_string() };

    while some_data_progress.progress < 100 {
        some_data_progress.progress_loading_bar(10);
        some_data_progress.show_loading_bar();
        thread::sleep(time::Duration::from_millis(1000));
    }
    println!("");
}