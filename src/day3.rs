use crate::maybe_from::MaybeFrom;
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
    fn run(&self, vector: Vector) -> Toboggan {
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

struct Vector {
    x: usize,
    y: usize,
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Right: {}, Down: {}", self.x, self.y)
    }
}

impl From<(usize, usize)> for Vector {
    fn from(tuple: (usize, usize)) -> Self {
        Self::from(&tuple)
    }
}

impl From<&(usize, usize)> for Vector {
    fn from(tuple: &(usize, usize)) -> Self {
        Vector {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Default)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Add<&Vector> for Coordinate {
    type Output = Self;

    fn add(self, rhs: &Vector) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Vector> for Coordinate {
    fn add_assign(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Toboggan<'a> {
    map: &'a Map,
    vec: Vector,
    pos: Coordinate,
}

impl<'a> Toboggan<'a> {
    fn new(map: &'a Map, vec: Vector) -> Self {
        Self {
            map,
            vec,
            pos: Coordinate::default(),
        }
    }
}

impl<'a> Iterator for Toboggan<'a> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.map.rows.get(self.pos.y).map(|row| row.get(self.pos.x));
        self.pos += &self.vec;
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
