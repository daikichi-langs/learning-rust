pub fn run() {
    // let s1 = String::from("hello");
    // let s2 = s1; // Owner ship moves s1 to s2.
    // println!("{} {}",s1, s2); // Can not outputs s1

    let i1: i32 = 1;
    let i2: i32 = i1; // case int, implement copy trt
    println!(
        "i1: {} i2: {}\r\nStack address are i1: {:p} i2: {:p}\r\n",
        i1, i2, &i1, &i2
    );

    // &str has reference.
    let sl1: &str = "literal";
    let sl2: &str = sl1;
    println!(
        "Values -> sl1: {} sl2: {}.\r\nStack address -> sl1: {:p} al2: {:p}\r\n",
        sl1, sl2, &sl1, &sl2,
    );

    let s3: String = String::from("hello");
    let s4: String = s3.clone();
    println!(
        "Values -> s3: {} s4: {}\r\nStack addresses -> s3: {:p} s4: {:p}\r\nHeap addresses -> s3 {:p} s4 {:p}\r\n",
        s3, s4, &s3, &s4, s3.as_ptr(), s4.as_ptr(),
    );
    let s5: String = String::from("hello");
    println!("s5 value -> {}\r\ns5 Stack address -> {:p}\r\ns5 heap address -> {:p}\r\ns5 len -> {}\r\ns5 capacity -> {}\r\n",
    s5, &s5,s5.as_ptr(), s5.len(),s5.capacity()
    );
    take_ownership(s5); // when called owner_ship(), ownership moved.
                        //println!("{}", s5); // s5 ownership does not exit.

    let s6 = String::from("hello");
    println!(
      "s6 value -> {}\r\ns6 Stack address -> {:p}\r\ns6 heap address -> {:p}\r\ns6 len -> {}\r\ns6 capacity -> {}\r\n",
      s6,
      &s6,
      s6.as_ptr(),
      s6.len(),
      s6.capacity()
    );
    let s7 = take_giveback_ownership(s6);
    println!(
      "s7 value -> {}\r\ns7 Stack address -> {:p}\r\ns7 heap address -> {:p}\r\ns7 len -> {}\r\ns7 capacity -> {}\r\n",
      s7,
      &s7,
      s7.as_ptr(),
      s7.len(),
      s7.capacity()
    );

    let s8 = String::from("hello");
    let len = len_str(&s8);
    println!("s8 val -> {}\r\ns8 len -> {}", s8, len);

    let mut s9 = String::from("hello");
    push_world(&mut s9);
    println!("s9: {}\r\n", s9);

    // allow to make references by immutable.
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("s10: {}, r1: {}, r2: {}\r\n", s10, r1, r2);

    // no coexist mutable reference and immutable reference.
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {} {}\r\n", s10, r1, r2)
    let mut s11 = String::from("hello");
    let r3 = &mut s11;
    // println!("{}\r\n", s11);
    // println!("{}\r\n", r3);
    println!("{}\r\n", r3);
    println!("{}\r\n", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("r1: {}, r2{}\r\n", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello update.");
    println!("r3: {}\r\n", r3);
}

fn take_ownership(s: String) {
    // s get arg ownership.
    println!("{}\r\n", s);
    println!(
        "s value -> {}\r\ns Stack address -> {:p}\r\ns heap address -> {:p}\r\ns len -> {}\r\ns capacity -> {}\r\n",
        s,
        &s,
        s.as_ptr(),
        s.len(),
        s.capacity()
    );
    // when breaking this fn, drop ownership.
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn len_str(s: &String) -> usize {
    s.len()
}

fn push_world(s: &mut String) {
    s.push_str(" world.");
}
