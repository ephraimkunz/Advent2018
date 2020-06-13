use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to open input file");

    // part1(&input);
    part2(&input);
}

fn part1(input: &str) -> usize {
    let mut char_vec: Vec<char> = Vec::from(input)
        .iter()
        .map(|i| *i as char)
        .filter(|c| c.is_alphabetic())
        .collect();
    let mut changes = true;

    while changes {
        changes = false;
        for i in 0..char_vec.len() - 1 {
            if is_opposite_polarity_pair(char_vec[i], char_vec[i + 1]) {
                char_vec.remove(i + 1);
                char_vec.remove(i);
                changes = true;
                break;
            }
        }
    }

    println!("{}", char_vec.len());
    char_vec.len()
}

fn part2(input: &str) {
    let mut shortest = input.len();
    for i in 65..=90 {
        let i = i as u8;
        let string_without = input.replace(&(i as char).to_string(), "");
        let string_without = string_without.replace(&((i + 32) as char).to_string(), "");
        let new_len = part1(&string_without);
        if new_len < shortest {
            shortest = new_len;
        }
    }

    println!("{}", shortest);
}

fn is_opposite_polarity_pair(a: char, b: char) -> bool {
    if a.is_lowercase() {
        return a.to_ascii_uppercase() == b;
    } else if b.is_lowercase() {
        return b.to_ascii_uppercase() == a;
    }

    false
}
