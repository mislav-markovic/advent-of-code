use std::str::FromStr;

mod part_1;
mod part_2;

struct Bus {
  id: Option<usize>,
}

impl FromStr for Bus {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parse_result = s.trim().parse::<usize>().ok();
    Ok(Self { id: parse_result })
  }
}

struct Schedule {
  earliest_departure: usize,
  bus_lines: Vec<Bus>,
}

impl FromStr for Schedule {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split_iter = s.trim().lines();
    let earliest_departure = split_iter.next().unwrap().parse::<usize>().unwrap();
    let bus_lines = split_iter
      .next()
      .unwrap()
      .split(",")
      .map(|id| id.parse::<Bus>().unwrap())
      .collect::<Vec<_>>();

    Ok(Self {
      earliest_departure,
      bus_lines,
    })
  }
}

pub fn solve_part_1(input_root: &str) {
  let result = part_1::mul_bus_id_and_wait_time(get_data(input_root));
  println!("(Day 13, Part 1) Result = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let result = part_2::earliest_timestamp(get_data(input_root));
  println!("(Day 13, Part 2) Result = {}", result);
}

fn get_data(root: &str) -> Schedule {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_13.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Schedule>(&path, "\r\n\r\n")
    .pop()
    .unwrap()
}
