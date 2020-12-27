use std::collections::HashMap;

use super::Bag;

pub(super) fn how_many_outermost_bags_contain_target(target_bag: &str, bag_roots: &[Bag]) -> usize {
  let map = bag_roots
    .iter()
    .map(bag_to_kv)
    .collect::<HashMap<&String, Vec<&String>>>();

  bag_roots
    .iter()
    .filter(|bag| does_bag_prodginy_contain_target(target_bag, bag, &map))
    .count()
}

fn does_bag_prodginy_contain_target(
  target: &str,
  bag: &Bag,
  roots: &HashMap<&String, Vec<&String>>,
) -> bool {
  let mut visited = Vec::<&String>::new();
  visited.push(&bag.color);

  let mut candidates = bag.contents.keys().collect::<Vec<&String>>();
  let mut next_candidates: Vec<&String> = Vec::new();

  while !candidates.is_empty() {
    for candidate in candidates {
      if !visited.contains(&candidate) {
        visited.push(candidate);
        if target == candidate {
          return true;
        }
        next_candidates.extend(
          roots
            .get(candidate)
            .unwrap_or(&Vec::with_capacity(0))
            .iter(),
        )
      }
    }
    candidates = next_candidates;
    next_candidates = Vec::new();
  }

  false
}

fn bag_to_kv(bag: &Bag) -> (&String, Vec<&String>) {
  (&bag.color, bag.contents.keys().collect())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn all_outermost_bags_counted() {
    let bag1 = "light red bags contain 1 bright white bag, 2 muted yellow bags."
      .parse::<Bag>()
      .unwrap();
    let bag2 = "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
      .parse::<Bag>()
      .unwrap();
    let bag3 = "bright white bags contain 1 shiny gold bag."
      .parse::<Bag>()
      .unwrap();
    let bag4 = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."
      .parse::<Bag>()
      .unwrap();
    let bag5 = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."
      .parse::<Bag>()
      .unwrap();
    let bag6 = "dark olive bags contain 3 faded blue bags, 4 dotted black bags."
      .parse::<Bag>()
      .unwrap();
    let bag7 = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
      .parse::<Bag>()
      .unwrap();
    let bag8 = "faded blue bags contain no other bags."
      .parse::<Bag>()
      .unwrap();
    let bag9 = "dotted black bags contain no other bags."
      .parse::<Bag>()
      .unwrap();
    let data: Vec<Bag> = vec![bag1, bag2, bag3, bag4, bag5, bag6, bag7, bag8, bag9];
    let target = "shiny gold";

    let result = how_many_outermost_bags_contain_target(target, &data);
    assert_eq!(4, result);
  }
}
