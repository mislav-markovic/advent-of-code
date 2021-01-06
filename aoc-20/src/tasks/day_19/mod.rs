use std::{collections::HashMap, str::FromStr};

mod part_1;
mod part_2;

type RuleId = usize;
type Subrule = [RuleId; 2];

#[derive(Clone)]
struct Rule {
  id: RuleId,
  matcher: Option<String>,
  subrules: Vec<Subrule>,
}

impl FromStr for Rule {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (id_str, rule) = s.trim().split_once(':').unwrap();
    let id = id_str
      .parse::<RuleId>()
      .map_err(|_| format!("Could not parse rule id '{}'", id_str))?;

    if rule.contains('"') {
      let val = rule.trim().trim_matches('"').to_string();
      Ok(Self {
        id,
        matcher: Some(val),
        subrules: Vec::new(),
      })
    } else {
      let subrules = rule
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|sub_rule_variant| {
          println!("{}", sub_rule_variant);
          let subrule = sub_rule_variant
            .split(' ')
            .map(|rule_id_ref| rule_id_ref.parse::<RuleId>().unwrap())
            .collect::<Vec<_>>();
          [subrule[0], subrule[1]]
        })
        .collect::<Vec<_>>();
      Ok(Self {
        id,
        matcher: None,
        subrules,
      })
    }
  }
}

struct Message {
  msg: String,
}

impl Message {
  fn new(msg: String) -> Self {
    Self { msg }
  }
}

impl FromStr for Message {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self::new(s.trim().to_string()))
  }
}

struct RuleEngine {
  rules: HashMap<RuleId, Rule>,
  cache: HashMap<RuleId, Vec<String>>,
}

impl RuleEngine {
  fn new(rules: &Vec<Rule>) -> Self {
    Self {
      rules: rules
        .iter()
        .map(|r| (r.id, r.clone()))
        .collect::<HashMap<_, _>>(),
      cache: HashMap::new(),
    }
  }

  fn matches(&mut self, rule_id: RuleId, msg: &Message) -> bool {
    let candidates = resolve_rule(
      self.rules.get(&rule_id).unwrap(),
      &self.rules,
      &mut self.cache,
    );
    candidates.iter().any(|candidate| candidate == &msg.msg)
  }
}

fn resolve_rule(
  rule: &Rule,
  rules: &HashMap<RuleId, Rule>,
  cache: &mut HashMap<RuleId, Vec<String>>,
) -> Vec<String> {
  // is leaf rule
  if rule.matcher.is_some() {
    let s = rule.matcher.as_ref().unwrap().clone();
    vec![s]
    // already calculated and in cache
  } else if cache.contains_key(&rule.id) {
    cache.get(&rule.id).unwrap().clone()
    // we need to resolve rule matchers
  } else {
    let mut result = Vec::new();
    for subrule in rule.subrules.iter() {
      let lhs = resolve_rule(rules.get(&subrule[0]).unwrap(), rules, cache);
      let rhs = resolve_rule(rules.get(&subrule[1]).unwrap(), rules, cache);
      let mut resolved = Vec::with_capacity(lhs.len() * rhs.len());
      for i in lhs {
        for j in &rhs {
          resolved.push(format!("{}{}", i, j));
        }
      }
      result.extend(resolved);
    }
    cache.insert(rule.id, result.clone());
    result
  }
}

struct InputReader {
  rules: Vec<Rule>,
  messages: Vec<Message>,
}

impl InputReader {
  fn from_str(rules_str: &str, messages_str: &str) -> Self {
    Self {
      rules: rules_str
        .lines()
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<_>>(),
      messages: messages_str
        .lines()
        .map(|l| l.parse::<Message>().unwrap())
        .collect::<Vec<_>>(),
    }
  }
}

pub fn solve_part_1(input_root: &str) {
  let result = part_1::count_matches_for_zero_rule(get_data(input_root));
  println!("(Day 19, Part 1) Messages matching rule 0 = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  println!("(Day 19, Part 2) Not Implemented");
}

fn get_data(root: &str) -> InputReader {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_19.input.txt", root);
  println!("Reading input from '{}'", path);

  let mut split_iter: Vec<String> = fr::read_unparsed(&path, "\r\n\r\n");
  let messages = split_iter.pop().unwrap();
  let rules = split_iter.pop().unwrap();
  InputReader::from_str(&rules, &messages)
}
