use std::fs;

fn main() {
    let string = fs::read_to_string("src/input.txt").expect("Couldn't open file");
    part1(&string);
    part2(&string);
}

fn part1(string: &str) {
    let count2: i32 = string.lines().map(|e| if word_has_exactly_x_of_letter(e, 2) {1} else {0}).sum();
    let count3: i32 = string.lines().map(|e| if word_has_exactly_x_of_letter(e, 3) {1} else {0}).sum();
    println!("{}", count2 * count3);
}

fn word_has_exactly_x_of_letter(word: &str, x: i32) -> bool {
    let mut chars: Vec<_> = word.chars().collect();
    chars.sort();

    let mut count = 0;
    let mut current = chars[0];
    for char in chars.iter() {
        if *char != current {
            if count == x {
                return true;
            }
            
            current = *char;
            count = 0;
        }

        count += 1;
    }

    if count == x {
        return true;
    }

    false
}

fn part2(string: &str) {
    let lines: Vec<_> = string.lines().collect();
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if lines[i].chars().zip(lines[j].chars()).map(|e| if e.0 == e.1 {0} else {1}).sum::<i32>() == 1 {
                let mut res = String::new();
                for c in 0..lines[i].len() {
                    if lines[i].chars().nth(c).unwrap() == lines[j].chars().nth(c).unwrap() {
                        res.push(lines[i].chars().nth(c).unwrap());
                    }
                }
                println!("{}", res);
                return;
            }
        }
    }
}
