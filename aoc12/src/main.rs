use std::collections::HashMap;

const RULE_LEN: usize = 5;

fn main() {
    let s = "##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#";
    let rules = "#.#.. => .
    ..##. => .
    ...#. => .
    ..#.. => .
    ##### => #
    .#.#. => .
    ####. => .
    ###.. => .
    .#..# => #
    #..#. => #
    #.#.# => .
    #...# => #
    ..### => .
    ...## => #
    ##..# => #
    #.... => .
    .#.## => #
    #.### => #
    .##.# => #
    #..## => .
    .#... => #
    .###. => .
    ##... => #
    ##.## => #
    ##.#. => #
    #.##. => #
    .##.. => .
    ..#.# => .
    ....# => .
    ###.# => .
    ..... => .
    .#### => .";

    let rules = parse_rules(&rules);
    part1(&rules, s);
}

fn parse_rules(s: &str) -> HashMap<[bool; 5], bool> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let line = line.trim().as_bytes();
        let mut key = [false; 5];
        for i in 0..line.len() {
            if i < RULE_LEN {
                key[i] = line[i] == b'#';
            } else if i == RULE_LEN + 4 {
                map.insert(key, line[i] == b'#');
            }
        }
    }

    map
}

fn part1(rules: &HashMap<[bool; 5], bool>, state: &str) {
    let state = state.as_bytes();

    const ARRAY_LEN: usize = 1_000_000;

    // Arbitrarily large array for state.
    let mut pots = [false; ARRAY_LEN];
    let zero: i32 = (ARRAY_LEN / 2) as i32;
    for i in 0..state.len() {
        pots[i + zero as usize] = state[i] == b'#';
    }

    for _ in 0..500 {
        let mut next_round = [false; ARRAY_LEN];
        next_round.copy_from_slice(&pots);

        let rule_half = RULE_LEN / 2;
        for i in (rule_half)..(pots.len() - rule_half - 1) {
            let matching_pattern = rules[&pots[(i - rule_half)..(i + rule_half + 1)]];
            next_round[i] = matching_pattern;
        }

        pots = next_round;
    }

    let mut sum = 0;
    for i in 0..pots.len() {
        let real_index = i as i32 - zero;
        if pots[i] {
            sum += real_index;
        }
    }

    println!("{}", sum);
}
