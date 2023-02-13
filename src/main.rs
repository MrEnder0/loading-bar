use std::{thread, time, io};

fn loading_bar(time: u64) {
    let _color_red = "\x1B[31m";
    let _color_green = "\x1B[32m";
    let _color_reset = "\x1B[0m";
    let mut percent: i32 = 0;

    print!("Loading |{}{}{}| (0%)", _color_red, "█".repeat(10), _color_reset);

    while percent < 100 {
        percent += 1;
        println!("\rLoading |{}{}{}{}{}| ({}%)\x1B[1A", _color_green, "█".repeat((percent / 10).try_into().unwrap()), _color_red, "█".repeat((10 - (percent / 10)).try_into().unwrap()), _color_reset, percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    println!("\rLoading |{}{}{}| (100%)", _color_green, "█".repeat(10), _color_reset);
    thread::sleep(time::Duration::from_millis(100));
}
fn main() {
    println!("Enter time in milliseconds: ");
    let mut time = String::new();

    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");
    
    loading_bar(time);
}