use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "D:\\Faks\\git\\advent-of-code-17\\corruption_checksum\\input.txt";

struct Row {
    elements: Vec<i32>,
    diff: i32,
}

impl Row {
    fn new(vec: Vec<i32>) -> Row{
        let diff: i32 = vec.iter().max().unwrap() - vec.iter().min().unwrap();
        Row { elements: vec, diff: diff}
    }
}

fn main() {
    let spreadsheet = read_input();
    let checksum = spreadsheet.iter().fold(0, |sum, row| sum+row.diff);
    println!("\n\nChecksum is: {}", checksum);
}

fn read_input() -> Vec<Row> {
    let mut f = File::open(FILE_PATH).expect("file not found");
    let mut result: Vec<Row> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Row {
    println!("Parsing line: {}", line);
    let vec: Vec<i32> = line.trim().split(char::is_whitespace).collect::<Vec<&str>>().iter()
                    .filter(|a| !a.is_empty() && a.trim().parse::<i32>().is_ok())
                    .map(|&a| a.trim().parse::<i32>().unwrap())
                    .collect();
    Row::new(vec)
}
