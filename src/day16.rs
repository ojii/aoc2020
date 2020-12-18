use crate::maybe_from::MaybeFrom;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Range {
    inclusive_low: usize,
    inclusive_high: usize,
}

impl Range {
    fn contains(&self, value: &usize) -> bool {
        value >= &self.inclusive_low && value <= &self.inclusive_high
    }
}

impl MaybeFrom<&str> for Range {
    fn maybe_from(value: &str) -> Option<Self> {
        let (start, end) = value.split_once("-")?;
        let inclusive_low = start.parse::<usize>().ok()?;
        let inclusive_high = end.parse::<usize>().ok()?;
        Some(Self {
            inclusive_low,
            inclusive_high,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    low_range: Range,
    high_range: Range,
}

impl Rule {
    fn valid(&self, value: &usize) -> bool {
        self.low_range.contains(value) || self.high_range.contains(value)
    }
}

impl MaybeFrom<&str> for Rule {
    fn maybe_from(value: &str) -> Option<Self> {
        let (lhs, rhs) = value.split_once(" or ")?;
        let low_range = Range::maybe_from(lhs)?;
        let high_range = Range::maybe_from(rhs)?;
        Some(Self {
            low_range,
            high_range,
        })
    }
}

#[derive(Debug)]
struct Field {
    name: String,
    rule: Rule,
}

impl MaybeFrom<&str> for Field {
    fn maybe_from(value: &str) -> Option<Self> {
        let (name, rules) = value.split_once(": ")?;
        Rule::maybe_from(rules).map(|rule| Self {
            name: name.to_string(),
            rule,
        })
    }
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<usize>,
}

impl Ticket {
    fn error_rate(&self, rules: &[Rule]) -> usize {
        self.numbers
            .iter()
            .filter(|num| !rules.iter().any(|rule| rule.valid(num)))
            .sum()
    }
}

impl From<&str> for Ticket {
    fn from(value: &str) -> Self {
        Self {
            numbers: value.split(",").flat_map(|num| num.parse().ok()).collect(),
        }
    }
}

#[derive(Debug)]
struct Notes {
    fields: Vec<Field>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Notes {
    fn rules(&self) -> Vec<Rule> {
        self.fields.iter().map(|field| field.rule).collect()
    }

    fn error_rate(&self) -> usize {
        let rules = self.rules();
        self.nearby_tickets
            .iter()
            .map(|ticket| ticket.error_rate(&rules))
            .sum()
    }

    // fn valid_tickets(&self) -> Vec<Ticket> {
    //     let rules = self.rules();
    //     self.nearby_tickets
    //         .iter()
    //         .chain(&[&self.your_ticket])
    //         .filter(|ticket| ticket.error_rate(&rules) == 0)
    //         .cloned()
    //         .collect()
    // }
    //
    // fn decode_fields_on_your_ticket(&self) -> HashMap<String, usize> {
    //     // let candidates = HashSet::from_iter(self.fields.iter().map(|field| field.name));
    //     // let mut columns = vec![candidates; candidates.len()];
    //     // for ticket in &self.valid_tickets() {
    //     //     for (num, column) in ticket.numbers.iter().zip(columns.iter_mut()) {
    //     //
    //     //     }
    //     // }
    //     HashMap::new()
    //     // let candidates = (0..self.fields.len()).map(|_|)
    //     // Fieild
    // }
}

fn parse(input: &str) -> Option<Notes> {
    let (fields, tickets) = input.split_once("\n\nyour ticket:\n")?;
    let fields = fields
        .lines()
        .flat_map(|line| Field::maybe_from(line))
        .collect_vec();
    let (your_ticket, nearby_tickets) = tickets.split_once("\n\nnearby tickets:\n")?;
    let your_ticket = Ticket::from(your_ticket);
    let nearby_tickets = nearby_tickets
        .lines()
        .map(|line| Ticket::from(line))
        .collect_vec();
    Some(Notes {
        fields,
        your_ticket,
        nearby_tickets,
    })
}

pub fn run() {
    //     let input = "class: 1-3 or 5-7
    // row: 6-11 or 33-44
    // seat: 13-40 or 45-50
    //
    // your ticket:
    // 7,1,14
    //
    // nearby tickets:
    // 7,3,47
    // 40,4,50
    // 55,2,20
    // 38,6,12";
    let notes = parse(include_str!("data/16/1")).expect("failed to parse input");
    println!("{}", notes.error_rate());
}
