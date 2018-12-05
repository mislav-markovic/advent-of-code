use crate::input_reader;

fn are_polar_opposite(lhs: char, rhs: char) -> bool {
    let eq_lhs = lhs.to_ascii_lowercase();
    let eq_rhs = rhs.to_ascii_lowercase();
    lhs != rhs && eq_lhs == eq_rhs
}

pub fn part1(input: &str) -> usize {
    let mut polymer = input_reader::read_all(input);
    let mut reacted = true;

    while reacted {
        let mut reaction = String::new();
        let first: char = polymer.chars().take(1).next().unwrap();
        reacted = false;
        reaction.push(first);
        polymer.chars().skip(1).scan(first, |state, elem| {
            if are_polar_opposite(*state, elem) {
                //println!("Oppsites: {}x{}", *state, elem);
                //println!("{}", reaction);
                reaction.pop();
                match reaction.pop(){
                    None => *state = 0 as char,
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
        }).for_each(|_x| {;});
        polymer = reaction;
    }
    polymer.len()
}

pub fn day5() {
    let input = String::from("day5");

    println!("***Day Five***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tRemaining units: {}", part1(&input));
    //println!("\t**Part Two**");
    //println!("\t\tClaim ID: {}", not_overlaping);
}

#[cfg(test)]
mod tests {}
