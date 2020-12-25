use super::{Field, PassportSchema};

pub(super) fn get_schema() -> PassportSchema {
  PassportSchema {
    fields: vec![
      Field::new("byr", false, super::noop),
      Field::new("iyr", false, super::noop),
      Field::new("eyr", false, super::noop),
      Field::new("hgt", false, super::noop),
      Field::new("hcl", false, super::noop),
      Field::new("ecl", false, super::noop),
      Field::new("pid", false, super::noop),
      Field::new("cid", true, super::noop),
    ],
  }
}
