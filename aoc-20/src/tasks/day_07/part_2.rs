use super::Bag;

pub(super) fn how_many_bags_must_target_contain(target: &str, bag_roots: &[Bag]) -> usize {
  let target_bag = bag_roots
    .iter()
    .skip_while(|bag| bag.color != target)
    .next()
    .unwrap();

  let count = direct_descendants_count(target_bag);

  return count
    + target_bag
      .contents
      .iter()
      .map(|(child, num_of_kids)| num_of_kids * how_many_bags_must_target_contain(child, bag_roots))
      .sum::<usize>();
}

fn direct_descendants_count(bag: &Bag) -> usize {
  bag.contents.values().sum()
}
