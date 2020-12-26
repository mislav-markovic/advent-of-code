use super::BoardingPass;

pub(super) fn highest_seat_id(data: &[BoardingPass]) -> usize {
  data
    .iter()
    .map(|pass| pass.seat())
    .map(|seat| seat.id())
    .max()
    .unwrap()
}
