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
#[derive(Clone)]
struct Mask {
  string_mask: String,
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
    Self {
      string_mask: string_mask.trim().to_string(),
    }
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

type DecoderT = fn(&Mask, &MemStore) -> Vec<(u64, u64)>;
struct Program {
  memory: HashMap<u64, u64>, // (address, val)
  active_mask: Mask,
  instructions: Vec<Instr>,
  bit_size: usize,
  decoder: DecoderT,
}

impl Program {
  fn new(mut instructions: Vec<Instr>, decoder: DecoderT) -> Self {
    if let Instr::Mask(active_mask) = instructions.remove(0) {
      Self {
        memory: HashMap::new(),
        active_mask,
        instructions,
        bit_size: 36,
        decoder,
      }
    } else {
      panic!("Invalid instruction set for program")
    }
  }

  fn store(&mut self, mem: &MemStore) {
    for (k, v) in (self.decoder)(&self.active_mask, mem) {
      self.memory.insert(k, truncate(&self.bit_size, &v));
    }
  }

  fn set_mask(&mut self, mask: &Mask) {
    self.active_mask = mask.clone();
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
  let result = part_1::memory_sum_after_execution(get_data(input_root));
  println!("(Day 14, Part 1) Memory sum = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 14, Part 2) Not Implemented");
}

fn get_data(root: &str) -> Vec<Instr> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_14.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<Instr>(&path, "\r\n")
}
