use std::str::FromStr;

mod part_1;
mod part_2;

struct Location {
  pub is_open: bool,
}

impl Location {
  pub fn new(is_open: bool) -> Self {
    Self { is_open }
  }

  pub fn from_char(representation: char) -> Self {
    let is_open = representation == '.';
    Self::new(is_open)
  }
}

struct MapRow {
  pub locations: Vec<Location>,
}

impl FromStr for MapRow {
  type Err = String;

  fn from_str(row: &str) -> Result<Self, Self::Err> {
    let locations = row
      .chars()
      .map(|c| Location::from_char(c))
      .collect::<Vec<Location>>();

    Ok(Self { locations })
  }
}

struct Map {
  rows: Vec<MapRow>,
  width: usize,
  height: usize,
}

impl Map {
  pub fn new(rows: Vec<MapRow>) -> Self {
    let width = rows.first().unwrap().locations.len();
    let height = rows.len();

    Self {
      rows,
      width,
      height,
    }
  }

  pub fn is_open(&self, pos: &Position) -> bool {
    let real_x = pos.x % self.width;
    let real_y = pos.y % self.height;

    self.rows[real_y].locations[real_x].is_open
  }

  pub fn height(&self) -> usize {
    self.height
  }
}

struct Step {
  pub delta_x: usize,
  pub delta_y: usize,
}

impl Step {
  pub fn new(delta_x: usize, delta_y: usize) -> Self {
    Self { delta_x, delta_y }
  }
}

#[derive(Copy, Clone)]
struct Position {
  pub x: usize,
  pub y: usize,
}

impl Position {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

fn advance_position(pos: &Position, step: &Step) -> Position {
  Position::new(pos.x + step.delta_x, pos.y + step.delta_y)
}

fn read_data(path: &str) -> Map {
  use crate::common::file_reader as fr;
  Map::new(fr::parse_input::<MapRow>(path, "\r\n"))
}

pub fn solve_part_1(input_root: &str) {
  let path = format!("{}/day_03.input.txt", input_root);
  println!("Reading input from {}", path);

  let map = read_data(path.as_str());
  let starting_position = Position::new(0, 0);
  let step = Step::new(3, 1);

  let tree_count = part_1::number_of_trees_on_slope(&map, &step, &starting_position);
  println!("Counted {} trees for part 1 solution!", tree_count);
}

pub fn solve_part_2(input_root: &str) {
  let path = format!("{}/day_03.input.txt", input_root);
  println!("Reading input from {}", path);

  let slopes = [
    Step::new(1, 1),
    Step::new(3, 1),
    Step::new(5, 1),
    Step::new(7, 1),
    Step::new(1, 2),
  ];

  let map = read_data(path.as_str());
  let starting_position = Position::new(0, 0);
  let result = part_2::mul_tree_counts_on_slopes(&map, &starting_position, &slopes);

  println!(
    "Multiplication result (part 2 solution) of all slopes is {}",
    result
  );
}
