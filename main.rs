use days::day01;
use days::day02;
use days::day03;
mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
}
use std::io;

fn main() {
    let mut input = String::new();
    println!("Daily code to run: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.as_str().trim()
    {
        "1" => day01::main(),
        "2" => day02::main(),
        "3" => day03::main(),
        _ => println!("Unexpected value: '{0}'", input.as_str().trim())
    };
}