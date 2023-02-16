pub mod colors;

use std::{thread, time::{self}};
use crate::colors::*;

struct LoadingBar {
    progress: i32,
    title: String
}

trait LoadingBarMethods {
    fn progress_loading_bar(&mut self, amount: i32);
    fn set_loading_bar(&mut self, progress: i32);
    fn show_loading_bar(&mut self);
    fn loading_bar_link_timer(&mut self, time: u64);
}

impl LoadingBarMethods for LoadingBar {
    fn progress_loading_bar(&mut self, amount: i32) {
        self.progress += amount
    }

    fn set_loading_bar(&mut self, progress: i32) {
        self.progress = progress
    }

    fn show_loading_bar(&mut self) {
        match self.progress {
            100 => println!("\r{} |{}{}{}| (Done {}✓{})", self.title, COLOR_GREEN, "█".repeat(10), COLOR_RESET, COLOR_GREEN, COLOR_RESET),
            _ => println!("\r{} |{}{}{}{}{}| ({}%)\x1B[1A", self.title, COLOR_GREEN, "█".repeat((self.progress / 10) as usize), COLOR_RED, "█".repeat((10 - (self.progress / 10)) as usize), COLOR_RESET, self.progress),
        }
    }

    fn loading_bar_link_timer(&mut self, time: u64) {
        let now = time::SystemTime::now();
        while self.progress < 100 {
            thread::sleep(time::Duration::from_millis(1 as u64));
            let elapsed = now.elapsed().unwrap().as_millis();
            self.progress = (elapsed * 100 / time as u128) as i32;
            self.show_loading_bar();
        }
    }
}

fn main() {
    let mut some_data_progress = LoadingBar{ progress: 0, title: "Loading".to_string() };

    some_data_progress.loading_bar_link_timer(10000);
}