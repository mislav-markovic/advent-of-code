use crate::input_reader;

type point_t = (isize, isize);

fn do_the_job(input_location: &str) -> u32 {
    let data = input_reader::read_all_lines(input_location);

    0
}

fn manhattan(p1: point_t, p2: point_t) -> isize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x1 - x2).abs() + (y1 - y2).abs()
}

fn point(input: &str) -> point_t {
    let arr = input.split(",").collect::<Vec<_>>();
    (arr[0].parse().unwrap(), arr[1].parse().unwrap())
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
