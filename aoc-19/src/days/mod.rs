pub mod day1;
pub mod day2;
pub mod day3;

pub trait Runner {
    fn run(&self) -> String;
}

pub fn runner_factory(day: &Days, part: &Parts, input_path: &str) -> Box<dyn Runner> {
    use day1::Day1Runner;
    use day2::Day2Runner;
    use day3::Day3Runner;

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
    }
}

pub enum Days {
    Day1,
    Day2,
    Day3,
}

#[derive(Clone)]
pub enum Parts {
    Part1,
    Part2,
}
