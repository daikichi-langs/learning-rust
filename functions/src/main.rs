fn main() {
    another_function(5);
    print_labeled_measurement(7, 'H');

    let y = {
        let x = 5;
        x + 7
    };
    println!("The value of x is {}", y);

    println!("Five return: {}", five());
}
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
fn print_labeled_measurement(val: i32, label: char) {
    println!("The measurement is: {}{}", val, label);
}
fn five() -> i32 {
    5
}
