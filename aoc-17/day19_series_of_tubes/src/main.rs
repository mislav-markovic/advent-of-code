use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const FILE_PATH: &str = "./input.txt";

enum State {
    Line(),
    Cross(),
    Minus(),
    Letter(String),
    Void(),
}

enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

fn main() {

}


fn read_input() -> Vec<Vec<State>> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<Vec<State>> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Vec<State> {
    let mut vec = Vec::new();

    line.trim().chars().for_each(|a| vec.push(get_state(a)));
    vec
}

fn get_state(c: &char) -> State {
    match *c {
        '|' => State::Line(),
        '-' => State::Minus(),
        '+' => State::Cross(),
        ' ' => State::Void(),
        _ => State::Letter(c.to_string()),
    }
}