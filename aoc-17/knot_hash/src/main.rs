use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

const FILE_PATH: &str = "./input.txt";

struct Hasher {
    list: HashMap<usize, usize>,
    pos: usize,
    skip: usize
}

impl Hasher {
    fn new() -> Hasher {
        let mut map = HashMap::new();
        for i in 0..256 {
            map.insert(i, i);
        }
        Hasher {list: map, pos: 0, skip: 0}
    }

    fn hash(&mut self, len: usize) {
        let (start, end) = (self.pos, self.pos+len % 256);
        let mut vec = Vec::<usize>::new();

        for i in 0..len {
            let index: usize = (self.pos+i) % 256;
            vec.push(self.list.get(&index).unwrap().clone());
        }
        let mut slice = vec.as_mut_slice();
        slice.reverse();

        for (i, elem) in slice.iter().enumerate() {
            self.list.insert(((self.pos+i) % 256), elem.clone());
        }
        self.pos = (self.pos + len + self.skip) % 256;
        self.skip += 1;        
    }

    fn mul_two(&self) -> usize {
        let first = self.list.get(&0).unwrap();
        let second = self.list.get(&1).unwrap();
        first*second
    }
}

fn main() {
    let input = read_input();
    let mut hasher = Hasher::new();

    for len in input {
        hasher.hash(len);
    }
    println!("{}", hasher.mul_two());

}

fn read_input() -> Vec<usize> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<usize> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.extend(parse(l.unwrap().trim()).as_slice()));
    result
}

fn parse<'a>(line: &'a str) -> Vec<usize> {
    let temp = line.split(",");
    let mut vec = Vec::new();

    for num in temp {
        vec.push(num.trim().parse().unwrap());
    }
    vec
}