use super::BoardingPass;

pub(super) fn get_missing_seat_id(passes: &[BoardingPass]) -> usize {
  let mut taken_seat_ids = passes
    .iter()
    .map(|pass| pass.seat().id())
    .collect::<Vec<usize>>();

  taken_seat_ids.sort();

  println!("Sorted seat ids");
  println!("{:?}", taken_seat_ids);

  let mut current_seat_id = taken_seat_ids.first().unwrap();
  for seat_id in taken_seat_ids.iter().skip(1) {
    if seat_id - current_seat_id > 1 {
      return seat_id - 1;
    } else {
      current_seat_id = seat_id;
    }
  }

  panic!("(Day 5, Part 2) Could not calculate missing seat id");
}
