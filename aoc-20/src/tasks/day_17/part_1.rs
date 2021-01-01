use super::{ConwayCube, Grid, GridBuilder, Position, Row};

pub(super) fn boot_cycle(starting_rows: Vec<Row>, boot_cycle_length: usize) -> usize {
  let mut grid = GridBuilder::new()
    .with_rows(starting_rows)
    .with_activation_fn(activation_fn)
    .with_deactivation_fn(deactivation_fn)
    .build()
    .unwrap();

  for i in 0..boot_cycle_length {
    grid.advance_time();
  }
  grid.active_cubes()
}

fn activation_fn(cube: &ConwayCube, neighbours: &[&ConwayCube]) -> bool {
  if !cube.active {
    let active_count = neighbours.iter().filter(|c| c.active).count();
    active_count == 3
  } else {
    false
  }
}

fn deactivation_fn(cube: &ConwayCube, neighbours: &[&ConwayCube]) -> bool {
  if cube.active {
    let active_count = neighbours.iter().filter(|c| c.active).count();
    !(active_count == 2 || active_count == 3)
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_data() -> String {
    ".#.
..#
###"
      .to_string()
  }

  fn get_data_real() -> String {
    "##...#.#
#..##..#
..#.####
.#..#...
########
######.#
.####..#
.###.#.."
      .to_string()
  }

  #[test]
  fn parse_and_active_cube_count_works() {
    let rows = get_data_real()
      .lines()
      .map(|l| l.parse::<super::super::Row>().unwrap())
      .collect::<Vec<_>>();
    let result = super::boot_cycle(rows, 0);

    assert_eq!(39usize, result);
  }

  #[test]
  fn boot_cycle_with_0_turns_produces_correct_number_of_active_cubes() {
    let rows = get_data()
      .lines()
      .map(|l| l.parse::<super::super::Row>().unwrap())
      .collect::<Vec<_>>();
    let result = super::boot_cycle(rows, 0);

    assert_eq!(5usize, result);
  }

  #[test]
  fn boot_cycle_with_1_turns_produces_correct_number_of_active_cubes() {
    let rows = get_data()
      .lines()
      .map(|l| l.parse::<super::super::Row>().unwrap())
      .collect::<Vec<_>>();
    let result = super::boot_cycle(rows, 1);

    assert_eq!(11usize, result);
  }

  #[test]
  fn boot_cycle_with_2_turns_produces_correct_number_of_active_cubes() {
    let rows = get_data()
      .lines()
      .map(|l| l.parse::<super::super::Row>().unwrap())
      .collect::<Vec<_>>();
    let result = super::boot_cycle(rows, 2);

    assert_eq!(21usize, result);
  }

  #[test]
  fn boot_cycle_with_6_turns_produces_correct_number_of_active_cubes() {
    let rows = get_data()
      .lines()
      .map(|l| l.parse::<super::super::Row>().unwrap())
      .collect::<Vec<_>>();
    let result = super::boot_cycle(rows, 6);

    assert_eq!(112usize, result);
  }
}
