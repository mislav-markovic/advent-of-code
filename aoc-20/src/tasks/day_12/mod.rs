use std::str::FromStr;

mod part_1;
mod part_2;

enum CardinalDirection {
  North(MoveDistance),
  South(MoveDistance),
  East(MoveDistance),
  West(MoveDistance),
}
enum RotateDirection {
  Left(RotationDistance),
  Right(RotationDistance),
}
enum Action {
  Move(CardinalDirection),
  Rotate(RotateDirection),
  Forward(MoveDistance),
}
#[derive(Clone, Copy)]
struct MoveDistance(usize);
#[derive(Clone, Copy)]
struct RotationDistance(usize);

impl FromStr for Action {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut char_iter = input.trim().chars();
    let action_str = char_iter.next().unwrap();
    let value_str = char_iter.collect::<String>();
    let value = value_str
      .parse::<usize>()
      .map_err(|_| "Could not parse action value".to_string())?;

    match action_str {
      'N' => Ok(Action::Move(CardinalDirection::North(MoveDistance(value)))),
      'S' => Ok(Action::Move(CardinalDirection::South(MoveDistance(value)))),
      'E' => Ok(Action::Move(CardinalDirection::East(MoveDistance(value)))),
      'W' => Ok(Action::Move(CardinalDirection::West(MoveDistance(value)))),
      'L' => Ok(Action::Rotate(RotateDirection::Left(RotationDistance(
        value,
      )))),
      'R' => Ok(Action::Rotate(RotateDirection::Right(RotationDistance(
        value,
      )))),
      'F' => Ok(Action::Forward(MoveDistance(value))),
      _ => Err("Could not parse action type".to_string()),
    }
  }
}

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

type PositionT = (isize, isize);
struct Ship {
  heading: Heading,
  position: PositionT,
}

fn sum_positions(me: &PositionT, other: &PositionT) -> PositionT {
  (me.0 + other.0, me.1 + other.1)
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

fn get_data(root: &str) -> Vec<Action> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_12.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Action>(&path, "\r\n")
}
pub fn solve_part_1(input_root: &str) {
  let result = part_1::manhattan_distance_after_navigation(get_data(input_root), (0, 0));
  println!(
    "(Day 12, Part 1) Manhattan distance from start after following navigation course is {}",
    result
  );
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 12, Part 2) Not Implemented");
}
#[cfg(test)]
mod tests {
  use super::*;

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
}
