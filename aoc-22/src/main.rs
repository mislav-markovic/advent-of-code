use std::str::FromStr;

pub mod day_exec;

fn main() {
    let input =
        std::fs::read_to_string("./input/day1.part1.txt").expect("Failed to read input file");

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
    let max_cals = inv.last().unwrap();
    let top_three_total: u32 = inv.iter().rev().take(3).sum();

    println!("Max number of calories: {}", max_cals);
    println!("Calories of top three carriers: {}", top_three_total);
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
