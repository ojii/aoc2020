use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::{FlatMap, Step};
use std::ops::{Add, AddAssign, Mul, Neg, Rem, Sub};

use itertools::Itertools;
use num::{abs, cast, CheckedAdd, CheckedSub, Integer, NumCast, One, Signed, Zero};

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
    T: Signed + Mul<Output = T> + Add<Output = T> + Ord + Copy,
{
    pub fn manhattan_distance(&self, other: &Point<T>) -> T {
        distance(self.x, other.x) + distance(self.y, other.y)
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

impl<T> AddAssign<Vector<T>> for Point<T>
where
    T: AddAssign<T> + Copy,
{
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

impl<T> Vector<T>
where
    T: Integer + Copy + Neg<Output = T> + Zero + NumCast,
{
    pub fn rotate(&self, degrees: T) -> Option<Vector<T>> {
        let zero: T = Zero::zero();
        let ninety: T = cast(90)?;
        let one_eighty: T = cast(180)?;
        let two_seventy: T = cast(270)?;
        let three_sixty: T = cast(360)?;
        match rem_euclid(degrees, three_sixty) {
            n if n == zero => Some(Vector::new(self.x, self.y)),
            n if n == ninety => Some(Vector::new(self.y, -self.x)),
            n if n == one_eighty => Some(Vector::new(-self.x, -self.y)),
            n if n == two_seventy => Some(Vector::new(-self.y, self.x)),
            _ => None,
        }
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

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, other: Vector<T>) {
        self.x += other.x;
        self.y += other.y;
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

fn distance<T: Signed + Ord + Copy + Sub<Output = T>>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn rem_euclid<T>(lhs: T, rhs: T) -> T
where
    T: Integer + Copy + Zero,
{
    let r = lhs % rhs;
    if r < Zero::zero() {
        if rhs < Zero::zero() {
            r - rhs
        } else {
            r + rhs
        }
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(cases = {
        (-90, Some(Vector::new(-4, 10))),
        (0, Some(Vector::new(10, 4))),
        (90, Some(Vector::new(4, -10))),
        (180, Some(Vector::new(-10, -4))),
        (270, Some(Vector::new(-4, 10))),
        (360, Some(Vector::new(10, 4))),
        (450, Some(Vector::new(4, -10))),
        (123, None),
    })]
    fn rotate(cases: (i32, Option<Vector<i32>>)) {
        let base = Vector::new(10, 4);
        assert_eq!(
            base.rotate(cases.0),
            cases.1,
            "rotate {:?} by {} ({}) to get {:?}, got {:?} instead",
            base,
            cases.0,
            rem_euclid(cases.0, 360),
            cases.1,
            base.rotate(cases.0)
        );
    }
}
