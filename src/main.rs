use colored::*;
use rand::{Rng, prelude::ThreadRng, thread_rng};
use terminal_size::{terminal_size};
use std::char;
use std::{thread, time};
use std::env;

fn random_line(width: u16, mut rng: ThreadRng) -> String {
    let mut res = String::new();

    for _ in 0..width {
        let print_or_not = rng.gen_range(0, 3) == 1 ;
        let n = rng.gen_range(0, 2);

        if print_or_not {
            res.push(char::from_digit(n, 10).unwrap());
        } else {
            res.push(' ');
        }
        
    }

    res
}

static DEFAULT_DELAY: u64 = 25;

fn main() {

    // Get terminal's width
    let size = terminal_size().unwrap();
    let width =  size.0;

    // Create a random generator
    let rng = thread_rng();

    let mut args = env::args();

    // Skip binary
    args.next();

    let delay_arg = if let Some(delay_arg) = args.next() {
        if let Ok(delay_arg) = delay_arg.parse::<u64>() {
            delay_arg
        } else {
            DEFAULT_DELAY
        }
    } else {
        DEFAULT_DELAY
    };

    let delay = time::Duration::from_millis(delay_arg);

    loop {
        println!("{}", random_line(width.0, rng).green());

        thread::sleep(delay);
    }
}
