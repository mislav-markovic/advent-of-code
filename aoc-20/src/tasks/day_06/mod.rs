use std::str::FromStr;

mod part_1;
mod part_2;

struct AnswerSheet {
  yes_answers: Vec<char>,
}

impl FromStr for AnswerSheet {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      yes_answers: input.trim().chars().collect::<Vec<char>>(),
    })
  }
}

struct Group {
  answer_sheets: Vec<AnswerSheet>,
}

impl FromStr for Group {
  type Err = String;

  fn from_str(group_str: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      answer_sheets: group_str
        .lines()
        .map(|l| l.parse::<AnswerSheet>().unwrap())
        .collect::<Vec<AnswerSheet>>(),
    })
  }
}

impl Group {
  fn which_questions_anyone_answered_yes(&self) -> Vec<char> {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    let answer_set = self
      .answer_sheets
      .iter()
      .map(|sheet| &sheet.yes_answers)
      .flatten()
      .copied()
      .collect::<HashSet<char>>();

    Vec::from_iter(answer_set)
  }

  fn which_questions_everyone_answered_yes(&self) -> Vec<char> {
    use std::collections::HashSet;
    self
      .answer_sheets
      .iter()
      .map(|sheet| sheet.yes_answers.iter().copied().collect::<HashSet<char>>())
      .fold_first(|acc, elem| acc.intersection(&elem).copied().collect())
      .unwrap()
      .iter()
      .copied()
      .collect()
  }
}

pub fn solve_part_1(input_root: &str) {
  let result = part_1::sum_of_group_yes_answers(&get_data(input_root));
  println!("(Day 6, Part 1) Sum of all group yes answers is {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let result = part_2::sum_of_answeres_where_everyone_said_yes(&get_data(input_root));
  println!("(Day 6, Part 2) Sum of all group yes answers is {}", result);
}

fn get_data(root: &str) -> Vec<Group> {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_06.input.txt", root);
  println!("Reading input from {}", &path);

  fr::parse_input::<Group>(&path, "\r\n\r\n")
}
