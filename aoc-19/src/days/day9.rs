use crate::days::*;
use day5::Intcode;

pub struct Day9Runner {
    path: String,
    part: Parts,
}

impl Day9Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> isize {
        let mut intcode = self.load(false);
        intcode.set_inputs(&[1]);
        intcode.run_program();
        *intcode.outputs.last().unwrap()
    }
    fn part2(&self) -> isize {
        let mut intcode = self.load(false);
        intcode.set_inputs(&[2]);
        intcode.run_program();
        *intcode.outputs.last().unwrap()
    }

    fn load(&self, pause_on_output: bool) -> Intcode {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        Intcode::parsed(
            &text.expect("Could not read instructions")[0],
            &[],
            pause_on_output,
        )
    }
}

impl Runner for Day9Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
