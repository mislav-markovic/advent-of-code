use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "D:\\Faks\\git\\advent-of-code-17\\high_entropy_passphrase\\input.txt";

fn main() {
    let lines = read_input();

    println!("Number of valid passphrases: {}",     lines.iter().filter(|a| is_valid(a)).count());
}

fn read_input() -> Vec<String> {
    let f = File::open(FILE_PATH).expect("file not found");
    let mut result: Vec<String> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(l.unwrap().trim().to_string()));
    result
}

fn is_valid(pswd: &str) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    
    for word in pswd.split_whitespace() {
        if set.contains(word) {
            return false;
        } else {
            set.insert(word);
        }
    }
    true
}