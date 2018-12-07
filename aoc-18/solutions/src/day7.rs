use crate::input_reader;

fn part1(input: &str) -> usize {
    let data = input_reader::read_all(input);
    0
}

fn part2(input: &str) -> usize {
    let data = input_reader::read_all(input);
    0
}

pub fn day7() {
    let input = String::from("day5");

    println!("***Day Seven***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tRemaining units: {}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tShortest reduction: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
