use std::fmt::Display;

const INPUT: &'static str = include_str!("input4.txt");

fn main() {
    println!("Day 04 Hello, World");

    let res = day4_exec().expect("Failed to execute day 4");

    println!("Day 04 result:");
    println!("{res}");
}

fn day4_exec() -> Result<String, Day4Err> {
    let p1_res = part1()?;
    let p2_res = part2()?;

    Ok(format!("{p1_res}\n{p2_res}"))
}

fn part1() -> Result<String, Day4Err> {
    Ok(format!("Part 1 not implemented yet"))
}

fn part2() -> Result<String, Day4Err> {
    Ok(format!("Part 2 not implemented yet"))
}

#[derive(Debug, Clone)]
struct Day4Err {
    msg: String,
}

impl Display for Day4Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR:Day 04: {msg}", msg = self.msg)
    }
}

impl Day4Err {
    fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        Self { msg: s.to_string() }
    }
}
