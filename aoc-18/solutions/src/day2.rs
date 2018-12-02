use crate::input_reader;

use std::collections::HashMap;
type map_t = HashMap<char, u32>;
type pair_t = Option<(String, String)>;

pub struct Checksum {
    doubles: u32,
    triples: u32,
    processed_inputs: Vec<String>,
    closest_pair: pair_t,
}

impl Checksum {
    pub fn new() -> Checksum {
        Checksum {
            doubles: 0,
            triples: 0,
            processed_inputs: Vec::new(),
            closest_pair: None,
        }
    }

    pub fn add(&mut self, input: &str) {
        let mut map = map_t::new();

        input.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        if Checksum::contains_double(&map) {
            self.doubles += 1;
        }
        if Checksum::contains_triple(&map) {
            self.triples += 1;
        }

        if self.closest_pair == None {
            self.check_for_closest_pair(input);
        }

        self.processed_inputs.push(String::from(input));
    }

    pub fn get_checksum(&self) -> u32 {
        self.doubles * self.triples
    }

    pub fn get_common_chars(&self) -> String {
        let (l, r) = self.closest_pair.clone().unwrap();

        l.chars()
            .zip(r.chars())
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .collect()
    }

    fn check_for_closest_pair(&mut self, input: &str) {
        for l in self.processed_inputs.iter() {
            if Checksum::difference(&l, &input) <= 1 {
                self.closest_pair = Some((String::from(&l[..]), String::from(input)));
            }
        }
    }

    fn contains_double(map: &map_t) -> bool {
        map.iter().map(|(_k, v)| v).filter(|v| **v == 2u32).count() > 0
    }

    fn contains_triple(map: &map_t) -> bool {
        map.iter().map(|(_k, v)| v).filter(|v| **v == 3u32).count() > 0
    }

    fn difference(lhs: &str, rhs: &str) -> u32 {
        let mut diff = 0;

        lhs.chars().zip(rhs.chars()).for_each(|(l, r)| {
            if l != r {
                diff += 1;
            }
        });
        diff
    }
}

fn do_the_job(input_location: &str) -> Checksum {
    let data = input_reader::read_all_lines(input_location);
    let mut checksum = Checksum::new();

    for l in data {
        checksum.add(&l);
    }
    checksum
}

pub fn day2() {
    let input = String::from("day2");
    let result = do_the_job(&input);

    println!("***Day Two***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tChecksum: {}", result.get_checksum());
    println!("\t**Part Two**");
    println!("\t\tCommon letters: {}", result.get_common_chars());
}

#[cfg(test)]
mod tests {}
