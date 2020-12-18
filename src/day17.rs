use crate::maybe_from::MaybeFrom;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn neighbors(&self) -> [Coordinate; 26] {
        [
            Coordinate::new(self.x, self.y - 1, self.z),
            Coordinate::new(self.x, self.y - 1, self.z - 1),
            Coordinate::new(self.x, self.y - 1, self.z + 1),
            Coordinate::new(self.x, self.y + 1, self.z),
            Coordinate::new(self.x, self.y + 1, self.z - 1),
            Coordinate::new(self.x, self.y + 1, self.z + 1),
            Coordinate::new(self.x, self.y, self.z - 1),
            Coordinate::new(self.x, self.y, self.z + 1),
            Coordinate::new(self.x + 1, self.y - 1, self.z),
            Coordinate::new(self.x + 1, self.y - 1, self.z - 1),
            Coordinate::new(self.x + 1, self.y - 1, self.z + 1),
            Coordinate::new(self.x + 1, self.y + 1, self.z),
            Coordinate::new(self.x + 1, self.y + 1, self.z - 1),
            Coordinate::new(self.x + 1, self.y + 1, self.z + 1),
            Coordinate::new(self.x + 1, self.y, self.z),
            Coordinate::new(self.x + 1, self.y, self.z - 1),
            Coordinate::new(self.x + 1, self.y, self.z + 1),
            Coordinate::new(self.x - 1, self.y - 1, self.z),
            Coordinate::new(self.x - 1, self.y - 1, self.z - 1),
            Coordinate::new(self.x - 1, self.y - 1, self.z + 1),
            Coordinate::new(self.x - 1, self.y + 1, self.z),
            Coordinate::new(self.x - 1, self.y + 1, self.z - 1),
            Coordinate::new(self.x - 1, self.y + 1, self.z + 1),
            Coordinate::new(self.x - 1, self.y, self.z),
            Coordinate::new(self.x - 1, self.y, self.z - 1),
            Coordinate::new(self.x - 1, self.y, self.z + 1),
        ]
    }
}

#[derive(Debug)]
enum CubeState {
    Active { active_neighbors: usize },
    Inactive { active_neighbors: usize },
}

#[derive(Debug, Clone)]
struct Universe {
    active_cubes: HashSet<Coordinate>,
}

impl Universe {
    fn cube_state(&self, coord: &Coordinate) -> CubeState {
        let active_neighbors = coord
            .neighbors()
            .iter()
            .filter(|coord| self.active_cubes.contains(coord))
            .count();
        if self.active_cubes.contains(coord) {
            CubeState::Active { active_neighbors }
        } else {
            CubeState::Inactive { active_neighbors }
        }
    }

    fn extent(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        self.active_cubes.iter().fold(
            ((0, 0), (0, 0), (0, 0)),
            |((min_x, max_x), (min_y, max_y), (min_z, max_z)), coord| {
                (
                    (
                        if coord.x < min_x { coord.x } else { min_x },
                        if coord.x > max_x { coord.x } else { max_x },
                    ),
                    (
                        if coord.y < min_y { coord.y } else { min_y },
                        if coord.y > max_y { coord.y } else { max_y },
                    ),
                    (
                        if coord.z < min_z { coord.z } else { min_z },
                        if coord.z > max_z { coord.z } else { max_z },
                    ),
                )
            },
        )
    }

    fn evolve(&self) -> Universe {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = self.extent();
        Self {
            active_cubes: HashSet::from_iter(
                (min_x - 1..=max_x + 1)
                    .flat_map(move |x| {
                        (min_y - 1..=max_y + 1).flat_map(move |y| {
                            (min_z - 1..=max_z + 1).map(move |z| Coordinate::new(x, z, y))
                        })
                    })
                    .flat_map(|coord| match self.cube_state(&coord) {
                        CubeState::Active { active_neighbors } => {
                            if active_neighbors == 2 || active_neighbors == 3 {
                                Some(coord)
                            } else {
                                None
                            }
                        }
                        CubeState::Inactive { active_neighbors } => {
                            if active_neighbors == 3 {
                                Some(coord)
                            } else {
                                None
                            }
                        }
                    }),
            ),
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = self.extent();
        for z in (min_z..=max_z) {
            write!(f, "z={}\n", z)?;
            for y in (min_y..=max_y) {
                write!(
                    f,
                    "{}\n",
                    (min_x..=max_x)
                        .map(
                            |x| if self.active_cubes.contains(&Coordinate::new(x, y, z)) {
                                '#'
                            } else {
                                '.'
                            }
                        )
                        .collect::<String>()
                );
            }
        }
        Ok(())
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        Self {
            active_cubes: HashSet::from_iter(value.lines().enumerate().flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Coordinate::new(x as i32, y as i32, 0))
            })),
        }
    }
}

pub fn run() {
    let real_input = "...#.#.#
..#..#..
#.#.##.#
###.##..
#####.##
#.......
#..#..##
...##.##";
    let input = ".#.
..#
###";
    let universe = Universe::from(input);
    let result = (0..6).fold(universe.clone(), |universe, evolution| {
        println!("evolution={}\n{}", evolution, universe);
        universe.evolve()
    });
    println!("evolution=6\n{}", result);
    println!("{}", result.active_cubes.len());
}
