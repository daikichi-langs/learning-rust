pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longer(&st1, &st2);
    println!("{}\r\n", res1);

    let st3 = String::from("x");
    {
        let st4 = String::from("y");
        let res2 = get_longer(&st3, &st4);
        println!("{}\r\n", res2);
    }
}
fn get_longer<'a>(s: &'a str, ss: &'a str) -> &'a str {
    if s.len() > ss.len() {
        s
    } else {
        ss
    }
}
//
// fn is_rtn_err<'a>() -> &'a str {
//     let s = String::from("hello");
//     &s
// }
// fn is_rtn_err<'a>() -> &'a i32 {
//     let i = 10;
//     &i
// }
// fn is_rtn_ok() -> String {
//     let s = String::from("ok");
//     s
// }
