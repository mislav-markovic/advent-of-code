use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const FILE_PATH: &str = "./input.txt";

#[derive(Clone, Debug)]
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
    Rcv(String),
}

struct Program {
    instructions: Vec<Instr>,
    regs: HashMap<String, i64>,
    id: i64,
    send: Sender<i64>,
    rec: Receiver<i64>,    
}

impl Program {
    fn new(id: i64, vec: &Vec<Instr>, send: Sender<i64>, rec: Receiver<i64>) -> Program {
        let mut regs = HashMap::<String, i64>::new();
        regs.insert("p".to_string(), id);
        Program {regs: regs, id: id, instructions: vec.clone(), send: send, rec: rec}
    }

    fn run_all(&mut self) {
        let mut i = 0;
        let mut send_cnt = 0;
        println!("Running program: {}", self.id);
        while i < self.instructions.len() {
            let mut skip = 1;
            if let Some(opt) = Program::do_instr(self.instructions.get(i).unwrap(), &mut self.regs) {
                match opt {
                    After::Jump(s) => skip = s,
                    After::Signal(s) => {
                        if let Err(err) = self.send.send(s){
                            println!("{}", err);
                        }
                        send_cnt += 1;
                    },
                    After::Rcv(reg) => {
                        let result = self.rec.recv_timeout(Duration::new(10, 0));
                        if let Err(er) = result {
                            println!("Program {} sent {} times", self.id, send_cnt);
                            break;
                        } else {
                            let r = self.regs.entry(reg).or_insert(0);
                            *r = result.unwrap();
                        }
                    }
                }
            }
            let new = i as i64 + skip;
            if new < 0 {
                println!("Program {} sent {}", self.id, send_cnt);
                break;
            }
            i = new as usize;
        }
        println!("Program {} sent {}", self.id, send_cnt);
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
                result = Some(After::Rcv(val.clone()));
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
}

fn main() {
    let input = read_input();
    let (s1, r1) = channel::<i64>();
    let (s2, r2) = channel::<i64>();

    let mut p1 = Program::new(0, &input, s1, r2);
    let mut p2 = Program::new(1, &input, s2, r1);

    let handle1 = thread::spawn(move || {
        p1.run_all();
    });
    p2.run_all();

    handle1.join();
        
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
                //result = Some(After::Rcv());
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