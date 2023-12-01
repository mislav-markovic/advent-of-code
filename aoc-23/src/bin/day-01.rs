use phf::phf_map;

const INPUT: &'static str = include_str!("input1.txt");
const EXAMPLE: &'static str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

fn main() {
    println!("Day01 Hello, World!");
    part1();
    part2();
}

fn part1() {
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

    println!("Day-01 Part 01 Result is: {result}");
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
}

fn is_digit_match(candidate: &str) -> Option<u8> {
    DIGIT_WORDS.get(candidate).cloned().map(|d| d)
}

fn is_digit_prefix(candidate: &str) -> bool {
    DIGIT_WORDS.keys().any(|d| d.starts_with(candidate))
}

fn part2() {
    let mut buf = String::with_capacity(buffer_size());
    let mut vals: Vec<u32> = Vec::new();

    let lines = INPUT.lines();
    for line in lines {
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;
        buf.clear();

        for c in line.chars() {
            buf.push(c);
            let digit = if c.is_numeric() {
                buf.clear();

                Some(c.to_digit(10).expect("we are working in base 10") as u8)
            } else {
                if let Some(d) = is_digit_match(&buf) {
                    buf.clear();
                    Some(d)
                    // assign
                } else if !is_digit_prefix(&buf) {
                    buf.clear();
                    buf.push(c);
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

        match (first_digit, last_digit) {
            (Some(f), Some(l)) => vals.push((f * 10 + l).into()),
            (Some(d), None) => vals.push((d * 10 + d).into()),
            _ => eprintln!("WARN: No digits in line: {line}"),
        };
        println!(
            "INFO: {val} for: {line}",
            val = vals.last().cloned().unwrap_or(0)
        );
    }

    let rv: u32 = vals.iter().sum();

    println!("Day-01 Part 02 Result is: {rv}");
}
