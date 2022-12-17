use std::fmt::Display;

trait DayExecutor {
    fn exec_part1(&self, input: String) -> Box<dyn Display>;
    fn exec_part2(&self, input: String) -> Box<dyn Display>;
}

trait DayExecutorFactory {
    fn make_day_executor(&self, day_arg: &str, part_arg: &str) -> Box<dyn DayExecutor>;
}

struct DayFactory {}

struct EmptyDisplay {}
impl Display for EmptyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO! This day is not implemented")
    }
}
struct DefaultDay {}
impl DayExecutor for DefaultDay {
    fn exec_part1(&self, input: String) -> Box<dyn Display> {
        Box::new(EmptyDisplay {})
    }

    fn exec_part2(&self, input: String) -> Box<dyn Display> {
        Box::new(EmptyDisplay {})
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

        Box::new(DefaultDay {})
    }
}
