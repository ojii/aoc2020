use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
enum LastCalls {
    Single(usize),
    Double(usize, usize),
}

impl LastCalls {
    fn adding(&self, other: usize) -> LastCalls {
        use LastCalls::*;
        match self {
            Single(n) => Double(*n, other),
            Double(a, b) => Double(*b, other),
        }
    }
}

fn memory_game(numbers: &[u64], num: usize) -> u64 {
    let mut memory: HashMap<u64, LastCalls> = HashMap::new();
    let (index, mut last) = numbers
        .iter()
        .enumerate()
        .map(|(index, number)| {
            match memory.entry(*number) {
                Entry::Vacant(mut entry) => {
                    entry.insert(LastCalls::Single(index + 1));
                }
                Entry::Occupied(mut entry) => {
                    entry.insert(entry.get().adding(index + 1));
                }
            };
            (index + 1, *number)
        })
        .last()
        .expect("empty input");

    (index + 1..=num).fold(last, |last, turn| {
        let new_last = match memory.get(&last) {
            None | Some(LastCalls::Single(_)) => 0,
            Some(LastCalls::Double(a, b)) => (b - a) as u64,
        };
        match memory.entry(new_last) {
            Entry::Vacant(mut entry) => {
                entry.insert(LastCalls::Single(turn));
            }
            Entry::Occupied(mut entry) => {
                entry.insert(entry.get().adding(turn));
            }
        };
        new_last
    })
}

pub fn run() {
    println!("{}", memory_game(&[15, 12, 0, 14, 3, 1], 2020));
    println!("{}", memory_game(&[15, 12, 0, 14, 3, 1], 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(case = {
        (&[0,3,6], 436),
        (&[1,3,2], 1),
        (&[2,1,3], 10),
        (&[1,2,3], 27),
        (&[2,3,1], 78),
        (&[3,2,1], 438),
        (&[3,1,2], 1836),
    })]
    #[test]
    fn test_memory_game_2020(case: (&[u64], u64)) {
        assert_eq!(memory_game(case.0, 2020), case.1)
    }

    /*
    Given 0,3,6, the 30000000th number spoken is 175594.
    Given 1,3,2, the 30000000th number spoken is 2578.
    Given 2,1,3, the 30000000th number spoken is 3544142.
    Given 1,2,3, the 30000000th number spoken is 261214.
    Given 2,3,1, the 30000000th number spoken is 6895259.
    Given 3,2,1, the 30000000th number spoken is 18.
    Given 3,1,2, the 30000000th number spoken is 362.
     */

    #[parameterized(case = {
        (&[0,3,6], 175594),
        (&[1,3,2], 2578),
        (&[2,1,3], 3544142),
        (&[1,2,3], 261214),
        (&[2,3,1], 6895259),
        (&[3,2,1], 18),
        (&[3,1,2], 362),
    })]
    #[test]
    fn test_memory_game_30000000(case: (&[u64], u64)) {
        assert_eq!(memory_game(case.0, 30000000), case.1)
    }
}
