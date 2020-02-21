use crate::days::{Parts, Runner};
use crate::input_reader::{ParseError, Parser};
struct Modules {
    module_weights: Vec<u64>,
}

impl Modules {
    fn new(weights: Vec<u64>) -> Self {
        Modules {
            module_weights: weights,
        }
    }

    fn parsed(text: &[&str]) -> Self {
        let result = Self::parse_all(text)
            .into_iter()
            .map(|elem| elem.expect("Could not parse!"))
            .collect();

        Self::new(result)
    }

    fn total_fuel(&self) -> u64 {
        self.module_weights.iter().map(|x| x / 3 - 2).sum()
    }
}

impl Parser for Modules {
    type R = u64;

    fn parse_line(line: &str) -> Result<Self::R, ParseError> {
        let result = line.parse::<u64>();

        match result {
            Ok(num) => Ok(num),
            Err(_) => Err(ParseError::new_copy("Could not parse number", line)),
        }
    }
}

pub struct Day1Runner {
    modules: Modules,
    part: Parts,
}

impl Day1Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        use crate::input_reader;
        let input = input_reader::read_lines_from_input(path)
            .expect(&format!("Failed to load input for day1 with path {}", path));
        let modules = Modules::parsed(
            input
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .as_ref(),
        );
        Day1Runner { modules, part }
    }

    fn part1(&self) -> u64 {
        self.modules.total_fuel()
    }
    fn part2(&self) -> u64 {
        self.modules
            .module_weights
            .iter()
            .map(|x| Day1Runner::module_total_fuel(*x))
            .sum()
    }

    fn module_total_fuel(module_weight: u64) -> u64 {
        let mut total_fuel = (module_weight / 3 - 2) as i64;

        let mut added_fuel = total_fuel / 3 - 2;

        while added_fuel > 0 {
            total_fuel += added_fuel;
            added_fuel = added_fuel / 3 - 2;
        }
        total_fuel as u64
    }
}

impl Runner for Day1Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Total Fuel: {}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Day1Runner, Modules};
    use crate::days::Parts;

    #[test]
    fn part1_test1() {
        let modules = Modules {
            module_weights: vec![1969u64],
        };

        let part = Parts::Part1;
        let runner = Day1Runner { modules, part };

        let result = runner.part1();

        assert_eq!(result, 654u64);
    }

    #[test]
    fn part1_test2() {
        let modules = Modules {
            module_weights: vec![14u64],
        };

        let part = Parts::Part2;
        let runner = Day1Runner { modules, part };

        let result = runner.part1();

        assert_eq!(result, 2u64);
    }

    #[test]
    fn part1_test3() {
        let modules = Modules {
            module_weights: vec![100756u64],
        };

        let part = Parts::Part2;
        let runner = Day1Runner { modules, part };

        let result = runner.part1();

        assert_eq!(result, 33583u64);
    }

    #[test]
    fn part2_test1() {
        let modules = Modules {
            module_weights: vec![1969u64],
        };

        let part = Parts::Part2;
        let runner = Day1Runner { modules, part };

        let result = runner.part2();

        assert_eq!(result, 966u64);
    }

    #[test]
    fn part2_test2() {
        let modules = Modules {
            module_weights: vec![14u64],
        };

        let part = Parts::Part2;
        let runner = Day1Runner { modules, part };

        let result = runner.part2();

        assert_eq!(result, 2u64);
    }

    #[test]
    fn part2_test3() {
        let modules = Modules {
            module_weights: vec![100756u64],
        };

        let part = Parts::Part2;
        let runner = Day1Runner { modules, part };

        let result = runner.part2();

        assert_eq!(result, 50346u64);
    }
}
