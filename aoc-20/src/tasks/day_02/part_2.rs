use super::DatabaseRecord;

pub fn solve_part(input_path: &str) -> usize {
  super::count_valid_passwords(super::read_data(input_path).as_slice(), is_valid_password)
}

fn is_valid_password(record: &DatabaseRecord) -> bool {
  let letter_at_first_pos = record
    .password
    .0
    .chars()
    .nth(record.policy.first_num - 1)
    .unwrap();
  let letter_at_second_pos = record
    .password
    .0
    .chars()
    .nth(record.policy.second_num - 1)
    .unwrap();

  (letter_at_first_pos == record.policy.letter) ^ (letter_at_second_pos == record.policy.letter)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid_password_detected() {
    let input = "1-3 a: abcde";
    let record = input.parse::<DatabaseRecord>().expect("test expect");

    assert!(is_valid_password(&record))
  }

  #[test]
  fn password_invalid_neither_position_matches_character() {
    let input = "1-3 b: cdefg";
    let record = input.parse::<DatabaseRecord>().expect("test expect");

    assert!(!is_valid_password(&record))
  }

  #[test]
  fn password_invalid_both_positions_match_character() {
    let input = "2-9 c: ccccccccc";
    let record = input.parse::<DatabaseRecord>().expect("test expect");

    assert!(!is_valid_password(&record))
  }
}
