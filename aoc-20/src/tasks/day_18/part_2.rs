use super::{Associativity, EquationParserBuilder, Operator, RawEquation, Variable};

pub(super) fn sum_equation_solutions(inputs: Vec<RawEquation>) -> isize {
  let eq_parser = EquationParserBuilder::new()
    .with_open_paren('(')
    .with_closing_paren(')')
    .register_operator('+', Operator::new("Add", 1, Associativity::Left, add))
    .register_operator('*', Operator::new("Mul", 0, Associativity::Left, mul))
    .build_parser();

  inputs
    .into_iter()
    .map(|raw_eq| eq_parser.parse(&raw_eq))
    .map(|eq| eq.solve())
    .sum()
}

fn add(lhs: &Variable, rhs: &Variable) -> Variable {
  Variable {
    value: lhs.value + rhs.value,
  }
}

fn mul(lhs: &Variable, rhs: &Variable) -> Variable {
  Variable {
    value: lhs.value * rhs.value,
  }
}

#[cfg(test)]
mod tests {
  use crate::tasks::day_18::EquationParser;

  use super::*;
  fn get_test_data() -> Vec<(RawEquation, isize)> {
    vec![
      (RawEquation::new("1 + (2 * 3) + (4 * (5 + 6))"), 51),
      (RawEquation::new("2 * 3 + (4 * 5)"), 46),
      (RawEquation::new("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445),
      (
        RawEquation::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        669060,
      ),
      (
        RawEquation::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340,
      ),
    ]
  }

  fn get_parser() -> EquationParser {
    EquationParserBuilder::new()
      .with_open_paren('(')
      .with_closing_paren(')')
      .register_operator('+', Operator::new("Add", 1, Associativity::Left, add))
      .register_operator('*', Operator::new("Mul", 0, Associativity::Left, mul))
      .build_parser()
  }

  #[test]
  fn test() {
    let parser = get_parser();

    get_test_data()
      .iter()
      .map(|(r_eq, expected)| (parser.parse(&r_eq).solve(), expected))
      .for_each(|(result, expected)| assert_eq!(*expected, result));
  }
}
