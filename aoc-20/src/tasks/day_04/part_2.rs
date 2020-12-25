use super::{Field, PassportSchema};

pub(super) fn get_schema() -> PassportSchema {
  PassportSchema {
    fields: vec![
      Field::new("byr", false, |val| number_in_range(val, 1920, 2002)),
      Field::new("iyr", false, |val| number_in_range(val, 2010, 2020)),
      Field::new("eyr", false, |val| number_in_range(val, 2020, 2030)),
      Field::new("hgt", false, |val| height_validator(val)),
      Field::new("hcl", false, |val| hair_color_validator(val)),
      Field::new("ecl", false, |val| eye_color_validator(val)),
      Field::new("pid", false, |val| {
        val.trim().chars().count() == 9 && val.trim().chars().all(|c| c.is_digit(10))
      }),
      Field::new("cid", true, super::noop),
    ],
  }
}

fn number_in_range(val: &str, min: usize, max: usize) -> bool {
  val
    .parse::<usize>()
    .map_or(false, |year| year >= min && year <= max)
}

fn height_validator(val: &str) -> bool {
  if let Some((split_index, _)) = val
    .char_indices()
    .take_while(|(_, c)| c.is_digit(10))
    .last()
  {
    let num_str = &val[..=split_index];
    let unit = &val[split_index + 1..];
    let num = num_str.parse::<usize>().expect("must parse");
    match unit {
      "in" => num >= 59 && num <= 76,
      "cm" => num >= 150 && num <= 193,
      _ => false,
    }
  } else {
    false
  }
}

fn hair_color_validator(val: &str) -> bool {
  if val.starts_with('#') {
    val.chars().skip(1).all(|c| c.is_digit(16))
  } else {
    false
  }
}

fn eye_color_validator(val: &str) -> bool {
  let allowed = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
  allowed.iter().any(|elem| elem == &val.trim())
}
