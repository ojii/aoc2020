use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::{FlatMap, Step};
use std::ops::{Add, AddAssign, Mul};

use itertools::Itertools;
use num::{CheckedAdd, CheckedSub, One};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: CheckedSub<Output = T> + CheckedAdd<Output = T> + One + Copy,
{
    pub fn neighbors(&self) -> impl Iterator<Item = Point<T>> + '_ {
        NEIGHBORS
            .iter()
            .flat_map(move |neighbor| neighbor.point::<T>(self, One::one()))
    }
}

impl<T: Add<Output = T>> Add<Vector<T>> for Point<T> {
    type Output = Self;

    fn add(self, other: Vector<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: AddAssign<T>> AddAssign<Vector<T>> for Point<T> {
    fn add_assign(&mut self, other: Vector<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Clone> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self::from(&value)
    }
}

impl<T: Clone> From<&(T, T)> for Point<T> {
    fn from(value: &(T, T)) -> Self {
        Self {
            x: value.0.clone(),
            y: value.1.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Neighbor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Neighbor {
    pub fn point<T>(&self, origin: &Point<T>, distance: T) -> Option<Point<T>>
    where
        T: CheckedAdd<Output = T>,
        T: CheckedSub<Output = T>,
        T: Copy,
    {
        use Neighbor::*;
        match self {
            TopLeft => origin.x.checked_sub(&distance).and_then(|x| {
                origin
                    .y
                    .checked_sub(&distance)
                    .map(move |y| Point::new(x, y))
            }),
            Top => origin
                .y
                .checked_sub(&distance)
                .map(|y| Point::new(origin.x, y)),
            TopRight => origin.x.checked_add(&distance).and_then(|x| {
                origin
                    .y
                    .checked_sub(&distance)
                    .map(move |y| Point::new(x, y))
            }),
            Left => origin
                .x
                .checked_sub(&distance)
                .map(|x| Point::new(x, origin.y)),
            Right => origin
                .x
                .checked_add(&distance)
                .map(|x| Point::new(x, origin.y)),
            BottomLeft => origin.x.checked_sub(&distance).and_then(|x| {
                origin
                    .y
                    .checked_add(&distance)
                    .map(move |y| Point::new(x, y))
            }),
            Bottom => origin
                .y
                .checked_add(&distance)
                .map(|y| Point::new(origin.x, y)),
            BottomRight => origin.x.checked_add(&distance).and_then(|x| {
                origin
                    .y
                    .checked_add(&distance)
                    .map(move |y| Point::new(x, y))
            }),
        }
    }
}

pub const NEIGHBORS: [Neighbor; 8] = [
    Neighbor::TopLeft,
    Neighbor::Top,
    Neighbor::TopRight,
    Neighbor::Left,
    Neighbor::Right,
    Neighbor::BottomLeft,
    Neighbor::Bottom,
    Neighbor::BottomRight,
];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Clone> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Self::from(&value)
    }
}

impl<T: Clone> From<&(T, T)> for Vector<T> {
    fn from(value: &(T, T)) -> Self {
        Self {
            x: value.0.clone(),
            y: value.1.clone(),
        }
    }
}
