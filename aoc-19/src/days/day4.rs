use crate::days::*;
use crate::input_reader::{read_sparated_values_from_input, ParseError, Parser};
const PASSWORD_LEN: usize = 6;

#[derive(Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
struct Password {
    digits: [usize; PASSWORD_LEN],
    value: usize,
}

impl Password {
    fn from_value(value: usize) -> Self {
        let mut digits = [0; PASSWORD_LEN];
        let mut working_value = value;
        for i in 1..=PASSWORD_LEN {
            digits[PASSWORD_LEN - i] = working_value % 10;
            working_value /= 10;
        }
        Self { digits, value }
    }
}

struct SecureContainer {
    rules: Vec<Box<dyn Fn(&Password) -> bool>>,
    range: Range,
    valid_passwords: Vec<Password>,
}

impl Parser for SecureContainer {
    type R = Range;
    fn parse_line(line: &str) -> Result<Self::R, ParseError> {
        let range_values = line
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Range::new(range_values[0], range_values[1]))
    }
}

impl SecureContainer {
    fn new(rules: Vec<Box<dyn Fn(&Password) -> bool>>, range: Range) -> Self {
        let valid_passwords = vec![];
        Self {
            rules,
            range,
            valid_passwords,
        }
    }

    fn parsed(text: &str) -> Self {
        let result = Self::parse_line(text).expect("Parse error");
        let mut rules: Vec<Box<dyn Fn(&Password) -> bool>> = Vec::new();

        let same_adjacent_digits_rule = |pass: &Password| -> bool {
            let result = pass
                .digits
                .iter()
                .fold(Some((0, 0)), |state, &x| match state {
                    None => Some((x, 1)),
                    Some((val, count)) => {
                        if val == x {
                            Some((x, count + 1))
                        } else {
                            if count != 2 {
                                Some((x, 1))
                            } else {
                                Some((val, count))
                            }
                        }
                    }
                })
                .unwrap()
                .1
                == 2;
            result
        };

        let range_rule = move |pass: &Password| -> bool {
            let result = pass.value >= result.start && pass.value <= result.end;
            result
        };

        let monotonicity_rule =
            |pass: &Password| -> bool { pass.digits.windows(2).all(|slc| slc[0] <= slc[1]) };

        rules.push(Box::new(same_adjacent_digits_rule));
        rules.push(Box::new(range_rule));
        rules.push(Box::new(monotonicity_rule));

        Self::new(rules, result)
    }

    fn generate_passwords(&mut self) {
        self.valid_passwords.clear();
        for value in self.range.start..=self.range.end {
            let pass = Password::from_value(value);
            if self.is_password_valid(&pass) {
                self.valid_passwords.push(pass)
            }
        }
    }

    fn valid_password_count(&self) -> usize {
        self.valid_passwords.len()
    }

    fn is_password_valid(&self, pass: &Password) -> bool {
        self.rules.iter().all(|rule| rule(pass))
    }
}

pub struct Day4Runner {
    path: String,
    part: Parts,
}

impl Day4Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let mut secure_contaier = self.load();
        secure_contaier.generate_passwords();
        secure_contaier.valid_password_count()
    }
    fn part2(&self) -> usize {
        0
    }
    fn load(&self) -> SecureContainer {
        let text = read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        SecureContainer::parsed(&text.expect("Could not read wires")[0])
    }
}

impl Runner for Day4Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Password, SecureContainer};
    #[test]
    fn part1_test1() {
        let sec_cont = SecureContainer::parsed("109165-576723");
        let pass = Password::from_value(122345);

        assert!(sec_cont.is_password_valid(&pass));
    }
}
