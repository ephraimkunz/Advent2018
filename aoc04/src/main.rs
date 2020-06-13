use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ord;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum EventType {
    Sleep,
    Wakeup,
    NewGuard{id: u32},
}

impl FromStr for EventType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => return Ok(EventType::Sleep),
            "wakes up" => return Ok(EventType::Wakeup),
            _ => {
                let id: String = s.matches(|c: char| c.is_digit(10)).collect();
                return Ok(EventType::NewGuard{id: id.parse()?});
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Event {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event_type: EventType,
}

impl FromStr for Event {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+)\s(?P<hour>\d+):(?P<minute>\d+)\]\s(?P<event>.+)").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("Unable to parse string")),
            Some(s) => s,
        };

        Ok(Event {
            year: captures["year"].parse()?,
            month: captures["month"].parse()?,
            day: captures["day"].parse()?,
            hour: captures["hour"].parse()?,
            minute: captures["minute"].parse()?,
            event_type: captures["event"].parse()?,
        })
    }
}

type StatsMap = HashMap<u32, [u32; 60]>;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Couldn't read input file");

    // Sort by time.
    let mut events: Vec<Event> = input.lines().map(|l| l.parse().unwrap()).collect();
    events.sort();

    let mut stats = StatsMap::new();
    let mut current_guard = 0;
    let mut sleep_start = 0;
    for event in events {
        match event.event_type {
            EventType::NewGuard{id} => current_guard = id,
            EventType::Sleep => sleep_start = event.minute,
            EventType::Wakeup => {
                let sleep_end = event.minute;
                let bitmap = stats.entry(current_guard).or_insert([0; 60]);
                for i in sleep_start..sleep_end {
                    bitmap[i as usize] += 1;
                }
            }
        }
    }

    part1(&stats);
    part2(&stats);
}

fn part1(stats: &StatsMap) {
    let longest_sleeper = stats
        .iter()
        .max_by(|a, b| a.1.iter().sum::<u32>().cmp(&b.1.iter().sum::<u32>()));
    if let Some(longest_sleeper) = longest_sleeper {
        let mut sleepiest_time = 0;
        for time in 0..60 {
            if longest_sleeper.1[time as usize] > sleepiest_time {
                sleepiest_time = time;
            }
        }

        println!("{}", longest_sleeper.0 * sleepiest_time);
    }
}

fn part2(stats: &StatsMap) {
    let mut id = 0;
    let mut most_times = 0;
    let mut most_times_min = 0;
    for event in stats {
        for i in 0..60 {
            if event.1[i] > most_times {
                id = *event.0;
                most_times = event.1[i];
                most_times_min = i as u32;
            }
        }
    }

    println!("{}, {}", id * most_times_min, most_times);
}
