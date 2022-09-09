fn main() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        println!("count is: {}", count);

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End of count {}", count);

    let mut i = 3;
    while i > 0 {
        println!("{}", i);
        i -= 1;
    }

    let arr = [0, 1, 2, 3, 4];
    for a in arr {
        println!("{}", a);
    }
    for a in arr.iter().rev() {
        println!("{}", a);
    }
}
