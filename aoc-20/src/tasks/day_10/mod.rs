mod part_1;
mod part_2;

fn get_data(root: &str) -> Vec<usize> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_10.input.txt", root);
  println!("Reading input from '{}'", &path);
  fr::parse_input::<usize>(&path, "\r\n")
}

pub fn solve_part_1(input_root: &str) {
  let result = part_1::mul_1count_and_3count(get_data(input_root));
  println!("(Day 10, Part 1) 1counter * 3counter = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let result = part_2::number_of_adapter_arangments(get_data(input_root));
  println!(
    "(Day 10, Part 2) Number of possible adapter arangments is {}",
    result
  );
}
