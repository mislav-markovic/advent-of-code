use super::{advance_position, Map, Position, Step};

pub(super) fn number_of_trees_on_slope(
  map: &Map,
  slope_step: &Step,
  starting_position: &Position,
) -> usize {
  let mut tree_count = 0usize;
  let mut current_position = *starting_position;

  while current_position.y < map.height {
    if !map.is_open(&current_position) {
      tree_count += 1;
    }
    current_position = super::advance_position(&current_position, slope_step);
  }

  tree_count
}
