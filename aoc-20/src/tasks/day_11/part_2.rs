use super::{CoordinateT, Position, Row, WaitingArea};

pub(super) fn count_occupied_seats_after_changes_stop(rows: Vec<Row>) -> usize {
  let mut prev = WaitingArea::new(rows, pos_transform);
  let mut current = prev.advance_time();

  while current != prev {
    let next = current.advance_time();
    prev = current;
    current = next;
  }

  super::count_occupied_seats(&current)
}

fn pos_transform((x, y): CoordinateT, pos: &Position, area: &Vec<Row>) -> Position {
  match pos {
    Position::Floor => Position::Floor,
    Position::Occupied => {
      if should_become_empty((x, y), area) {
        Position::Empty
      } else {
        Position::Occupied
      }
    }
    Position::Empty => {
      if should_become_occupied((x, y), area) {
        Position::Occupied
      } else {
        Position::Empty
      }
    }
  }
}

fn should_become_occupied((x, y): CoordinateT, area: &Vec<Row>) -> bool {
  todo!()
}

fn should_become_empty((x, y): CoordinateT, area: &Vec<Row>) -> bool {
  todo!()
}
