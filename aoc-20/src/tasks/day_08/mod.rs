use std::str::FromStr;

mod part_1;
mod part_2;

#[derive(Debug, Clone, Copy)]
enum Instr {
  Acc(i32),
  Jmp(i32),
  Nop(i32),
}

impl FromStr for Instr {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let (instr_str, num_str) = input.trim().split_once(' ').unwrap();
    let num = num_str
      .parse::<i32>()
      .map_err(|_| "Instruction offset not parsable".to_string())?;

    match instr_str {
      "acc" => Ok(Instr::Acc(num)),
      "jmp" => Ok(Instr::Jmp(num)),
      "nop" => Ok(Instr::Nop(num)),
      _ => Err(format!("Unknown instruction: '{}'", instr_str)),
    }
  }
}

struct Executor {
  ops: Vec<Instr>,
  acc: i32,
  op_ptr: usize,
}

impl Executor {
  fn new(ops: Vec<Instr>) -> Self {
    let acc = 0i32;
    let op_ptr = 0usize;
    Self { ops, acc, op_ptr }
  }

  // returns (instr_index, acc state), acc state is AFTER instruction is executed
  fn execute_next(&mut self) -> Option<(usize, i32)> {
    if self.op_ptr >= self.ops.len() {
      None
    } else {
      let old_ptr = self.op_ptr;
      self.op_ptr += 1; // all instructions advance pointer forward
      match self.ops[old_ptr] {
        Instr::Acc(val) => self.acc += val,
        Instr::Jmp(offset) => self.op_ptr = usize_i32_addition(self.op_ptr, offset - 1),
        Instr::Nop(_) => {}
      };
      Some((old_ptr, self.acc))
    }
  }
}

fn usize_i32_addition(num: usize, sub: i32) -> usize {
  if sub < 0i32 {
    num - (sub.abs() as usize)
  } else {
    num + (sub as usize)
  }
}

fn get_data(root: &str) -> Vec<Instr> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_08.input.txt", root);
  println!("Reading input from '{}'", &path);
  fr::parse_input::<Instr>(&path, "\r\n")
}

pub fn solve_part_1(input_root: &str) {
  let data = get_data(input_root);
  let result = part_1::get_acc_state_before_loop_starts(data);
  println!(
    "(Day 8, Part 1) Accumulator state before infinite loop start was '{}'",
    result
  );
}

pub fn solve_part_2(input_root: &str) {
  let data = get_data(input_root);
  let result = part_2::accumulator_of_terminating_version(data);
  println!(
    "(Day 8, Part 2) Accumulator after termination has value of '{}'",
    result
  );
}
