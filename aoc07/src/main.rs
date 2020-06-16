use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive (Clone)]
struct Graph {
    g: HashMap<char, DependencyItem>,
}

impl Graph {
    fn new() -> Graph {
        Graph { g: HashMap::new() }
    }

    fn insert(&mut self, item: &DependencyLink) {
        self.g.entry(item.prec).or_default().succ.insert(item.succ);
        self.g.entry(item.succ).or_default().num_prec += 1;
    }

    fn iter_topological_lexicographical(&self) -> GraphIter {
        GraphIter {
            g: self.clone(),
            returned: HashSet::new()
        }
    }
}

struct GraphIter {
    g: Graph,
    returned: HashSet<char>
}

impl Iterator for GraphIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let mut next_candidates: Vec<_> = self
            .g
            .g
            .iter()
            .filter(|(k, v)| !self.returned.contains(k) && v.num_prec == 0)
            .map(|(k, _)| k)
            .collect();

        next_candidates.sort();

        match next_candidates.first() {
            None => return None,
            Some(&&c) => {
                let succs: Vec<_> = self.g.g[&c].succ.iter().map(|&s| s).collect();
                for succ in succs {
                    self.g.g.entry(succ).and_modify(|item| item.num_prec -= 1);
                }
                self.returned.insert(c);

                Some(c)
            }
        }
    }
}

#[derive(Default, Clone)]
struct DependencyItem {
    num_prec: usize,
    succ: HashSet<char>,
}

#[derive(Debug)]
struct DependencyLink {
    prec: char,
    succ: char,
}

impl FromStr for DependencyLink {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Step (?P<prec>[A-Z]) must be finished before step (?P<succ>[A-Z]) can begin."
            )
            .unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(From::from("Wrong input format")),
            Some(c) => c,
        };

        Ok(DependencyLink {
            prec: captures["prec"].as_bytes()[0] as char,
            succ: captures["succ"].as_bytes()[0] as char,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/input.txt").expect("Couldn't read input");
    let dependencies: Vec<DependencyLink> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut g = Graph::new();
    for d in &dependencies {
        g.insert(d);
    }

    part1(&g);
    Ok(())
}

fn part1(g: &Graph) {
    let result: String = g.iter_topological_lexicographical().collect();
    println!("{}", result);
}
