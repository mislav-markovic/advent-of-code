use super::{Instr, Mask, MemStore, Program};

struct ValueMask {
  or_mask: u64,
  and_mask: u64,
}

impl ValueMask {
  fn from_mask(mask: &Mask) -> Self {
    let mut or_mask = 0u64;
    let mut and_mask = 0u64;

    let one_mask = u64::MAX >> (super::type_size_in_bits::<u64>() - 1);
    let zero_mask = u64::MAX << 1;

    for symbol in mask.string_mask.trim().chars() {
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

  fn apply_mask_to(&self, target: &u64) -> u64 {
    (target | self.or_mask) & self.and_mask
  }
}

pub(super) fn memory_sum_after_execution(instr: Vec<Instr>) -> u64 {
  let mut program = Program::new(instr, decoder);
  program.execute_all();
  program.memory_sum()
}

fn decoder(mask: &Mask, mem: &MemStore) -> Vec<(u64, u64)> {
  let value_mask = ValueMask::from_mask(&mask);
  vec![(mem.address, value_mask.apply_mask_to(&mem.val))]
}

#[cfg(test)]
mod tests {
  use super::super::Program;
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
      super::decoder,
    );

    program.execute_all();
    let result = program.memory_sum();
    assert_eq!(165, result);
  }
}
