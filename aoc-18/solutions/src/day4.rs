use crate::input_reader;
use std::collections::HashMap;

type guard_id_t = u32;
type minutes_t = HashMap<u32, bool>;
type shifts_t = Vec<Shift>;

struct Date {
    year: u32,
    month: u32,
    day: u32
}

struct Guard{
    id: guard_id_t,
    shifts: shifts_t
}

struct Shift{
    date: Date,
    minutes_asleep: minutes_t
}

fn do_the_job(input_location: &str) {
    let mut data = input_reader::read_all_lines(input_location);
    data.sort();


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