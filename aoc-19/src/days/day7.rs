use crate::days::*;
use day5::Intcode;

struct Amplifiers {
    original: Intcode,
}

impl Amplifiers {
    fn output_for_phase_configuration(&self, phases: &[usize]) -> isize {
        let mut inputs: Vec<isize> = vec![0, 0];

        for &phase in phases {
            inputs[0] = phase as isize;
            let mut amp = self.original.clone();
            amp.set_inputs(&inputs);
            amp.run_program();
            inputs[1] = *amp.outputs.last().unwrap();
        }
        inputs[1]
    }

    fn output_feedback_loop(&self, phases: &[usize]) -> isize {
        let mut amps: Vec<Intcode> = Vec::new();
        let mut phases: Vec<isize> = phases.iter().map(|&x| x as isize).collect();

        (0..phases.len()).for_each(|_| amps.push(self.original.clone()));
        let mut last_output = 0isize;
        let mut halted_counter = 0;
        let amps_len = amps.len();

        'outer: loop {
            for amp in amps.iter_mut() {
                if amp.is_halted() {
                    if halted_counter == amps_len {
                        break 'outer;
                    } else {
                        halted_counter += 1;
                        continue;
                    }
                }
                if !phases.is_empty() {
                    let phase = phases.pop().unwrap();
                    amp.add_inputs(&[phase]);
                }

                amp.add_inputs(&[last_output]);
                last_output = amp.run_program().unwrap();
            }
        }
        *amps.last().unwrap().outputs.last().unwrap()
    }
}

pub struct Day7Runner {
    path: String,
    part: Parts,
}

fn factorial(num: usize) -> usize {
    (2..=num).fold(1, |acc, x| acc * x)
}

impl Day7Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> isize {
        use superslice::*;
        let mut phases: [usize; 5] = [0, 1, 2, 3, 4];
        let amp = Amplifiers {
            original: self.load(false),
        };
        let mut max_output = isize::MIN;

        for _ in 0..factorial(phases.len()) {
            phases.next_permutation();
            let output = amp.output_for_phase_configuration(&phases);

            if output > max_output {
                max_output = output;
            }
        }
        max_output
    }
    fn part2(&self) -> isize {
        use superslice::*;
        let mut phases = (5..=9).collect::<Vec<usize>>();
        let amp = Amplifiers {
            original: self.load(true),
        };
        let mut max_output = isize::MIN;

        for _ in 0..factorial(phases.len()) {
            phases.next_permutation();
            let output = amp.output_feedback_loop(&phases);

            if output > max_output {
                max_output = output;
            }
        }
        max_output
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

impl Runner for Day7Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
