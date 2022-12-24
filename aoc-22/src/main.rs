use crate::{
    day_exec::DayExecutor,
    days::{
        day1::Day1, day10::Day10, day11::Day11, day12::Day12, day13::Day13, day2::Day2, day3::Day3,
        day4::Day4, day5::Day5, day6::Day6, day7::Day7, day8::Day8, day9::Day9,
    },
};

pub mod day_exec;
pub mod days;

fn main() {
    let input = std::fs::read_to_string("./input/day13.part1.part2.txt")
        .expect("Failed to read input file");

    let exec = Day13 {};
    let part1_solution = exec.exec_part1(input.clone());
    // let part1_solution = exec.exec_part1(EXAMPLE.to_owned());

    let part2_solution = exec.exec_part2(input);
    // let part2_solution = exec.exec_part2(EXAMPLE.to_string());

    println!("Part 1:\n\t{}", part1_solution);
    println!("Part 2:\n\t{}", part2_solution);
}

const EXAMPLE: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
