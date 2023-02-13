use std::{thread, time, io};

fn main() {
    let mut percent: i32 = 0;

    println!("Enter time in milliseconds: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");


    print!("Loading {}%", percent);

    while percent < 100 {
        percent += 1;
        println!("\rLoading {}% \x1B[1A", percent);
        thread::sleep(time::Duration::from_millis(time/100));
    }

    thread::sleep(time::Duration::from_millis(1000));
    println!("Done!       ");
}