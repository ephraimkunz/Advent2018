
use rayon::prelude::*;
use std::cmp::Ordering;

struct Grid {
    g: [[i32; Grid::N]; Grid::N]
}

impl Grid {
    const N: usize = 300;

    fn new(serial: i32) -> Grid {
        let mut g = Grid {
            g: [[0; Grid::N]; Grid::N]
        };

        for r in 0..Grid::N {
            for c in 0..Grid::N {
                let rack_id = (c + 1 + 10) as i32;
                let mut power_level = rack_id * (r as i32 + 1);
                power_level += serial;
                power_level *= rack_id;
                
                let hundreds_digit = (power_level / 100) % 10;
                power_level = hundreds_digit - 5;
                g.g[r][c] = power_level;
            }
        }

        g
    }

    fn max_square(&self, size: usize) -> (usize, usize, i32) {
        let mut max_power = i32::MIN;
        let mut max_coord = (0, 0);
        for r in 0..(Grid::N - size + 1) {
            for c in 0..(Grid::N - size + 1) {
                let mut power = 0;
                for g_r in 0..size {
                    for g_c in 0..size {
                        power += self.g[r + g_r][c + g_c];
                    }
                }

                if power > max_power {
                    max_power = power;
                    max_coord = (c + 1, r + 1);
                }
            }
        }

        (max_coord.0, max_coord.1, max_power)
    }

    fn total_power(&self, size: usize) -> (usize, usize) {
       let coord = self.max_square(size);
       (coord.0, coord.1)
    }

    fn total_power_all_sizes(&self) -> (usize, usize, usize) {
        let max_size = (1..=Grid::N).into_par_iter().map(|i| {
            let (_, _, power) = self.max_square(i);
            (i, power)
        }).reduce_with(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Equal => a,
                Ordering::Less => b,
                Ordering::Greater => a
            }
        }).unwrap();
        
        let total_power = self.total_power(max_size.0);
        (total_power.0, total_power.1, max_size.0)
    }
}

fn main() {
    let g = Grid::new(5177);
    part1(&g);
    part2(&g);
}

fn part1(g: &Grid) {
    let (x, y) = g.total_power(3);
    println!("{},{}", x, y);
}

fn part2(g: &Grid) {
    let (x, y, size) = g.total_power_all_sizes();
    println!("{},{},{}", x, y, size);
}
