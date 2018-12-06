use crate::input_reader;

fn do_the_job(input_location: &str) -> u32 {
    let data = input_reader::read_all_lines(input_location);

    0
}

pub fn day6() {
    let input = String::from("day6");
    println!("***Day Three***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOverlaping inches: {}", do_the_job(&input));
    //println!("\t**Part Two**");
    //println!("\t\tClaim ID: {}", not_overlaping);
}

#[cfg(test)]
mod tests {}
