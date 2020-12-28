use std::{fmt, fmt::Display, str::FromStr};

mod part_1;
mod part_2;

#[derive(Debug, PartialEq, Eq)]
enum Position {
  Floor,
  Occupied,
  Empty,
}

impl FromStr for Position {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let trimmed = input.trim();

    if trimmed.len() > 1 {
      Err("Invalid input, can't parse position from more than 1 character".to_string())
    } else {
      match trimmed.chars().next().unwrap() {
        'L' => Ok(Position::Empty),
        '#' => Ok(Position::Occupied),
        '.' => Ok(Position::Floor),
        any => Err(format!("Unkown representation of position: '{}'", any)),
      }
    }
  }
}

fn position_to_str(pos: &Position) -> String {
  match pos {
    Position::Floor => ".".to_string(),
    Position::Occupied => "#".to_string(),
    Position::Empty => "L".to_string(),
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
  positions: Vec<Position>,
}

impl FromStr for Row {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let positions = s
      .trim()
      .chars()
      .map(|c| c.to_string().parse::<Position>())
      .collect::<Result<Vec<_>, _>>()?;
    Ok(Self::new(positions))
  }
}

impl Row {
  fn new(positions: Vec<Position>) -> Self {
    Self { positions }
  }
}
fn row_to_str(row: &Row) -> String {
  let mut result = String::with_capacity(row.positions.len());
  result.extend(row.positions.iter().map(position_to_str));
  result
}
type TransformT = fn((usize, usize), &Position, &Vec<Row>) -> Position;
struct WaitingArea {
  rows: Vec<Row>,
  position_transformation: TransformT,
}

impl Display for WaitingArea {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", area_to_str(&self))
  }
}

fn area_to_str(area: &WaitingArea) -> String {
  let mut result =
    String::with_capacity(area.rows.len() * area.rows.first().unwrap().positions.len());
  result.extend(
    area
      .rows
      .iter()
      .map(row_to_str)
      .map(|s| format!("{}\r\n", s)),
  );
  result
}

impl PartialEq for WaitingArea {
  fn eq(&self, other: &Self) -> bool {
    self.rows == other.rows
  }
}

impl Eq for WaitingArea {}

impl WaitingArea {
  fn new(rows: Vec<Row>, position_transformation: TransformT) -> Self {
    Self {
      rows,
      position_transformation,
    }
  }

  fn advance_time(&self) -> Self {
    let rows = self
      .rows
      .iter()
      .enumerate()
      .map(|(y, row)| {
        Row::new(
          row
            .positions
            .iter()
            .enumerate()
            .map(|(x, position)| (self.position_transformation)((x, y), position, &self.rows))
            .collect(),
        )
      })
      .collect();
    Self::new(rows, self.position_transformation)
  }
}

fn count_occupied_seats(area: &WaitingArea) -> usize {
  area
    .rows
    .iter()
    .flat_map(|row| row.positions.iter())
    .filter(|&pos| *pos == Position::Occupied)
    .count()
}

type CoordinateT = (usize, usize);

fn is_valid_offset(coordinate: usize, offset: isize, max: usize) -> bool {
  if offset >= 0 {
    coordinate + (offset as usize) < max
  } else {
    coordinate >= (offset.abs() as usize)
  }
}

fn usize_isize_addition(unum: usize, inum: isize) -> usize {
  if inum < 0 {
    unum - (inum.abs() as usize)
  } else {
    unum + (inum as usize)
  }
}

fn product(xs: &[isize], ys: &[isize]) -> Vec<(isize, isize)> {
  xs.iter()
    .flat_map(|&x| ys.iter().clone().map(move |&y| (x, y)))
    .collect()
}
pub fn solve_part_1(input_root: &str) {
  let result = part_1::occupied_seats_after_changes_stop(get_data(input_root));
  println!(
    "(Day 11, Part 1) Number of occupied seats after changes stop {}",
    result
  );
}

pub fn solve_part_2(input_root: &str) {
  let result = part_2::count_occupied_seats_after_changes_stop(get_data(input_root));
  println!(
    "(Day 11, Part 2) Number of occupied seats after changes stop {}",
    result
  );
}

fn get_data(root: &str) -> Vec<Row> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_11.input.txt", root);
  println!("Reading input from {}", &path);
  fr::parse_input::<Row>(&path, "\r\n")
}
