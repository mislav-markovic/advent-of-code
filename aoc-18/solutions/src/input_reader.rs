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
    read_all(file_name).lines().map(|s| s.to_owned()).collect()
}

fn open_file(file_name: &str) -> File {
    let relative_path = "input/".to_owned() + file_name;
    File::open(relative_path).expect("file not found")
}
