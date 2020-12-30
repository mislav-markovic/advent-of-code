use std::collections::HashMap;

mod part_1;
mod part_2;

struct Game {
  numbers: HashMap<usize, Vec<usize>>,
  last_number: usize,
  current_turn: usize,
}

impl Game {
  fn from_seed(starting_numbers: &[usize]) -> Self {
    let mut numbers = HashMap::<usize, Vec<usize>>::new();
    let mut last_number = 0;
    let mut current_turn = 0;
    for (turn, number) in starting_numbers.iter().enumerate() {
      current_turn += 1;
      numbers.insert(*number, vec![current_turn]);
      last_number = *number;
    }
    Self {
      numbers,
      last_number,
      current_turn,
    }
  }

  fn next_turn(&mut self) {
    self.current_turn += 1;
    let last_numbers_turns = self.numbers.get(&self.last_number).unwrap();
    let number_to_play = if last_numbers_turns.len() == 1 {
      0
    } else {
      last_numbers_turns[last_numbers_turns.len() - 1]
        - last_numbers_turns[last_numbers_turns.len() - 2]
    };

    self.last_number = number_to_play;
    self
      .numbers
      .entry(number_to_play)
      .or_insert(vec![self.current_turn])
      .push(self.current_turn);
  }
}

pub fn solve_part_1(input_root: &str) {
  let result =
    part_1::number_played_at_turn(Game::from_seed(get_data(input_root).as_slice()), &2020);
  println!("(Day 15, Part 1) Number on turn 2020 was {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let result =
    part_1::number_played_at_turn(Game::from_seed(get_data(input_root).as_slice()), &30000000);
  println!("(Day 15, Part 2) Number on turn 30000000 was {}", result);
}

fn get_data(root: &str) -> Vec<usize> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_15.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<usize>(&path, ",")
}
