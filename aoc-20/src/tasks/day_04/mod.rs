mod part_1;
mod part_2;

use std::{collections::HashMap, str::FromStr};

struct Field {
  pub short_name: String,
  pub optional: bool,
  pub validator: fn(&str) -> bool,
}

impl Field {
  fn new(name: &str, opt: bool, validator: fn(&str) -> bool) -> Field {
    Field {
      short_name: name.to_string(),
      optional: opt,
      validator: validator,
    }
  }
}

struct PassportSchema {
  pub fields: Vec<Field>,
}

struct Passport {
  field_value_map: HashMap<String, String>,
}

impl FromStr for Passport {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let map = input
      .lines()
      .map(|line| line.split(' '))
      .flatten()
      .map(|kv_str| {
        let mut split_iter = kv_str.split(':');
        (
          split_iter.next().unwrap().to_string(),
          split_iter.next().unwrap().to_string(),
        )
      })
      .collect::<HashMap<String, String>>();

    Ok(Passport {
      field_value_map: map,
    })
  }
}

impl Passport {
  fn contains(&self, field: &Field) -> bool {
    self.field_value_map.contains_key(&field.short_name)
  }

  fn get(&self, field: &Field) -> Option<&str> {
    self
      .field_value_map
      .get(&field.short_name)
      .map(|e| e.as_str())
  }
}

fn read_data(path: &str) -> Vec<Passport> {
  use crate::common::file_reader as fr;

  fr::parse_input::<Passport>(path, "\r\n\r\n")
}

fn noop(_: &str) -> bool {
  true
}

fn count_valid_passports(schema: &PassportSchema, passports: &[Passport]) -> usize {
  passports
    .iter()
    .filter(|passport| is_passport_valid(schema, passport))
    .count()
}

fn is_passport_valid(schema: &PassportSchema, passport: &Passport) -> bool {
  schema
    .fields
    .iter()
    .filter(|elem| !elem.optional)
    .all(|mandatory_field| {
      passport.contains(mandatory_field)
        && (mandatory_field.validator)(passport.get(mandatory_field).unwrap())
    })
}

pub fn solve_part_1(input_root: &str) {
  let path = format!("{}/day_04.input.txt", input_root);
  println!("Reading input for day 4 part 1 fron '{}'", path);

  let passports = read_data(&path);
  let schema = part_1::get_schema();
  let result = count_valid_passports(&schema, &passports);

  println!("(Day 4, Part 1) Found {} valid passports", result);
}

pub fn solve_part_2(input_root: &str) {
  let path = format!("{}/day_04.input.txt", input_root);
  println!("Reading input for day 4 part 2 fron '{}'", path);

  let passports = read_data(&path);
  let schema = part_2::get_schema();
  let result = count_valid_passports(&schema, &passports);

  println!("(Day 4, Part 2) Counted {} valid passports", result);
}
