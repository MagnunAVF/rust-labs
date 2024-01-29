// lyrics from: https://www.letras.mus.br/natal/1050132/

use std::collections::HashMap;

fn mount_segment(day: u8, days: &HashMap<u8, &str>, quotes: &HashMap<u8, &str>) -> String {
    let mut result = String::new();

    let day_str = match days.get(&day) {
        Some(value) => value,
        None => "unknown",
    };
    let intro = format!("On the {} day of Christmas,\n", day_str);
    result.push_str(&intro);
    result.push_str("My true love sent to me:\n");

    for d in (1..=day).rev() {
        let quote = match quotes.get(&d) {
            Some(value) => value,
            None => "none",
        };
        result.push_str(quote);
        result.push('\n');
    }

    result
}

fn main() {
    println!("### The Twelve Days of Christmas ###\n");

    let days: HashMap<u8, &str> = HashMap::from([
        (1, "first"),
        (2, "second"),
        (3, "third"),
        (4, "fourth"),
        (5, "fifth"),
        (6, "sixth"),
        (7, "seventh"),
        (8, "eighth"),
        (9, "ninth"),
        (10, "tenth"),
        (11, "eleventh"),
        (12, "twelfth"),
    ]);

    let quotes_of_the_day: HashMap<u8, &str> = HashMap::from([
        (1, "A partridge in a pear tree."),
        (2, "Two turtle doves,"),
        (3, "Three French Hens,"),
        (4, "Four calling birds,"),
        (5, "Five golden rings,"),
        (6, "Six geese a laying,"),
        (7, "Seven swans a swimming,"),
        (8, "Eight maids a milking,"),
        (9, "Nine ladies dancing,"),
        (10, "Ten lords a leaping,"),
        (11, "Eleven pipers piping,"),
        (12, "Twelve drummers drumming,"),
    ]);

    let mut lyrics = String::new();
    for day in 1..=12 {
        lyrics.push_str(&mount_segment(day, &days, &quotes_of_the_day));
        lyrics.push('\n');
    }

    println!("{}", lyrics);
}
