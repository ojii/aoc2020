use itertools::Itertools;

pub fn run() {
    let input = include_str!("data/10/1");
    let (ones, threes) = input
        .lines()
        .flat_map(|s| s.parse::<u32>().ok())
        .chain((0..=0))
        .sorted()
        .tuple_windows::<(_, _)>()
        .fold((0, 1), |(acc1, acc3), (a, b)| match b - a {
            1 => (acc1 + 1, acc3),
            3 => (acc1, acc3 + 1),
            _ => (acc1, acc3),
        });
    println!("{} * {} = {:?}", ones, threes, ones * threes);
}
