use std::collections::HashMap;

pub(super) fn number_of_adapter_arangments(mut data: Vec<usize>) -> u64 {
  use std::cmp;
  let device = data.iter().max().unwrap() + 3;
  let outlet = 0usize;
  data.push(device);
  data.push(outlet);
  data.sort();
  let data_size = data.len();
  let clamp_to_range_end = |e| cmp::min(data_size, e);
  let adapter_compatibility_list = data
    .iter()
    .enumerate()
    .map(|(i, e)| number_of_compatible_descendants(e, &data[i + 1..clamp_to_range_end(i + 3 + 1)]))
    .collect::<Vec<usize>>();

  paths_from_first_to_last_element(
    adapter_compatibility_list.as_slice(),
    0,
    &mut HashMap::new(),
  )
}

fn number_of_compatible_descendants(output: &usize, adapters: &[usize]) -> usize {
  adapters
    .iter()
    .filter(|&&adapter| adapter - output <= 3)
    .count()
}

fn paths_from_first_to_last_element(
  data: &[usize],
  absolute_start_index: usize,
  cache: &mut HashMap<usize, u64>,
) -> u64 {
  if let Some((index, &count)) = data.iter().enumerate().find(|(_, &elem)| elem > 1) {
    (1..=count)
      .rev()
      .map(|e| {
        let new_absolute_start = absolute_start_index + index + e;
        if let Some(&cached_instance) = cache.get(&new_absolute_start) {
          cached_instance
        } else {
          let result =
            paths_from_first_to_last_element(&data[(index + e)..], new_absolute_start, cache);
          cache.insert(new_absolute_start, result);
          result
        }
      })
      .sum()
  } else {
    1u64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn number_of_adapter_arrangments_calculated_correctly() {
    let data: Vec<usize> = vec![
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17,
      7, 9, 4, 2, 34, 10, 3,
    ];

    let result = number_of_adapter_arangments(data);
    assert_eq!(19208u64, result);
  }

  #[test]
  fn adapter_arrangments_of_small_set() {
    let data: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    let result = number_of_adapter_arangments(data);
    assert_eq!(8u64, result);
  }
}
