use crate::maybe_from::MaybeFrom;
use intbits::Bits;
use itertools::Itertools;
use num::traits::real::Real;
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Uint36 {
    bits: [bool; 36],
}

impl MaybeFrom<&str> for Uint36 {
    fn maybe_from(value: &str) -> Option<Self> {
        value.parse::<u64>().ok().and_then(|i| Self::maybe_from(i))
    }
}

impl Into<u64> for Uint36 {
    fn into(self) -> u64 {
        self.bits
            .iter()
            .enumerate()
            .fold(0u64, |acc, (index, bit)| acc.with_bit(35 - index, *bit))
    }
}

impl MaybeFrom<u64> for Uint36 {
    fn maybe_from(value: u64) -> Option<Self> {
        (0..=35)
            .rev()
            .map(|bit| value.bit(bit))
            .collect_vec()
            .try_into()
            .ok()
            .map(|bits| Self { bits })
    }
}

#[derive(Debug, Copy, Clone)]
struct BitMask {
    bits: [MaskBit; 36],
}

impl BitMask {
    fn transform_value(&self, value: &Uint36) -> Uint36 {
        use MaskBit::*;
        Uint36 {
            bits: self
                .bits
                .iter()
                .zip(value.bits.iter())
                .map(|(mask, bit)| match mask {
                    Zero => false,
                    One => true,
                    X => *bit,
                })
                .collect_vec()
                .try_into()
                .expect("failed to apply mask"),
        }
    }

    fn transform_memory_address(&self, address: &Uint36) -> Vec<u64> {
        use MaskBit::*;
        let fluctuating_indices = self
            .bits
            .iter()
            .enumerate()
            .filter_map(|(index, mask)| match mask {
                X => Some(index),
                _ => None,
            })
            .collect_vec();

        let indexmap: HashMap<usize, usize> = HashMap::from_iter(
            fluctuating_indices
                .iter()
                .enumerate()
                .map(|(index, fluctuating_index)| (*fluctuating_index, index)),
        );

        (0..fluctuating_indices.len())
            .map(|_| &[true, false])
            .multi_cartesian_product()
            .map(|values| {
                let bits = self
                    .bits
                    .iter()
                    .zip(address.bits.iter())
                    .enumerate()
                    .map(|(index, (mask, bit))| match mask {
                        Zero => *bit,
                        One => true,
                        X => values[indexmap[&index]].clone(),
                    })
                    .collect_vec()
                    .try_into()
                    .expect("failed to convert back");
                Uint36 { bits }.into()
            })
            .collect_vec()
    }
}

impl Default for BitMask {
    fn default() -> Self {
        Self {
            bits: [MaskBit::X; 36],
        }
    }
}

impl MaybeFrom<&str> for BitMask {
    fn maybe_from(value: &str) -> Option<Self> {
        value
            .chars()
            .flat_map(|c| MaskBit::maybe_from(c))
            .collect_vec()
            .try_into()
            .ok()
            .map(|bits| Self { bits })
    }
}

#[derive(Debug, Copy, Clone)]
enum MaskBit {
    Zero,
    One,
    X,
}

impl MaybeFrom<char> for MaskBit {
    fn maybe_from(value: char) -> Option<Self> {
        use MaskBit::*;
        match value {
            '0' => Some(Zero),
            '1' => Some(One),
            'X' => Some(X),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Bitmask(BitMask),
    Write { location: u64, value: Uint36 },
}

impl MaybeFrom<&str> for Instruction {
    fn maybe_from(value: &str) -> Option<Self> {
        use Instruction::*;
        let (lhs, rhs) = value.split_once(" = ")?;
        if lhs == "mask" {
            BitMask::maybe_from(rhs).map(|mask| Bitmask(mask))
        } else {
            lhs.chars()
                .filter(|c| c.is_numeric())
                .join("")
                .parse::<u64>()
                .ok()
                .and_then(|location| Uint36::maybe_from(rhs).map(|value| Write { location, value }))
        }
    }
}

#[derive(Debug, Default)]
struct Memory {
    ram: HashMap<u64, Uint36>,
    mask: BitMask,
}

impl Memory {
    fn apply_v1(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Bitmask(mask) => self.mask = *mask,
            Write { location, value } => {
                self.ram.insert(*location, self.mask.transform_value(value));
            }
        }
    }

    fn apply_v2(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Bitmask(mask) => self.mask = *mask,
            Write { location, value } => {
                for address in self.mask.transform_memory_address(
                    &Uint36::maybe_from(*location).expect("failed to transform"),
                ) {
                    self.ram.insert(address, *value);
                }
            }
        }
    }
}

pub fn run() {
    let instructions = include_str!("data/14/1")
        .lines()
        .flat_map(|line| Instruction::maybe_from(line))
        .collect_vec();
    let mut memory = Memory::default();
    for instruction in &instructions {
        memory.apply_v1(instruction);
    }
    println!(
        "{}",
        memory
            .ram
            .values()
            .map(|uint36| -> u64 { (*uint36).into() })
            .sum::<u64>()
    );
    let mut memory = Memory::default();
    for instruction in &instructions {
        memory.apply_v2(instruction);
    }
    println!(
        "{}",
        memory
            .ram
            .values()
            .map(|uint36| -> u64 { (*uint36).into() })
            .sum::<u64>()
    );
}

mod tests {
    use super::*;
    #[test]
    fn test_11() {
        let u36 = Uint36::maybe_from("11").expect("failed to parse");
        assert_eq!(
            u36.bits,
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, true, false, true, true
            ]
        );
        let i: u64 = u36.into();
        assert_eq!(i, 11);
    }

    #[test]
    fn transform_address() {
        let address = Uint36::maybe_from("42").expect("failed to parse");
        let mask =
            BitMask::maybe_from("000000000000000000000000000000X1001X").expect("failed to parse");
        assert_eq!(
            mask.transform_memory_address(&address)
                .iter()
                .sorted()
                .cloned()
                .collect_vec(),
            vec![26, 27, 58, 59]
        );
    }
}
