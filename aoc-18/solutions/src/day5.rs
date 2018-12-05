use crate::input_reader;
use std::collections::HashSet;

fn are_polar_opposite(lhs: char, rhs: char) -> bool {
    let eq_lhs = lhs.to_ascii_lowercase();
    let eq_rhs = rhs.to_ascii_lowercase();
    lhs != rhs && eq_lhs == eq_rhs
}

fn do_the_job(input: String) -> usize {
    let mut polymer = input;
    let mut reacted = true;

    while reacted {
        let mut reaction = String::new();
        let first: char = polymer.chars().take(1).next().unwrap();
        reacted = false;
        reaction.push(first);
        polymer
            .chars()
            .skip(1)
            .scan(first, |state, elem| {
                if are_polar_opposite(*state, elem) {
                    reaction.pop();
                    match reaction.pop() {
                        None => *state = ' ',
                        Some(s) => *state = s,
                    };

                    reaction.push(*state);
                    reacted = true;
                    Some(*state)
                } else {
                    reaction.push(elem);
                    *state = elem;
                    Some(*state)
                }
            })
            .for_each(|_x| {});
        polymer = reaction;
    }
    polymer.replace(" ", "").chars().count()
}

fn part1(input: &str) -> usize {
    do_the_job(input_reader::read_all(input))
}

fn part2(input: &str) -> usize {
    let data = input_reader::read_all(input);
    let symbols = get_symbols(&data);
    symbols.iter().map(|sym| do_the_job(data.replace(&sym.to_string(), "").replace(sym.to_ascii_uppercase(), ""))).min().unwrap()
}

fn get_symbols(input: &str) -> HashSet<char> {
    input.chars().map(|c| c.to_ascii_lowercase()).collect()
}

pub fn day5() {
    let input = String::from("day5");

    println!("***Day Five***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tRemaining units: {}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tShortest reduction: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
