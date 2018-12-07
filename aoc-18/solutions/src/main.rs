#![feature(try_trait)]
#![feature(vec_remove_item)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod input_reader;

use elapsed::measure_time;
use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() -> Result<(), std::option::NoneError> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let mut v = HASHMAP
        .iter()
        .filter(|(&k, _)| if args.len() == 1 { true } else { k == args[2] })
        .map(|(k, v)| (&k[..], v))
        .collect::<Vec<(&str, &fn())>>();
    v.sort_by_key(|k| k.0);

    let (elapsed, _) = measure_time(|| v.iter().for_each(|t| t.1()));
    println!("Run Time = {}", elapsed);

    Ok(())
}

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, fn()> = {
        let mut m = HashMap::new();
        m.insert("day1", day1::day1 as fn());
        m.insert("day2", day2::day2 as fn());
        m.insert("day3", day3::day3 as fn());
        m.insert("day4", day4::day4 as fn());
        m.insert("day5", day5::day5 as fn());
        m.insert("day6", day6::day6 as fn());
        m.insert("day7", day7::day7 as fn());
        m.insert("day8", day8::day8 as fn());
        m
    };
}
