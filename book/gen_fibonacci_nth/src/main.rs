use std::collections::HashMap;
use std::env;

fn fibo(n: u64, m: &mut HashMap<u64, u64>) -> u64 {
    return match m.get(&n) {
        None => {
            let nn = fibo(n - 1, m) + fibo(n - 2, m);
            m.insert(n, nn);
            nn
        }
        _ => m[&n],
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let n = &args[1];
            let n: u64 = match n.parse() {
                Ok(nn) => nn,
                Err(_) => {
                    eprintln!("failed to parse.");
                    return;
                }
            };

            let mut memo: HashMap<u64, u64> = HashMap::new();
            memo.insert(0, 0);
            memo.insert(1, 1);
            println!("{}", fibo(n, &mut memo));
        }
        _ => eprintln!("Please enter an integer as argument."),
    }
}
