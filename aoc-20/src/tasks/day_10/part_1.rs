pub(super) fn mul_1count_and_3count(mut data: Vec<usize>) -> usize {
  data.sort();
  let output = 0;
  let device = data.last().unwrap() + 3;
  data.insert(0, output);
  data.push(device);
  let mut one_counter = 0usize;
  let mut three_counter = 0usize;
  data
    .iter()
    .skip(1)
    .scan(data.first().unwrap().clone(), |acc, elem| {
      let old = *acc;
      *acc = *elem;
      Some(elem - old)
    })
    .filter(|&elem| elem == 1 || elem == 3)
    .for_each(|e| match e {
      1usize => one_counter += 1,
      3usize => three_counter += 1,
      _ => {}
    });

  one_counter * three_counter
}
