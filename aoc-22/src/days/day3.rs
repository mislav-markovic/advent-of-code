use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day3;

impl DayExecutor for Day3 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Sum of priorities of items contained in both compartments of each rucksack: {}",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new("TODO!")
    }
}

fn solve_part1(input: &str) -> u32 {
    let rucksacks = input
        .lines()
        .map(|l| {
            l.parse::<Rucksack>()
                .expect("Failed to parse line as rucksack")
        })
        .collect::<Vec<_>>();

    rucksacks
        .iter()
        .map(|r| find_overlapping_item(r).priority)
        .sum()
}

fn find_overlapping_item(rucksack: &Rucksack) -> Item {
    let lhs = rucksack.first_compartment();
    let rhs = rucksack.second_compartment();
    lhs.iter()
        .skip_while(|item| !rhs.contains(item))
        .next()
        .unwrap()
        .clone()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    id: char,
    priority: u32,
}

struct ItemError(char);
impl Item {
    fn new(id: char) -> Result<Self, ItemError> {
        if id.is_ascii_alphabetic() {
            Ok(Self {
                id,
                priority: get_priorty(id),
            })
        } else {
            Err(ItemError { 0: id })
        }
    }
}

fn get_priorty(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!("Priority precondition violated"),
    }
}

struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    fn first_compartment(&self) -> &[Item] {
        &self.items[..self.items.len() / 2]
    }

    fn second_compartment(&self) -> &[Item] {
        &self.items[self.items.len() / 2..]
    }
}

#[derive(Debug)]
struct RuckaskParseError(String);

impl FromStr for Rucksack {
    type Err = RuckaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .trim()
            .chars()
            .map(|c| Item::new(c))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|ItemError(c)| Self::Err {
                0: format!(
                    "Failed to parse line '{}' as rucksak! First error on char '{}'",
                    s, c
                ),
            })?;

        Ok(Self::new(items))
    }
}
