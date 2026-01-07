use std::cmp::Ordering;
use colored::*;
use std::{thread, time::Duration};

pub const BUILDING_TIME: u64 = 0;

pub fn print_tree(number: i32, iter: u8) {
    let mut count = iter;
    for i in 1..=number {
        let dot = match count % 2 {
            0 => ".".red(),
            1 => ".".green(),
            _ => std::process::exit(1),
        };
        for _j in (-(number-i)..=-1).rev() { print!(" "); }
        for _j in 1..=i { print!("{}", dot); }
        for _j in 1..=(i-1) { print!("{}", dot) };
        println!();
        count+=1;
        sleepy(BUILDING_TIME);
    }
    let half: i32 = match number % 2 {
        0 => number / 2,
        1 => (number-1) / 2,
        _ => std::process::exit(1),
    };

    let green = ".".green();
    let amount = match number.cmp(&10) {
        Ordering::Less => 1,
        Ordering::Equal => 3,
        Ordering::Greater => 5,
    };
    let spacer = match number.cmp(&10) {
        Ordering::Less => 1,
        Ordering::Equal => 2,
        Ordering::Greater => 3,
    };
    for _i in 1..=half {
        for _i in 1..=(number-spacer) { print!(" "); }
        for _i in 1..=amount { print!("{}", green); }
        println!();
        sleepy(BUILDING_TIME);
    }
}

pub fn print_circle(number: i32) {
    let _count = 0;
    for _i in 1..=number {
        for _j in 1..=number { 
            print!(". "); 
        }
        println!();
        sleepy(BUILDING_TIME);
    }
}



pub fn sleepy(time: u64) {
    thread::sleep(Duration::from_millis(time));
}
