use std::collections::HashSet;

use super::{Executor, Instr};

pub(super) fn accumulator_of_terminating_version(instructions: Vec<Instr>) -> i32 {
  let indexes = instructions
    .iter()
    .enumerate()
    .filter_map(|(i, elem)| match elem {
      Instr::Acc(_) => None,
      _ => Some(i),
    })
    .collect::<Vec<usize>>();

  for index in indexes {
    let mut version = instructions.to_vec();
    version[index] = flip_jmp_and_nop(&version[index]);

    if let Some(term_acc) = execute_until_termination(version) {
      return term_acc;
    }
  }

  panic!("Terminating version of program not found!");
}

fn flip_jmp_and_nop(instr: &Instr) -> Instr {
  match instr {
    Instr::Acc(_) => panic!("Cant flip acc instr"),
    Instr::Jmp(offset) => Instr::Nop(*offset),
    Instr::Nop(num) => Instr::Jmp(*num),
  }
}

fn execute_until_termination(instr_version: Vec<Instr>) -> Option<i32> {
  let mut exec = Executor::new(instr_version);
  let mut history = HashSet::<usize>::new();

  loop {
    match exec.execute_next() {
      Some((instr_ptr, _)) => {
        // we looped
        if history.contains(&instr_ptr) {
          return None;
        } else {
          history.insert(instr_ptr);
        }
      }
      None => return Some(exec.acc), // program terminated, get lass accumulator
    }
  }
}
