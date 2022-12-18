use crate::{day_exec::DayExecutor, days::day1::Day1};

pub mod day_exec;
pub mod days;

fn main() {
    let input =
        std::fs::read_to_string("./input/day1.part1.txt").expect("Failed to read input file");

    let exec = Day1 {};
    let day1_part1_solution = exec.exec_part1(input.clone());
    let day1_part2_solution = exec.exec_part2(input);

    println!("Part 1:\n\t{}", day1_part1_solution);
    println!("Part 2:\n\t{}", day1_part2_solution);
}
