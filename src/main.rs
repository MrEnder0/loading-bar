pub mod colors;

use std::{thread, time, io};

use crate::colors::{COLOR_RED, COLOR_RESET, COLOR_GREEN, COLOR_CYAN};

fn loading_bar(time: u64) {
    let mut percent: i32 = 0;

    print!("Loading |{}{}{}| (0%)", COLOR_RED, "█".repeat(10), COLOR_RESET);

    while percent < 100 {
        percent += 1;
        println!("\rLoading |{}{}{}{}{}| ({}%)\x1B[1A", COLOR_GREEN, "█".repeat((percent / 10) as usize), COLOR_RED, "█".repeat((10 - (percent / 10)) as usize), COLOR_RESET, percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    println!("\rLoading |{}{}{}| (100%)", COLOR_CYAN, "█".repeat(10), COLOR_RESET);
    thread::sleep(time::Duration::from_millis(100));
}
fn main() {
    println!("Enter time in milliseconds: ");
    let mut time = String::new();

    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");
    
    loading_bar(time);
}