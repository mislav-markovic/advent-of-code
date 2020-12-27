pub(super) fn get_encription_weakness(target: usize, data: &[usize]) -> usize {
  let mut range_start = 0usize;
  let mut range_end = 1usize;
  let mut range = &data[range_start..range_end];

  loop {
    let sum = range_sum(range);
    if sum == target {
      return calculate_weakness(range);
    } else if sum < target {
      range_end += 1;
      range = &data[range_start..range_end];
    } else {
      range_start += 1;
      if range_start == range_end {
        range_end += 1;
      }
      range = &data[range_start..range_end];
    }
  }
}

fn range_sum(range: &[usize]) -> usize {
  range.iter().sum()
}

fn calculate_weakness(data: &[usize]) -> usize {
  let mut vec = data.iter().collect::<Vec<&usize>>();
  vec.sort();
  *vec.first().unwrap() + *vec.last().unwrap()
}
