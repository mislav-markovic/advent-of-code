mod part_1;
mod part_2;

pub fn solve_part_1(input_root: &str) {
  println!("(Day 10, Part 1) Not Implemented");
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 10, Part 2) Not Implemented");
}

fn get_data(root: &str) -> Vec {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_12.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input(&path, "\r\n")
}
