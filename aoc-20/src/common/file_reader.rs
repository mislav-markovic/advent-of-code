use std::{fmt::Debug, fs, str::FromStr};

pub fn parse_input<TResult: FromStr>(filename: &str, delimiter: &str) -> Vec<TResult>
where
  <TResult as FromStr>::Err: Debug,
{
  fs::read_to_string(filename)
    .unwrap()
    .split(delimiter)
    .map(|elem| {
      elem
        .parse::<TResult>()
        .expect(format!("Could not parse elem: '{}'", elem).as_ref())
    })
    .collect::<Vec<TResult>>()
}
