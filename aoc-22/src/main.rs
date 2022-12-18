use crate::{
    day_exec::DayExecutor,
    days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5},
};

pub mod day_exec;
pub mod days;

fn main() {
    let input =
        std::fs::read_to_string("./input/day05.part1.txt").expect("Failed to read input file");

    let exec = Day5 {};
    let part1_solution = exec.exec_part1(input.clone());
    let part2_solution = exec.exec_part2(input);

    println!("Part 1:\n\t{}", part1_solution);
    println!("Part 2:\n\t{}", part2_solution);
}
