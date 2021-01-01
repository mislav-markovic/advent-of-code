use std::{collections::HashMap, str::FromStr};

mod part_1;
mod part_2;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
  x: isize,
  y: isize,
  z: isize,
}

impl Position {
  fn new(x: isize, y: isize, z: isize) -> Self {
    Self { x, y, z }
  }

  fn neighbours(&self) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();

    for x_offset in -1isize..=1 {
      for y_offset in -1isize..=1 {
        for z_offset in -1isize..=1 {
          if x_offset == 0 && y_offset == 0 && z_offset == 0 {
            continue;
          }
          result.push(Position::new(
            self.x + x_offset,
            self.y + y_offset,
            self.z + z_offset,
          ));
        }
      }
    }
    result
  }
}
struct Grid {
  cubes: HashMap<Position, ConwayCube>,
}

impl Grid {
  fn from_rows(rows: Vec<Row>) -> Self {
    Self {
      cubes: rows
        .into_iter()
        .enumerate()
        .flat_map(move |(y, row)| {
          row
            .cubes
            .into_iter()
            .enumerate()
            .map(move |(x, cube)| (Position::new(x as isize, y as isize, 0isize), cube))
        })
        .collect::<HashMap<_, _>>(),
    }
  }
}
struct ConwayCube {
  active: bool,
}

impl FromStr for ConwayCube {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let trimmed = s.trim();

    if trimmed.len() > 1 {
      Err(format!("Cube must be 1 char: '{}'", trimmed))
    } else {
      let symbol = trimmed.chars().next().unwrap();
      match symbol {
        '#' => Ok(Self { active: true }),
        '.' => Ok(Self { active: false }),
        _ => Err(format!("Unknown cube state: '{}'", symbol)),
      }
    }
  }
}

pub fn solve_part_1(input_root: &str) {
  println!("(Day 17, Part 1) Not Implemented");
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 17, Part 2) Not Implemented");
}

fn get_data(root: &str) -> Grid {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_17.input.txt", root);
  println!("Reading input from '{}'", path);

  Grid::from_rows(fr::parse_input::<Row>(&path, ""))
}

struct Row {
  cubes: Vec<ConwayCube>,
}

impl FromStr for Row {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      cubes: s
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<ConwayCube>().unwrap())
        .collect::<Vec<_>>(),
    })
  }
}
