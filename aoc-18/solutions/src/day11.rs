use crate::input_reader;


fn part1(input: &str) {
    let data = input_reader::read_all_lines(input);
}

fn part2(input: &str) {
    let data = input_reader::read_all(input);
}

pub fn day11() {
    let input = 8444;

    println!("***Day Eleven***");
    println!("\tInput is {}", input);
    println!("\t**Part One**");
    part1(&input);
    // println!("\t**Part Two**");
    // println!("\t\tWinning elfs score: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
