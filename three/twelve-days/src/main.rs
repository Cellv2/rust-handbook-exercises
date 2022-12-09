fn main() {
    for num in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            ORDINALS[num]
        );

        // the verse changes if it's the first iteration (there's no 'and' at the front)
        if num == 0 {
            println!("A partridge in a pear tree")
        } else {
            for num in (0..num + 1).rev() {
                println!("{}", VERSES[num])
            }
        }
    }
}

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const VERSES: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
