use crate::input_reader;

pub struct Freq {
    current: i32,
}

impl Freq {
    pub fn new() -> Freq {
        Freq { current: 0 }
    }

    pub fn get_current(&self) -> i32 {
        self.current
    }

    pub fn calibrate(&mut self, change: i32) -> i32 {
        self.current += change;
        self.current
    }

    pub fn calibrate_str(&mut self, change: &str) -> i32 {
        self.calibrate(change.parse().unwrap())
    }

    pub fn calibrate_all(&mut self, changes: &[i32]) -> i32 {
        self.current += changes.iter().sum::<i32>();
        self.current
    }

    pub fn calibrate_str_all(&mut self, changes: &[&str]) -> i32 {
        let transform = changes
            .iter()
            .map(|elem| elem.parse().unwrap())
            .collect::<Vec<i32>>();
        self.calibrate_all(&transform[..])
    }
}

pub fn part1(input_location: &str) -> i32 {
    let mut freq = Freq::new();
    let data = input_reader::read_all_lines(input_location);
    let input = data.iter().map(|s| &s[..]).collect::<Vec<&str>>();
    freq.calibrate_str_all(&input[..])
}

pub fn part2(input_location: &str) -> i32 {
    use std::collections::HashSet;

    let mut freq = Freq::new();
    let data = input_reader::read_all_lines(input_location);
    let v = data
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut duplicate_detection = HashSet::new();
    duplicate_detection.insert(freq.get_current());

    'outer: loop {
        for l in v.iter() {
            freq.calibrate(*l);
            if duplicate_detection.contains(&freq.get_current()) {
                break 'outer;
            } else {
                duplicate_detection.insert(freq.get_current());
            }
        }
    }

    freq.get_current()
}

pub fn day1() {
    let input = String::from("day1");

    println!("***Day One***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tFrequency: {}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tFirst duplicate: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::Freq;

    #[test]
    fn init() {
        let f = Freq::new();
        assert_eq!(f.get_current(), 0);
    }

    #[test]
    fn add() {
        let mut f = Freq::new();
        f.calibrate(2);
        assert_eq!(f.get_current(), 2);
    }

    #[test]
    fn sub() {
        let mut f = Freq::new();
        f.calibrate(-2);
        assert_eq!(f.get_current(), -2);
    }

    #[test]
    fn add_sub() {
        let mut f = Freq::new();
        f.calibrate(5);
        f.calibrate(-2);
        assert_eq!(f.get_current(), 3);
    }

    #[test]
    fn add_str() {
        let mut f = Freq::new();
        f.calibrate_str("+4");
        assert_eq!(f.get_current(), 4);
    }

    #[test]
    fn sub_str() {
        let mut f = Freq::new();
        f.calibrate_str("-2");
        assert_eq!(f.get_current(), -2);
    }

    #[test]
    fn add_sub_str() {
        let mut f = Freq::new();
        f.calibrate_str("+5");
        f.calibrate_str("-2");
        assert_eq!(f.get_current(), 3);
    }

    #[test]
    fn complex_case_1() {
        let mut f = Freq::new();

        f.calibrate(5);
        f.calibrate(-7);
        f.calibrate(12);

        f.calibrate_str("+10");
        f.calibrate_str("-2");
        f.calibrate_str("-8");
        assert_eq!(f.get_current(), 10);
    }

    #[test]
    fn complex_case_2() {
        let mut f = Freq::new();

        f.calibrate(5);
        f.calibrate_str("+10");
        f.calibrate(-7);
        f.calibrate_str("-2");
        f.calibrate(12);
        f.calibrate_str("-8");

        assert_eq!(f.get_current(), 10);
    }

    #[test]
    fn calibrate_all() {
        let mut f = Freq::new();
        let input: [i32; 6] = [1, 2, 3, -1, -3, -3];

        assert_eq!(f.calibrate_all(&input[..]), -1);
    }

    #[test]
    fn calibrate_str_all() {
        let mut f = Freq::new();
        let input = ["+1", "+1", "+1", "1", "-5", "+2", "-3"];
        assert_eq!(f.calibrate_str_all(&input[..]), -2);
    }

    #[test]
    fn complex_case_3() {
        let mut f = Freq::new();
        let input1: [i32; 6] = [1, 2, 3, -1, -3, -3];
        let input2 = ["+1", "+1", "+1", "1", "-5", "+2", "-3"];
        f.calibrate_all(&input1[..]);
        f.calibrate_str_all(&input2);
        assert_eq!(f.get_current(), -3);
    }
}
