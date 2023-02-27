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
    fn render(&mut self);
    fn loading_bar_link_timer(&mut self, time: u64);
}

impl LoadingBarMethods for LoadingBar {
    fn progress_loading_bar(&mut self, amount: i32) {
        self.progress += amount
    }

    fn set_loading_bar(&mut self, progress: i32) {
        self.progress = progress
    }

    fn render(&mut self) {
        match self.progress {
            100 => println!("\r{title} |{green}{progress_filled}{reset}| (Done {green}✓{reset})", title = self.title, green = COLOR_GREEN, progress_filled = "█".repeat(10), reset = COLOR_RESET),
            _ => println!("\r{title} |{green}{progress_filled}{red}{progress_empty}{reset}| ({progress}%)\x1B[1A", title = self.title, green = COLOR_GREEN, progress_filled = "█".repeat((self.progress / 10) as usize), red = COLOR_RED, progress_empty = "█".repeat((10 - (self.progress / 10)) as usize), reset = COLOR_RESET, progress = self.progress),
        }
    }

    fn loading_bar_link_timer(&mut self, time: u64) {
        let now = time::SystemTime::now();
        while self.progress < 100 {
            thread::sleep(time::Duration::from_millis(1 as u64));
            let elapsed = now.elapsed().unwrap().as_millis();
            self.progress = (elapsed * 100 / time as u128) as i32;
            self.render();
        }
    }
}

fn main() {
    //Shows how to init a loading bar
    let mut some_data_progress = LoadingBar{ progress: 0, title: "Loading".to_string() };

    //Shows how to link a loading bar to a timer
    some_data_progress.loading_bar_link_timer(10000);

    thread::sleep(time::Duration::from_millis(2500));

    //Shows how to set the loading bar
    some_data_progress.set_loading_bar(0);
    println!("Reset the loading bar to {}%", some_data_progress.progress);

    thread::sleep(time::Duration::from_millis(2500));

    //Shows how to iterate a loading bar in a loop
    while some_data_progress.progress < 100 {
        thread::sleep(time::Duration::from_millis(50));
        some_data_progress.progress_loading_bar(1);

        //Only renders the updated version when it reached a % which is divisible by 5
        if some_data_progress.progress % 5 == 0 {
            some_data_progress.render();
        }
    }
}