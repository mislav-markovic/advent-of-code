use crate::{
    day_exec::DayExecutor,
    days::{
        day1::Day1, day10::Day10, day11::Day11, day2::Day2, day3::Day3, day4::Day4, day5::Day5,
        day6::Day6, day7::Day7, day8::Day8, day9::Day9,
    },
};

pub mod day_exec;
pub mod days;

fn main() {
    let input = std::fs::read_to_string("./input/day11.part1.part2.txt")
        .expect("Failed to read input file");

    let exec = Day11 {};
    let part1_solution = exec.exec_part1(input.clone());
    let part2_solution = exec.exec_part2(EXAMPLE.to_string());

    println!("Part 1:\n\t{}", part1_solution);
    println!("Part 2:\n\t{}", part2_solution);
}

const EXAMPLE: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
