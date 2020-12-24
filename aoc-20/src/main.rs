mod common;
mod runner;
mod tasks;

fn main() {
  let input_root = "E:/Programming Projects/advent-of-code/aoc-20/input";
  println!("Solving day_01!");
  println!("Input root: {}", input_root);

  let result_part_01 = tasks::day_01::part_1::solve(input_root);
  println!("Solution for day_01-part_01: '{}'", result_part_01);

  let result_part_02 = tasks::day_01::part_2::solve(input_root);
  println!("Solution for day_01-part_02: '{}'", result_part_02);
}
