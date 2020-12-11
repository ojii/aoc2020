use crate::twod::Point;
use itertools::Itertools;
use itertools::__std_iter::Step;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub trait Render {
    fn render(&self) -> String;
}

impl Render for dyn Display {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

impl Render for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

pub fn render<T: Hash + Eq + PartialOrd + Clone + Copy + Step, U: Render, B: Render>(
    map: &HashMap<Point<T>, U>,
    background: B,
) -> String {
    let (min_x, max_x) = map
        .keys()
        .map(|point| point.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = map
        .keys()
        .map(|point| point.y)
        .minmax()
        .into_option()
        .unwrap();
    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| {
                    map.get(&Point::new(x, y))
                        .map(|tile| tile.render())
                        .unwrap_or_else(|| background.render())
                })
                .join("")
        })
        .join("\n")
}
