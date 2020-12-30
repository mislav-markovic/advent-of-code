use super::{Instr, Mask, MemStore, Program};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bit {
  is_one: bool,
}

impl Bit {
  fn one() -> Self {
    Self { is_one: true }
  }
  fn zero() -> Self {
    Self { is_one: false }
  }
}

#[derive(PartialEq)]
struct XMask {
  index_bit_pairs: Vec<(usize, Bit)>,
}

impl XMask {
  fn apply_to(&self, target: &u64) -> u64 {
    let mut result = *target;

    for (idx, bit) in self.index_bit_pairs.iter() {
      result = set_bit_at_index(&result, &idx, &bit);
    }
    result
  }
}
struct AddressMask {
  floating_masks: Vec<XMask>,
  stabile_mask: u64,
}

impl AddressMask {
  fn from_mask(mask: &Mask) -> Self {
    let mut stabile_mask = 0u64;
    let zero_mask = u64::MAX << 1;

    for symbol in mask.string_mask.chars() {
      match symbol {
        '1' => {
          stabile_mask <<= 1;
          stabile_mask |= 1;
        }
        '0' | 'X' => {
          stabile_mask <<= 1;
          stabile_mask &= zero_mask;
        }
        _ => {}
      }
    }
    let x_indexes = mask
      .string_mask
      .chars()
      .rev()
      .enumerate()
      .filter(|(_, c)| *c == 'X')
      .map(|(i, _)| i)
      .collect::<Vec<_>>();

    let significant_digits = x_indexes.len();
    let combination_count = 2usize.pow(significant_digits as u32) as u64;

    let combinations = (0u64..combination_count)
      .into_iter()
      .map(|combinaiton| num_to_bit_vec(&combinaiton, &significant_digits))
      .collect::<Vec<_>>();

    let mut x_masks = Vec::with_capacity(combinations.len());
    for combination in combinations {
      x_masks.push(
        x_indexes
          .iter()
          .copied()
          .zip(combination.into_iter())
          .collect::<Vec<_>>(),
      );
    }

    Self {
      floating_masks: x_masks
        .into_iter()
        .map(|bits| XMask {
          index_bit_pairs: bits,
        })
        .collect(),
      stabile_mask: stabile_mask,
    }
  }

  fn apply_to(&self, target: &u64) -> Vec<u64> {
    let masked = target | self.stabile_mask;
    self
      .floating_masks
      .iter()
      .map(|mask| mask.apply_to(&masked))
      .collect::<Vec<_>>()
  }
}

fn num_to_bit_vec(num: &u64, mask_size: &usize) -> Vec<Bit> {
  let mut copy = *num;
  let last_bit_mask = 1u64;
  let mut result = Vec::with_capacity(*mask_size);

  for _ in 0usize..*mask_size {
    let bit = if (copy & last_bit_mask) == 1 {
      Bit::one()
    } else {
      Bit::zero()
    };
    result.push(bit);
    copy >>= 1;
  }
  result
}

pub(super) fn memory_sum_after_execution(instr: Vec<Instr>) -> u64 {
  let mut program = Program::new(instr, decoder);
  program.execute_all();
  program.memory_sum()
}

fn decoder(mask: &Mask, mem: &MemStore) -> Vec<(u64, u64)> {
  let address_mask = AddressMask::from_mask(&mask);
  address_mask
    .apply_to(&mem.address)
    .into_iter()
    .map(|masked_address| (masked_address, mem.val))
    .collect::<Vec<_>>()
}

fn set_bit_at_index(target: &u64, index: &usize, val: &Bit) -> u64 {
  if val.is_one {
    let mask = 1u64 << index;
    target | mask
  } else {
    let mask = (u64::MAX << 1).rotate_left(*index as u32);
    target & mask
  }
}
#[cfg(test)]
mod tests {
  use super::super::Program;
  use super::*;

  fn get_data() -> String {
    "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
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
    assert_eq!(208, result);
  }

  #[test]
  fn address_mask_correctly_constructed() {
    let mask = "mask = 000000000000000000000000000000X1001X"
      .parse::<Mask>()
      .unwrap();
    let addres_mask = AddressMask::from_mask(&mask);
    let expected_stabile_mask = 18u64;
    assert_eq!(expected_stabile_mask, addres_mask.stabile_mask);
  }

  #[test]
  fn address_mask_correctly_constructed_2() {
    let mask = "mask = 00000000000000000000000000000000X0XX"
      .parse::<Mask>()
      .unwrap();
    let address_mask = AddressMask::from_mask(&mask);
    let expected_stabile_mask = 0u64;
    let mut x_masks = Vec::<XMask>::new();

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::zero()), (1, Bit::zero()), (3, Bit::zero())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::zero()), (1, Bit::zero()), (3, Bit::one())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::zero()), (1, Bit::one()), (3, Bit::zero())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::zero()), (1, Bit::one()), (3, Bit::one())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::one()), (1, Bit::zero()), (3, Bit::zero())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::one()), (1, Bit::zero()), (3, Bit::one())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::one()), (1, Bit::one()), (3, Bit::zero())],
    });

    x_masks.push(super::XMask {
      index_bit_pairs: vec![(0, Bit::one()), (1, Bit::one()), (3, Bit::one())],
    });

    assert_eq!(x_masks.len(), address_mask.floating_masks.len());
    assert!(x_masks
      .iter()
      .all(|mask| address_mask.floating_masks.contains(mask)));
    assert_eq!(expected_stabile_mask, address_mask.stabile_mask);
  }
}
