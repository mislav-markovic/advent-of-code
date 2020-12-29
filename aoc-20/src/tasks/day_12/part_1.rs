use super::{Action, Heading, PositionT, Ship};

pub(super) fn manhattan_distance_after_navigation(
  course: Vec<Action>,
  start_position: PositionT,
) -> usize {
  let mut ship = Ship {
    position: start_position.clone(),
    heading: Heading { direction: 0 },
  };

  for action in course {
    ship.take_action(&action);
  }

  manhattan_distance(&start_position, &ship.position)
}

fn manhattan_distance(start: &PositionT, end: &PositionT) -> usize {
  ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_data() -> String {
    "F10
N3
F7
R90
F11"
      .to_string()
  }
  #[test]
  fn distance_calcualted_after_navigation() {
    let actions = get_data()
      .lines()
      .map(|l| l.parse::<Action>().unwrap())
      .collect::<Vec<_>>();
    let start = (0isize, 0isize);

    let result = super::manhattan_distance_after_navigation(actions, start);
    assert_eq!(25usize, result);
  }
}
