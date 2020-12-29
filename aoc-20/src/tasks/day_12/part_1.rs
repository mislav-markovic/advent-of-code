use super::{
  Action, CardinalDirection, MoveDistance, PositionT, RotateDirection, RotationDistance,
};

struct Heading {
  direction: usize,
}

const FULL_CIRCLE: usize = 360;
impl Heading {
  fn rotate_left(&mut self, rotation: &RotationDistance) {
    let val = rotation.0 % FULL_CIRCLE;
    self.direction = (self.direction + val) % FULL_CIRCLE;
  }

  fn rotate_right(&mut self, rotation: &RotationDistance) {
    let val = rotation.0 % FULL_CIRCLE;
    if val > self.direction {
      self.direction += FULL_CIRCLE - val;
    } else {
      self.direction -= val;
    }
  }

  fn rotate(&mut self, rotation: &RotateDirection) {
    match rotation {
      RotateDirection::Left(rt_val) => self.rotate_left(rt_val),
      RotateDirection::Right(rt_val) => self.rotate_right(rt_val),
    }
  }
}
struct Ship {
  heading: Heading,
  position: PositionT,
}
impl Ship {
  fn take_action(&mut self, action: &Action) {
    match action {
      Action::Move(dir) => self.move_in_cardinal_direction(dir),
      Action::Rotate(rt) => self.heading.rotate(rt),
      Action::Forward(dist) => self.move_in_heading(dist),
    }
  }
  fn move_in_cardinal_direction(&mut self, dir: &CardinalDirection) {
    match dir {
      CardinalDirection::North(MoveDistance(distance)) => self.position.1 += *distance as isize,
      CardinalDirection::South(MoveDistance(distance)) => self.position.1 -= *distance as isize,
      CardinalDirection::East(MoveDistance(distance)) => self.position.0 += *distance as isize,
      CardinalDirection::West(MoveDistance(distance)) => self.position.0 -= *distance as isize,
    }
  }

  fn move_in_heading(&mut self, distance: &MoveDistance) {
    let heading_to_dir = |heading: &Heading| -> CardinalDirection {
      let val = heading.direction % FULL_CIRCLE;
      match val {
        0 => CardinalDirection::East(*distance),
        90 => CardinalDirection::North(*distance),
        180 => CardinalDirection::West(*distance),
        270 => CardinalDirection::South(*distance),
        _ => panic!("Cant transform heading to cardinal direction"), // should return Result but im lazy
      }
    };

    self.move_in_cardinal_direction(&heading_to_dir(&self.heading));
  }
}
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

  super::manhattan_distance(&start_position, &ship.position)
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
#[test]
fn step_by_step_ship_movements() {
  let forward_10 = Action::Forward(MoveDistance(10));
  let north = Action::Move(CardinalDirection::North(MoveDistance(3)));
  let forward_7 = Action::Forward(MoveDistance(7));
  let forward_11 = Action::Forward(MoveDistance(11));
  let right_rotate = Action::Rotate(RotateDirection::Right(RotationDistance(90)));
  let mut ship = Ship {
    heading: Heading { direction: 0 },
    position: (0, 0),
  };

  ship.take_action(&forward_10);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((10, 0), ship.position);

  ship.take_action(&north);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((10, 3), ship.position);

  ship.take_action(&forward_7);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((17, 3), ship.position);

  ship.take_action(&right_rotate);
  assert_eq!(270, ship.heading.direction);
  assert_eq!((17, 3), ship.position);

  ship.take_action(&forward_11);
  assert_eq!(270, ship.heading.direction);
  assert_eq!((17, -8), ship.position);
}

#[test]
fn multiple_rotations_work() {
  let right_rotate_0 = Action::Rotate(RotateDirection::Right(RotationDistance(0)));
  let right_rotate_90 = Action::Rotate(RotateDirection::Right(RotationDistance(90)));
  let right_rotate_180 = Action::Rotate(RotateDirection::Right(RotationDistance(180)));
  let right_rotate_270 = Action::Rotate(RotateDirection::Right(RotationDistance(270)));
  let right_rotate_360 = Action::Rotate(RotateDirection::Right(RotationDistance(360)));

  let left_rotate_0 = Action::Rotate(RotateDirection::Left(RotationDistance(0)));
  let left_rotate_90 = Action::Rotate(RotateDirection::Left(RotationDistance(90)));
  let left_rotate_180 = Action::Rotate(RotateDirection::Left(RotationDistance(180)));
  let left_rotate_270 = Action::Rotate(RotateDirection::Left(RotationDistance(270)));
  let left_rotate_360 = Action::Rotate(RotateDirection::Left(RotationDistance(360)));

  let forward_10 = Action::Forward(MoveDistance(10));

  let mut ship = Ship {
    heading: Heading { direction: 0 },
    position: (0, 0),
  };

  ship.take_action(&right_rotate_0);
  ship.take_action(&forward_10);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((10, 0), ship.position);

  ship.take_action(&right_rotate_90);
  ship.take_action(&forward_10);
  assert_eq!(270, ship.heading.direction);
  assert_eq!((10, -10), ship.position);

  ship.take_action(&right_rotate_180);
  ship.take_action(&forward_10);
  assert_eq!(90, ship.heading.direction);
  assert_eq!((10, 0), ship.position);

  ship.take_action(&right_rotate_270);
  ship.take_action(&forward_10);
  assert_eq!(180, ship.heading.direction);
  assert_eq!((0, 0), ship.position);

  ship.take_action(&right_rotate_360);
  ship.take_action(&forward_10);
  assert_eq!(180, ship.heading.direction);
  assert_eq!((-10, 0), ship.position);

  ship.take_action(&left_rotate_0);
  ship.take_action(&forward_10);
  assert_eq!(180, ship.heading.direction);
  assert_eq!((-20, 0), ship.position);

  ship.take_action(&left_rotate_90);
  ship.take_action(&forward_10);
  assert_eq!(270, ship.heading.direction);
  assert_eq!((-20, -10), ship.position);

  ship.take_action(&left_rotate_180);
  ship.take_action(&forward_10);
  assert_eq!(90, ship.heading.direction);
  assert_eq!((-20, 0), ship.position);

  ship.take_action(&left_rotate_270);
  ship.take_action(&forward_10);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((-10, 0), ship.position);

  ship.take_action(&left_rotate_360);
  ship.take_action(&forward_10);
  assert_eq!(0, ship.heading.direction);
  assert_eq!((0, 0), ship.position);
}
