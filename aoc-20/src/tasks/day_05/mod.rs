use std::str::FromStr;

mod part_1;
mod part_2;

struct Seat {
  column: usize,
  row: usize,
  column_count: usize,
}

impl Seat {
  fn new(column: usize, row: usize, column_count: usize) -> Self {
    Self {
      column,
      row,
      column_count,
    }
  }

  fn id(&self) -> usize {
    self.row * self.column_count + self.column
  }
}

#[derive(PartialEq, Debug)]
enum Instruction {
  Upper,
  Lower,
}
struct BoardingPass {
  row_instructions: Vec<Instruction>,
  column_instructions: Vec<Instruction>,
}

impl BoardingPass {
  pub fn seat(&self) -> Seat {
    let column_range_init = Range::new(0, 2usize.pow(self.column_instructions.len() as u32) - 1);
    let column_count = column_range_init.len();
    let column_range = self
      .column_instructions
      .iter()
      .fold(column_range_init, |acc, elem| acc.split(elem));

    let row_range_init = Range::new(0, 2usize.pow(self.row_instructions.len() as u32) - 1);
    let row_range = self
      .row_instructions
      .iter()
      .fold(row_range_init, |acc, elem| acc.split(elem));

    let column = column_range.collapse().unwrap();
    let row = row_range.collapse().unwrap();

    Seat::new(column, row, column_count)
  }
}

impl FromStr for BoardingPass {
  type Err = String;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let row_chars = ['F', 'B'];

    let columns = line
      .trim()
      .chars()
      .filter(|c| !row_chars.contains(c))
      .map(|c| column_to_instr(c))
      .collect::<Vec<Instruction>>();

    let rows = line
      .trim()
      .chars()
      .filter(|c| row_chars.contains(c))
      .map(|c| row_to_instr(c))
      .collect::<Vec<Instruction>>();

    Ok(Self {
      row_instructions: rows,
      column_instructions: columns,
    })
  }
}

fn row_to_instr(symbol: char) -> Instruction {
  match symbol {
    'F' => Instruction::Lower,
    'B' => Instruction::Upper,
    _ => panic!(),
  }
}

fn column_to_instr(symbol: char) -> Instruction {
  match symbol {
    'L' => Instruction::Lower,
    'R' => Instruction::Upper,
    _ => panic!(),
  }
}

#[derive(Debug)]
struct Range {
  start: usize,
  end: usize,
}

impl Range {
  fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }

  fn split(&self, inst: &Instruction) -> Range {
    match inst {
      Instruction::Lower => {
        Range::new(self.start, Self::get_half_point(self.start, self.end, inst))
      }
      Instruction::Upper => Range::new(Self::get_half_point(self.start, self.end, inst), self.end),
    }
  }

  fn collapse(&self) -> Option<usize> {
    if self.start == self.end {
      Some(self.start)
    } else {
      None
    }
  }

  fn len(&self) -> usize {
    Self::from_to_len(self.start, self.end)
  }

  fn from_to_len(start: usize, end: usize) -> usize {
    end - start + 1usize // range is inclusive
  }

  fn get_half_point(start: usize, end: usize, inst: &Instruction) -> usize {
    let elem_count = Self::from_to_len(start, end);
    let half_point = end - elem_count / 2usize;
    match inst {
      Instruction::Lower => half_point,
      Instruction::Upper => half_point + 1,
    }
  }
}

pub fn solve_part_1(input_root: &str) {
  let path = format!("{}/day_05.input.txt", input_root);
  println!("(Day 5, Part 1) Reading input from '{}'", &path);

  let data = get_data(&path);
  let solution = part_1::highest_seat_id(&data);

  println!("(Day 5, Part 1) Highest seat id is '{}'", solution);
}
pub fn solve_part_2(input_root: &str) {
  let path = format!("{}/day_05.input.txt", input_root);
  println!("(Day 5, Part 2) Reading input from '{}'", &path);

  let data = get_data(&path);
  let missing_seat_id = part_2::get_missing_seat_id(&data);

  println!("(Day 5, Part 2) Missing seat id {}", missing_seat_id);
}

fn get_data(path: &str) -> Vec<BoardingPass> {
  use crate::common::file_reader as fr;
  fr::parse_input::<BoardingPass>(path, "\r\n")
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn boarding_pass_correclty_parses() {
    use super::Instruction::{Lower, Upper};
    let input = "FBBFFFBRRL";
    let result = input.parse::<BoardingPass>();

    assert!(result.is_ok());
    let pass = result.expect("test expect");

    assert_eq!(7usize, pass.row_instructions.len());
    assert_eq!(3usize, pass.column_instructions.len());
    assert_eq!(
      vec![Lower, Upper, Upper, Lower, Lower, Lower, Upper],
      pass.row_instructions
    );
    assert_eq!(vec![Upper, Upper, Lower], pass.column_instructions);
  }

  #[test]
  fn seat_is_calculated_correctly() {
    let input = "FBFBBFFRLR";
    let pass = input.parse::<BoardingPass>().unwrap();

    let seat = pass.seat();

    assert_eq!(44, seat.row);
    assert_eq!(5, seat.column);
    assert_eq!(357, seat.id());
  }
}
