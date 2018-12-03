use crate::input_reader;

fn do_the_job(input_location: &str) {
    let data = input_reader::read_all_lines(input_location);
}

pub fn day4() {
    let input = String::from("day4");

    println!("***Day Four***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    //println!("\t\tOverlaping inches: {}", overlap);
    //println!("\t**Part Two**");
    //println!("\t\tClaim ID: {}", not_overlaping);
}

#[cfg(test)]
mod tests {}