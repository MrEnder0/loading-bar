pub mod colors;

use std::{thread, time::{self}};
use crate::colors::*;

struct LoadingBar {
    progress: i32,
    title: String
}

trait LoadingBarMethods {
    fn move_progress(&mut self, amount: i32);
    fn set_progress(&mut self, progress: i32);
    fn render(&mut self);
    fn link_timer(&mut self, time: u64);
}

impl LoadingBarMethods for LoadingBar {
    fn move_progress(&mut self, amount: i32) {
        self.progress += amount
    }

    fn set_progress(&mut self, progress: i32) {
        self.progress = progress
    }

    fn render(&mut self) {
        match self.progress {
            100 => println!("\r{title} |{green}{progress_filled}{reset}| (Done {green}✓{reset})", title = self.title, green = COLOR_GREEN, progress_filled = "█".repeat(10), reset = COLOR_RESET),
            _ => println!("\r{title} |{green}{progress_filled}{red}{progress_empty}{reset}| ({progress}%)\x1B[1A", title = self.title, green = COLOR_GREEN, progress_filled = "█".repeat((self.progress / 10) as usize), red = COLOR_RED, progress_empty = "█".repeat((10 - (self.progress / 10)) as usize), reset = COLOR_RESET, progress = self.progress),
        }
    }

    fn link_timer(&mut self, time: u64) {
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
    //Use loading bars here
}

#[cfg(test)]
mod tests {
    use std::{thread, time::{self}};

    use crate::{LoadingBar, LoadingBarMethods};

    #[test]
    fn time_linked_loadingbar() {
        let mut loading_bar = LoadingBar{ progress: 0, title: "Loading".to_string() };
        loading_bar.link_timer(time::Duration::from_secs(1).as_millis() as u64);
        assert_eq!(loading_bar.progress, 100);
    }

    #[test]
    fn set_loadingbar() {
        let mut loading_bar = LoadingBar{ progress: 0, title: "Loading".to_string() };
        loading_bar.set_progress(69);
        assert_eq!(loading_bar.progress, 69);
    }

    #[test]
    fn itterate_loadingbar() {
        let mut loading_bar = LoadingBar{ progress: 0, title: "Loading".to_string() };
        let mut rendered_frames = 0;

        while loading_bar.progress < 100 {
            thread::sleep(time::Duration::from_millis(10));
            loading_bar.move_progress(1);
    
            //Only renders the updated version when it reached a % which is divisible by 5
            if loading_bar.progress % 5 == 0 {
                loading_bar.render();
                rendered_frames += 1;
            }
        }
        assert_eq!(loading_bar.progress, 100);
        assert_eq!(rendered_frames, 20);
    }
}
