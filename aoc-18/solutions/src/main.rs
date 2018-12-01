mod day1;
mod input_reader;

fn main() {
    day1();
}

fn day1() {
    day1_part1();
    day1_part2();
}

fn day1_part1() {
    println!("Day 1, Part1");
    let mut freq = day1::Freq::new();

    for l in input_reader::read_all_lines("day1_part1") {
        freq.calibrate_str(&l);
    }
    println!("Current freq: {}", freq.get_current());
}

fn day1_part2() {
    use std::collections::HashSet;

    println!("Day 1, Part2");
    let mut freq = day1::Freq::new();
    let v = input_reader::read_all_lines("day1_part1");
    let mut duplicate_detection = HashSet::new();
    duplicate_detection.insert(freq.get_current());

    'outer: loop {
        for l in v.iter() {
            freq.calibrate_str(&l);
            if duplicate_detection.contains(&freq.get_current()) {
                break 'outer;
            } else {
                duplicate_detection.insert(freq.get_current());
            }
        }
    }

    println!("First duplicate freq: {}", freq.get_current());
}
