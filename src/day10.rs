use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("data/10/1");
    let mut nums: HashSet<u32> = input
        .lines()
        .flat_map(|s| s.parse::<u32>().ok())
        .chain((0..=0))
        .collect();
    nums.insert(nums.iter().max().expect("empty input") + 3);
    let (ones, threes) =
        nums.iter()
            .sorted()
            .tuple_windows::<(_, _)>()
            .fold((0, 0), |(acc1, acc3), (a, b)| match b - a {
                1 => (acc1 + 1, acc3),
                3 => (acc1, acc3 + 1),
                _ => (acc1, acc3),
            });
    println!("{} * {} = {:?}", ones, threes, ones * threes);
}
