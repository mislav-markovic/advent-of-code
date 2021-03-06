use super::{
  Action, CardinalDirection, MoveDistance, PositionT, RotateDirection, RotationDistance,
};

pub(super) fn manhattan_distance_after_waypoint_navigation(
  actions: Vec<Action>,
  ship_start: PositionT,
  waypoint_start: PositionT,
) -> usize {
  let mut ship = ship_start;
  let mut waypoint = waypoint_start;

  for action in actions {
    match action {
      Action::Forward(dist) => ship = move_ship_towards_waypoint(&ship, &waypoint, &dist),
      Action::Move(dir) => waypoint = move_waypoint(&waypoint, &dir),
      Action::Rotate(rt_dir) => waypoint = rotate_waypoint(&waypoint, &rt_dir),
    }
  }

  super::manhattan_distance(&ship_start, &ship)
}

fn move_waypoint(waypoint: &PositionT, dir: &CardinalDirection) -> PositionT {
  match dir {
    CardinalDirection::North(dist) => (waypoint.0, waypoint.1 + dist.0 as isize),
    CardinalDirection::South(dist) => (waypoint.0, waypoint.1 - dist.0 as isize),
    CardinalDirection::East(dist) => (waypoint.0 + dist.0 as isize, waypoint.1),
    CardinalDirection::West(dist) => (waypoint.0 - dist.0 as isize, waypoint.1),
  }
}
const FULL_CIRCLE: usize = 360;
fn rotate_waypoint(waypoint: &PositionT, rotation: &RotateDirection) -> PositionT {
  match rotation {
    RotateDirection::Left(dist) => rotate_left(waypoint, dist),
    RotateDirection::Right(dist) => rotate_left(waypoint, &RotationDistance(FULL_CIRCLE - dist.0)),
  }
}
fn rotate_left(waypoint: &PositionT, dist: &RotationDistance) -> PositionT {
  match dist.0 {
    0 => *waypoint,
    90 => (-1 * waypoint.1, waypoint.0),
    180 => (-1 * waypoint.0, -1 * waypoint.1),
    270 => (waypoint.1, -1 * waypoint.0),
    _ => panic!("Cant rotate waypoint"),
  }
}

fn move_ship_towards_waypoint(
  ship: &PositionT,
  waypoint: &PositionT,
  distance: &MoveDistance,
) -> PositionT {
  let x_offset = waypoint.0 * distance.0 as isize;
  let y_offset = waypoint.1 * distance.0 as isize;

  (ship.0 + x_offset, ship.1 + y_offset)
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
  fn waypoint_manhattan_distance() {
    let actions = get_data()
      .lines()
      .map(|l| l.parse::<Action>().unwrap())
      .collect::<Vec<_>>();
    let start = (0isize, 0isize);
    let waypoint = (10, 1);

    let result = super::manhattan_distance_after_waypoint_navigation(actions, start, waypoint);
    assert_eq!(286usize, result);
  }

  #[test]
  fn step_by_step_ship_and_waypoint_movements() {
    let forward_10 = MoveDistance(10);
    let north = CardinalDirection::North(MoveDistance(3));
    let forward_7 = MoveDistance(7);
    let forward_11 = MoveDistance(11);
    let right_rotate = RotateDirection::Right(RotationDistance(90));
    let mut ship: PositionT = (0, 0);
    let mut waypoint: PositionT = (10, 1);

    ship = super::move_ship_towards_waypoint(&ship, &waypoint, &forward_10);
    assert_eq!((100, 10), ship);
    assert_eq!((10, 1), waypoint);

    waypoint = super::move_waypoint(&waypoint, &north);
    assert_eq!((100, 10), ship);
    assert_eq!((10, 4), waypoint);

    ship = super::move_ship_towards_waypoint(&ship, &waypoint, &forward_7);
    assert_eq!((170, 38), ship);
    assert_eq!((10, 4), waypoint);

    waypoint = super::rotate_waypoint(&waypoint, &right_rotate);
    assert_eq!((170, 38), ship);
    assert_eq!((4, -10), waypoint);

    ship = super::move_ship_towards_waypoint(&ship, &waypoint, &forward_11);
    assert_eq!((214, -72), ship);
    assert_eq!((4, -10), waypoint);
  }
}
