use std::{fmt::Display, str::FromStr};

pub trait DayExecutor {
    fn exec_part1(&self, input: String) -> Box<dyn Display>;
    fn exec_part2(&self, input: String) -> Box<dyn Display>;
}

trait DayExecutorFactory {
    fn make_day_executor(&self, day_arg: &str, part_arg: &str) -> Box<dyn DayExecutor>;
}

struct DayFactory {}

#[derive(Clone)]
struct UnknownDayExecutor {
    day_arg: String,
    part_arg: String,
}

impl UnknownDayExecutor {
    fn new(day_arg: String, part_arg: String) -> Self {
        Self { day_arg, part_arg }
    }
}

impl Display for UnknownDayExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Can't provide executor for day {} part {}",
            self.day_arg, self.part_arg
        ))
    }
}

impl DayExecutor for UnknownDayExecutor {
    fn exec_part1(&self, input: String) -> Box<dyn Display> {
        Box::new(self.clone())
    }

    fn exec_part2(&self, input: String) -> Box<dyn Display> {
        Box::new(self.clone())
    }
}

impl DayExecutorFactory for DayFactory {
    fn make_day_executor(&self, day_arg: &str, part_arg: &str) -> Box<dyn DayExecutor> {
        let parsed_day: usize = day_arg
            .trim_start_matches(|c: char| !c.is_digit(10))
            .trim()
            .parse()
            .expect("Wrong argument for selecting day to execute");

        let parsed_part: usize = part_arg
            .trim_start_matches(|c: char| !c.is_digit(10))
            .trim()
            .parse()
            .expect("Wrong argument for selecting which part of day to execute");

        Box::new(UnknownDayExecutor::new(
            day_arg.to_owned(),
            part_arg.to_owned(),
        ))
    }
}
