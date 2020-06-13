use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Couldn't read input");
    let mut initial_points = vec![];
    for line in input.lines() {
        let mut split = line.split(", ");
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        initial_points.push(Point { x, y });
    }

    part1(&initial_points);
    part2(&initial_points);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        u32::max(self.x, other.x) - u32::min(self.x, other.x) + u32::max(self.y, other.y)
            - u32::min(self.y, other.y)
    }
}

fn part1(initial_points: &Vec<Point>) {
    // Find the smallest grid that bounds everyone.
    let mut min_x = initial_points[0].x;
    let mut max_x = initial_points[0].x;
    let mut min_y = initial_points[0].y;
    let mut max_y = initial_points[0].y;
    for point in initial_points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    // Calculate all points within grid.
    let mut points_by_closest: HashMap<&Point, Vec<Point>> = HashMap::new();
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let mut shortest_distance = max_x.max(max_y);
            let mut num_with_this_dist = 0;
            let mut shortest_distance_origin = &Point { x: 0, y: 0 };

            let new_point = Point { x: i, y: j };

            for k in initial_points {
                let dist = new_point.distance(k);
                match dist.cmp(&shortest_distance) {
                    Ordering::Equal => num_with_this_dist += 1,
                    Ordering::Less => {
                        num_with_this_dist = 1;
                        shortest_distance = dist;
                        shortest_distance_origin = k;
                    }
                    Ordering::Greater => {}
                }
            }

            if num_with_this_dist == 1 {
                points_by_closest
                    .entry(shortest_distance_origin)
                    .or_default()
                    .push(new_point);
            }
        }
    }

    // Anyone on the border is infinite. Remove them.
    let finite_area = points_by_closest.iter().filter(|(_, v)| {
        v.iter()
            .all(|gp| gp.x != min_x || gp.x != max_x || gp.y != min_y || gp.y != max_y)
    });

    let largest_area = finite_area.fold(0, |accum, elem| accum.max(elem.1.len()));
    println!("{}", largest_area)
}

fn part2(initial_points: &Vec<Point>) {
    static MAX_DIST: u32 = 10_000;

    // Find the smallest grid that bounds everyone.
    let mut min_x = initial_points[0].x;
    let mut max_x = initial_points[0].x;
    let mut min_y = initial_points[0].y;
    let mut max_y = initial_points[0].y;
    for point in initial_points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    let mut count = 0;
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let mut distance_accum = 0;
            for k in initial_points {
                distance_accum += k.distance(&Point{x: i, y: j});
                if distance_accum > MAX_DIST {
                    break;
                }
            }

            if distance_accum <MAX_DIST {
                count += 1;
            }
        }
    }

    println!("{}", count);
}