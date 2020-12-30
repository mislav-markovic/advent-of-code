use std::{collections::HashMap, str::FromStr};

mod part_1;
mod part_2;

struct MemStore {
  address: u64,
  val: u64,
}

impl FromStr for MemStore {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (instr_str, val_str) = s.split_once('=').unwrap();
    let address = instr_str
      .split_once('[')
      .unwrap()
      .1
      .trim()
      .trim_end_matches(']')
      .trim()
      .parse::<u64>()
      .map_err(|_| "Could not parse mem store address".to_string())?;

    let val = val_str
      .trim()
      .parse::<u64>()
      .map_err(|_| "Could not parse mem store value".to_string())?;

    Ok(Self { address, val })
  }
}

#[derive(Clone, Copy)]
struct Mask {
  or_mask: u64,
  and_mask: u64,
}

impl FromStr for Mask {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (_, val) = s.split_once('=').unwrap();
    Ok(Self::new(val))
  }
}

impl Mask {
  fn new(string_mask: &str) -> Self {
    let mut or_mask = 0u64;
    let mut and_mask = 0u64;

    let one_mask = u64::MAX >> (type_size_in_bits::<u64>() - 1);
    let zero_mask = u64::MAX << 1;

    for symbol in string_mask.trim().chars() {
      match symbol {
        'X' => {
          or_mask <<= 1;
          and_mask <<= 1;

          and_mask |= one_mask;
          or_mask &= zero_mask;
        }
        '1' => {
          or_mask <<= 1;
          and_mask <<= 1;

          and_mask |= one_mask;
          or_mask |= one_mask;
        }
        '0' => {
          or_mask <<= 1;
          and_mask <<= 1;

          and_mask &= zero_mask;
          or_mask |= one_mask;
        }
        _ => {}
      }
    }
    Self { or_mask, and_mask }
  }
  fn apply_mask(&self, target: &u64) -> u64 {
    (target | self.or_mask) & self.and_mask
  }
}

enum Instr {
  Mask(Mask),
  Mem(MemStore),
}

impl FromStr for Instr {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s.starts_with("mask") {
      Instr::Mask(s.parse::<Mask>()?)
    } else {
      Instr::Mem(s.parse::<MemStore>()?)
    })
  }
}

struct Program {
  memory: HashMap<u64, u64>, // (address, val)
  active_mask: Mask,
  instructions: Vec<Instr>,
  bit_size: usize,
}

impl Program {
  fn new(mut instructions: Vec<Instr>) -> Self {
    if let Instr::Mask(active_mask) = instructions.remove(0) {
      Self {
        memory: HashMap::new(),
        active_mask,
        instructions,
        bit_size: 36,
      }
    } else {
      panic!("Invalid instruction set for program")
    }
  }

  fn store(&mut self, mem: &MemStore) {
    self.memory.insert(
      mem.address,
      truncate(&self.bit_size, &self.active_mask.apply_mask(&mem.val)),
    );
  }

  fn set_mask(&mut self, mask: &Mask) {
    self.active_mask = *mask;
  }

  fn execute_all(&mut self) {
    let instr_copy = std::mem::replace(&mut self.instructions, Vec::new());
    for instr in instr_copy.iter() {
      match instr {
        Instr::Mask(next_mask) => self.set_mask(&next_mask),
        Instr::Mem(mem) => self.store(&mem),
      }
    }
    self.instructions = instr_copy;
  }

  fn memory_sum(&self) -> u64 {
    self.memory.values().sum()
  }
}

fn truncate(bit_size: &usize, target: &u64) -> u64 {
  let bit_mask = u64::MAX >> (type_size_in_bits::<u64>() - bit_size);
  let result = target & bit_mask;

  result
}

fn type_size_in_bits<T>() -> usize {
  std::mem::size_of::<T>() * 8
}

pub fn solve_part_1(input_root: &str) {
  let mut program = get_data(input_root);
  program.execute_all();
  println!("(Day 14, Part 1) Memory sum = {}", program.memory_sum());
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 14, Part 2) Not Implemented");
}

fn get_data(root: &str) -> Program {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_14.input.txt", root);
  println!("Reading input from '{}'", path);

  let instructions = fr::parse_input::<Instr>(&path, "\r\n");
  Program::new(instructions)
}
#[cfg(test)]
mod tests {
  use super::*;

  fn get_data() -> String {
    "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
      .to_string()
  }
  #[test]
  fn memory_sum_is_correct_after_execution() {
    let mut program = Program::new(
      get_data()
        .lines()
        .map(|l| l.parse::<Instr>().unwrap())
        .collect::<Vec<_>>(),
    );

    program.execute_all();
    let result = program.memory_sum();
    assert_eq!(165, result);
  }
}
