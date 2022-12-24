use crate::{
    day_exec::DayExecutor,
    days::{
        day1::Day1, day10::Day10, day11::Day11, day12::Day12, day2::Day2, day3::Day3, day4::Day4,
        day5::Day5, day6::Day6, day7::Day7, day8::Day8, day9::Day9,
    },
};

pub mod day_exec;
pub mod days;

fn main() {
    let input = std::fs::read_to_string("./input/day12.part1.part2.txt")
        .expect("Failed to read input file");

    let exec = Day12 {};
    let part1_solution = exec.exec_part1(input.clone());
    // let part1_solution = exec.exec_part1(EXAMPLE.to_owned());

    let part2_solution = exec.exec_part2(input);
    // let part2_solution = exec.exec_part2(EXAMPLE.to_string());

    println!("Part 1:\n\t{}", part1_solution);
    println!("Part 2:\n\t{}", part2_solution);
}

const EXAMPLE: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
