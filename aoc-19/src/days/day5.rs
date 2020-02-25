use crate::days::*;
use std::collections::VecDeque;
struct ContinueExecution(pub bool);

impl From<bool> for ContinueExecution {
    fn from(continue_: bool) -> Self {
        Self(continue_)
    }
}

#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Halt,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
}

impl OpCode {
    // returns tuple that represents (parameter count, store count)
    fn parameter_count(&self) -> (usize, bool) {
        match self {
            OpCode::Add => (2, true),
            OpCode::Multiply => (2, true),
            OpCode::Halt => (0, false),
            OpCode::Input => (0, true),
            OpCode::Output => (1, false),
            OpCode::JumpIfTrue => (2, false),
            OpCode::JumpIfFalse => (2, false),
            OpCode::LessThan => (2, true),
            OpCode::Equals => (2, true),
            OpCode::RelativeBaseOffset => (1, false),
        }
    }

    fn from_value(value: usize) -> Option<Self> {
        match value {
            1 => Some(Self::Add),
            2 => Some(Self::Multiply),
            3 => Some(Self::Input),
            4 => Some(Self::Output),
            5 => Some(Self::JumpIfTrue),
            6 => Some(Self::JumpIfFalse),
            7 => Some(Self::LessThan),
            8 => Some(Self::Equals),
            9 => Some(Self::RelativeBaseOffset),
            99 => Some(Self::Halt),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_value(value: usize) -> Option<Self> {
        match value {
            2 => Some(Self::Relative),
            1 => Some(Self::Immediate),
            0 => Some(Self::Position),
            _ => None,
        }
    }
}

struct Parameter {
    mode: ParameterMode,
    index: usize,
}

impl Parameter {
    fn new(mode: ParameterMode, index: usize) -> Self {
        Self { mode, index }
    }

    fn from_value(mode: usize, index: usize) -> Self {
        let mode = ParameterMode::from_value(mode).unwrap();
        Self::new(mode, index)
    }
}

struct Instruction {
    opcode: OpCode,
    parameters: Vec<Parameter>,
    store: Option<Parameter>,
}

impl Instruction {
    fn new(opcode: OpCode, parameters: Vec<Parameter>, store: Option<Parameter>) -> Self {
        Self {
            opcode,
            parameters,
            store,
        }
    }

    fn pointer_offset(&self) -> usize {
        let store_len = if let Some(_) = self.store { 1 } else { 0 };
        // opcode + parameter num + store location if it exists
        self.parameters.len() + store_len + 1
    }
}

#[derive(Clone)]
pub struct Intcode {
    instructions: Vec<isize>,
    instruction_pointer: usize,
    inputs: VecDeque<isize>,
    pub outputs: Vec<isize>,
    diagnostic_code: Option<isize>,
    pointer_jumped: bool,
    pause_on_output: bool,
    halted: bool,
    relative_base: isize,
}

impl Intcode {
    fn new(instructions: &[isize], inputs: &[isize], pause_on_output: bool) -> Self {
        let instructions = instructions.iter().map(|&x| x).collect::<Vec<_>>();
        let inputs = inputs.iter().map(|&x| x).collect::<VecDeque<_>>();
        Intcode {
            instructions,
            instruction_pointer: 0,
            inputs,
            outputs: vec![],
            diagnostic_code: None,
            pointer_jumped: false,
            pause_on_output,
            halted: false,
            relative_base: 0,
        }
    }

    fn current_instruction(&self) -> Instruction {
        let mut meta_value = self.instructions[self.instruction_pointer] as usize;
        let opcode = OpCode::from_value(meta_value % 100).unwrap();
        meta_value /= 100;

        let param_count = opcode.parameter_count();
        let mut params = Vec::<Parameter>::with_capacity(param_count.0);

        for i in 1..=param_count.0 {
            let operand = Parameter::from_value(meta_value % 10, self.instruction_pointer + i);
            meta_value /= 10;
            params.push(operand);
        }

        let store = if param_count.1 {
            Some(Parameter::from_value(
                meta_value % 10,
                self.instruction_pointer + param_count.0 + 1,
            ))
        } else {
            None
        };

        Instruction::new(opcode, params, store)
    }

    fn parameter_value(&self, param: &Parameter) -> isize {
        match param.mode {
            ParameterMode::Immediate => self.at(param.index),
            ParameterMode::Position => self.at(self.at(param.index) as usize),
            ParameterMode::Relative => {
                self.at((self.relative_base + self.at(param.index)) as usize)
            }
        }
    }

    fn at(&self, index: usize) -> isize {
        *self.instructions.get(index).unwrap_or(&0)
    }

    fn store(&mut self, store_param: &Parameter, value: isize) {
        let location = match store_param.mode {
            ParameterMode::Position => self.at(store_param.index) as usize,
            ParameterMode::Relative => (self.relative_base + self.at(store_param.index)) as usize,
            ParameterMode::Immediate => {
                panic!("Immediate paramter mode not allowed for store operation")
            }
        };

        if location >= self.instructions.len() {
            let to_add = location - self.instructions.len() + 1;
            let mut slice: Vec<isize> = (0..to_add).map(|_| 0).collect();
            self.instructions.append(&mut slice);
        }
        self.instructions[location] = value;
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> ContinueExecution {
        match instruction.opcode {
            OpCode::Halt => {
                self.halted = true;
                false.into()
            }
            OpCode::Output => {
                let param = instruction.parameters.first().unwrap();
                let val = self.parameter_value(param);
                self.outputs.push(val);
                self.diagnostic_code = Some(val);
                (!self.pause_on_output).into()
            }
            OpCode::Input => {
                self.diagnostic_code = None;
                let value = self.inputs.pop_front().unwrap();
                self.store(instruction.store.as_ref().unwrap(), value);
                true.into()
            }
            OpCode::Add => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                self.store(
                    instruction.store.as_ref().unwrap(),
                    first_operand + second_operand,
                );
                true.into()
            }
            OpCode::Multiply => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                self.store(
                    instruction.store.as_ref().unwrap(),
                    first_operand * second_operand,
                );
                true.into()
            }
            OpCode::JumpIfTrue => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                if first_operand != 0 {
                    self.instruction_pointer = second_operand as usize;
                    self.pointer_jumped = true;
                }
                true.into()
            }
            OpCode::JumpIfFalse => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                if first_operand == 0 {
                    self.instruction_pointer = second_operand as usize;
                    self.pointer_jumped = true;
                }
                true.into()
            }
            OpCode::Equals => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                if first_operand == second_operand {
                    self.store(instruction.store.as_ref().unwrap(), 1);
                } else {
                    self.store(instruction.store.as_ref().unwrap(), 0);
                }
                true.into()
            }
            OpCode::LessThan => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);
                let second_operand = self.parameter_value(&instruction.parameters[1]);

                if first_operand < second_operand {
                    self.store(instruction.store.as_ref().unwrap(), 1);
                } else {
                    self.store(instruction.store.as_ref().unwrap(), 0);
                }
                true.into()
            }
            OpCode::RelativeBaseOffset => {
                self.diagnostic_code = None;
                let first_operand = self.parameter_value(&instruction.parameters[0]);

                self.relative_base += first_operand;
                true.into()
            }
        }
    }

    pub fn run_program(&mut self) -> Option<isize> {
        if self.halted {
            None
        } else {
            let mut insctruction = self.current_instruction();

            while self.execute_instruction(&insctruction).0 {
                self.advance_to_next_instruction(&insctruction);
                insctruction = self.current_instruction();
            }
            if !self.halted {
                self.advance_to_next_instruction(&insctruction);
            }
            self.outputs.last().map_or(None, |&x| Some(x))
        }
    }

    pub fn add_inputs(&mut self, inputs: &[isize]) {
        inputs.iter().for_each(|&x| self.inputs.push_back(x))
    }
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn advance_to_next_instruction(&mut self, current_instruction: &Instruction) {
        if !self.pointer_jumped {
            self.instruction_pointer += current_instruction.pointer_offset();
        }
        self.pointer_jumped = false;
    }

    pub fn parsed(text: &str, inputs: &[isize], pause_on_output: bool) -> Self {
        let result: Vec<isize> = text
            .split(",")
            .map(|x| x.parse::<isize>().expect("Not valid intcode"))
            .collect();
        Self::new(result.as_ref(), inputs, pause_on_output)
    }

    pub fn set_inputs(&mut self, inputs: &[isize]) {
        self.inputs = inputs.iter().map(|x| *x).collect();
    }
}

pub struct Day5Runner {
    path: String,
    part: Parts,
}

impl Day5Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> isize {
        let mut intcode = self.load(&[1]);
        intcode.run_program();
        intcode.diagnostic_code.unwrap()
    }
    fn part2(&self) -> isize {
        let mut intcode = self.load(&[5]);
        intcode.run_program();
        intcode.diagnostic_code.unwrap()
    }

    fn load(&self, inputs: &[isize]) -> Intcode {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        Intcode::parsed(
            &text.expect("Could not read instructions")[0],
            inputs,
            false,
        )
    }
}

impl Runner for Day5Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
