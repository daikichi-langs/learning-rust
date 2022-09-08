// pub mod sub_a;
// mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars.");
    // sub_a::run();
    // sub_b::run();
    let mut x = 5;
    println!("Value {}", x);
    x = 6;
    println!("Value {}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    let i2: i64 = 1;
    let i3: i64 = 3;

    let y = 5;
    let y = y + 1;

    println!("{}", usize::BITS);
    println!("Memory address of const is {:p}", &MAX_POINTS);

    println!("Stack address of i32 is {:p}", &i2);
    println!("Stack address of i32 is {:p}", &i3);

    println!("Stack address is {:p}", &y);
    let y = y * 2;
    println!("y is {}", y);
    {
        let y = 0;
        println!("Local y is {}", y);
    }
    println!("y out of scope {}", y);

    let t1 = (0, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("t1 values are {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut x2_ptr), _) = t2;
    *x1_ptr = 5;
    *x2_ptr = -5;
    println!("{:?}", t2);

    let arr1 = [0, 1, 2, 3, 4, 5];
    let arr2 = [0; 10];
    println!("{:?} {:?} {} {}", arr1, arr2, arr1[3], arr1[4]);

    let str1 = "Helloこんにちは挨拶"; // 1bytes*5 + 3bytes * 7 = 26bytes
    let str2 = "Hello";
    println!("Stack {:p} {:p}", &str1, &str2); // 16の差分 1bytes/1番地
    println!(
        "Static memory address of str1 is {:?}, length is {} bytes.",
        str1.as_ptr(),
        str1.len()
    );
    println!(
        "Static memory address of str2 is {:?}, length is {} bytes.",
        str2.as_ptr(),
        str2.len()
    );

    let mut s1 = String::from("helloこんにちは");
    let mut s2 = String::from("helloworld");
    println!(
        "Stack address of s1 is {:p}, heap address is {:?}, length is {}, capacity is {}",
        &s1,
        s1.as_ptr(),
        s1.len(),
        s1.capacity()
    );
    println!(
        "Stack address of s2 is {:p}, heap address is {:?}, length is {}, capacity is {}",
        &s2,
        s2.as_ptr(),
        s2.len(),
        s2.capacity()
    );

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} / {}", s1, s2);
}
