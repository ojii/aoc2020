use crate::maybe_from::MaybeFrom;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    col: usize,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        (self.row * 8) + self.col
    }
}

impl MaybeFrom<&str> for BoardingPass {
    fn maybe_from(value: &str) -> Option<Self> {
        if value.chars().count() != 10 {
            return None;
        }
        let row = value
            .chars()
            .take(7)
            .try_fold((0usize..128).collect_vec(), |bucket, pos| match pos {
                'F' => Some(bucket.iter().cloned().take(bucket.len() / 2).collect_vec()),
                'B' => Some(bucket.iter().cloned().skip(bucket.len() / 2).collect_vec()),
                _ => None,
            })?
            .first()?
            .clone();
        let col = value
            .chars()
            .skip(7)
            .take(3)
            .try_fold((0usize..8).collect_vec(), |bucket, pos| match pos {
                'L' => Some(bucket.iter().cloned().take(bucket.len() / 2).collect_vec()),
                'R' => Some(bucket.iter().cloned().skip(bucket.len() / 2).collect_vec()),
                _ => None,
            })?
            .first()?
            .clone();
        Some(BoardingPass { col, row })
    }
}

pub fn run() {
    let input = include_str!("data/5/1");
    let boarding_passes = input
        .lines()
        .flat_map(|line| BoardingPass::maybe_from(line))
        .collect_vec();
    let seat_ids: HashSet<usize> =
        HashSet::from_iter(boarding_passes.iter().map(|bp| bp.seat_id()));
    println!("{}", seat_ids.iter().max().expect("no passes"));
    let free_seat = (0usize..128)
        .cartesian_product((0usize..8))
        .map(|(row, col)| BoardingPass { row, col })
        .filter(|bp| {
            let seat_id = bp.seat_id();
            !seat_ids.contains(&seat_id)
                && seat_ids.contains(&(seat_id + 1))
                && seat_ids.contains(&(seat_id - 1))
        })
        .exactly_one()
        .expect("more than one found");
    println!("{:?}", free_seat.seat_id());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boarding_pass() {
        assert_eq!(
            BoardingPass::maybe_from("BFFFBBFRRR"),
            Some(BoardingPass { row: 70, col: 7 })
        );
        assert_eq!(
            BoardingPass::maybe_from("FFFBBBFRRR"),
            Some(BoardingPass { row: 14, col: 7 })
        );
        assert_eq!(
            BoardingPass::maybe_from("BBFFBBFRLL"),
            Some(BoardingPass { row: 102, col: 4 })
        );
    }

    #[test]
    fn seat_id() {
        assert_eq!(BoardingPass { row: 70, col: 7 }.seat_id(), 567);
        assert_eq!(BoardingPass { row: 14, col: 7 }.seat_id(), 119);
        assert_eq!(BoardingPass { row: 102, col: 4 }.seat_id(), 820);
    }
}
