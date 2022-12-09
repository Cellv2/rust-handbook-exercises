// use std::io::stdin;

fn main() {
    println!("Please enter a number to convert");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read from stdin");

    // trim is the important bit
    match input.trim().parse::<i32>() {
        Ok(num) => {
            println!("Celsius to Farenheit: {}", c_to_f(num));
            println!("Farenheit to Celsius: {}", f_to_c(num));
        }
        Err(_e) => {
            println!("Value was not a number! Please try again")
        }
    }
}

fn c_to_f(num: i32) -> i32 {
    (num * 9 / 5) + 32
}

fn f_to_c(num: i32) -> i32 {
    (num - 32) * 5 / 9
}
