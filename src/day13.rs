use crate::maybe_from::MaybeFrom;
use crate::utils::Counter;
use itertools::Itertools;
use num::Integer;

#[derive(Debug, Copy, Clone)]
struct Bus {
    id: usize,
}

impl Bus {
    fn next_departure(&self, after: &usize) -> usize {
        after.div_ceil(&self.id) * self.id
    }
}

#[derive(Debug)]
struct Airport {
    timestamp: usize,
    lines: Vec<Bus>,
}

impl Airport {
    fn find_next_departure(&self) -> Option<(usize, Bus)> {
        self.lines
            .iter()
            .min_by_key(|line| line.next_departure(&self.timestamp))
            .map(|bus| {
                (
                    bus.next_departure(&self.timestamp) - self.timestamp,
                    bus.clone(),
                )
            })
    }
}

impl MaybeFrom<&str> for Airport {
    fn maybe_from(value: &str) -> Option<Self> {
        let (timestamp, lines) = value.split_once("\n")?;
        Some(Self {
            timestamp: timestamp.parse::<usize>().ok()?,
            lines: lines
                .split(",")
                .flat_map(|id| id.parse::<usize>().ok())
                .map(|id| Bus { id })
                .collect(),
        })
    }
}

#[inline(always)]
fn check_timestamp(timestamp: usize) -> bool {
    timestamp % 13 == 0
        && (timestamp + 3) % 41 == 0
        && (timestamp + 13) % 569 == 0
        && (timestamp + 15) % 29 == 0
        && (timestamp + 32) & 19 == 0
        && (timestamp + 36) % 23 == 0
        && (timestamp + 44) % 937 == 0
        && (timestamp + 50) % 37 == 0
        && (timestamp + 61) % 17 == 0
}

#[derive(Debug)]
struct Rule {
    offset: u64,
    bus_id: u64,
}

fn find_timestamp(rules: &str) -> u64 {
    // Stolen from reddit
    rules
        .split(",")
        .enumerate()
        .flat_map(|(index, bus)| {
            bus.parse::<u64>().ok().map(|bus_id| Rule {
                offset: index as u64,
                bus_id,
            })
        })
        .sorted_by_key(|rule| rule.bus_id)
        .fold(
            (0, 1),
            |(possible_solution, least_common_denominator), rule| {
                let mut new_possible_solution = possible_solution;
                while (new_possible_solution + rule.offset) % rule.bus_id != 0 {
                    new_possible_solution += least_common_denominator;
                }
                (
                    new_possible_solution,
                    least_common_denominator * rule.bus_id,
                )
            },
        )
        .0
}

pub fn run() {
    let airport = Airport::maybe_from(include_str!("data/13/1")).expect("Failed to parse");
    let (delay, bus) = airport.find_next_departure().expect("No departure found");
    println!("{}", delay * bus.id);
    println!(
        "{}",
        find_timestamp(
            include_str!("data/13/1")
                .lines()
                .skip(1)
                .next()
                .expect("invalid input")
        )
    );
}
