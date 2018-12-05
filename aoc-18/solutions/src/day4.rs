use crate::input_reader;
use std::collections::HashMap;
use std::collections::HashSet;

type guard_id_t = u32;
type minutes_t = HashSet<u32>;
type shifts_t = Vec<Shift>;

#[derive(PartialEq, Clone)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn as_empty() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
        }
    }
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
    fn as_empty() -> Guard {
        Guard {
            id: 0,
            shifts: shifts_t::new(),
        }
    }
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

    fn most_slept_minute(&self) -> (u32, u32) {
        // (minute, number of times spent asleep)
        let mut map: HashMap<u32, u32> = HashMap::new(); // (minute, times spent asleep)
        self.shifts
            .iter()
            .flat_map(|s| s.minutes_asleep.iter())
            .for_each(|m| *map.entry(*m).or_insert(0) += 1);

        match map.into_iter().max_by_key(|(_k, v)| *v) {
            None => (self.id, 0),
            Some(v) => v
        }
    }
}

struct Shift {
    date: Date,
    minutes_asleep: minutes_t,
}

impl Shift {
    fn as_empty() -> Shift {
        Shift {
            date: Date::as_empty(),
            minutes_asleep: minutes_t::new(),
        }
    }
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

fn do_the_job(input_location: &str) -> HashMap<u32, Guard> {
    let mut data = input_reader::read_all_lines(input_location);
    data.sort();
    let mut guards: HashMap<u32, Guard> = HashMap::new();

    let mut curr_guard: &mut Guard = &mut Guard::as_empty();
    let mut fell_asleep = 0u32;
    let mut curr_shift = Shift::as_empty();
    let mut is_first = true;

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

        if is_first {
            is_first = false;
            curr_shift = Shift::new(date);
            let id = split[3][1..].parse::<u32>().unwrap();
            curr_guard = guards.entry(id).or_insert(Guard::new(id));
        } else {
            match split[2] {
                "Guard" => {
                    curr_guard.add_shift(curr_shift);
                    curr_shift = Shift::new(date);

                    let id = split[3][1..].parse::<u32>().unwrap();
                    curr_guard = guards.entry(id).or_insert(Guard::new(id));
                }
                "falls" => {
                    fell_asleep = time;
                }
                "wakes" => {
                    curr_shift.sleep(fell_asleep, time);
                }
                _ => {}
            }
        }
    }
    curr_guard.add_shift(curr_shift);
    guards
}

fn part1(guards: &HashMap<u32, Guard>) -> u32 {
    let g_id = guards
        .iter()
        .map(|(k, v)| (k, v.total_minutes_slept()))
        .max_by_key(|(_k, v)| *v)
        .unwrap()
        .0;

    let guard = guards.get(g_id).unwrap();

    guard.id * guard.most_slept_minute().0
}

fn part2(guards: &HashMap<u32, Guard>) -> u32 {
    let g_id = guards
        .iter()
        .map(|(k, v)| (k, v.most_slept_minute().1))
        .max_by_key(|(_k, v)| *v)
        .unwrap()
        .0;
        
    let guard = guards.get(g_id).unwrap();

    guard.id * guard.most_slept_minute().0
}

pub fn day4() {
    let input = String::from("day4");
    let guards = do_the_job(&input);

    println!("***Day Four***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tGuard x Minutes: {}", part1(&guards));
    println!("\t**Part Two**");
    println!("\t\tGuard x Minutes: {}", part2(&guards));
}

#[cfg(test)]
mod tests {}
