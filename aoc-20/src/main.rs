#![feature(iterator_fold_self)]
#![feature(str_split_once)]

mod common;
mod runner;
mod tasks;

use tasks::{Day, Part};
fn main() {
  let input_root = "E:/Programming Projects/advent-of-code/aoc-20/input";
  tasks::dispatch(input_root, &Day::Day17, &Part::First);
  tasks::dispatch(input_root, &Day::Day17, &Part::Second);
}
