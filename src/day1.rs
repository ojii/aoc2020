use crate::utils;
use itertools::Itertools;

fn find_entries(src: &[i32], num: i32, target: i32) -> Option<Vec<i32>> {
    (0..num)
        .map(|_| src)
        .multi_cartesian_product()
        .find(|candidates| candidates.iter().map(|a| a.clone()).sum::<i32>() == target)
        .map(|v| v.iter().map(|&a| a.clone()).collect())
}

pub fn run() {
    let input = utils::str_to_ints(include_str!("data/1/1"));
    let result = find_entries(&input, 2, 2020).expect("failed to find matching entries");
    println!("{:?} => {}", result, result.iter().fold(1, |a, b| a * b));
    let result = find_entries(&input, 3, 2020).expect("failed to find matching entries");
    println!("{:?} => {}", result, result.iter().fold(1, |a, b| a * b));
}
