#![feature(linked_list_cursors)]

use regex::Regex;
use std::collections::linked_list::{CursorMut, LinkedList};
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive (Clone)]
struct Game {
    num_players: u32,
    num_marbles: u32,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let r = Regex::new(
            r"(?P<players>\d+) players; last marble is worth (?P<num_marbles>\d+) points",
        )
        .expect("Invalid regex");

        let caps = r.captures(s).expect("Couldn't parse input");

        Ok(Game {
            num_players: caps["players"].parse()?,
            num_marbles: caps["num_marbles"].parse()?,
        })
    }
}

fn next_no_ghost(cursor: &mut CursorMut<u32>) {
    if let None = cursor.peek_next() {
        cursor.move_next();
    }
    cursor.move_next();
}

fn prev_no_ghost(cursor: &mut CursorMut<u32>) {
    if let None = cursor.peek_prev() {
        cursor.move_prev();
    }
    cursor.move_prev();
}

impl Game {
    fn run(&self) -> u32 {
        let mut circle = LinkedList::new();
        let mut current = circle.cursor_front_mut();
        current.insert_after(0);
        current.move_next();

        let mut player_scores = vec![0; self.num_players as usize];
        let mut current_player = 0;
        for marble_id in 1..self.num_marbles {
            if marble_id % 23 == 0 {
                let mut score = marble_id;
                for _ in 0..7 {
                    prev_no_ghost(&mut current);
                }

                let removed = current.remove_current();
                if let Some(removed) = removed {
                    score += removed;
                }

                player_scores[current_player as usize] += score;
            } else {
                next_no_ghost(&mut current);
                current.insert_after(marble_id);
                next_no_ghost(&mut current);
            }

            current_player = (current_player + 1) % self.num_players;
        }

        *player_scores.iter().max().unwrap()
    }
}

fn main() {
    let s = fs::read_to_string("src/input.txt").expect("Couldn't read input");
    let game = s.parse().unwrap();

    part1(&game);
    part2(&game);
}

fn part1(game: &Game) {
    println!("{}", game.run());
}

fn part2(game: &Game) {
    let mut g = game.clone();
    g.num_marbles *= 100;
    println!("{}", g.run());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9_25() {
        let game = Game {
            num_players: 9,
            num_marbles: 25,
        };

        assert_eq!(32, game.run());
    }

    #[test]
    fn test_13_7999() {
        let game = Game {
            num_players: 13,
            num_marbles: 7999,
        };

        assert_eq!(146373, game.run());
    }
}
