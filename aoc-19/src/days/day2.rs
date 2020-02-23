use crate::days::*;
use crate::input_reader::{read_sparated_values_from_input, ParseError, Parser};

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

#[derive(Clone)]
struct Intcode {
    instructions: Vec<usize>,
    position: usize,
}

impl Intcode {
    fn new(instructions: Vec<usize>) -> Self {
        Intcode {
            instructions,
            position: 0,
        }
    }
    fn parsed(text: &[&str]) -> Self {
        let result = Self::parse_all(text)
            .into_iter()
            .map(|elem| elem.expect("Could not parse!"))
            .collect();

        Self::new(result)
    }

    fn store(&mut self, index: usize, value: usize) {
        self.instructions[index] = value;
    }

    fn at(&self, index: usize) -> usize {
        self.instructions[index]
    }

    fn instruction_at_current_position(&self) -> Instruction {
        let opcode = self.at(self.position);
        let first_index = self.at(self.position + 1);
        let second_index = self.at(self.position + 2);
        let store_index = self.at(self.position + 3);

        Instruction::new(
            opcode,
            first_index,
            second_index,
            store_index,
            &self.instructions,
        )
    }

    fn advance_position_to_next_instruction(&mut self) {
        self.position += 4
    }

    fn run(&mut self) {
        loop {
            let inst = self.instruction_at_current_position();
            let result = inst.perform();

            match result {
                Some(value) => self.store(value.index, value.value),
                None => break,
            }

            self.advance_position_to_next_instruction();
        }
    }
}

impl Parser for Intcode {
    type R = usize;
    fn parse_line(line: &str) -> Result<Self::R, ParseError> {
        line.parse::<usize>()
            .map_err(|_| ParseError::new_copy("Could not parse number", line))
    }
}

#[derive(Debug)]
struct IntcodeValue {
    index: usize,
    value: usize,
}

impl IntcodeValue {
    fn new(index: usize, value: usize) -> Self {
        IntcodeValue { index, value }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Option<OpCode>,
    first_operand: IntcodeValue,
    second_operand: IntcodeValue,
    store_location_index: usize,
}

impl Instruction {
    fn new(
        opcode_num: usize,
        first_operand_index: usize,
        second_operand_index: usize,
        store_location_index: usize,
        instructions: &[usize],
    ) -> Self {
        let opcode = match opcode_num {
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Multiply),
            99 => Some(OpCode::Halt),
            _ => None,
        };

        if let Some(OpCode::Halt) = opcode {
            let first_operand = IntcodeValue::new(first_operand_index, 0);
            let second_operand = IntcodeValue::new(second_operand_index, 0);

            Self {
                opcode,
                first_operand,
                second_operand,
                store_location_index,
            }
        } else {
            let first_operand =
                IntcodeValue::new(first_operand_index, instructions[first_operand_index]);
            let second_operand =
                IntcodeValue::new(second_operand_index, instructions[second_operand_index]);

            Self {
                opcode,
                first_operand,
                second_operand,
                store_location_index,
            }
        }
    }

    fn perform(&self) -> Option<IntcodeValue> {
        if let Some(operation) = self.opcode {
            match operation {
                OpCode::Halt => None,
                OpCode::Add => Some(IntcodeValue::new(
                    self.store_location_index,
                    self.first_operand.value + self.second_operand.value,
                )),
                OpCode::Multiply => Some(IntcodeValue::new(
                    self.store_location_index,
                    self.first_operand.value * self.second_operand.value,
                )),
            }
        } else {
            None
        }
    }
}

pub struct Day2Runner {
    path: String,
    part: Parts,
}

impl Day2Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let mut intcode = self.load();
        intcode.store(1, 12);
        intcode.store(2, 2);
        intcode.run();
        intcode.at(0)
    }
    fn part2(&self) -> usize {
        let wanted_result = 19690720usize;
        let original_intcode = self.load();

        for noun in 0..100 {
            for verb in 0..100 {
                let mut working_intcode = original_intcode.clone();
                working_intcode.store(1, noun);
                working_intcode.store(2, verb);
                working_intcode.run();
                if working_intcode.at(0) == wanted_result {
                    return Day2Runner::noun_verb_result(noun, verb);
                }
            }
        }
        0
    }

    fn noun_verb_result(noun: usize, verb: usize) -> usize {
        100 * noun + verb
    }

    fn load(&self) -> Intcode {
        let text = read_sparated_values_from_input(self.path.as_ref(), ",");
        Intcode::parsed(
            text.expect("Could not read instructions")
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .as_ref(),
        )
    }
}

impl Runner for Day2Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::Intcode;
    #[test]
    fn part1_test1() {
        let mut intcode = Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        intcode.run();
        let result = intcode.at(0);
        assert_eq!(3500, result);
    }
}
