use std::collections::HashSet;
use std::fs;

fn main() {
    let file_string = fs::read_to_string("src/input/input.txt").expect("Couldn't read file");
    part1(&file_string);
    part2(&file_string);
}

fn part1(string: &str) {
    let freq = string
        .lines()
        .map(|l| l.parse::<i32>().unwrap_or(0))
        .fold(0, |accum, elem| accum + elem);
    println!("{}", freq);
}

fn part2(string: &str) {
    let mut set = HashSet::new();
    let mut sum = 0;
    set.insert(sum);
    loop {
        for l in string.lines() {
            let i = l.parse().unwrap_or(0);
            sum += i;
            if set.contains(&sum) {
                println!("{}", sum);
                return;
            } else {
                set.insert(sum);
            }
        }
    }
}
