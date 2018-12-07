use crate::input_reader;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Step {
    id: char,
    prerequisites: HashSet<char>,
    time: u32,
}

impl Step {
    fn new(id: char) -> Step {
        let prerequisites = HashSet::new();
        let time = u32::from(60 + (id as u8 - (b'A') + 1));
        Step {
            id,
            prerequisites,
            time,
        }
    }

    fn parse_line(input: &str) -> (Step, char) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Step (.) .+ step (.) .+$").unwrap();
        }

        let ids = RE.captures(input).unwrap();
        let prereq = ids
            .get(1)
            .map_or(' ', |m| m.as_str().chars().nth(0).unwrap());
        let id = ids
            .get(2)
            .map_or(' ', |m| m.as_str().chars().nth(0).unwrap());
        let mut step = Step::new(id);
        step.add_prereq(prereq);
        (step, prereq)
    }

    fn add_prereq(&mut self, prereq: char) {
        self.prerequisites.insert(prereq);
    }

    fn add_multi(&mut self, prereqs: &[char]) {
        for c in prereqs.iter() {
            self.add_prereq(*c);
        }
    }

    fn prereq_finished(&mut self, prereq: char) {
        self.prerequisites.remove(&prereq);
    }

    fn prereq_finished_multi(&mut self, prereq: &[char]) {
        prereq.iter().for_each(|c| self.prereq_finished(*c));
    }

    fn is_ready(&self) -> bool {
        self.prerequisites.is_empty()
    }
}

struct Workers {
    steps: Vec<Step>,
    number_of_workers: usize,
    time: u32,
}

impl Workers {
    fn new(number_of_workers: usize) -> Workers {
        let steps = Vec::with_capacity(number_of_workers);
        let time = 0;
        Workers {
            steps,
            number_of_workers,
            time,
        }
    }

    fn work(&mut self) -> Vec<char> {
        self.steps.iter_mut().for_each(|s| s.time -= 1);
        self.time += 1;
        self.remove_finished()
    }

    fn remove_finished(&mut self) -> Vec<char> {
        let return_ids = self
            .steps
            .iter()
            .filter_map(|s| if s.time == 0 { Some(s.id) } else { None })
            .collect::<Vec<_>>();
        self.steps.retain(|s| s.time > 0);

        return_ids
    }

    fn add_step(&mut self, step: Step) -> bool {
        if self.can_add() {
            self.steps.push(step);
            true
        } else {
            false
        }
    }

    fn can_add(&self) -> bool {
        self.steps.len() < self.number_of_workers
    }

    fn free_workers(&self) -> usize {
        self.number_of_workers - self.steps.len()
    }

    fn total_time(&self) -> u32 {
        self.time
    }

    fn is_working(&self) -> bool {
        !self.steps.is_empty()
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
        let first: char = *ids
            .iter()
            .skip_while(|c| !&steps[c].is_ready())
            .next()
            .unwrap();
        ids.remove_item(&first);
        steps.remove(&first);
        steps.iter_mut().for_each(|(_, v)| v.prereq_finished(first));
        result.push(first);
    }

    result
}

fn part2(input: &str) -> u32 {
    let data = input_reader::read_all_lines(input);
    let mut steps = HashMap::new();
    let mut workers = Workers::new(5);

    for l in data {
        let (s, prereq) = Step::parse_line(&l);
        steps.entry(s.id).or_insert(s).add_prereq(prereq);
        steps.entry(prereq).or_insert_with(|| Step::new(prereq));
    }

    let mut ids = steps.keys().cloned().collect::<Vec<_>>();
    ids.sort();

    while !ids.is_empty() {
        let ready: Vec<char> = ids
            .iter()
            .filter(|c| steps[c].is_ready())
            .take(workers.free_workers())
            .cloned()
            .collect::<Vec<_>>();

        ready.into_iter().for_each(|c| {
            if workers.add_step(steps[&c].clone()) {
                ids.remove_item(&c);
            }
        });
        let finished_work = workers.work();
        steps
            .iter_mut()
            .for_each(|(_, v)| v.prereq_finished_multi(&finished_work));
        finished_work.iter().for_each(|w| {
            steps.remove(w);
        });
    }

    while workers.is_working() {
        workers.work();
    }

    workers.total_time()
}

pub fn day7() {
    let input = String::from("day7");

    println!("***Day Seven***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOrder of steps: {}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tTime needed with 5 workers: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::Step;
    #[test]
    fn parste_step_test() {
        let text = "Step C must be finished before step A can begin.";
        let (step, _) = Step::parse_line(text);
        assert_eq!('A', step.id);
        assert_eq!(1, step.prerequisites.len());
        assert_eq!('C', *step.prerequisites.get(&'A').unwrap());
    }
}
