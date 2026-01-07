use std::io;
use std::{thread, time::Duration};

pub mod term_print;

pub const FLASHING_TIME: u64 = 200;

fn main() {
    println!("CLI spin");

    let mut pass: bool = false;
    let mut f_uinput: i32 = 0;
    let mut f_choice: u16 = 0;

    while pass==false {

        println!("What object?");
        println!("1. Square");
        println!("2. Tree");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("No");

        let choice: u16 = match choice.trim().parse::<u16>() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a number");
                continue;
            },
        };
        if choice > 2 {
                println!("That wasn't a valid number");
                continue
        }

        println!("How big do you want to object?");
        let mut uinput = String::new();
        io::stdin().read_line(&mut uinput).expect("No");

        // Check for number
        let uinput: i32 = match uinput.trim().parse::<i32>() {
            Ok(num) => {
                pass = true;
                num
            },
            Err(_) => {
                println!("That wasn't a number");
                continue;
            },
        };

        f_uinput = uinput;
        f_choice = choice;

    }

    let mut iter: u8 = 1;
    match f_choice {
        1 => { term_print::print_circle(f_uinput) },
        2 => {
            loop {
                print!("{esc}c", esc = 27 as char);
                term_print::print_tree(f_uinput, iter);
                sleepy(FLASHING_TIME);
                iter += 1;
            }
        }
        _ => print!("Error"),
    }

}


pub fn sleepy(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

