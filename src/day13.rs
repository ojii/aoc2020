use crate::maybe_from::MaybeFrom;
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

pub fn run() {
    let airport = Airport::maybe_from(include_str!("data/13/1")).expect("Failed to parse");
    let (delay, bus) = airport.find_next_departure().expect("No departure found");
    println!("{}", delay * bus.id)
}
