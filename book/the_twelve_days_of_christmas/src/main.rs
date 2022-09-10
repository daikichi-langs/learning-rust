fn parse(n: usize) -> String {
    return match n {
        1 => (String::from("first")),
        2 => (String::from("second")),
        3 => (String::from("third")),
        4 => (String::from("fourth")),
        5 => (String::from("fifth")),
        6 => (String::from("sixth")),
        7 => (String::from("seventh")),
        8 => (String::from("eighth")),
        9 => (String::from("ninth")),
        10 => (String::from("tenth")),
        11 => (String::from("eleventh")),
        12 => (String::from("twelfth")),
        _ => (String::from("_")),
    };
}

fn main() {
    let lyrics = [
        "and a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for i in 0..lyrics.len() {
        let alias = parse(i + 1);
        println!(
            "On the {} day of Christmas\r\nmy true love sent to me:",
            alias
        );
        for j in 0..i + 1 {
            println!("{}", lyrics[i - j]);
        }
        println!("");
    }
}
