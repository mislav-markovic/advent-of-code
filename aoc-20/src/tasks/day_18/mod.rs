use std::{collections::HashMap, str::FromStr};

mod part_1;
mod part_2;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Associativity {
  Left,
  Right,
}

type OperationFnT = fn(lhs: &Variable, rhs: &Variable) -> Variable;
#[derive(Clone)]
struct Operator {
  name: String,
  precedence: usize,
  associativity: Associativity,
  operation_fn: OperationFnT,
}

impl Operator {
  fn new(
    name: &str,
    precedence: usize,
    associativity: Associativity,
    operation_fn: OperationFnT,
  ) -> Self {
    Self {
      name: name.to_string(),
      precedence,
      associativity,
      operation_fn,
    }
  }
}

#[derive(Clone, Copy)]
struct Variable {
  value: isize,
}

enum Symbol {
  Op(Operator),
  Num(Variable),
  OpenParen,
  CloseParen,
}

struct Equation {
  input: Vec<Symbol>,
}

impl Equation {
  fn solve(&self) -> isize {
    let mut output: Vec<&Symbol> = Vec::new();
    let mut operator_stack: Vec<&Symbol> = Vec::new();

    for sym in self.input.iter() {
      match sym {
        Symbol::Op(op_sym) => {
          if operator_stack.is_empty() {
            operator_stack.push(sym);
          } else {
            match operator_stack.last().unwrap() {
              Symbol::OpenParen => {
                operator_stack.push(sym);
              }
              Symbol::Op(stack_top) => {
                if stack_top.precedence > op_sym.precedence
                  || (op_sym.associativity == Associativity::Left
                    && stack_top.precedence == op_sym.precedence)
                {
                  output.push(operator_stack.pop().unwrap());
                  operator_stack.push(sym);
                } else {
                  operator_stack.push(sym);
                }
              }
              _ => panic!("Invalid operator on operator stack"),
            };
          }
        }
        Symbol::Num(_) => output.push(sym),
        Symbol::OpenParen => operator_stack.push(sym),
        Symbol::CloseParen => {
          while let s @ Symbol::Op(_) = operator_stack.pop().unwrap() {
            output.push(s)
          }
        }
      }
    }

    if !operator_stack.is_empty() {
      output.extend(operator_stack.into_iter().rev());
    }

    let mut variable_stack: Vec<Variable> = Vec::new();
    for sym in output {
      match sym {
        Symbol::Op(op) => {
          let rhs = variable_stack.pop().unwrap();
          let lhs = variable_stack.pop().unwrap();
          let op_res = (op.operation_fn)(&lhs, &rhs);
          variable_stack.push(op_res);
        }
        Symbol::Num(var) => variable_stack.push(*var),
        _ => panic!("Output can only containt operators and variables"),
      }
    }
    variable_stack.last().unwrap().value
  }
}

struct EquationParser {
  operator_map: HashMap<char, Operator>,
  open_paren: char,
  closing_paren: char,
}

impl EquationParser {
  fn parse(&self, raw_eq: &RawEquation) -> Equation {
    let mut num_temp = String::new();
    let mut input: Vec<Symbol> = Vec::new();

    for sym in raw_eq.input.chars() {
      if sym.is_whitespace() {
        if !num_temp.is_empty() {
          input.push(Symbol::Num(Variable {
            value: num_temp.parse().unwrap(),
          }));
          num_temp.clear();
        }
      } else if sym == self.open_paren {
        input.push(Symbol::OpenParen);
      } else if sym == self.closing_paren {
        if !num_temp.is_empty() {
          input.push(Symbol::Num(Variable {
            value: num_temp.parse().unwrap(),
          }));
          num_temp.clear();
        }
        input.push(Symbol::CloseParen);
      } else if self.operator_map.contains_key(&sym) {
        input.push(Symbol::Op(self.operator_map.get(&sym).unwrap().clone()));
      } else {
        num_temp.push(sym);
      }
    }

    if !num_temp.is_empty() {
      input.push(Symbol::Num(Variable {
        value: num_temp.parse().unwrap(),
      }));
      num_temp.clear();
    }
    Equation { input }
  }
}

struct RawEquation {
  input: String,
}

impl FromStr for RawEquation {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let input = s.trim().to_string();
    Ok(Self { input })
  }
}

impl RawEquation {
  fn new(input: &str) -> Self {
    Self {
      input: input.to_string(),
    }
  }
}
struct EquationParserBuilder {
  map: HashMap<char, Operator>,
  open_paren: Option<char>,
  closing_paren: Option<char>,
}

impl EquationParserBuilder {
  fn new() -> Self {
    Self {
      map: HashMap::new(),
      open_paren: None,
      closing_paren: None,
    }
  }

  fn register_operator(mut self, symbol: char, op: Operator) -> Self {
    self.map.insert(symbol, op);
    self
  }

  fn with_closing_paren(mut self, closing_paren: char) -> Self {
    self.closing_paren = Some(closing_paren);
    self
  }

  fn with_open_paren(mut self, open_paren: char) -> Self {
    self.open_paren = Some(open_paren);
    self
  }

  fn build_parser(self) -> EquationParser {
    let open_paren = self.open_paren.unwrap();
    let closing_paren = self.closing_paren.unwrap();

    EquationParser {
      operator_map: self.map,
      open_paren,
      closing_paren,
    }
  }
}

pub fn solve_part_1(input_root: &str) {
  let result = part_1::sum_equation_solutions(get_data(input_root));
  println!("(Day 18, Part 1) Sum of equation results = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let result = part_2::sum_equation_solutions(get_data(input_root));
  println!("(Day 18, Part 2) Sum of equation results = {}", result);
}

fn get_data(root: &str) -> Vec<RawEquation> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_18.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<RawEquation>(&path, "\r\n")
}
