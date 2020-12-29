use std::str::FromStr;

mod part_1;
mod part_2;

enum CardinalDirection {
  North(MoveDistance),
  South(MoveDistance),
  East(MoveDistance),
  West(MoveDistance),
}
enum RotateDirection {
  Left(RotationDistance),
  Right(RotationDistance),
}
enum Action {
  Move(CardinalDirection),
  Rotate(RotateDirection),
  Forward(MoveDistance),
}
#[derive(Clone, Copy)]
struct MoveDistance(usize);
#[derive(Clone, Copy)]
struct RotationDistance(usize);

impl FromStr for Action {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut char_iter = input.trim().chars();
    let action_str = char_iter.next().unwrap();
    let value_str = char_iter.collect::<String>();
    let value = value_str
      .parse::<usize>()
      .map_err(|_| "Could not parse action value".to_string())?;

    match action_str {
      'N' => Ok(Action::Move(CardinalDirection::North(MoveDistance(value)))),
      'S' => Ok(Action::Move(CardinalDirection::South(MoveDistance(value)))),
      'E' => Ok(Action::Move(CardinalDirection::East(MoveDistance(value)))),
      'W' => Ok(Action::Move(CardinalDirection::West(MoveDistance(value)))),
      'L' => Ok(Action::Rotate(RotateDirection::Left(RotationDistance(
        value,
      )))),
      'R' => Ok(Action::Rotate(RotateDirection::Right(RotationDistance(
        value,
      )))),
      'F' => Ok(Action::Forward(MoveDistance(value))),
      _ => Err("Could not parse action type".to_string()),
    }
  }
}

type PositionT = (isize, isize);
fn manhattan_distance(start: &PositionT, end: &PositionT) -> usize {
  ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize
}

fn get_data(root: &str) -> Vec<Action> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_12.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Action>(&path, "\r\n")
}
pub fn solve_part_1(input_root: &str) {
  let result = part_1::manhattan_distance_after_navigation(get_data(input_root), (0, 0));
  println!(
    "(Day 12, Part 1) Manhattan distance from start after following navigation course is {}",
    result
  );
}

pub fn solve_part_2(input_root: &str) {
  let result =
    part_2::manhattan_distance_after_waypoint_navigation(get_data(input_root), (0, 0), (10, 1));
  println!(
    "(Day 12, Part 2) Manhattan distance from start after following navigation course is {}",
    result
  );
}
