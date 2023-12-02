use phf::phf_map;

const INPUT: &'static str = include_str!("input1.txt");
const _EXAMPLE: &'static str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

fn main() {
    println!("Day01 Hello, World!");
    let res = day1_exec();
    println!("{res}");
}

fn part1() -> String {
    let result = INPUT
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .filter(|v| v.len() >= 1)
        .map(|d_arr| (*d_arr.first().unwrap(), *d_arr.last().unwrap()))
        .map(|(f, l)| {
            let mut rv = String::with_capacity(2);
            rv.push(f);
            rv.push(l);
            rv
        })
        .map(|num_str| num_str.parse::<u32>().expect("To be able to parse num"))
        .sum::<u32>();

    format!("Day-01 Part 01 Result is: {result}")
}

static DIGIT_WORDS: phf::Map<&'static str, u8> = phf_map! {
    "one" => 1,
     "two" => 2,
     "three" => 3,
     "four" => 4,
     "five" => 5,
     "six" => 6,
     "seven" => 7,
     "eight" => 8,
     "nine" => 9,
};

fn buffer_size() -> usize {
    DIGIT_WORDS
        .keys()
        .map(|w| w.len())
        .max()
        .expect("max size can be calculated")
        + 1 // allow for extra chacater which causes us to detec we are no longer prefix
}

fn is_digit_match(candidate: &str) -> Option<u8> {
    DIGIT_WORDS.get(candidate).cloned().map(|d| d)
}

fn is_digit_prefix(candidate: &str) -> bool {
    DIGIT_WORDS.keys().any(|d| d.starts_with(candidate))
}

fn adjust_buffer(buf: &mut String) {
    let mut skip = 0usize;
    let mut should_clear = true;

    while skip < buf.len() {
        let maybe_prefix = &buf[skip..];

        if is_digit_prefix(maybe_prefix) {
            buf.replace_range(..skip, "");
            should_clear = false;
        }

        skip += 1;
    }

    if should_clear {
        buf.clear();
    }
}

fn extract_val(line: &str) -> u32 {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;
    let mut buf = String::with_capacity(buffer_size());

    for c in line.chars() {
        buf.push(c);
        let digit = if c.is_numeric() {
            buf.clear();

            Some(c.to_digit(10).expect("we are working in base 10") as u8)
        } else {
            if let Some(d) = is_digit_match(&buf) {
                buf.clear();
                // is overlap like `oneight` -> on[e]ight -> 18 supported?
                buf.push(c);
                Some(d)
            } else if !is_digit_prefix(&buf) {
                adjust_buffer(&mut buf);
                None
            } else {
                None
            }
        };

        if let Some(d) = digit {
            match (first_digit, last_digit) {
                (None, _) => first_digit = Some(d),
                _ => last_digit = Some(d),
            }
        }
    }

    let val = match (first_digit, last_digit) {
        (Some(f), Some(l)) => (f * 10 + l).into(),
        (Some(d), None) => (d * 10 + d).into(),
        _ => 0,
    };

    val
}

fn part2() -> String {
    let mut sum = 0u32;

    let lines = INPUT.lines();
    for line in lines {
        sum += extract_val(line);
    }

    format!("Day-01 Part 02 Result is: {sum}")
}

pub fn day1_exec() -> String {
    let part1_res = part1();
    let part2_res = part2();
    format!("{part1_res}\n{part2_res}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_word_digits_detected() {
        assert_eq!(is_digit_match("one"), Some(1));
        assert_eq!(is_digit_match("two"), Some(2));
        assert_eq!(is_digit_match("three"), Some(3));
        assert_eq!(is_digit_match("four"), Some(4));
        assert_eq!(is_digit_match("five"), Some(5));
        assert_eq!(is_digit_match("six"), Some(6));
        assert_eq!(is_digit_match("seven"), Some(7));
        assert_eq!(is_digit_match("eight"), Some(8));
        assert_eq!(is_digit_match("nine"), Some(9));
    }

    #[test]
    fn extract_with_tricky_prefix() {
        assert_eq!(extract_val("onine1"), 91);
    }
}
