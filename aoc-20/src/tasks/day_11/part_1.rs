use super::{Position, Row, WaitingArea};
type CoordinateT = (usize, usize);

pub(super) fn occupied_seats_after_changes_stop(rows: Vec<Row>) -> usize {
  let mut prev = WaitingArea::new(rows, pos_transform);
  let mut current = prev.advance_time();

  while current != prev {
    let next = current.advance_time();
    prev = current;
    current = next;
  }

  count_occupied_seats(&current)
}

fn count_occupied_seats(area: &WaitingArea) -> usize {
  area
    .rows
    .iter()
    .flat_map(|row| row.positions.iter())
    .filter(|&pos| *pos == Position::Occupied)
    .count()
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
  let range = (-1isize..=1).collect::<Vec<_>>();
  product(range.as_slice(), range.as_slice())
    .iter()
    .filter(|&&tpl| tpl != (0, 0))
    .all(|(x_offset, y_offset)| !is_occupied((x, y), *x_offset, *y_offset, area))
}

fn should_become_empty((x, y): CoordinateT, area: &Vec<Row>) -> bool {
  let range = (-1isize..=1).collect::<Vec<_>>();
  let count = product(range.as_slice(), range.as_slice())
    .iter()
    .filter(|&&tpl| tpl != (0, 0))
    .filter(|(x_offset, y_offset)| is_occupied((x, y), *x_offset, *y_offset, area))
    .count();
  count >= 4usize
}

fn is_occupied((x, y): CoordinateT, x_offset: isize, y_offset: isize, area: &Vec<Row>) -> bool {
  let y_max = area.len();
  let x_max = area.first().unwrap().positions.len();
  if is_valid_offset(x, x_offset, x_max) && is_valid_offset(y, y_offset, y_max) {
    let new_x = usize_isize_addition(x, x_offset);
    let new_y = usize_isize_addition(y, y_offset);
    match area[new_y].positions[new_x] {
      Position::Floor => false,
      Position::Occupied => true,
      Position::Empty => false,
    }
  } else {
    false
  }
}

fn is_valid_offset(coordinate: usize, offset: isize, max: usize) -> bool {
  if offset >= 0 {
    coordinate + (offset as usize) < max
  } else {
    coordinate >= (offset.abs() as usize)
  }
}

fn usize_isize_addition(unum: usize, inum: isize) -> usize {
  if inum < 0 {
    unum - (inum.abs() as usize)
  } else {
    unum + (inum as usize)
  }
}

fn product(xs: &[isize], ys: &[isize]) -> Vec<(isize, isize)> {
  xs.iter()
    .flat_map(|&x| ys.iter().clone().map(move |&y| (x, y)))
    .collect()
}
#[cfg(test)]
mod tests {
  use std::str::FromStr;
  type TestResultT = Result<(), String>;
  use super::*;

  fn get_data_string() -> String {
    "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
      .to_string()
  }
  #[test]
  fn correct_number_of_occupied_seats_counted_after_changes_stop() -> TestResultT {
    let data_str = get_data_string();

    let rows = data_str
      .lines()
      .map(<Row as FromStr>::from_str)
      .collect::<Result<Vec<_>, _>>()?;

    let result = super::occupied_seats_after_changes_stop(rows);
    assert_eq!(37usize, result);

    Ok(())
  }
}
