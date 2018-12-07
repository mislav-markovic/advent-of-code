use crate::input_reader;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Step {
    id: char,
    prerequisites: HashSet<char>,
    time: u32
}

impl Step {
    fn new(id: char) -> Step {
        let prerequisites = HashSet::new();
        let time = u32::from(60 + (id as u8 - (b'A')));
        Step {id, prerequisites, time}
    }

    fn parse_line(input: &str) -> (Step, char) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Step (.) .+ step (.) .+$").unwrap();
        }

        let ids = RE.captures(input).unwrap();
        let prereq = ids.get(1).map_or(' ', |m| m.as_str().chars().nth(0).unwrap());
        let id = ids.get(2).map_or(' ', |m| m.as_str().chars().nth(0).unwrap());
        let mut step = Step::new(id);
        step.add_prereq(prereq);
        (step, prereq)
    }

    fn add_prereq(&mut self, prereq: char) {
        self.prerequisites.insert(prereq);
    }

    fn add_multi(&mut self, prereqs: &[char]){
        for c in prereqs.iter() {
            self.add_prereq(*c);
        }
    }

    fn prereq_finished(&mut self, prereq: char) {
        self.prerequisites.remove(&prereq);
    }

    fn is_ready(&self) -> bool {
        self.prerequisites.is_empty()
    }
}

struct Workers<'a> {
    steps: Vec<&'a mut Step>,
    number_of_workers: usize
}

impl<'a> Workers<'a> {
    fn new(number_of_workers: usize) -> Workers<'a> {
        let steps = Vec::with_capacity(number_of_workers);
        Workers {steps, number_of_workers}
    }

    fn work(&mut self) {
        self.steps.iter_mut().for_each(|s| s.time -= 1);
    }
}

fn part1(input: &str) -> String {
    let data = input_reader::read_all_lines(input);
    let mut steps = HashMap::new();

    for l in data {
        let (s, prereq) = Step::parse_line(&l);
        steps.entry(s.id).or_insert(s).add_prereq(prereq);        
        steps.entry(prereq).or_insert_with(|| Step::new(prereq));
    }
    
    let mut ids = steps.keys().cloned().collect::<Vec<_>>();
    let mut result = String::new();
    ids.sort();

    while !ids.is_empty() {
        let first: char = *ids.iter().skip_while(|c| !&steps[c].is_ready()).next().unwrap();
        ids.remove_item(&first);
        steps.remove(&first);
        steps.iter_mut().for_each(|(_, v)| v.prereq_finished(first));
        result.push(first);
    }


    result
}

fn part2(input: &str) -> String {
    let data = input_reader::read_all_lines(input);
    let mut steps = HashMap::new();
    let mut workers: Vec<&mut Step> = Vec::new();

    for l in data {
        let (s, prereq) = Step::parse_line(&l);
        steps.entry(s.id).or_insert(s).add_prereq(prereq);        
        steps.entry(prereq).or_insert_with(|| Step::new(prereq));
    }
    
    let mut ids = steps.keys().cloned().collect::<Vec<_>>();
    let mut result = String::new();
    ids.sort();

    while !ids.is_empty() {
        let first: char = *ids.iter().skip_while(|c| !&steps[c].is_ready()).next().unwrap();
        ids.remove_item(&first);
        steps.remove(&first);
        steps.iter_mut().for_each(|(_, v)| v.prereq_finished(first));
        result.push(first);
    }

    result
}

pub fn day7() {
    let input = String::from("day7");

    println!("***Day Seven***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOrder of steps: {}", part1(&input));
    println!("\t**Part Two**");
    //println!("\t\tShortest reduction: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::Step;
    #[test]
    fn parste_step_test(){
        let text = "Step C must be finished before step A can begin.";
        let (step, _) = Step::parse_line(text);
        assert_eq!('A', step.id);
        assert_eq!(1, step.prerequisites.len());
        assert_eq!('C', *step.prerequisites.get(& 'A').unwrap());
    }
}
