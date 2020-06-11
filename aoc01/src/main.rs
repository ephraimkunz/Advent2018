use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("src/input/input.txt").expect("Input file not found");
    let reader = io::BufReader::new(file);
    let freq = reader
        .lines()
        .map(|l| l.unwrap_or(String::from("0")).parse::<i32>().unwrap_or(0))
        .fold(0, |accum, elem| accum + elem);
    println!("{}", freq);
}
