pub mod colors;

use std::{thread, time, io};
use crate::colors::*;

fn loading_bar(time: u64, title: &str) {
    let mut percent: i32 = 0;

    print!("{} |{}{}{}| (0%)", title, COLOR_RED, "█".repeat(10), COLOR_RESET);

    while percent < 100 {
        percent += 1;
        println!("\r{} |{}{}{}{}{}| ({}%)\x1B[1A", title, COLOR_GREEN, "█".repeat((percent / 10) as usize), COLOR_RED, "█".repeat((10 - (percent / 10)) as usize), COLOR_RESET, percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    println!("\r{} |{}{}{}| (Done {}✓{})", title, COLOR_GREEN, "█".repeat(10), COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    thread::sleep(time::Duration::from_millis(100));
}
fn main() {
    println!("Enter time in milliseconds: ");
    let mut time = String::new();

    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");
    
    loading_bar(time, "Loading");
}