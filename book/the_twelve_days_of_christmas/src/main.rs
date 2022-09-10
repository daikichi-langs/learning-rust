fn parse(n: usize) -> &'static str {
    return match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "_",
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
