use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

const FILE_PATH: &str = "./input.txt";

enum Condition {
    Equal((String, i32)),
    UnEq((String, i32)),
    Greater((String, i32)),
    Less((String, i32)),
    Grt_Eq((String, i32)),
    Less_Eq((String, i32))
}

enum Operation {
    Inc(i32),
    Dec(i32)
}

struct Engine {
    registers: HashMap<String, i32>,
    largest: i32
}

impl Engine {
    fn largest(&self) -> (String, i32){
        let vec: Vec<(String, i32)> = self.registers.clone().into_iter().collect();
        vec.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().clone()
    }

    fn new() -> Engine {
        Engine{registers: HashMap::new(), largest: std::i32::MIN}
    }

    fn run(&mut self, reg: String, op: Operation, cond: Condition) {
        if !self.registers.contains_key(&reg) {
            self.registers.insert(reg.clone(), 0);
        }
        if self.test_condition(cond) {
            let v = self.do_operation(reg, op);
            if v > self.largest {
                self.largest = v;
            }
        }
    }

    fn do_operation(&mut self, reg: String, op: Operation) -> i32 {
        match op {
            Operation::Inc(v) => {
                let mut t = self.registers.entry(reg).or_insert(0);
                *t += v;
                *t
            },
            Operation::Dec(v) => {
                let mut t = self.registers.entry(reg).or_insert(0);
                *t -= v;
                *t
            },
        }
    }

    fn test_condition(&mut self, cond: Condition) -> bool{
        match cond {
            Condition::Equal((reg, v)) => *self.registers.entry(reg).or_insert(0) == v,
            Condition::UnEq((reg, v)) => *self.registers.entry(reg).or_insert(0) != v,
            Condition::Greater((reg, v)) => *self.registers.entry(reg).or_insert(0) > v,
            Condition::Less((reg, v)) => *self.registers.entry(reg).or_insert(0) < v,
            Condition::Grt_Eq((reg, v)) => *self.registers.entry(reg).or_insert(0) >= v,
            Condition::Less_Eq((reg, v)) => *self.registers.entry(reg).or_insert(0) <= v,
        }
    }
}

fn main() {
    let input = read_input();
    let mut eng = Engine::new();

    for (reg, op, cond) in input {
        eng.run(reg, op, cond);
    }
    let largest = eng.largest();
    println!("Largest {} with value {}", largest.0, largest.1);
    println!("Largest value ever {}", eng.largest);
}

fn read_input() -> Vec<(String, Operation, Condition)> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<(String, Operation, Condition)> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim())));
    result
}

fn parse(line: &str) -> (String, Operation, Condition) {
    let temp: Vec<&str> = line.trim().split_whitespace().collect();
    let register = temp[0];
    let op = temp[1];
    let val = temp[2];
    let key_word = temp[3];
    let cond_reg = temp[4];
    let cmp_sign = temp[5];
    let cond_val = temp[6];

    let operation = make_operation(&op, &val);
    let condition = make_condition(&cond_reg, &cmp_sign, &cond_val);
    let register = register.to_string();
    (register, operation, condition)
}

fn make_condition(reg: &str, cmp: &str, val: &str) -> Condition {
    let val: i32 = val.parse().unwrap();
    let reg = reg.to_string();
    match cmp {
        "==" => Condition::Equal((reg, val)),
        "!=" => Condition::UnEq((reg, val)),
        "<" => Condition::Less((reg, val)),
        ">" => Condition::Greater((reg, val)),
        "<=" => Condition::Less_Eq((reg, val)),
        ">=" => Condition::Grt_Eq((reg, val)),
        _ => Condition::Equal((reg, 0))
    }
}

fn make_operation(op: &str, val: &str) -> Operation {
    match op {
        "inc" => Operation::Inc(val.parse().unwrap()),
        "dec" => Operation::Dec(val.parse().unwrap()),
        _ => Operation::Inc(0),
    }
}