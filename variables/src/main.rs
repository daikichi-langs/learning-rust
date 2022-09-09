#![allow(unused)]

use std::io;

const MAX_POINT: u32 = 100_000;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // let mut spaces = "    ";
    // spaces = spaces.len();
    let spaces: &str = "    ";
    let spaces: usize = spaces.len();

    // let guess = "11".parse().expect("Not a number.");
    let guess: i32 = "11".parse().expect("Not a number.");
    println!("the guess is: {}", guess);

    let f = 2.0; // f64 basically
    let f: f32 = 2.0; // f32

    let sum: i32 = 7 + 11;
    let difference: f64 = 7.7 - 11.11;
    let product: i32 = 7 * 11;
    let quotient: f64 = 11.1 / 77.7;
    let floored: i32 = 2 / 3; // 0
    let remainder: i32 = 11 % 7;

    let _t = true;
    let _t: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat: char = '😻';

    let tup: (i16, f32, i64) = (3, 5.0, 7);
    let (x, y, z) = tup;
    println!("The values of tuple are: ({}, {}, {})", x, y, z);
    let x = tup.0;
    println!("The value of x is: {}", x);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let arr: [i32; 6] = [0, 1, 2, 3, 4, 5];

    println!("Please enter array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index.trim().parse().expect("Index entered not a number.");

    let element = months[index]; // if index => month.len(),  go to panic
    println!(
        "The value of element at index {} in months: {}",
        index, element
    );
}
