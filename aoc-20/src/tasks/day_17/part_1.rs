use super::{ConwayCube, Grid, GridBuilder, Position, Row};

pub(super) fn boot_cycle(starting_rows: Vec<Row>) -> usize {
  let grid = GridBuilder::new()
    .with_rows(starting_rows)
    .with_activation_fn(activation_fn)
    .with_deactivation_fn(deactivation_fn)
    .build();

  0
}

fn activation_fn(cube: &ConwayCube, active_neighbours: &[&Position]) -> bool {}

fn deactivation_fn(cube: &ConwayCube, active_neighbours: &[&Position]) -> bool {}

fn find_bounding_cube(positions: &[&Position]) -> BoundingCube {
  let mut x_min = isize::MAX;
  let mut x_max = isize::MIN;

  let mut y_min = isize::MAX;
  let mut y_max = isize::MIN;

  let mut z_min = isize::MAX;
  let mut z_max = isize::MIN;

  for position in positions {
    use std::cmp;
    x_min = cmp::min(position.x, x_min);
    x_max = cmp::max(position.x, x_max);

    y_min = cmp::min(position.y, y_min);
    y_max = cmp::max(position.y, y_max);

    z_min = cmp::min(position.z, z_min);
    z_max = cmp::max(position.z, z_max);
  }

  BoundingCube::new(
    MinMax::new(x_min - 1, x_max + 1),
    MinMax::new(y_min - 1, y_max + 1),
    MinMax::new(z_min - 1, z_max + 1),
  )
}

struct MinMax {
  min: isize,
  max: isize,
}

impl MinMax {
  fn new(min: isize, max: isize) -> Self {
    Self { min, max }
  }

  fn in_range(&self, val: &isize) -> bool {
    val >= &self.min && val <= &self.max
  }
}

struct BoundingCube {
  x_minmax: MinMax,
  y_minmax: MinMax,
  z_minmax: MinMax,
}

impl BoundingCube {
  fn new(x_minmax: MinMax, y_minmax: MinMax, z_minmax: MinMax) -> Self {
    Self {
      x_minmax,
      y_minmax,
      z_minmax,
    }
  }

  fn contains(&self, pos: &Position) -> bool {
    self.x_minmax.in_range(&pos.x)
      && self.y_minmax.in_range(&pos.y)
      && self.z_minmax.in_range(&pos.z)
  }
}
