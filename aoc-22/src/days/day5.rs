use std::str::FromStr;

use crate::day_exec::DayExecutor;
pub struct Day5;

impl DayExecutor for Day5 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "After processing all instructions top crates construct message: '{}'",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "After processing all instructions with new crate, top crates construct message: '{}'",
            solve_part2(&input)
        ))
    }
}

fn solve_part1(input: &str) -> String {
    let (mut crane, instruction_set) = get_parsed_input(input);

    for instr in instruction_set {
        crane.process_by_moving_single(&instr);
    }

    crane.top_message()
}

fn solve_part2(input: &str) -> String {
    let (mut crane, instruction_set) = get_parsed_input(input);

    for instr in instruction_set {
        crane.process_by_moving_stack(&instr);
    }

    crane.top_message()
}

fn get_parsed_input(input: &str) -> (Crane, Vec<Instruction>) {
    let (crane_str, instructions_str) = input
        .split_once("\n\n")
        .expect("Could not split input into crate init part and instruction set part");

    let crane = crane_str
        .parse::<Crane>()
        .expect("Could not parse crane initial state");
    let instructions = instructions_str
        .lines()
        .map(|l| {
            l.trim()
                .parse::<Instruction>()
                .expect("Failed to parse instructions")
        })
        .collect::<Vec<_>>();

    (crane, instructions)
}

struct StackId(usize);
struct Stack {
    id: StackId,
    crates: Vec<Crate>,
}

impl Stack {
    fn new(id: StackId) -> Self {
        Self {
            id,
            crates: Vec::new(),
        }
    }

    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    fn pop_n(&mut self, how_many: usize) -> Option<Vec<Crate>> {
        if how_many > self.crates.len() {
            None
        } else {
            Some(self.crates.split_off(self.crates.len() - how_many))
        }
    }

    fn push(&mut self, krate: Crate) {
        self.crates.push(krate)
    }

    fn push_n(&mut self, mut crates: Vec<Crate>) {
        self.crates.append(&mut crates);
    }

    fn peek_top(&self) -> Option<&Crate> {
        self.crates.last()
    }
}

struct Crane {
    stacks: Vec<Stack>,
}

impl Crane {
    fn new(stacks: Vec<Stack>) -> Self {
        Self { stacks }
    }

    fn process_by_moving_single(&mut self, instr: &Instruction) {
        let source_index = instr.from.0 - 1;
        let target_index = instr.to.0 - 1;

        for _ in 0..instr.how_many {
            let popped = self.stacks[source_index]
                .pop()
                .expect("Instructions made us pop from stack that has no crates");

            self.stacks[target_index].push(popped);
        }
    }

    fn process_by_moving_stack(&mut self, instr: &Instruction) {
        let source_index = instr.from.0 - 1;
        let target_index = instr.to.0 - 1;

        let popped = self.stacks[source_index]
            .pop_n(instr.how_many)
            .expect("Instructions made us pop from stack more crates  than there exist on stack");

        self.stacks[target_index].push_n(popped);
    }

    fn top_message(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.peek_top().map(|krate| krate.id.0))
            .collect()
    }
}

#[derive(Debug)]
struct CraneParseError(String);
impl FromStr for Crane {
    type Err = CraneParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<Vec<_>>();
        let stack_ids = lines.pop().ok_or(CraneParseError(s.to_owned())).map(|l| {
            l.split_whitespace()
                .map(|id_str| {
                    id_str
                        .trim()
                        .parse::<usize>()
                        .expect("Can not parse stack id")
                })
                .map(|id| StackId(id))
                .collect::<Vec<_>>()
        })?;

        let mut stacks = stack_ids
            .into_iter()
            .map(|id| Stack::new(id))
            .collect::<Vec<_>>();

        for stack_line in lines.into_iter().rev() {
            let slice = stack_line.chars().collect::<Vec<_>>();

            let crates = slice
                .chunks(4)
                .map(|ch| ch.into_iter().collect::<String>())
                .map(|id| {
                    let trimmed = id.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.parse::<Crate>().expect("Could not parse crate"))
                    }
                })
                .collect::<Vec<_>>();

            for (i, crate_opt) in crates.into_iter().enumerate() {
                if let Some(krate) = crate_opt {
                    stacks[i].push(krate)
                }
            }
        }

        Ok(Self::new(stacks))
    }
}

struct CrateId(char);
struct Crate {
    id: CrateId,
}

impl Crate {
    fn new(id: CrateId) -> Self {
        Self { id }
    }
}

#[derive(Debug)]
struct CrateParseError(String);
impl FromStr for Crate {
    type Err = CrateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trim_pattern: &[_] = &['[', ']'];

        let id = s
            .trim_matches(trim_pattern)
            .chars()
            .next()
            .ok_or(CrateParseError(s.to_owned()))?;

        Ok(Self::new(CrateId(id)))
    }
}

struct Instruction {
    from: StackId,
    to: StackId,
    how_many: usize,
}

impl Instruction {
    fn new(from: StackId, to: StackId, how_many: usize) -> Self {
        Self { from, to, how_many }
    }
}

#[derive(Debug)]
struct InstructionParseError(String);
impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut how_many_str, rest) = s
            .trim()
            .split_once("from")
            .ok_or(InstructionParseError(s.to_owned()))?;

        let (from_str, to_str) = rest
            .trim()
            .split_once("to")
            .ok_or(InstructionParseError(s.to_owned()))?;

        how_many_str = how_many_str
            .trim()
            .trim_start_matches(|c: char| !c.is_digit(10))
            .trim();

        let how_many = how_many_str
            .parse::<usize>()
            .map_err(|_| InstructionParseError(s.to_owned()))?;

        let from = from_str
            .trim()
            .parse::<usize>()
            .map_err(|_| InstructionParseError(s.to_owned()))?;

        let to = to_str
            .trim()
            .parse::<usize>()
            .map_err(|_| InstructionParseError(s.to_owned()))?;

        Ok(Self::new(StackId(from), StackId(to), how_many))
    }
}
