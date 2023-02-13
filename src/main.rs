use std::{thread, time, io};

fn main() {
    let mut percent: i32 = 0;

    println!("Enter time in milliseconds: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");


    print!("Loading ----- (0%)");

    while percent < 100 {
        percent += 1;
        println!("\rLoading {}{} ({}%)\x1B[1A", "=".repeat((percent / 20).try_into().unwrap()), "-".repeat((5 - (percent / 20)).try_into().unwrap()), percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    println!("\rLoading {} (100%)", "=".repeat(5));
    thread::sleep(time::Duration::from_millis(100));
}