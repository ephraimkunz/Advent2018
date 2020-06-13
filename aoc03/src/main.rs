use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

type Grid = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Claim {
    id: i32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,

    iter_idx: u32,
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("unrecognized claim string")),
            Some(caps) => caps,
        };

        Ok(Claim {
            id: captures["id"].parse()?,
            x: captures["x"].parse()?,
            y: captures["y"].parse()?,
            width: captures["width"].parse()?,
            height: captures["height"].parse()?,
            iter_idx: 0,
        })
    }
}

impl Claim {
    fn iter_points(&self) -> IterPoints {
        IterPoints{
            index: 0,
            claim: self,
        }
    }
}

struct IterPoints<'a> {
    index: u32,
    claim: &'a Claim,
}

impl<'a> Iterator for IterPoints<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let num_items = self.claim.width * self.claim.height;

        if self.index == num_items {
            self.index = 0; // Restart the iterator
            return None;
        }

        let idx = self.index;
        self.index += 1;

        let x = idx % self.claim.width + self.claim.x;
        let y = idx / self.claim.width + self.claim.y;
        Some((x, y))
    }
}

fn main() {
    let string = fs::read_to_string("src/input.txt").unwrap();
    let claims: Vec<Claim> = string.lines().map(|l| l.parse().unwrap()).collect();

    let mut grid = Grid::new();
    for claim in &claims {
        for point in claim.iter_points() {
            *grid.entry(point).or_default() += 1;
        }
    }

    part1(&grid);
    part2(&grid, &claims);
}

fn part1(grid: &Grid) {
    let res = grid.values().filter(|&&v| v > 1).count();
    println!("{}", res);
}

fn part2(grid: &Grid, claims: &[Claim]) {
    for claim in claims {
        if claim.iter_points().all(|p| grid[&p] == 1) {
            println!("{}", claim.id);
            return;
        }
    }
}
