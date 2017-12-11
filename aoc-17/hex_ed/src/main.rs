use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "./input.txt";

struct Hex {
    x: i32,
    y: i32,
    z: i32
}

impl Hex {
    fn new() -> Hex {
        Hex{x: 0, y: 0, z: 0}
    }

    fn move_hex(&mut self, direction: &str) {
        match direction {
            "n" => {self.y += 1; self.z -= 1},
            "s" => {self.z += 1; self.y -= 1},
            "nw" => {self.y += 1; self.x -= 1},
            "ne" => {self.x += 1; self.z -= 1},
            "sw" => {self.z += 1; self.x -= 1},
            "se" => {self.x += 1; self.y -= 1},
            _ => (),
        };
    }

    fn dist_to(&self, other: &Hex) -> i32{
        let vec = vec!(self.x - other.x, self.y - other.y, self.z - other.z);
        *vec.iter().max().unwrap()
    }
}

fn main() {
    let input = read_input();
    let mut start = Hex::new();
    let mut furthest = 0;
    let origin = Hex::new();

    for dir in input {
        start.move_hex(&dir[..]);
        let dist = start.dist_to(&origin);
        if dist > furthest {
            furthest = dist;
        }
    }

    println!("Shortest path to reach him: {}", start.dist_to(&origin));
    println!("Furthest he got: {}", furthest);

}


fn read_input() -> Vec<String> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<String> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.extend(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Vec<String> {
    let temp = line.split(",");
    let mut vec = Vec::new();

    for st in temp {
        vec.push(st.trim().to_string());
    }
    vec
}