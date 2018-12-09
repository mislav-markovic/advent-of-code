use crate::input_reader;

fn part1(input: &str) {}

fn part2(input: &str) {
    let data = input_reader::read_all(input);
}

pub fn day10() {
    let input = String::from("day10");

    println!("***Day Ten***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    // println!("\t\tWinning elfs score: {}", part1(&input));
    // println!("\t**Part Two**");
    // println!("\t\tWinning elfs score: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
