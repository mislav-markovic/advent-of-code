use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "./input.txt";

struct Memory {
    banks: Vec<u32>,
    confgis: HashSet<String>
}

impl Memory {
    fn new() -> Memory {
        Memory{banks: Vec::new(), confgis: HashSet::new()}
    }

    fn add(&mut self, bank: u32){
        self.banks.push(bank);
    }

    fn reallocate(&mut self) {
        let (mut max, pos) = self.max_pos();
        self.banks[pos] = 0;

        let mut pos = pos+1;

        while max > 0 {
            if pos >= self.banks.len(){
                pos = 0;
            }
            self.banks[pos] += 1;
            pos += 1;
            max -= 1;
        }
    }

    fn max_pos(&self) -> (u32, usize) {
        let max = self.banks.iter().max().unwrap();
        let pos = self.banks.iter().position(|&a| a == *max).unwrap();
        (*max, pos)
    }

    fn make_config(&self) -> String {
        let mut s = String::new();
        for num in self.banks.iter() {
            s.push_str(&num.to_string());
            s.push(' ');
        }
        s
    }
}

fn main() {
    let input = read_input();
    let mut memory = Memory::new();

    for n in input {
        memory.add(n);
    }

    let mut counter = 0;
    let mut loop_counter = 1;

    loop {
        let conf = memory.make_config();
        if memory.confgis.contains(&conf) {
            loop {
                memory.reallocate();
                let conf2 = memory.make_config();
                if conf2 == conf {
                    break;
                }
                loop_counter += 1;
            }
            break;
        }
        memory.confgis.insert(conf);
        counter += 1;
        memory.reallocate();
    }
    println!("Redistribution cycle: {}", counter);
    println!("Loop size: {}", loop_counter)
}

fn read_input() -> Vec<u32> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<u32> = Vec::new();
    let reader = BufReader::new(f);

    for line in reader.lines(){
        line.unwrap().trim().split_whitespace().for_each(|a| result.push(a.parse().unwrap()));
    }

    result
}
