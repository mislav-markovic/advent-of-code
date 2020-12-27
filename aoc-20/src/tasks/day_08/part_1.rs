use super::{Executor, Instr};
use std::collections::HashSet;

pub(super) fn get_acc_state_before_loop_starts(instructions: Vec<Instr>) -> i32 {
  let mut exec = Executor::new(instructions);
  let mut execution_history = HashSet::<usize>::new();
  let mut prev_acc = exec.acc;

  loop {
    let (instr_ptr, current_acc) = exec.execute_next().unwrap(); // part 1 program will never finish, otherwise this should panic
    if execution_history.contains(&instr_ptr) {
      return prev_acc;
    } else {
      execution_history.insert(instr_ptr);
      prev_acc = current_acc;
    }
  }
}
