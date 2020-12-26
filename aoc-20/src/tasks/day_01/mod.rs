mod part_1;
mod part_2;

pub fn solve_part_1(input_root: &str) {
  let input = format!("{}/day_01.input.txt", input_root);
  println!("Reading input for day 1 from: '{}'", &input);

  let data = get_data(&input);
  let result = part_1::solve(&data);

  println!("(Day 1, Part 1) Solution: {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let input = format!("{}/day_01.input.txt", input_root);
  println!("Reading input for day 1 from: '{}'", &input);

  let data = get_data(&input);
  let result = part_2::solve(&data);

  println!("(Day 1, Part 2) Solution: {}", result);
}

fn get_data(path: &str) -> Vec<u64> {
  use crate::common::file_reader as fr;
  fr::parse_input::<u64>(path, "\r\n")
}
