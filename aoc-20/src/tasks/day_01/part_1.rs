use input::parse_input;

use crate::common::file_reader as input;

pub fn solve(input_root: &str) -> u64 {
    let input_path = format!("{}/day_01.input.txt", input_root);
    println!("Reading from input: '{}'", &input_path);
    let expenses = parse_input::<u64>(&input_path, "\r\n");
    let (num1, num2) = find_sum(2020u64, &expenses).unwrap();
    num1*num2
}

fn find_sum(target: u64, numbers: &[u64]) -> Option<(u64, u64)> {
    for (index, num) in numbers.iter().enumerate() {
        for candidate in numbers[index+1..].iter() {
            if num + candidate == target {
                return Some((*num, *candidate));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*; // make visible everything in outer scope (everything that we want to test)

    #[test]
    fn sum_finds_correct_target() {
        let inputs: Vec<u64> = vec![1, 5, 7, 11, 22, 102, 555, 1116];
        let target = 562u64;

        let result = find_sum(target, &inputs);
        assert!(result.is_some());
        assert_eq!((7u64, 555u64), result.unwrap());
    }

    #[test]
    fn find_sum_with_test_input_finds_correct_pair() {
        let inputs: Vec<u64> = vec![1721, 979, 366, 299, 675, 1456];
        let target = 2020u64;

        let result = find_sum(target, &inputs);
        assert!(result.is_some());
        assert_eq!((1721u64, 299u64), result.unwrap());
    }
} 