use crate::day_exec::DayExecutor;

use std::{
    fmt::{Display, Write},
    iter::Cycle,
    str::FromStr,
};

pub struct Day10;

impl DayExecutor for Day10 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Single strength after six samples: {}",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(solve_part2(&input))
    }
}

fn solve_part1(input: &str) -> isize {
    let sample_frequencies = [20, 60, 100, 140, 180, 220];
    let mut sampler = Sampler::new(sample_frequencies);
    let mut cpu = CPU::new(get_instr_set(input));

    while let CycleResult::Continue(cycle, reg) = cpu.cycle() {
        sampler.inspect(cycle, reg);
    }

    sampler.samples.iter().sum()
}

fn solve_part2(input: &str) -> Screen {
    let mut cpu = CPU::new(get_instr_set(input));

    while let CycleResult::Continue(_, _) = cpu.cycle() {}

    cpu.screen
}

fn get_instr_set(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|l| l.trim().parse::<Instr>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not parse list of instructions")
}

struct Screen {
    screen: Vec<bool>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let screen = vec![false; width * height];

        Self {
            screen,
            width,
            height,
        }
    }

    fn inspect(&mut self, cycle: usize, reg: isize) {
        let pixel_position = (cycle - 1) % (self.screen.len());

        let pixel_pos_in_sprite_coords = (pixel_position % self.width) as isize;

        if (reg - 1..=reg + 1).contains(&pixel_pos_in_sprite_coords) {
            self.screen[pixel_position] = true;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ON: char = '#';
        const OFF: char = '.';

        f.write_char('\n')?;

        for (idx, &is_on) in self.screen.iter().enumerate() {
            if is_on {
                f.write_char(ON)?;
            } else {
                f.write_char(OFF)?;
            }

            if (idx + 1) % self.width == 0 {
                f.write_char('\n')?;
            }
        }

        f.write_char('\n')
    }
}

struct CPU {
    cycle_counter: usize,
    register: isize,
    instr_set: Vec<Instr>,
    instr_idx: usize,
    execution_countdown: usize,
    screen: Screen,
}

impl CPU {
    fn new(instr: Vec<Instr>) -> Self {
        Self {
            cycle_counter: 1,
            register: 1,
            instr_set: instr,
            instr_idx: 0,
            execution_countdown: 0,
            screen: Screen::new(40, 6),
        }
    }

    fn cycle(&mut self) -> CycleResult {
        if self.execution_countdown == 0 && self.instr_idx == self.instr_set.len() {
            return CycleResult::End;
        }

        self.screen.inspect(self.cycle_counter, self.register);
        self.cycle_counter += 1;
        if self.execution_countdown > 1 {
            self.execution_countdown -= 1;
        } else if self.execution_countdown == 1 {
            self.execution_countdown -= 1;
            let instr = self.instr_set[self.instr_idx];
            self.process_instr(&instr);
            self.instr_idx += 1;
        } else {
            let instr = self.instr_set[self.instr_idx];
            match instr {
                Instr::Noop => self.instr_idx += 1,
                Instr::AddX(_) => self.execution_countdown = 1,
            }
        }

        CycleResult::Continue(self.cycle_counter, self.register)
    }

    fn process_instr(&mut self, instr: &Instr) {
        match instr {
            Instr::Noop => (),
            Instr::AddX(val) => {
                self.register += val;
            }
        }
    }
}

enum CycleResult {
    Continue(usize, isize),
    End,
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Noop,
    AddX(isize),
}

#[derive(Debug)]
struct InstrParseError;
impl FromStr for Instr {
    type Err = InstrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((instr_str, add_val_str)) = s.trim().split_once(" ") {
            if instr_str == "addx" {
                add_val_str
                    .parse::<isize>()
                    .map(|val| Instr::AddX(val))
                    .map_err(|_| InstrParseError)
            } else {
                Err(InstrParseError)
            }
        } else if s.trim() == "noop" {
            Ok(Instr::Noop)
        } else {
            Err(InstrParseError)
        }
    }
}

#[derive(Debug)]
struct Sampler<const N: usize> {
    freq: [usize; N],
    samples: [isize; N],
}

impl<const N: usize> Sampler<N> {
    fn new(sample_freq: [usize; N]) -> Self {
        Self {
            freq: sample_freq,
            samples: [0; N],
        }
    }

    fn inspect(&mut self, current_cycle: usize, reg_value: isize) {
        for (i, freq) in self.freq.iter().enumerate() {
            if *freq == current_cycle {
                self.samples[i] = reg_value * current_cycle as isize;
            }
        }
    }

    fn samples(&self) -> &[isize; N] {
        &self.samples
    }
}
