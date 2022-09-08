enum List {
    Node(i32, Box<List>), // Node() sizes are define (i32: 4 bytes, box pointer: 8 bytes)
    Nil,                  // nil: 0 bytes
}

pub fn run() {
    // let arr1: [u8; 7000000] = [1; 7000000]; // 7MB

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7];
    let mut v3 = vec![8, 9, 10];

    println!(
        "v1 stack address stand on {:p}, v2 stack address stand on {:p}, v3 stack address stand on {:p}",
        &v1, &v2, &v3
    );
    println!(
        "Heap memory address of v1 is {:?}, Len of v1 is {}, Capacity of v1 is {}",
        v1.as_ptr(),
        v1.len(),
        v1.capacity()
    );
    v1.insert(1, 10);
    println!(
        "v1 value is {:?}, Heap memory address of v1 is {:?}, Len of v1 is {}, Capacity of v1 is {}",
        v1,
        v1.as_ptr(),
        v1.len(),
        v1.capacity()
    );
    v1.remove(1);
    println!(
        "v1 value is {:?}, Heap memory address of v1 is {:?}, Len of v1 is {}, Capacity of v1 is {}",
        v1,
        v1.as_ptr(),
        v1.len(),
        v1.capacity()
    );
    v1.append(&mut v3);
    println!(
        "v1 value is {:?}, Heap memory address of v1 is {:?}, Len of v1 is {}, Capacity of v1 is {}",
        v1,
        v1.as_ptr(),
        v1.len(),
        v1.capacity()
    );
    println!(
        "v3 value is {:?}, Stack address is {:p}, Heap memory address of v3 is {:?}, Len of v3 is {}, Capacity of v3 is {}",
        v3,
        &v3,
        v3.as_ptr(),
        v3.len(),
        v3.capacity()
    );

    let t1: (i64, String) = (10, String::from("hello"));
    println!(
        "Stack address of t1 is {:p}, t1.1 reference {:?} on Heap, t1.1 has {} Length, {} capacity.",
        &t1,
        t1.1.as_ptr(),
        t1.1.len(),
        t1.1.capacity()
    );

    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!(
        "b1 located {:p} on Stack, and has {:p} as a value its heap address of t1.\r\ntherefore it can access to t1 values, such as b1.0 is \"{}\", b1.1 is \"{}\" like this.",
        &b1, b1, b1.0, b1.1
    );
}
