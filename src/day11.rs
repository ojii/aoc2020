use crate::day11::Seat::Empty;
use crate::maybe_from::MaybeFrom;
use crate::render::{render, Render};
use crate::twod::{Point, NEIGHBORS};
use crate::utils::Counter;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Empty,
}

impl Seat {
    fn is_occupied(&self) -> bool {
        match self {
            Seat::Occupied => true,
            Seat::Empty => false,
        }
    }

    fn is_empty(&self) -> bool {
        !self.is_occupied()
    }
}

impl Render for Seat {
    fn render(&self) -> String {
        match self {
            Seat::Occupied => "#",
            Seat::Empty => "L",
        }
        .to_string()
    }
}

impl MaybeFrom<char> for Seat {
    fn maybe_from(value: char) -> Option<Self> {
        match value {
            'L' => Some(Seat::Empty),
            '#' => Some(Seat::Occupied),
            _ => None,
        }
    }
}

enum Evolution {
    Evolved(WaitingArea),
    Stabilized(WaitingArea),
}

#[derive(Debug, Clone)]
struct WaitingArea {
    seats: HashMap<Point<usize>, Seat>,
    max: Point<usize>,
}

impl WaitingArea {
    fn new(seats: HashMap<Point<usize>, Seat>) -> Self {
        let max = seats
            .keys()
            .max()
            .cloned()
            .unwrap_or_else(|| Point::new(0, 0));
        Self { seats, max }
    }
}

impl From<&str> for WaitingArea {
    fn from(input: &str) -> Self {
        Self::new(HashMap::from_iter(input.lines().enumerate().flat_map(
            |(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(move |(x, c)| Seat::maybe_from(c).map(|seat| (Point { x, y }, seat)))
            },
        )))
    }
}

impl WaitingArea {
    fn evolve<'a, N, I>(&'a self, neighbors: N, tolerance: usize) -> Evolution
    where
        I: Iterator<Item = Seat>,
        N: Fn(&'a Self, &'a Point<usize>) -> I + 'a,
    {
        let mut stable = true;
        let waiting_area = WaitingArea::new(HashMap::from_iter(self.seats.iter().map(
            |(point, seat)| {
                match seat {
                    Seat::Occupied => {
                        if neighbors(&self, point)
                            .filter(|seat| seat.is_occupied())
                            .count()
                            >= tolerance
                        {
                            stable = false;
                            (point.clone(), Seat::Empty)
                        } else {
                            (point.clone(), Seat::Occupied)
                        }
                    }
                    Seat::Empty => {
                        if neighbors(&self, point).all(|seat| seat.is_empty()) {
                            stable = false;
                            (point.clone(), Seat::Occupied)
                        } else {
                            (point.clone(), Seat::Empty)
                        }
                    }
                }
            },
        )));
        if stable {
            Evolution::Stabilized(waiting_area)
        } else {
            Evolution::Evolved(waiting_area)
        }
    }
}

fn direct_neighbors<'a>(
    waiting_area: &'a WaitingArea,
    point: &'a Point<usize>,
) -> impl Iterator<Item = Seat> + 'a {
    point
        .neighbors()
        .flat_map(move |point| waiting_area.seats.get(&point).cloned())
}

fn full_neighbors<'a>(
    waiting_area: &'a WaitingArea,
    point: &'a Point<usize>,
) -> impl Iterator<Item = Seat> + 'a {
    NEIGHBORS.iter().flat_map(move |neighbor| {
        for distance in Counter::new() {
            match neighbor.point(point, distance) {
                Some(other) => {
                    if other > waiting_area.max {
                        return None;
                    } else {
                        match waiting_area.seats.get(&other) {
                            Some(seat) => return Some(seat.clone()),
                            None => (),
                        }
                    }
                }
                None => break,
            }
        }
        return None;
    })
}

fn evolve_until_stability<'a, I, N>(
    waiting_area: &WaitingArea,
    neighbors: N,
    tolerance: usize,
) -> WaitingArea
where
    I: Iterator<Item = Seat>,
    N: Fn(&'a WaitingArea, &'a Point<usize>) -> I + 'a,
{
    let mut waiting_area = waiting_area.clone();
    loop {
        match waiting_area.evolve(direct_neighbors, tolerance) {
            Evolution::Evolved(new_waiting_area) => waiting_area = new_waiting_area,
            Evolution::Stabilized(stable_waiting_area) => return stable_waiting_area,
        }
    }
}

pub fn run() {
    let waiting_area = WaitingArea::from(include_str!("data/11/1"));
    let stable = evolve_until_stability(&waiting_area, direct_neighbors, 4);
    println!(
        "{}",
        stable
            .seats
            .values()
            .filter(|seat| seat.is_occupied())
            .count()
    );
    let stable = evolve_until_stability(&waiting_area, full_neighbors, 5);
    println!(
        "{}",
        stable
            .seats
            .values()
            .filter(|seat| seat.is_occupied())
            .count()
    );
}
