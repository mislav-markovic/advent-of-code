pub mod day1;

pub trait Runner {
    fn run(&self) -> String;
}

pub fn runner_factory(day: &Days, part: &Parts, input_path: &str) -> Box<dyn Runner> {
    use day1::Day1Runner;
    match day {
        Days::Day1 => match part {
            Parts::Part1 => Box::new(Day1Runner::with_input_path(input_path, part.clone())),
            Parts::Part2 => Box::new(Day1Runner::with_input_path(input_path, part.clone())),
        },
    }
}

pub enum Days {
    Day1,
}

#[derive(Clone)]
pub enum Parts {
    Part1,
    Part2,
}
