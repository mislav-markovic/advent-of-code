use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day1;

impl DayExecutor for Day1 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Max number of calories carried by single elf: {}",
            part1_solution(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Sum of top 3 calories: {}", part2_solution(&input)))
    }
}

fn part1_solution(input: &str) -> u32 {
    *make_total_cal_vec_sorted(&input).last().unwrap()
}

fn part2_solution(input: &str) -> u32 {
    make_total_cal_vec_sorted(&input)
        .into_iter()
        .rev()
        .take(3)
        .sum()
}

fn make_total_cal_vec_sorted(input: &str) -> Vec<u32> {
    let mut inv = input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|x| {
            x.parse::<Inventory>()
                .expect("Expected to parse group of lines as elf inventory")
        })
        .map(|inv| inv.total_calories())
        .collect::<Vec<_>>();
    inv.sort();
    inv
}
struct Snack {
    calories: u32,
}

impl Snack {
    fn new(cal: u32) -> Self {
        Self { calories: cal }
    }
}

#[derive(Debug)]
struct ParseSnackError;
impl FromStr for Snack {
    type Err = ParseSnackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .parse::<u32>()
            .map(|num| Self::new(num))
            .map_err(|_| Self::Err {})
    }
}

struct Inventory {
    snacks: Vec<Snack>,
}

impl Inventory {
    fn new(snacks: Vec<Snack>) -> Self {
        Self { snacks }
    }

    fn total_calories(&self) -> u32 {
        self.snacks.iter().map(|s| s.calories).sum()
    }
}

#[derive(Debug)]
struct ParseInventoryError;
impl FromStr for Inventory {
    type Err = ParseInventoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .lines()
            .map(|l| l.trim().parse::<Snack>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self::new(v))
            .map_err(|_| ParseInventoryError {})
    }
}
