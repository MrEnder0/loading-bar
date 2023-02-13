use std::{thread, time, io};

fn loading_bar(time: u64) {
    let mut percent: i32 = 0;

    print!("Loading {} (0%)", "-".repeat(10));

    while percent < 100 {
        percent += 1;
        println!("\rLoading {}{} ({}%)\x1B[1A", "=".repeat((percent / 10).try_into().unwrap()), "-".repeat((10 - (percent / 10)).try_into().unwrap()), percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    println!("\rLoading {} (100%)", "=".repeat(10));
    thread::sleep(time::Duration::from_millis(100));
}
fn main() {
    println!("Enter time in milliseconds: ");
    let mut time = String::new();

    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");
    
    loading_bar(time);
}