use crate::maybe_from::MaybeFrom;
use crate::twod::{Point, Vector};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

impl Square {
    fn count<S>(&self, squares: S) -> usize
    where
        S: IntoIterator<Item = Square>,
    {
        squares.into_iter().filter(|s| s == self).count()
    }
}

impl MaybeFrom<char> for Square {
    fn maybe_from(value: char) -> Option<Self> {
        match value {
            '.' => Some(Square::Open),
            '#' => Some(Square::Tree),
            _ => None,
        }
    }
}

struct Row {
    squares: Vec<Square>,
}

impl Row {
    fn get(&self, index: usize) -> Square {
        self.squares
            .get(index % self.squares.len())
            .unwrap()
            .clone()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row {
            squares: value.chars().flat_map(|c| Square::maybe_from(c)).collect(),
        }
    }
}

struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn run(&self, vector: Vector<usize>) -> Toboggan {
        Toboggan::new(&self, vector)
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            rows: value.lines().map(|line| Row::from(line)).collect(),
        }
    }
}

struct Toboggan<'a> {
    map: &'a Map,
    vec: Vector<usize>,
    pos: Point<usize>,
}

impl<'a> Toboggan<'a> {
    fn new(map: &'a Map, vec: Vector<usize>) -> Self {
        Self {
            map,
            vec,
            pos: Point::default(),
        }
    }
}

impl<'a> Iterator for Toboggan<'a> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.map.rows.get(self.pos.y).map(|row| row.get(self.pos.x));
        self.pos += self.vec;
        result
    }
}

pub fn run() {
    let input = include_str!("data/3/1");
    let map = Map::from(input);
    println!("{}", Square::Tree.count(map.run((3, 1).into())));
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: usize = slopes
        .iter()
        .map(|slope| Square::Tree.count(map.run(slope.into())))
        .fold(1, |a, b| a * b);
    println!("{}", result);
}
