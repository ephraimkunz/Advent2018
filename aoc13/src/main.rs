use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

fn main() {
    let s = fs::read_to_string("src/input.txt").unwrap();
    let w: World = s.parse().unwrap();
    part1(w.clone());
    part2(w.clone());
}

fn part1(mut w: World) {
    loop {
        if let Some(crash_loc) = w.tick(false) {
            println!("{:?}", crash_loc);
            return;
        }
    }
}

fn part2(mut w: World) {
    loop {
        if let Some(final_loc) = w.tick(true) {
            println!("{:?}", final_loc);
            return;
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    w: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl World {
    // Return value is the location of the first crash if it happened in this tick.
    fn tick(&mut self, continue_after_collision: bool) -> Option<(usize, usize)> {
        self.carts.sort();
        let mut crashed = HashSet::new();

        for i in 0..self.carts.len() {
            if crashed.contains(&i) {
                continue;
            }

            let mut carts_cloned = vec![Cart::default(); self.carts.len()];
            carts_cloned.clone_from_slice(&self.carts);
            let mut cart = self.carts.get_mut(i).unwrap();

            let diff = cart.orientation.diff();
            let next_map_coord = (
                (cart.r as i32 + diff.0) as usize,
                (cart.c as i32 + diff.1) as usize,
            );
            let next_map = self.w[next_map_coord.0][next_map_coord.1];

            // Handle collision, turning, or orientation changes.
            cart.r = next_map_coord.0;
            cart.c = next_map_coord.1;

            let collision = carts_cloned.iter().any(|c| next_map_coord == (c.r, c.c));
            if collision {
                if continue_after_collision {
                    // Find crashing carts, remove after.
                    crashed.insert(i);
                    for j in 0..carts_cloned.len() {
                        if next_map_coord == (carts_cloned[j].r, carts_cloned[j].c) {
                            crashed.insert(j);
                        }
                    }
                    continue;
                } else {
                    return Some((cart.c, cart.r));
                }
            }

            match next_map {
                '/' => {
                    cart.orientation = match cart.orientation {
                        Orientation::Left | Orientation::Right => {
                            cart.orientation.turn(&Turn::Left)
                        }

                        Orientation::Up | Orientation::Down => cart.orientation.turn(&Turn::Right),
                    };
                }
                '\\' => {
                    cart.orientation = match cart.orientation {
                        Orientation::Left | Orientation::Right => {
                            cart.orientation.turn(&Turn::Right)
                        }
                        Orientation::Up | Orientation::Down => cart.orientation.turn(&Turn::Left),
                    };
                }
                '+' => {
                    cart.orientation = cart.orientation.turn(&cart.next_turn);
                    cart.next_turn = cart.next_turn.next();
                }
                '-' => assert!(
                    cart.orientation == Orientation::Left || cart.orientation == Orientation::Right
                ),
                '|' => assert!(
                    cart.orientation == Orientation::Up || cart.orientation == Orientation::Down
                ),
                _ => panic!("Unexpected"),
            };
        }

        if continue_after_collision {
            let mut to_remove: Vec<&usize> = crashed.iter().collect();
            to_remove.sort();
            for &&i in to_remove.iter().rev() {
                self.carts.remove(i);
            }

            if self.carts.len() == 1 {
                Some((self.carts[0].c, self.carts[0].r))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromStr for World {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut w: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        let mut carts = vec![];
        for r in 0..w.len() {
            for c in 0..w[0].len() {
                if let Ok(orientation) = Orientation::try_from(w[r][c]) {
                    w[r][c] = orientation.replacement_track();
                    carts.push(Cart {
                        next_turn: Turn::default(),
                        orientation,
                        r,
                        c,
                    });
                }
            }
        }

        Ok(World { w, carts })
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Up
    }
}

impl Orientation {
    fn replacement_track(&self) -> char {
        match self {
            Orientation::Left | Orientation::Right => '-',
            _ => '|',
        }
    }

    fn diff(&self) -> (i32, i32) {
        match self {
            Orientation::Left => (0, -1),
            Orientation::Right => (0, 1),
            Orientation::Up => (-1, 0),
            Orientation::Down => (1, 0),
        }
    }

    fn turn(&self, turn: &Turn) -> Self {
        if *turn == Turn::Straight {
            return self.clone();
        }

        match self {
            Orientation::Left => {
                if *turn == Turn::Left {
                    Orientation::Down
                } else {
                    Orientation::Up
                }
            }
            Orientation::Right => {
                if *turn == Turn::Left {
                    Orientation::Up
                } else {
                    Orientation::Down
                }
            }
            Orientation::Up => {
                if *turn == Turn::Left {
                    Orientation::Left
                } else {
                    Orientation::Right
                }
            }
            Orientation::Down => {
                if *turn == Turn::Left {
                    Orientation::Right
                } else {
                    Orientation::Left
                }
            }
        }
    }
}

impl TryFrom<char> for Orientation {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            'v' => Ok(Orientation::Down),
            '^' => Ok(Orientation::Up),
            '<' => Ok(Orientation::Left),
            '>' => Ok(Orientation::Right),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
enum Turn {
    Left,
    Right,
    Straight,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Turn::Left
    }
}

#[derive(Eq, Debug, Clone, Default)]
struct Cart {
    orientation: Orientation,
    next_turn: Turn,
    r: usize,
    c: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        let c_comp = self.c.cmp(&other.c);
        if c_comp == Ordering::Equal {
            return self.r.cmp(&other.r);
        } else {
            c_comp
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.c == other.c
    }
}
