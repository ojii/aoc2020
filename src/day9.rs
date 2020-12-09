use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn valid(num: &u64, buffer: &VecDeque<u64>) -> bool {
    buffer
        .iter()
        .unique()
        .tuple_combinations()
        .find(|(a, b)| *a + *b == *num)
        .is_some()
}

fn find_first_error(nums: &[u64], buf_size: usize) -> Option<u64> {
    let mut buffer = VecDeque::from_iter(nums.iter().cloned().take(buf_size));
    nums.iter().skip(buf_size).find_map(|num| {
        if valid(num, &buffer) {
            buffer.pop_front();
            buffer.push_back(num.clone());
            None
        } else {
            Some(num.clone())
        }
    })
}

fn find_run(nums: &[u64], error: u64) -> Option<Vec<u64>> {
    (2..nums.len()).find_map(|length| {
        nums.windows(length).find_map(|candidates| {
            if candidates.iter().cloned().sum::<u64>() == error {
                Some(candidates.to_vec())
            } else {
                None
            }
        })
    })
}

pub fn run() {
    let nums = include_str!("data/9/1")
        .lines()
        .flat_map(|line| line.parse::<u64>().ok())
        .collect_vec();
    let error = find_first_error(&nums, 25).expect("no errors found");
    println!("{}", error);
    let run = find_run(&nums, error).expect("no run found");
    println!("{}", run.iter().max().unwrap() + run.iter().min().unwrap());
}
