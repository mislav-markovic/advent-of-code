use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day13;
impl DayExecutor for Day13 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Part 1 TODO"))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Part 2 TODO"))
    }
}

mod details {
    use std::{iter::Peekable, slice::Iter};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Token {
        OpenBracket,
        CloseBracket,
        Comma,
        Number(u32),
    }

    pub enum Element {
        List(Vec<Element>),
        Number(u32),
    }

    pub fn tokenize(s: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut stream = s.chars().peekable();
        const OPEN_BRACKET: char = '[';
        const CLOSE_BRACKET: char = ']';
        const COMMA: char = ',';

        loop {
            while let Some(matched) =
                stream.next_if(|c| [OPEN_BRACKET, CLOSE_BRACKET, COMMA].contains(c))
            {
                let token = match matched {
					OPEN_BRACKET => Token::OpenBracket,
					CLOSE_BRACKET => Token::CloseBracket,
					COMMA => Token::Comma,
					_ => unreachable!("Logic error, characte must match open bracket, close bracket or comma at this point")
				};
                tokens.push(token);
            }

            if stream.peek().is_none() {
                break;
            }

            let mut num_str = String::new();
            while let Some(digit) = stream.next_if(|c| c.is_digit(10)) {
                num_str.push(digit);
            }

            let num = num_str.parse::<u32>().expect("Failed to parse num");
            tokens.push(Token::Number(num));
        }
        tokens
    }

    enum ParserError {
        UnexpectedToken(Token),
        EndOfStream,
    }

    type ParserResult = Result<Element, ParserError>;
    struct Parser<'a> {
        token_stream: &'a mut Peekable<Iter<'a, Token>>,
    }

    impl<'a> Parser<'a> {
        fn new(token_stream: &'a mut Peekable<Iter<'a, Token>>) -> Self {
            Self { token_stream }
        }

        fn parse(&mut self) -> Element {
            todo!()
        }

        fn list_begin(&mut self) -> bool {
            self.token_stream.next_if_eq(&&Token::OpenBracket).is_some()
        }

        fn list_end(&mut self) -> bool {
            self.token_stream
                .next_if_eq(&&Token::CloseBracket)
                .is_some()
        }

        fn comma(&mut self) -> bool {
            self.token_stream.next_if_eq(&&Token::Comma).is_some()
        }

        fn num(&mut self) -> ParserResult {
            match self
                .token_stream
                .next_if(|&tok| matches!(tok, Token::Number(_)))
            {
                Some(Token::Number(n)) => Ok(Element::Number(*n)),
                None => {
                    if let Some(&tok) = self.token_stream.peek() {
                        Err(ParserError::UnexpectedToken(tok.clone()))
                    } else {
                        Err(ParserError::EndOfStream)
                    }
                }
                _ => unreachable!(),
            }
        }

        fn element(&mut self) -> ParserResult {
            todo!()
        }

        fn n_elements(&mut self) -> Vec<ParserResult> {
            todo!()
        }

        fn list(&mut self) -> ParserResult {
            if !self.list_begin() {
                return match self.token_stream.peek() {
                    Some(&tok) => Err(ParserError::UnexpectedToken(tok.clone())),
                    None => Err(ParserError::EndOfStream),
                };
            }

            // we hit empty list
            if self.list_end() {
                Ok(Element::List(Vec::new()))
            } else {
                // try to match n_elements, if that fails try to match single element
                self.n_elements()
				.map(|v| )
                    .or_else(|_| self.element())
                    .map(|e| Element::List(()))
            }
        }

        fn packet(&mut self) -> ParserResult {
            self.list()
        }
    }
}

/*
GRAMMAR:

<packet> ::= <list>
<list_begin> ::= "["
<list_end> ::= "]"
<comma> ::= ","
<num> ::= [0-9]+
<element> ::= <num> | <list>
<n_elements> ::= (<element> <comma>)+ <element>
<list> ::= <list_begin> (<n_elements> | <element> | E) <list_end>
*/
