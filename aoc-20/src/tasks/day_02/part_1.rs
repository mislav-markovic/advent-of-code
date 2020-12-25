use super::DatabaseRecord;

pub fn solve_part(input_path: &str) -> usize {
  super::count_valid_passwords(super::read_data(input_path).as_slice(), is_valid_password)
}

fn is_valid_password(record: &DatabaseRecord) -> bool {
  let number_of_occurrences = record
    .password
    .0
    .chars()
    .filter(|c| *c == record.policy.letter)
    .count();
  number_of_occurrences >= record.policy.first_num
    && number_of_occurrences <= record.policy.second_num
}

#[cfg(test)]
mod test {
  use super::*;

  fn valid_password_detected() {
    let input = "2-9 c: ccccccccc";
    let record = input.parse::<DatabaseRecord>().unwrap();

    assert!(is_valid_password(&record));
  }

  fn invalid_password_detected() {
    let input = "1-3 b: cdefg";
    let record = input.parse::<DatabaseRecord>().unwrap();

    assert!(!is_valid_password(&record));
  }
}
