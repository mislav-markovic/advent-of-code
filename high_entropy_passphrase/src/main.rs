use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashSet;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let lines = read_input();

    println!("Number of valid passphrases: {}",     lines.iter().filter(|a| is_valid(a)).count());
}

fn read_input() -> Vec<String> {
    let f = File::open(fs::canonicalize(FILE_PATH).unwrap()).expect("file not found");
    let mut result: Vec<String> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(l.unwrap().trim().to_string()));
    result
}

fn is_valid(pswd: &str) -> bool {
    let mut set = HashSet::new();
    
    for word in pswd.split_whitespace() {
        if set.contains(word) {
            return false;
        } else {
            set.insert(word);
        }
    }
    !is_anagram(&set) //remove this line to get part 1 solution
}

fn is_anagram(set: &HashSet<&str>) -> bool {
    for word in set {
        let temp = make_vec(&word[..]);
        for candidate in get_same_lenght_words(word, set){
            let len = make_vec(&candidate[..]).iter().filter(|c| temp.contains(c)).collect::<Vec<&char>>().len();
            if temp.len() == len {
                return true
            }
        }
    }
    false
}

fn make_vec(word: &str) -> Vec<char> {
    word.chars().collect()
}

fn get_same_lenght_words<'a>(s: &'a str, set: &HashSet<&'a str>) -> Vec<&'a str> {
    let mut vec = Vec::new();

    set.iter().filter(|a| a.len() == s.len() && **a != s).for_each(|a| vec.push(*a));
    vec
}