use std::str::FromStr;

mod part_1;
mod part_2;

pub struct PasswordPolicy {
  pub first_num: usize,
  pub second_num: usize,
  pub letter: char,
}
#[derive(Debug, PartialEq)]
pub struct Password(String);

pub struct DatabaseRecord {
  pub policy: PasswordPolicy,
  pub password: Password,
}

impl FromStr for DatabaseRecord {
  type Err = String;

  fn from_str(input_line: &str) -> Result<Self, Self::Err> {
    let mut str_split = input_line.split(':');
    let policy_str: &str = str_split
      .next()
      .ok_or("Could not split input line correctly to get policy definition".to_string())?;
    let password_str: &str = str_split
      .next()
      .ok_or("Could not split input line correctly to get password value".to_string())?;

    let mut policy_str_split = policy_str.trim().split(' ');
    let range_str: &str = policy_str_split
      .next()
      .ok_or("Could not split input line correctly to get policy range".to_string())?;
    let policy_letter_str = policy_str_split
      .next()
      .ok_or("Could not split input line correctly to get policy letter".to_string())?;

    let mut range_str_split = range_str.trim().split('-');
    let min_str = range_str_split
      .next()
      .ok_or("Could not split input line correctly to get min of range".to_string())?;
    let max_str = range_str_split
      .next()
      .ok_or("Could not split input line correctly to get max of range".to_string())?;

    let min = min_str.parse::<usize>().map_err(|err| err.to_string())?;
    let max = max_str.parse::<usize>().map_err(|err| err.to_string())?;
    let letter = policy_letter_str
      .trim()
      .chars()
      .next()
      .ok_or("Could not get policy letter".to_string())?;

    let policy = PasswordPolicy {
      first_num: min,
      second_num: max,
      letter,
    };
    let password = Password(password_str.trim().to_string());

    Ok(DatabaseRecord { policy, password })
  }
}

pub fn solve_part_1(input_root: &str) {
  let input_path = format!("{}/day_02.input.txt", input_root);
  println!("Reading data from input path: {}", &input_path);

  let solution = part_1::solve_part(&input_path);
  println!("Day 2, Part 1 || Found {} valid passwords", solution);
}

pub fn solve_part_2(input_root: &str) {
  let input_path = format!("{}/day_02.input.txt", input_root);
  println!("Reading data from input path: {}", &input_path);

  let solution = part_2::solve_part(&input_path);
  println!("Day 2, Part 2 || Found {} valid passwords", solution);
}

fn read_data(path: &str) -> Vec<DatabaseRecord> {
  use crate::common::file_reader as fr;
  fr::parse_input::<DatabaseRecord>(path, "\r\n")
}

fn count_valid_passwords(data: &[DatabaseRecord], predicate: fn(&DatabaseRecord) -> bool) -> usize {
  data.iter().filter(|elem| predicate(elem)).count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn database_record_correctly_parses() {
    let input_str = "1-22 l: abc";
    let result = input_str.parse::<DatabaseRecord>();

    assert!(result.is_ok());
    let record = result.expect("test unwrap");

    assert_eq!(1usize, record.policy.first_num);
    assert_eq!(22usize, record.policy.second_num);
    assert_eq!('l', record.policy.letter);
    assert_eq!("abc", record.password.0);
    assert_eq!(Password("abc".to_string()), record.password);
  }
}
