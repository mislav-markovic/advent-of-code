#![feature(try_trait)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod input_reader;

use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() -> Result<(), std::option::NoneError> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut v = HASHMAP.iter().map(|(k, v)| (&k[..], v)).collect::<Vec<(&str, &fn())>>();
        v.sort_by_key(|k| k.0);
        v.iter().for_each(|t| t.1());
    } 
    else {
        let day = &args[1];
        //let part = &args[2];

        HASHMAP.get(day.as_str())?();
    }

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
        m
    };
}
