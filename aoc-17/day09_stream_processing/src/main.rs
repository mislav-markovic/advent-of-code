use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let input = read_input();
    let mut escape = false;
    let mut garbage = false;
    let mut sum = 0;
    let mut garbage_sum = 0;
    let mut val = 1;

    for ch in input.chars(){
        let test = ch;
        if garbage {
            if escape {
                escape = false;
            } else if ch == '>' {
                garbage = false;
            } else if ch == '!' {
                escape = true;
            } else {
                garbage_sum += 1;
            }
        } else {
            match test {
                '<' => garbage = true,
                '{' => {
                    sum += val;
                    val += 1;
                },
                '}' => val -= 1,
                _ => (),
            };
        }
    }
    println!("Group sum {}", sum);
    println!("Garbage sum {}", garbage_sum);
}

fn read_input() -> String {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result = String::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push_str(l.unwrap().trim()));
    result
}
