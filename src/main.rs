use std::{thread, time, io};

fn main() {
    let mut percent: i32 = 0;

    println!("Enter time in milliseconds: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: u64 = time.trim().parse().expect("Please type a number!");


    print!("Loading ----- (0%");

    while percent < 100 {
        percent += 1;
        if percent == 100 {
            println!("\rLoading ===== ({}%)\x1B[1A", percent);
        }
        else if percent > 80 {
            println!("\rLoading ====- ({}%)\x1B[1A", percent);
        }
        else if percent > 60 {
            println!("\rLoading ===-- ({}%)\x1B[1A", percent);
        }
        else if percent > 40 {
            println!("\rLoading ==--- ({}%)\x1B[1A", percent);
        }
        else if percent > 20 {
            println!("\rLoading =---- ({}%)\x1B[1A", percent);
        }
        else {
            println!("\rLoading ----- ({}%)\x1B[1A", percent);
        }
        thread::sleep(time::Duration::from_millis(time/100));
    }

    thread::sleep(time::Duration::from_millis(100));
}