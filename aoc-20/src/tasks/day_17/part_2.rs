use super::{ConwayCube, GridBuilder, Row};

pub(super) fn boot_cycle(starting_rows: Vec<Row>, boot_cycle_length: usize) -> usize {
  let mut grid = GridBuilder::new()
    .with_rows(starting_rows)
    .with_activation_fn(activation_fn)
    .with_deactivation_fn(deactivation_fn)
    .with_4th_dimension()
    .build()
    .unwrap();

  for _ in 0..boot_cycle_length {
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
