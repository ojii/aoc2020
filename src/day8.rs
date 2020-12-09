use crate::maybe_from::MaybeFrom;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl MaybeFrom<&str> for Opcode {
    fn maybe_from(value: &str) -> Option<Self> {
        let (operation, argument) = value.split_once(" ")?;
        argument
            .parse::<i32>()
            .ok()
            .and_then(|arg| match operation {
                "nop" => Some(Opcode::Nop(arg)),
                "acc" => Some(Opcode::Acc(arg)),
                "jmp" => Some(Opcode::Jmp(arg)),
                _ => None,
            })
    }
}

#[derive(Debug)]
enum GameboyState {
    Normal,
    Crashed,
    Booted,
    Looped,
}

#[derive(Debug, Eq, PartialEq)]
struct Gameboy {
    rom: Vec<Opcode>,
    ins: usize,
    acc: i32,
    seen: HashSet<usize>,
}

impl Gameboy {
    fn advance(&mut self) -> GameboyState {
        if self.ins == self.rom.len() {
            GameboyState::Booted
        } else {
            let (ins, acc, state) = self
                .rom
                .get(self.ins)
                .map(|operation| match operation {
                    Opcode::Nop(_) => (self.ins + 1, self.acc, GameboyState::Normal),
                    Opcode::Acc(value) => (self.ins + 1, self.acc + value, GameboyState::Normal),
                    Opcode::Jmp(by) => (
                        (self.ins as i32 + by) as usize,
                        self.acc,
                        GameboyState::Normal,
                    ),
                })
                .unwrap_or((self.ins, self.acc, GameboyState::Crashed));
            self.ins = ins;
            self.acc = acc;
            state
        }
    }

    fn flip(&self, at: usize) -> Option<Self> {
        self.rom
            .get(at)
            .and_then(|operation| match operation {
                Opcode::Nop(value) => Some(patch_rom(&self.rom, at, Opcode::Jmp(value.clone()))),
                Opcode::Acc(_) => None,
                Opcode::Jmp(by) => Some(patch_rom(&self.rom, at, Opcode::Nop(by.clone()))),
            })
            .map(|rom| Gameboy {
                rom,
                ins: 0,
                acc: 0,
                seen: HashSet::new(),
            })
    }
}

impl From<&str> for Gameboy {
    fn from(value: &str) -> Self {
        Self {
            rom: value
                .lines()
                .flat_map(|line| Opcode::maybe_from(line))
                .collect(),
            ins: 0,
            acc: 0,
            seen: HashSet::new(),
        }
    }
}

fn patch_rom(vec: &[Opcode], at: usize, with: Opcode) -> Vec<Opcode> {
    vec.iter()
        .take(at)
        .chain(&[with])
        .chain(vec.iter().skip(at + 1))
        .cloned()
        .collect_vec()
}

fn boot(gameboy: &mut Gameboy) -> GameboyState {
    let mut seen = HashSet::<usize>::new();
    while seen.insert(gameboy.ins) {
        match gameboy.advance() {
            GameboyState::Normal => (),
            state => return state,
        }
    }
    GameboyState::Looped
}

pub fn run() {
    let mut gameboy = Gameboy::from(include_str!("data/8/1"));
    boot(&mut gameboy);
    println!("{}", gameboy.acc);
    println!(
        "{}",
        (0..gameboy.rom.len())
            .find_map(|at| gameboy
                .flip(at)
                .as_mut()
                .and_then(|patched| match boot(patched) {
                    GameboyState::Booted => Some(patched.acc),
                    _ => None,
                }))
            .expect("no booted gamebody found")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(flip = {
        0,
        1,
        2,
        3
    }, result = {
        Some(Gameboy{
            rom: vec![Opcode::Nop(1), Opcode::Acc(2), Opcode::Nop(3)],
            ins: 0,
            acc: 0,
            seen: HashSet::new()
        }),
        None,
        Some(Gameboy{
            rom: vec![Opcode::Jmp(1), Opcode::Acc(2), Opcode::Jmp(3)],
            ins: 0,
            acc: 0,
            seen: HashSet::new()
        }),
        None
    })]
    #[test]
    fn tests_patch_rom(flip: usize, result: Option<Gameboy>) {
        let gameboy = Gameboy {
            rom: vec![Opcode::Jmp(1), Opcode::Acc(2), Opcode::Nop(3)],
            ins: 0,
            acc: 0,
            seen: HashSet::new(),
        };
        assert_eq!(gameboy.flip(flip), result);
    }
}
