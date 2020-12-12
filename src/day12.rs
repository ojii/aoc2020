use crate::maybe_from::{MaybeFrom, MaybeInto};
use crate::twod::{Point, Vector};
use itertools::Itertools;
use num::Integer;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl MaybeFrom<&str> for Instruction {
    fn maybe_from(value: &str) -> Option<Self> {
        use Instruction::*;
        let mut chars = value.chars();
        let code = chars.next()?;
        let num = chars.join("").parse::<i32>().ok()?;
        match code {
            'N' => Some(North(num)),
            'S' => Some(South(num)),
            'E' => Some(East(num)),
            'W' => Some(West(num)),
            'L' => Some(Left(num)),
            'R' => Some(Right(num)),
            'F' => Some(Forward(num)),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self, degrees: i32) -> Direction {
        self.right(-degrees)
    }

    fn right(&self, degrees: i32) -> Direction {
        (self.degrees() + degrees)
            .maybe_into()
            .expect(&format!("Invalid turn: {}", degrees))
    }

    fn vector(&self, distance: i32) -> Vector<i32> {
        use Direction::*;
        match self {
            North => Vector::new(0, distance),
            East => Vector::new(distance, 0),
            South => Vector::new(0, -distance),
            West => Vector::new(-distance, 0),
        }
    }

    fn degrees(&self) -> i32 {
        use Direction::*;
        match self {
            North => 0,
            East => 90,
            South => 180,
            West => 270,
        }
    }
}

impl MaybeFrom<i32> for Direction {
    fn maybe_from(value: i32) -> Option<Self> {
        use Direction::*;
        match value.rem_euclid(360) {
            0 => Some(North),
            90 => Some(East),
            180 => Some(South),
            270 => Some(West),
            _ => None,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

#[derive(Debug)]
struct Ship {
    position: Point<i32>,
    facing: Direction,
    waypoint: Vector<i32>,
}

impl Ship {
    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            North(num) => self.position += Vector::new(0, *num),
            South(num) => self.position += Vector::new(0, -*num),
            East(num) => self.position += Vector::new(*num, 0),
            West(num) => self.position += Vector::new(-*num, 0),
            Left(num) => self.facing = self.facing.left(*num),
            Right(num) => self.facing = self.facing.right(*num),
            Forward(num) => self.position += self.facing.vector(*num),
        }
    }

    fn execute_waypoint(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            North(num) => self.waypoint += Vector::new(0, *num),
            South(num) => self.waypoint += Vector::new(0, -*num),
            East(num) => self.waypoint += Vector::new(*num, 0),
            West(num) => self.waypoint += Vector::new(-*num, 0),
            Left(num) => self.waypoint = self.waypoint.rotate(-*num).expect("invalid turn"),
            Right(num) => self.waypoint = self.waypoint.rotate(*num).expect("invalid turn"),
            Forward(num) => self.position += self.waypoint * *num,
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Point::default(),
            facing: Direction::default(),
            waypoint: Vector::new(10, 1),
        }
    }
}

pub fn run() {
    let instructions = include_str!("data/12/1")
        .lines()
        .flat_map(|line| Instruction::maybe_from(line))
        .collect_vec();
    let mut ship = Ship::default();
    for instruction in &instructions {
        ship.execute(instruction);
    }
    println!("{}", ship.position.manhattan_distance(&Point::default()));
    let mut ship = Ship::default();
    for instruction in &instructions {
        ship.execute_waypoint(instruction);
    }
    println!("{}", ship.position.manhattan_distance(&Point::default()));
}
