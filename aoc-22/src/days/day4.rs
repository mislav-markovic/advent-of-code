use std::str::FromStr;

use crate::day_exec::DayExecutor;
pub struct Day4;

impl DayExecutor for Day4 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Elf pairs where ones assignments in fully contained in others: {}",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Elf pairs that overlap at all: {}",
            solve_part2(&input)
        ))
    }
}

fn solve_part1(input: &str) -> u32 {
    get_elf_pairs_from_input(input)
        .into_iter()
        .fold(0u32, |acc, elem| {
            if elem.do_assignments_fully_overlap() {
                acc + 1
            } else {
                acc
            }
        })
}

fn solve_part2(input: &str) -> u32 {
    get_elf_pairs_from_input(input)
        .into_iter()
        .filter(|elf_pair| elf_pair.do_assignments_overlap())
        .count()
        .try_into()
        .expect("Counting number of pairs does not fit in u32!")
}

fn get_elf_pairs_from_input(input: &str) -> Vec<ElfPair> {
    input
        .lines()
        .map(|l| l.parse::<ElfPair>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse elf pair")
}

struct SectionRange {
    start: u32,
    end: u32,
}

struct SectionRangeError(u32, u32);
impl SectionRange {
    fn new(start: u32, end: u32) -> Result<Self, SectionRangeError> {
        if end < start {
            Err(SectionRangeError(start, end))
        } else {
            Ok(Self { start, end })
        }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        let range = self.start..=self.end;
        range.contains(&other.start) || range.contains(&other.end)
    }
}

struct SectionRangeParseError(String);
impl FromStr for SectionRange {
    type Err = SectionRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .trim()
            .split_once('-')
            .ok_or(SectionRangeParseError(s.to_owned()))?;

        let start = start_str
            .parse::<u32>()
            .map_err(|_| SectionRangeParseError(s.to_owned()))?;

        let end = end_str
            .parse::<u32>()
            .map_err(|_| SectionRangeParseError(s.to_owned()))?;

        Self::new(start, end).map_err(|_| SectionRangeParseError(s.to_owned()))
    }
}

struct ElfPair {
    first_assignment: SectionRange,
    second_assignment: SectionRange,
}

impl ElfPair {
    fn new(first_assignment: SectionRange, second_assignment: SectionRange) -> Self {
        Self {
            first_assignment,
            second_assignment,
        }
    }

    fn do_assignments_fully_overlap(&self) -> bool {
        self.first_assignment
            .fully_contains(&self.second_assignment)
            || self
                .second_assignment
                .fully_contains(&self.first_assignment)
    }

    fn do_assignments_overlap(&self) -> bool {
        self.first_assignment.overlaps(&self.second_assignment)
            || self.second_assignment.overlaps(&self.first_assignment)
    }
}

#[derive(Debug)]
struct ElfPairParseError(String);
impl FromStr for ElfPair {
    type Err = ElfPairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_elf_str, second_elf_str) = s
            .trim()
            .split_once(',')
            .ok_or(ElfPairParseError(s.to_owned()))?;

        let first_assignment = first_elf_str
            .parse::<SectionRange>()
            .map_err(|_| ElfPairParseError(s.to_owned()))?;

        let second_assignment = second_elf_str
            .parse::<SectionRange>()
            .map_err(|_| ElfPairParseError(s.to_owned()))?;

        Ok(Self::new(first_assignment, second_assignment))
    }
}
