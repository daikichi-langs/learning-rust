fn main() {
    let num = 5;
    if num != 5 {
        println!("The condition is true.");
    } else {
        println!("The condition is false.");
    }
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3 or 2.");
    }

    let number = if num == 5 { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
