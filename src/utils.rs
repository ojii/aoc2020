use num::{One, Zero};
use std::ops::AddAssign;
use std::str::FromStr;

pub fn str_to_ints<F: FromStr>(s: &str) -> Vec<F> {
    s.lines().flat_map(|l| l.parse::<F>()).collect()
}

pub struct Counter<T> {
    value: T,
}

impl<T: One + Zero + Copy> Counter<T> {
    pub fn new() -> Self {
        Counter {
            value: Zero::zero(),
        }
    }
}

impl<T: One + Zero + Copy + AddAssign> Iterator for Counter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += One::one();
        Some(self.value)
    }
}
