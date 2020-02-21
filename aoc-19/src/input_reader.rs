pub fn read_lines_from_input(path: &str) -> Result<Vec<String>, ReaderError> {
    use std::fs;

    let contents = fs::read_to_string(path)
        .map_err(|_| ReaderError {})
        .map(|text| text.lines().map(|s| String::from(s)).collect());

    contents
}

pub trait Parser {
    type R;

    fn parse_line(line: &str) -> Result<Self::R, ParseError>;

    fn parse_all(input: &[&str]) -> Vec<Result<Self::R, ParseError>> {
        let mut result: Vec<Result<Self::R, ParseError>> = Vec::with_capacity(input.len());
        for line in input.into_iter() {
            result.push(Self::parse_line(line));
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct ReaderError {}

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
    line: String,
}

impl ParseError {
    pub fn new(message: String, line: String) -> Self {
        ParseError { message, line }
    }

    pub fn new_copy(message: &str, line: &str) -> Self {
        Self::new(String::from(message), String::from(line))
    }

    pub fn from(error: &Self) -> Self {
        Self::new_copy(&error.message, &error.line)
    }
}
