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

    fn hash(&mut self, len: &usize) {
        let (start, end) = (self.pos, self.pos+len % 256);
        let mut vec = Vec::<usize>::new();

        for i in 0..*len {
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

    fn dense_hash(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut block = 0;

        while block < 16 {
            let mut val = *self.list.get(&(block*16)).unwrap() as u8;
            for i in 1..16 {
                val = val ^ (*self.list.get(&(block*16 + i)).unwrap() as u8);
            }
            vec.push(val);
            block += 1;
        }
        vec
    }
}

fn main() {
    let mut input = read_input();
    input.extend(&[17, 31, 73, 47, 23]);
    let mut hasher = Hasher::new();

    for _ in 0..64 {
        for len in &input {
            let l = *len as usize;
            hasher.hash(&l);
        }
    }

    for num in hasher.dense_hash() {
        print!("{:02x}", num);
    }
    println!("");

}

fn read_input() -> Vec<u8> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.extend(parse(l.unwrap().trim()).as_slice()));
    result
}

fn parse<'a>(line: &'a str) -> Vec<u8> {
    let mut vec = Vec::new();

    vec.extend(line.as_bytes());

    vec
}