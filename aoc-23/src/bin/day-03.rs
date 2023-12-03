use std::{
    fmt::{write, Display},
    str::FromStr,
};

const INPUT: &'static str = include_str!("input3.txt");

fn main() {
    println!("Day 03 Hello, World");

    let res = day3_exec().expect("Failed to execute day 3");

    println!("Day 03 result:");
    println!("{res}");
}

fn day3_exec() -> Result<String, Day3Err> {
    let p1_res = part1()?;
    let p2_res = part2()?;

    Ok(format!("{p1_res}\n{p2_res}"))
}

#[derive(Debug, Clone)]
struct Day3Err {
    msg: String,
}

impl Display for Day3Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {msg}", msg = self.msg)
    }
}

impl Day3Err {
    fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        Self { msg: s.to_string() }
    }
}

// [from, to)
#[derive(Debug, Clone)]
struct Range {
    from: usize,
    // exclusive to
    to: usize,
}

impl Range {
    fn len(&self) -> usize {
        self.to - self.from
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{from}, {to})", from = self.from, to = self.to)
    }
}

impl Range {
    fn new(from: usize, to: usize) -> Result<Self, Day3Err> {
        if from >= to {
            Err(Day3Err::new(
                "Invalid range, from ({from}) is greater-or-equal than to ({to})",
            ))
        } else {
            Ok(Self { from, to })
        }
    }
}

#[derive(Debug, Clone)]
struct SchemaLocation {
    row: usize,
    col: Range,
}

impl SchemaLocation {
    fn is_adjacent_to(&self, other: &SchemaLocation) -> bool {
        let min_row = self.row.saturating_sub(1);
        let max_row = self.row.saturating_add(1);

        // row condition
        other.row >= min_row && other.row <= max_row &&
            // column condition
        other.col.to >= self.col.from && other.col.to <= (self.col.to + other.col.len())
    }
}

impl Display for SchemaLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R: {row}; C: {range}", row = self.row, range = self.col)
    }
}

impl SchemaLocation {
    fn new(row: usize, col: Range) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct Number {
    val: u32,
    loc: SchemaLocation,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{num} @ {loc}", num = self.val, loc = self.loc)
    }
}

impl Number {
    fn new(val: u32, loc: SchemaLocation) -> Self {
        Self { val, loc }
    }
}

struct Symbol {
    sym: char,
    loc: SchemaLocation,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{sym} @ {loc}", sym = self.sym, loc = self.loc)
    }
}

impl Symbol {
    fn new(sym: char, loc: SchemaLocation) -> Self {
        Self { sym, loc }
    }
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    ratio: u32,
}

impl Gear {
    fn new(part1: &Number, part2: &Number) -> Self {
        Self {
            ratio: part1.val * part2.val,
        }
    }
}

struct Schema {
    nums: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Schema {
    fn parts(&self) -> Vec<Number> {
        self.nums
            .iter()
            .filter(|n| is_part(n, &self.symbols))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn gears(&self) -> Vec<Gear> {
        let mut rv = Vec::<Gear>::new();
        let parts = self.parts();
        'next_gear: for gear_candidate in self.symbols.iter().filter(|s| s.sym == '*') {
            let mut part1: Option<Number> = None;
            let mut part2: Option<Number> = None;

            for part in parts
                .iter()
                .filter(|p| p.loc.is_adjacent_to(&gear_candidate.loc))
            {
                match (&part1, &part2) {
                    (None, _) => part1 = Some(part.clone()),
                    (Some(_), None) => part2 = Some(part.clone()),
                    _ => continue 'next_gear,
                }
            }

            if let (Some(part1), Some(part2)) = (part1, part2) {
                rv.push(Gear::new(&part1, &part2));
            }
        }

        rv
    }
}

fn is_part(num: &Number, syms: &[Symbol]) -> bool {
    syms.iter()
        .map(|s| &s.loc)
        .any(|sym_loc| num.loc.is_adjacent_to(sym_loc))
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Numbers:")?;
        for num in self.nums.iter() {
            writeln!(f, "\t{num}")?;
        }
        writeln!(f, "----------------------------")?;
        writeln!(f, "Symbols:")?;
        for sym in self.symbols.iter() {
            writeln!(f, "\t{sym}")?;
        }

        Ok(())
    }
}

impl FromStr for Schema {
    type Err = Day3Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SWALLOW_CHAR: char = '.';

        let mut nums: Vec<Number> = Vec::new();
        let mut symbols: Vec<Symbol> = Vec::new();

        // dont re-create to preserve allocated capacity
        let mut num_buf = Vec::new();
        for (row, l) in s.lines().enumerate() {
            let mut start_col = 0;
            let mut current_col = 0;
            let mut reading_num = false;
            num_buf.clear();

            for (col, c) in l.trim().chars().enumerate() {
                current_col = col;
                // swallow dots to find something interesting
                if c == SWALLOW_CHAR && !reading_num {
                    continue;
                }

                // we started reading number or are _still_ reading number
                if c.is_numeric() {
                    if !reading_num {
                        start_col = col;
                        reading_num = true;
                    }

                    num_buf.push(c);

                    continue;
                }

                // we detected symbol (either meaningful or not)
                // but first we need to take care of number we were reading so far
                if reading_num {
                    let n = to_num(&num_buf);
                    let r = Range::new(start_col, current_col).expect("To have valid Range");
                    let loc = SchemaLocation::new(row, r);
                    nums.push(Number::new(n, loc));

                    reading_num = false;
                    num_buf.clear();
                }

                // now check if symbol we found is meaningful
                if c != SWALLOW_CHAR {
                    let r = Range::new(col, col + 1)
                        .expect("Range for symbol should always be correct");
                    let loc = SchemaLocation::new(row, r);
                    let sym = Symbol::new(c, loc);
                    symbols.push(sym);
                }
            }

            // check if number ended at the line end
            if !num_buf.is_empty() {
                let n = to_num(&num_buf);
                let r = Range::new(start_col, current_col).expect("To have valid Range");
                let loc = SchemaLocation::new(row, r);
                nums.push(Number::new(n, loc));
            }
        }

        Ok(Self { nums, symbols })
    }
}

fn to_num(chars: &[char]) -> u32 {
    chars
        .iter()
        .map(|c| c.to_digit(10).expect("to be working with digits only"))
        .fold(0, |acc, e| acc * 10 + e)
}

fn get_schema() -> Schema {
    let sch = INPUT.parse::<Schema>().expect("to parse schema");
    sch
}

fn part1() -> Result<String, Day3Err> {
    let schema = get_schema();
    let sum = schema.parts().into_iter().map(|p| p.val).sum::<u32>();

    Ok(format!("Day 03 Part 01: Parts numbers sum is: {sum}"))
}

fn part2() -> Result<String, Day3Err> {
    let schema = get_schema();
    let sum = schema.gears().into_iter().map(|g| g.ratio).sum::<u32>();

    Ok(format!("Day 03 Part 01: Parts numbers sum is: {sum}"))
}
