use std::convert::TryFrom;

use crate::maybe_from::MaybeFrom;

#[derive(Debug)]
struct Entry {
    letter: char,
    low: usize,
    high: usize,
    password: String,
}

impl Entry {
    fn valid_sled(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        self.low <= count && count <= self.high
    }

    fn valid_toboggan(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        chars.get(self.low - 1).map_or(false, |&c| c == self.letter)
            ^ chars
                .get(self.high - 1)
                .map_or(false, |&c| c == self.letter)
    }
}

impl TryFrom<&str> for Entry {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (rule, password) = value.split_once(": ").ok_or(())?;
        let (nums, letter) = value.split_once(" ").ok_or(())?;
        let (low, high) = nums.split_once("-").ok_or(())?;
        let low: usize = low.parse().map_err(|_| ())?;
        let high: usize = high.parse().map_err(|_| ())?;
        Ok(Entry {
            letter: letter.chars().next().ok_or(())?,
            low,
            high,
            password: password.to_owned(),
        })
    }
}

fn count_valid_entries_sled(entries: &[Entry]) -> usize {
    entries.iter().filter(|entry| entry.valid_sled()).count()
}

fn count_valid_entries_toboggan(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|entry| entry.valid_toboggan())
        .count()
}

fn get_entries(input: &str) -> Vec<Entry> {
    input
        .lines()
        .flat_map(|line| Entry::maybe_from(line))
        .collect()
}

pub fn run() {
    let input = include_str!("data/2/1");
    let entries = get_entries(input);
    println!("{}", count_valid_entries_sled(&entries));
    println!("{}", count_valid_entries_toboggan(&entries));
}
