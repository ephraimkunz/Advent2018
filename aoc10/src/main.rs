use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let re = Regex::new(r"position=<\s?(?P<x>-?\d+),\s\s?(?P<y>-?\d+)>\svelocity=<\s?(?P<vel_x>-?\d+),\s\s?(?P<vel_y>-?\d+)>").unwrap();
        let caps = re.captures(s).unwrap();
        let p = Point {
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            vel_x: caps["vel_x"].parse().unwrap(),
            vel_y: caps["vel_y"].parse().unwrap(),
        };

        Ok(p)
    }
}

impl Point {
    fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

fn main() {
    let s = fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<Point> = s.lines().map(|l| l.parse().unwrap()).collect();

    part1(&input);
}

fn part1(input: &Vec<Point>) {
    let mut mut_points = input.to_owned();
    let mut count = 0;
    for _ in 0..1_000_000 {
        for p in &mut mut_points {
            p.update();
        }
        count += 1;
        print_points(&mut_points, &count);
    }
}

fn print_points(input: &[Point], count: &i32) {
    let mut min_x = input[0].x;
    let mut min_y = input[0].y;
    let mut max_x = input[0].x;
    let mut max_y = input[0].y;

    for point in input {
        if point.x < min_x {
            min_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        }

        if point.x > max_x {
            max_x = point.x;
        }

        if point.y > max_y {
            max_y = point.y;
        }
    }

    if (max_x - min_x) > 80 && (max_y - min_y) > 80 {
        return;
    }

    let mut grid = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for point in input {
        grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = '.';
    }

    println!("{}", count);

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }

    println!("\n\n\n");
}
