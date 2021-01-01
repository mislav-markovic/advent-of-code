use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

mod part_1;
mod part_2;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
  x: isize,
  y: isize,
  z: isize,
  w: isize,
}

impl Position {
  fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
    Self { x, y, z, w }
  }

  fn neighbours(&self, include_4th_dimension: bool) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    if include_4th_dimension {
      for x_offset in -1isize..=1 {
        for y_offset in -1isize..=1 {
          for z_offset in -1isize..=1 {
            for w_offset in -1isize..=1 {
              if x_offset == 0 && y_offset == 0 && z_offset == 0 && w_offset == 0 {
                continue;
              }
              result.push(Position::new(
                self.x + x_offset,
                self.y + y_offset,
                self.z + z_offset,
                self.w + w_offset,
              ));
            }
          }
        }
      }
    } else {
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
              0,
            ));
          }
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
  include_4th_dimension: bool,
}

impl Grid {
  fn new(
    cubes: HashMap<Position, ConwayCube>,
    activation_fn: TransformT,
    deactivation_fn: TransformT,
    include_4th_dimension: bool,
  ) -> Self {
    Self {
      cubes,
      activation_fn,
      deactivation_fn,
      include_4th_dimension,
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
        let mut v = pos.neighbours(self.include_4th_dimension);
        v.push(*pos);
        v.into_iter()
      })
      .collect::<HashSet<_>>();

    let mut diff: Vec<(Position, ConwayCube)> = Vec::new();
    for pos in check_iter.into_iter() {
      let neighbours = pos
        .neighbours(self.include_4th_dimension)
        .into_iter()
        .filter_map(|n_pos| self.cubes.get(&n_pos))
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
        diff.push((
          pos,
          ConwayCube {
            active: !current_cube.active,
          },
        ));
      }
    }

    for (pos, cube) in diff {
      self.cubes.insert(pos, cube);
    }
  }
}
struct GridBuilder {
  cubes: Option<HashMap<Position, ConwayCube>>,
  activation_fn: Option<TransformT>,
  deactivation_fn: Option<TransformT>,
  include_4th_dimension: bool,
}

impl GridBuilder {
  fn new() -> Self {
    Self {
      cubes: None,
      activation_fn: None,
      deactivation_fn: None,
      include_4th_dimension: false,
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
            .map(move |(x, cube)| (Position::new(x as isize, y as isize, 0isize, 0isize), cube))
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

  fn with_4th_dimension(mut self) -> Self {
    self.include_4th_dimension = true;
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

    Ok(Grid::new(
      cubes,
      activation_fn,
      deactivation_fn,
      self.include_4th_dimension,
    ))
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
  let result = part_2::boot_cycle(get_data(input_root), 6);
  println!("(Day 17, Part 2) Active cubes after boot cycle: {}", result);
}

fn get_data(root: &str) -> Vec<Row> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_17.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Row>(&path, "\r\n")
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn correct_number_of_position_neighbours_generated() {
    let pos = Position::new(-100, 100, 50, 0);
    let neighbours = pos.neighbours(false);

    assert_eq!(26, neighbours.len());
    assert!(!neighbours.contains(&pos));
  }
  #[test]
  fn correct_number_of_position_neighbours_generated_in_4th_dimension() {
    let pos = Position::new(-100, 100, 50, 0);
    let neighbours = pos.neighbours(true);

    assert_eq!(80, neighbours.len());
    assert!(!neighbours.contains(&pos));
  }
  #[test]
  fn position_doesnt_generate_duplicate_neighbours() {
    let pos = Position::new(-100, 100, 50, 0);
    let neighbours = pos.neighbours(false).into_iter().collect::<HashSet<_>>();

    assert_eq!(26, neighbours.len());
    assert!(!neighbours.contains(&pos));
  }

  #[test]
  fn position_generate_correct_neighbours() {
    let pos = Position::new(-100, 100, 50, 0);
    let neighbours = pos.neighbours(false).into_iter().collect::<HashSet<_>>();

    assert!(neighbours
      .iter()
      .all(|n| abs_diff_less_than(&n.x, &pos.x, 2)
        && abs_diff_less_than(&n.y, &pos.y, 2)
        && abs_diff_less_than(&n.z, &pos.z, 2)));
  }

  fn abs_diff_less_than(a: &isize, b: &isize, less_than: isize) -> bool {
    (a - b).abs() < less_than
  }
}
