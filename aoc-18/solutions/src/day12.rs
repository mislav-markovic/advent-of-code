use crate::input_reader;

fn part1(input: &str) {
    let data = input_reader::read_all_lines(input);
}

fn part2(input: &str) {
    let data = input_reader::read_all(input);
}

pub fn day12() {
    let input = String::from("day12");

    println!("***Day Ten***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    part1(&input);
    // println!("\t**Part Two**");
    // println!("\t\tWinning elfs score: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
