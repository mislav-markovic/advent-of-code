mod input_reader;
mod day1;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];

    day1::day1();
}