use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Group {
    people: Vec<HashSet<char>>,
}

impl Group {
    fn yes_count(&self) -> usize {
        self.people
            .iter()
            .fold(HashSet::<char>::new(), |a, b| (&a | b).clone())
            .len()
    }

    fn all_yes_count(&self) -> usize {
        self.people
            .iter()
            .fold(HashSet::from_iter(('a'..='z')), |a, b| (&a & b).clone())
            .len()
    }
}

// fn oneline() {
//     println!(
//         "{}",
//         include_str!("data/6/1")
//             .split("\n\n")
//             .map(|group| group.lines().map(|line| HashSet::from_iter(line.chars())))
//             .map(|people| people.fold(
//                 (HashSet::<char>::new(), HashSet::from_iter(('a'..='z'))),
//                 |a, b| ((&a.0 | &b).clone(), (&a.1 & &b).clone())
//             ))
//             .map(|tuple| (tuple.0.len(), tuple.1.len()))
//             .fold([0usize, 0usize], |a, b| [a[0] + b.0, a[1] + b.1])
//             .iter()
//             .join("\n")
//     )
// }

pub fn run() {
    let input = include_str!("data/6/1");
    let groups = input
        .split("\n\n")
        .map(|group| Group {
            people: group
                .lines()
                .map(|line| HashSet::from_iter(line.chars()))
                .collect_vec(),
        })
        .collect_vec();

    println!(
        "{}",
        groups.iter().map(|group| group.yes_count()).sum::<usize>()
    );
    println!(
        "{}",
        groups
            .iter()
            .map(|group| group.all_yes_count())
            .sum::<usize>()
    );
}
