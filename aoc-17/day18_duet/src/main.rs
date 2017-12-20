use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

const FILE_PATH: &str = "./input.txt";

enum Instr {
    Snd(String),
    Set(String, String),
    Add(String, String),
    Mul(String, String),
    Mod(String, String),
    Rcv(String),
    Jgz(String, String)
}

enum After {
    Jump(i64),
    Signal(i64),
    Rcv(),
}

fn main() {
    let input = read_input();
    let mut regs = HashMap::<String, i64>::new();
    let mut last_sound = 0;
    let mut i = 0;

    while i < input.len() {
        let mut skip = 1;
        if let Some(opt) = do_instr(input.get(i).unwrap(), &mut regs) {
            match opt {
                After::Jump(s) => skip = s,
                After::Signal(s) => last_sound = s,
                After::Rcv() => {
                    println!("Recovered signal: {}", last_sound);
                    break;
                }
            }
        }
        let new = i as i64 + skip;
        if new < 0 {
            break;
        }
        i = new as usize;
    }
        
}

fn do_instr(instr: &Instr, map: &mut HashMap<String, i64>) -> Option<After> {
    let mut result = None::<After>;
    match *instr {
        Instr::Snd(ref val) => {
            result = Some(After::Signal(num_or_reg(&val, map)));
        },
        Instr::Add(ref r_id, ref val) => {
            let val = num_or_reg(&val, map);
            let r1 = map.entry(r_id.clone()).or_insert(0);
            *r1 += val;
        },
        Instr::Set(ref r_id, ref val) => {
            let val = num_or_reg(&val, map);
            let r1 = map.entry(r_id.clone()).or_insert(0);
            *r1 = val;
        },
        Instr::Mul(ref r_id, ref val) => {
            let val = num_or_reg(&val, map);
            let r1 = map.entry(r_id.clone()).or_insert(0);
            *r1 *= val;
        },
        Instr::Mod(ref r_id, ref val) => {
            let val = num_or_reg(&val, map);
            let r1 = map.entry(r_id.clone()).or_insert(0);
            *r1 %= val;
        },
        Instr::Rcv(ref val) => {
            if num_or_reg(&val, map) != 0 {
                result = Some(After::Rcv());
            }
        },
        Instr::Jgz(ref r1, ref r2) => {
            let r1 = num_or_reg(&r1, map);
            let r2 = num_or_reg(&r2, map);
            if r1 > 0 {
                result = Some(After::Jump(r2));
            }
        },
    };
    result
}

fn num_or_reg(val: &String, map: &mut HashMap<String, i64>) -> i64 {
    let v = val.parse::<i64>();
    if let Ok(n) = v {
        n
    } else {
        *map.entry(val.clone()).or_insert(0)
    }
}

fn read_input() -> Vec<Instr> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<Instr> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Instr {
    let temp: Vec<&str> = line.split_whitespace().collect();

    match temp[0].trim() {
        "snd" => Instr::Snd(temp[1].trim().to_string()),
        "set" => Instr::Set(temp[1].trim().to_string(), temp[2].trim().to_string()),
        "add" => Instr::Add(temp[1].trim().to_string(), temp[2].trim().to_string()),
        "mul" => Instr::Mul(temp[1].trim().to_string(), temp[2].trim().to_string()),
        "mod" => Instr::Mod(temp[1].trim().to_string(), temp[2].trim().to_string()),
        "rcv" => Instr::Rcv(temp[1].trim().to_string()),
        "jgz" => Instr::Jgz(temp[1].trim().to_string(), temp[2].trim().to_string()),
        _ => Instr::Rcv("0".to_string()),
    }    
}