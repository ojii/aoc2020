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

impl MaybeFrom<&str> for Entry {
    fn maybe_from(value: &str) -> Option<Self> {
        let (rule, password) = value.split_once(": ")?;
        let (nums, letter) = value.split_once(" ")?;
        let (low, high) = nums.split_once("-")?;
        let low: usize = low.parse().ok()?;
        let high: usize = high.parse().ok()?;
        Some(Entry {
            letter: letter.chars().next()?,
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
