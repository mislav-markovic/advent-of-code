use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let mut instructions = read_input();
    println!("Instructions needed to escape: {}", cycle_instructions(&mut instructions));
}

fn cycle_instructions(instr: &mut Vec<isize>) -> u32 {
    let mut counter = 0u32;
    let mut pos = 0;

    loop {
        let offset = instr[pos].clone();
        instr[pos] += 1;
        counter += 1;
        match new_pos(pos, offset, instr.len()){
            Ok(p) => pos = p,
            Err(_) => break,
        }
    }
    counter
}

fn new_pos(pos: usize, offset: isize, len: usize) -> Result<usize, isize> {
    let new_pos: isize = pos as isize + offset;
    if new_pos < 0 || new_pos >= len as isize {
        return Err(new_pos);
    } else {
        return Ok(new_pos as usize);
    }
}

fn read_input() -> Vec<isize> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<isize> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(l.unwrap().trim().to_string().parse().unwrap()));
    result
}