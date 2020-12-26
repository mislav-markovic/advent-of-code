mod common;
mod runner;
mod tasks;

fn main() {
  let input_root = "E:/Programming Projects/advent-of-code/aoc-20/input";
  tasks::day_05::solve_part_1(input_root);
  tasks::day_05::solve_part_2(input_root);
}
