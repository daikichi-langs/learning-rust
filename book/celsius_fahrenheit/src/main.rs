use std::env;

fn convert(degree: f64, unit: &String) {
    let deg;
    let uni;
    if unit == "c" {
        deg = (degree * 9.0 / 5.0) + 32.0;
        uni = "f";
    } else {
        deg = (5.0 / 9.0) * (degree - 32.0);
        uni = "c";
    }
    println!("{}{} near equals {}{}", degree, unit, deg, uni);
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
            if &unit[..] == "f" || &unit[..] == "c" {
                convert(number, &unit)
            } else {
                eprintln!("error: second argument must be \"c\" or \"f\"");
            };
        }
        _ => {
            help();
        }
    }
}
