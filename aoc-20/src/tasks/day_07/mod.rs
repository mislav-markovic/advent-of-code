use std::{
  cmp::Eq,
  collections::HashMap,
  hash::{Hash, Hasher},
  str::FromStr,
};

mod part_1;
mod part_2;

struct Bag {
  color: String,
  contents: HashMap<String, usize>,
}

impl PartialEq for Bag {
  fn eq(&self, other: &Self) -> bool {
    self.color == other.color
  }
}
impl Eq for Bag {}

impl Hash for Bag {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.color.hash(state);
  }
}

impl FromStr for Bag {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut split_iter = input.split("bags contain");
    let color = split_iter.next().unwrap().trim().to_string();
    let bag_contents = split_iter.next().unwrap().trim().trim_end_matches('.');

    let contents = if bag_contents == "no other bags" {
      HashMap::new()
    } else {
      bag_contents
        .split(", ")
        .map(|val| val.trim())
        .map(|trimmed| trimmed.split_once(' ').unwrap())
        .map(|(count_str, inside_bag_name)| {
          (
            inside_bag_name
              .trim()
              .trim_end_matches("bags")
              .trim_end_matches("bag")
              .trim_end()
              .to_string(),
            count_str.trim().parse::<usize>().unwrap(),
          )
        })
        .collect::<HashMap<String, usize>>()
    };
    Ok(Self { color, contents })
  }
}

pub fn solve_part_1(input_root: &str) {
  let target_bag = "shiny gold";
  let bags = get_data(input_root);

  let result = part_1::how_many_outermost_bags_contain_target(target_bag, &bags);
  println!(
    "(Day 7, Part 1) {} outermost bags can contain {} bag",
    result, target_bag
  );
}
pub fn solve_part_2(input_root: &str) {
  let target_bag = "shiny gold";
  let bags = get_data(input_root);

  let result = part_2::how_many_bags_must_target_contain(target_bag, &bags);
  println!(
    "(Day 7, Part 2) {} must contain {} other bags",
    target_bag, result
  );
}

fn get_data(root: &str) -> Vec<Bag> {
  use crate::common::file_reader as fr;

  let path = format!("{}/day_07.input.txt", root);
  println!("Reading input from '{}'", &path);

  fr::parse_input::<Bag>(&path, "\r\n")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn bag_correctly_parsed_when_it_contains_no_other_bag() {
    let data = "faded blue bags contain no other bags.";
    let result = data.parse::<Bag>();

    assert!(result.is_ok());
    let bag = result.unwrap();

    assert_eq!("faded blue", bag.color);
    assert_eq!(0, bag.contents.len());
  }

  #[test]
  fn bag_correctly_parsed_when_it_contains_other_bags() {
    let data = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";
    let result = data.parse::<Bag>();

    assert!(result.is_ok());
    let bag = result.unwrap();

    assert_eq!("dark olive", bag.color);
    assert_eq!(2, bag.contents.len());
    assert!(bag.contents.contains_key("faded blue"));
    assert!(bag.contents.contains_key("dotted black"));
    assert_eq!(3, *bag.contents.get("faded blue").unwrap());
    assert_eq!(4, *bag.contents.get("dotted black").unwrap());
  }
}
