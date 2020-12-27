use std::{collections::VecDeque, iter::FromIterator};

mod part_1;
mod part_2;

struct Encoder {
  preamble: VecDeque<usize>,
}

impl Encoder {
  fn new(preamble_slice: &[usize]) -> Self {
    Self {
      preamble: VecDeque::from_iter(preamble_slice.iter().copied()),
    }
  }

  // consumes number and returns if it was valid according to previous preamble
  fn consume(&mut self, num: usize) -> bool {
    let mut is_valid = false;

    'outer: for (indx, first_num) in self.preamble.iter().enumerate() {
      for second_num in self.preamble.iter().skip(indx + 1) {
        if first_num + second_num == num {
          is_valid = true;
          break 'outer;
        }
      }
    }
    self.preamble.push_back(num);
    self.preamble.pop_front();
    is_valid
  }
}

pub fn solve_part_1(input_root: &str) {
  let data = get_data(input_root);
  let preamble_size = 25usize;
  let first_invalid =
    part_1::find_first_invalid_number(&data[..preamble_size], &data[preamble_size..]);
  println!(
    "(Day 9, Part 1) First found invalid encoder number is {}",
    first_invalid
  );
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 9, Part 2) Not Implemented")
}

fn get_data(root: &str) -> Vec<usize> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_09.input.txt", root);
  println!("Reading input from '{}'", &path);
  fr::parse_input::<usize>(&path, "\r\n")
}
