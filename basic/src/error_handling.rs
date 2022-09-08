pub fn run() {
    let res = division_option(5.0, 0.0);
    match res {
        Some(a) => println!("{:.3}", a),
        None => println!("Not Allowed."),
    }
    let res = division_result(0.0, 2.0);
    match res {
        Ok(r) => println!("{:.3}", r),
        Err(r) => println!("{}", r),
    }

    let nums = [0, 1];
    let res = sum(&nums);
    match res {
        Some(r) => println!("{}", r),
        None => println!("Out Of Index."),
    }
}
fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not Allowed."))
    } else {
        Ok(x / y)
    }
}
fn sum(nums: &[i32]) -> Option<i32> {
    let num0 = nums.get(0)?;
    let num1 = nums.get(1)?;
    let num2 = nums.get(2)?;

    Some(num0 + num1 + num2)
}
