use crate::input_reader;
use std::collections::HashMap;
use std::collections::HashSet;

type guard_id_t = u32;
type minutes_t = HashSet<u32>;
type shifts_t = Vec<Shift>;

#[derive(PartialEq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn new(year: u32, month: u32, day: u32) -> Date {
        Date { year, month, day }
    }

    fn from_str(date: &str) -> Date {
        let arr = date.split('-').collect::<Vec<_>>();
        Date {
            year: arr[0].parse().unwrap(),
            month: arr[1].parse().unwrap(),
            day: arr[2].parse().unwrap(),
        }
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
    use std::mem;
    let mut data = input_reader::read_all_lines(input_location);
    data.sort();
    let mut guards: HashMap<u32, Guard> = HashMap::new();

    let mut curr_guard: Option<&Guard> = None;
    let mut last_date: Option<Date> = None;
    let mut fell_asleep = 0u32;
    let mut curr_shift: Option<Shift> = None;

    for l in data.iter() {
        let split = l.split(' ').collect::<Vec<_>>();
        let date = Date::from_str(&split[0][1..]);
        let time = split[1][0..split[1].len() - 1]
            .split(':')
            .skip(1)
            .take(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap(); //take just minutes

        match &last_date {
            None => last_date = Some(date),
            Some(ref d) => {
                if date != *d {
                    curr_guard.as_mut().and_then(|g| {
                        g.add_shift(mem::replace(&mut curr_shift, None).unwrap());
                        None<Shift>
                    });
                }
            }
        };

        match curr_shift {
            None => {
                curr_shift = Some(Shift::new(date));
            }
            Some(ref s) => {}
        }

        match split[2] {
            "Guard" => {
                let id = split[3][0..].parse().unwrap();
                curr_guard = Some(guards.entry(id).or_insert(Guard::new(id)));
            }
            "falls" => {
                fell_asleep = time;
            }
            "wakes" => {
                curr_shift.as_mut().and_then(|s| {
                    s.sleep(fell_asleep, time);
                    Some(s)
                });
            }
            _ => {}
        }
    }
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
