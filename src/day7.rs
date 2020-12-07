use crate::maybe_from::MaybeFrom;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct Rules {
    bag_rules: HashMap<String, HashMap<String, usize>>,
}

impl Rules {
    fn can_contain(&self, color: &str) -> HashSet<String> {
        let mut colors = HashSet::new();
        for (bag_color, rules) in &self.bag_rules {
            if rules.contains_key(color) {
                colors.insert(bag_color.clone());
                colors.extend(self.can_contain(&bag_color));
            }
        }
        colors
    }

    fn count_required_bags(&self, color: &str) -> usize {
        self.bag_rules
            .get(color)
            .map(|rules| {
                rules
                    .iter()
                    .map(|(key, value)| {
                        value.clone() + (value.clone() * self.count_required_bags(key))
                    })
                    .sum()
            })
            .unwrap_or(0)
    }
}

impl From<&str> for Rules {
    fn from(value: &str) -> Self {
        Rules {
            bag_rules: HashMap::from_iter(value.lines().flat_map(|line| {
                let (color, rules) = line.split_once(" bags contain ")?;
                if rules == "no other bags." {
                    Some((color.to_string(), HashMap::new()))
                } else {
                    Some((
                        color.to_string(),
                        HashMap::from_iter(rules.split(", ").flat_map(|rule| {
                            let (num, color) = rule.rsplit_once(" bag")?.0.split_once(" ")?;
                            Some((color.to_string(), num.parse::<usize>().ok()?))
                        })),
                    ))
                }
            })),
        }
    }
}

pub fn run() {
    let rules = Rules::from(include_str!("data/7/1"));
    println!("{}", rules.can_contain("shiny gold").len());
    println!("{}", rules.count_required_bags("shiny gold"));
}
