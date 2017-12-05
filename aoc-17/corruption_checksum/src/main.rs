use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const FILE_PATH: &str = "D:\\Faks\\git\\advent-of-code-17\\corruption_checksum\\input.txt";

struct Row {
    elements: Vec<i32>,
    diff: i32,
    div: i32,
}

impl Row {
    fn new(vec: Vec<i32>) -> Row{
        let diff: i32 = vec.iter().max().unwrap() - vec.iter().min().unwrap();
        let mut div = 0;

        'outer: for (i, elem) in vec.iter().enumerate() {
            for (j, temp) in vec.iter().enumerate(){
                if i == j {
                    continue;
                } else if elem % temp == 0 {
                    div = elem / temp;
                    break 'outer;
                } else if temp % elem == 0{
                    div = temp / elem;
                    break 'outer;
                }
            }
        }

        Row { elements: vec, diff: diff, div: div}
    }
}

fn main() {
    let spreadsheet = read_input();
    let diff_checksum = spreadsheet.iter().fold(0, |sum, row| sum+row.diff);
    let div_checksum = spreadsheet.iter().fold(0, |sum, row| sum+row.div);
    println!("\n\nDiff checksum is: {}", diff_checksum);
    println!("\nDiv checksum is: {}", div_checksum);
}

fn read_input() -> Vec<Row> {
    let f = File::open(FILE_PATH).expect("file not found");
    let mut result: Vec<Row> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Row {
    let vec: Vec<i32> = line.trim().split(char::is_whitespace).collect::<Vec<&str>>().iter()
                    .filter(|a| !a.is_empty() && a.trim().parse::<i32>().is_ok())
                    .map(|&a| a.trim().parse::<i32>().unwrap())
                    .collect();
    Row::new(vec)
}
