fn return_int(i: i32) -> i32 {
    i
}

fn get_first_word(str: &str) -> &str {
    let str_byte = str.as_bytes();

    for (i, &val) in str_byte.iter().enumerate() {
        if val == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}

fn get_second_word(str: &str) -> &str {
    let str_byte = str.as_bytes();

    for (i, &val) in str_byte.iter().enumerate() {
        if val == b' ' {
            return &str[i + 1..str.len() - 1];
        }
    }

    &str[..]
}

fn no_dangle() -> String {
    // return s with his ownership
    let s = String::from("generate a ownership in no_dangle"); // made s's ownership here
    s
}

fn fix_string(s: &mut String) {
    s.push_str("_fixed");
}

fn count_length(s: &str) -> usize {
    s.len()
}

fn main() {
    let mut s = String::from("Hello");
    let mut ss = s.clone(); // another heap zone s and ss
    fix_string(&mut ss);
    s.push_str(" World!");
    let no_dangle = no_dangle();

    let i: i32 = 22;
    // {
    //     let s = String::from("");
    // } // s in scopes drop at this point

    println!("s: {}, ss: {}, no_dangle: {}", s, ss, no_dangle);
    println!(
        "s length: {}, ss length: {}",
        count_length(&s),
        count_length(&ss)
    );
    println!(
        "first word of s: {}, second word of s: {}",
        get_first_word(&s),
        get_second_word(&s)
    );

    println!("{}", return_int(i));
}
