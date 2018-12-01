use std::fs::File;
use std::io::prelude::*;

pub fn read_all(file_name: &str) -> String {
    let mut f = open_file(file_name);

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn read_all_lines(file_name: &str) -> Vec<String> {
    let mut contents = read_all(file_name);
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_owned());
    }
    result
}

fn open_file(file_name: &str) -> File {
    let relative_path = String::from("input/".to_owned() + file_name);
    File::open(relative_path).expect("file not found")
}
