use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("guessing number game. please input your guess.");

    let the_num: u32 = rand::thread_rng().gen_range(1..100);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        match input.cmp(&the_num) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You hit!");
                break;
            }
        }
    }
}
