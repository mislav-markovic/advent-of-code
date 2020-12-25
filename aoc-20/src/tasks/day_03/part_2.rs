use super::{Map, Position, Step};

pub(super) fn mul_tree_counts_on_slopes(
  map: &Map,
  starting_pos: &Position,
  slopes: &[Step],
) -> usize {
  use super::part_1::number_of_trees_on_slope as tree_counter;

  slopes
    .iter()
    .map(|slope_step| tree_counter(map, slope_step, starting_pos))
    .fold(1usize, |acc, elem| acc * elem)
}
