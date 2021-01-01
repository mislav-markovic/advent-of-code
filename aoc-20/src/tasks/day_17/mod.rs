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

type TransformT = fn(&ConwayCube, &[&ConwayCube]) -> bool;
struct Grid {
  cubes: HashMap<Position, ConwayCube>,
  activation_fn: TransformT,
  deactivation_fn: TransformT,
}

impl Grid {
  fn new(
    cubes: HashMap<Position, ConwayCube>,
    activation_fn: TransformT,
    deactivation_fn: TransformT,
  ) -> Self {
    Self {
      cubes,
      activation_fn,
      deactivation_fn,
    }
  }

  fn active_cubes(&self) -> usize {
    self.cubes.values().filter(|cube| cube.active).count()
  }

  fn advance_time(&mut self) {
    let check_iter = self
      .cubes
      .iter()
      .filter(|(_, cube)| cube.active)
      .flat_map(|(pos, _)| {
        let mut v = pos.neighbours();
        v.push(*pos);
        v.into_iter()
      })
      .collect::<Vec<_>>();

    for pos in check_iter.into_iter() {
      let neighbours = pos
        .neighbours()
        .into_iter()
        .map(|n_pos| {
          self
            .cubes
            .get(&n_pos)
            .unwrap_or(&ConwayCube { active: false })
        })
        .collect::<Vec<_>>();

      let current_cube = self
        .cubes
        .get(&pos)
        .unwrap_or(&ConwayCube { active: false })
        .clone();

      let should_flip = if current_cube.active {
        (self.deactivation_fn)(&current_cube, &neighbours)
      } else {
        (self.activation_fn)(&current_cube, &neighbours)
      };

      if should_flip {
        self.cubes.insert(
          pos,
          ConwayCube {
            active: !current_cube.active,
          },
        );
      }
    }
  }
}
struct GridBuilder {
  cubes: Option<HashMap<Position, ConwayCube>>,
  activation_fn: Option<TransformT>,
  deactivation_fn: Option<TransformT>,
}

impl GridBuilder {
  fn new() -> Self {
    Self {
      cubes: None,
      activation_fn: None,
      deactivation_fn: None,
    }
  }

  fn with_rows(mut self, rows: Vec<Row>) -> Self {
    self.cubes = Some(
      rows
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
    );
    self
  }

  fn with_activation_fn(mut self, func: TransformT) -> Self {
    self.activation_fn = Some(func);
    self
  }

  fn with_deactivation_fn(mut self, func: TransformT) -> Self {
    self.deactivation_fn = Some(func);
    self
  }

  fn build(self) -> Result<Grid, String> {
    let cubes = self.cubes.ok_or("Starting cubes not set!".to_string())?;
    let activation_fn = self
      .activation_fn
      .ok_or("Activation function not set!".to_string())?;
    let deactivation_fn = self
      .deactivation_fn
      .ok_or("Deactivation function not set".to_string())?;

    Ok(Grid::new(cubes, activation_fn, deactivation_fn))
  }
}

#[derive(Clone, Copy)]
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
  let result = part_1::boot_cycle(get_data(input_root), 6);
  println!("(Day 17, Part 1) Active cubes after boot cycle: {}", result);
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 17, Part 2) Not Implemented");
}

fn get_data(root: &str) -> Vec<Row> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_17.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Row>(&path, "")
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
