use crate::input_reader;
use std::collections::HashSet;

type guard_id_t = u32;
type minutes_t = HashSet<u32>;
type shifts_t = Vec<Shift>;

struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn new(year: u32, month: u32, day: u32) -> Date {
        Date { year, month, day }
    }
}

struct Guard {
    id: guard_id_t,
    shifts: shifts_t,
}

impl Guard {
    fn new(id: guard_id_t) -> Guard {
        Guard {
            id,
            shifts: shifts_t::new(),
        }
    }

    fn add_shift(&mut self, shift: Shift) {
        self.shifts.push(shift);
    }

    fn total_minutes_slept(&self) -> u32 {
        self.shifts.iter().map(|s| s.minutes_slept()).sum()
    }

    fn most_slept_minute(&self) -> u32 {
        use std::collections::HashMap;
        let mut map: HashMap<u32, u32> = HashMap::new(); // (minute, times spent asleep)
        self.shifts
            .iter()
            .flat_map(|s| s.minutes_asleep.iter())
            .for_each(|m| *map.entry(*m).or_insert(0) += 1);
        *map.iter().max_by_key(|(_k, v)| *v).unwrap().0
    }
}

struct Shift {
    date: Date,
    minutes_asleep: minutes_t,
}

impl Shift {
    fn new(date: Date) -> Shift {
        Shift {
            date,
            minutes_asleep: minutes_t::new(),
        }
    }

    fn sleep(&mut self, fall_asleep: u32, wake_up: u32) {
        for m in fall_asleep..wake_up {
            self.minutes_asleep.insert(m);
        }
    }
    fn minutes_slept(&self) -> u32 {
        self.minutes_asleep.len() as u32
    }
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
