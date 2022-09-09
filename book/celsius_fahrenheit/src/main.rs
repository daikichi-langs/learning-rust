use std::env;

fn convert(num: f64, unit: &String) {
    if unit == "c" {
        let degree = (num * 9.0 / 5.0) + 32.0;
        println!("{}{} near equals {}f", num, unit, degree);
    } else {
        let degree = (5.0 / 9.0) * (num - 32.0);
        println!("{}{} near equals {}c", num, unit, degree);
    }
}
fn help() {
    println!("usage: cargo run [Integer] [\"s\" or \"f\"]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Please enter two arguments.");
            help();
        }
        2 => {
            println!("argument must be two values.");
            help();
        }
        3 => {
            let num = &args[1];
            let unit = &args[2];

            let number: f64 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: first argument is not an integer.");
                    help();
                    return;
                }
            };
            match &unit[..] {
                "c" => convert(number, &unit),
                "f" => convert(number, &unit),
                _ => {
                    eprintln!("error: second argument must be \"c\" or \"f\"");
                }
            }
        }
        _ => {
            help();
        }
    }
}
