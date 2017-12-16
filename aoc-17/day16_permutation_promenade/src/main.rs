use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "./input.txt";

enum Op {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((char, char))
}

fn main() {
    let mut vec = Vec::new();
    let mut starting = Vec::new();
    let input = read_input();

    for c in "abcdefghijklmnop".chars(){
        vec.push(c);
        starting.push(c);
    }

    let mut counter = 0;
    loop {
        for command in input.iter() {
            match *command {
                Op::Spin(n) => spin(&mut vec, n),
                Op::Exchange(n) => exchange(& mut vec, n.0, n.1),
                Op::Partner(n) => partner(&mut vec, n.0, n.1),
            }
        }
        counter += 1;
        if vec == starting {
            break;
        }
    }

    let iter_num = 1_000_000_000 % counter;
    for _ in 0..iter_num {
        for command in input.iter() {
            match *command {
                Op::Spin(n) => spin(&mut starting, n),
                Op::Exchange(n) => exchange(& mut starting, n.0, n.1),
                Op::Partner(n) => partner(&mut starting, n.0, n.1),
            }
        }
    }


    starting.iter().for_each(|a| print!("{}", a));
    println!("");     
}

fn spin(vec: &mut Vec<char>, n: usize){
    let len = vec.len() - n;
    let temp = vec.split_off(len);

    temp.iter().rev().for_each(|a| vec.insert(0, *a));
}

fn exchange(vec: &mut Vec<char>, f: usize, s: usize) {
    vec.swap(f, s);
}

fn partner(vec: &mut Vec<char>, f: char, s: char){
    let first = vec.iter().enumerate().find(|&a| *a.1 == f).unwrap().0;
    let seconnd = vec.iter().enumerate().find(|&a| *a.1 == s).unwrap().0;
    exchange(vec, first, seconnd);
}

fn read_input() -> Vec<Op> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<Op> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.extend(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Vec<Op> {
    let temp = line.split(",");
    let mut vec = Vec::new();

    for st in temp {
        if st.starts_with("s") {
            vec.push(Op::Spin(st[1..].parse().unwrap()));
        } else if st.starts_with("x") {
            let temp = String::from(st);
            let split: Vec<&str> = temp.split("/").collect();
            let first: usize = split.get(0).unwrap()[1..].parse().unwrap();
            let second: usize = split.get(1).unwrap().parse().unwrap();

            vec.push(Op::Exchange((first, second)));
        } else if st.starts_with("p") {
            let temp = String::from(st);
            let split: Vec<&str> = temp.split("/").collect();
            let first = split.get(0).unwrap()[1..].trim();
            let second = split.get(1).unwrap().trim();

            vec.push(Op::Partner((first.chars().nth(0).unwrap(), second.chars().nth(0).unwrap())));
        }
    }
    vec
}
