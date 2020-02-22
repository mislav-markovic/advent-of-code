use crate::days::*;
use crate::input_reader::{read_sparated_values_from_input, ParseError, Parser};

enum OpCode {
    Add,
    Multiply,
    Halt,
}

struct Intcode {
    instructions: Vec<usize>,
}

impl Intcode {
    fn new(instructions: Vec<usize>) -> Self {
        Intcode { instructions }
    }
    fn parsed(text: &[&str]) -> Self {
        let result = Self::parse_all(text)
            .into_iter()
            .map(|elem| elem.expect("Could not parse!"))
            .collect();

        Self::new(result)
    }
}

impl Parser for Intcode {
    type R = usize;
    fn parse_line(line: &str) -> Result<Self::R, ParseError> {
        line.parse::<usize>()
            .map_err(|_| ParseError::new_copy("Could not parse number", line))
    }
}

struct IntcodeValue {
    index: usize,
    value: usize,
}

impl IntcodeValue {
    fn new(index: usize, value: usize) -> Self {
        IntcodeValue { index, value }
    }
}
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

        let first_operand =
            IntcodeValue::new(first_operand_index, instructions[first_operand_index]);
        let second_operand =
            IntcodeValue::new(second_operand_index, instructions[second_operand_index]);

        Instruction {
            opcode,
            first_operand,
            second_operand,
            store_location_index,
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

struct Day2Runner {
    path: String,
    part: Parts,
}

impl Day2Runner {
    pub fn with_input_path(part: Parts, path: &str) -> Self {
        let path = path.to_string();
        Day2Runner { path, part }
    }

    fn part1(&self) -> usize {
        0
    }
    fn part2(&self) -> usize {
        0
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
