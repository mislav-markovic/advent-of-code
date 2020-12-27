use super::Encoder;

pub(super) fn find_first_invalid_number(preamble: &[usize], data: &[usize]) -> usize {
  let mut enc = Encoder::new(preamble);

  *data
    .iter()
    .skip_while(|elem| enc.consume(**elem))
    .next()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_correct_invalid_number() {
    let data: [usize; 20] = [
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let preamble_size = 5usize;

    let result = find_first_invalid_number(&data[..preamble_size], &data[preamble_size..]);
    assert_eq!(127usize, result);
  }
}
