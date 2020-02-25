pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub trait Runner {
    fn run(&self) -> String;
}

pub fn runner_factory(day: &Days, part: &Parts, input_path: &str) -> Box<dyn Runner> {
    use day1::Day1Runner;
    use day2::Day2Runner;
    use day3::Day3Runner;
    use day4::Day4Runner;
    use day5::Day5Runner;
    use day6::Day6Runner;
    use day7::Day7Runner;
    use day8::Day8Runner;

    match day {
        Days::Day1 => match part {
            Parts::Part1 => Box::new(Day1Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day1Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day2 => match part {
            Parts::Part1 => Box::new(Day2Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day2Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day3 => match part {
            Parts::Part1 => Box::new(Day3Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day3Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day4 => match part {
            Parts::Part1 => Box::new(Day4Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day4Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day5 => match part {
            Parts::Part1 => Box::new(Day5Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day5Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day6 => match part {
            Parts::Part1 => Box::new(Day6Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day6Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day7 => match part {
            Parts::Part1 => Box::new(Day7Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day7Runner::with_input_path(input_path, part.clone())),
        },
        Days::Day8 => match part {
            Parts::Part1 => Box::new(Day8Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day8Runner::with_input_path(input_path, part.clone())),
        },
    }
}

pub enum Days {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
}

#[derive(Clone)]
pub enum Parts {
    Part1,
    Part2,
}
