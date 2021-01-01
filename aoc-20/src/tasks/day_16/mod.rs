use std::{collections::HashMap, str::FromStr};

mod part_1;
mod part_2;

#[derive(Copy, Clone)]
struct Range {
  start: usize,
  end: usize,
  inclusive: bool,
}

impl FromStr for Range {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let split_vec = input.trim().split('-').collect::<Vec<_>>();

    if split_vec.len() != 2 {
      Err("Range must consist of two numbers seperated by '-'".to_string())
    } else {
      let start = split_vec[0]
        .parse::<usize>()
        .map_err(|parse_err| parse_err.to_string())?;
      let end = split_vec[1]
        .parse::<usize>()
        .map_err(|parse_err| parse_err.to_string())?;

      Ok(Self {
        start,
        end,
        inclusive: true,
      })
    }
  }
}

impl Range {
  fn contains(&self, target: &usize) -> bool {
    *target >= self.start && (*target < self.end || (self.inclusive && *target == self.end))
  }
}

struct TicketSchema {
  fields: HashMap<String, Vec<Range>>,
}

impl FromStr for TicketSchema {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let fields = s
      .lines()
      .map(|l| {
        let (name, ranges_str) = l.split_once(':').unwrap();
        let field_name = name.trim().to_string();
        let ranges = ranges_str
          .split("or")
          .map(|r_str| r_str.trim().parse::<Range>().unwrap())
          .collect::<Vec<_>>();
        (field_name, ranges)
      })
      .collect::<HashMap<String, Vec<Range>>>();

    Ok(Self { fields })
  }
}

struct Ticket {
  field_values: Vec<usize>,
}

impl FromStr for Ticket {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let field_values = s
      .split(',')
      .map(<usize as FromStr>::from_str)
      .map(Result::unwrap)
      .collect::<Vec<_>>();

    Ok(Self { field_values })
  }
}

struct TicketSystem {
  schema: TicketSchema,
  your_ticket: Ticket,
  nearby_tickets: Vec<Ticket>,
}

impl FromStr for TicketSystem {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split_iter = s.trim().split("\r\n\r\n");
    let schema_str = split_iter.next().unwrap();
    let your_ticket_str = split_iter.next().unwrap();
    let nearby_tickets_str = split_iter.next().unwrap();

    let schema = schema_str.parse::<TicketSchema>()?;
    let your_ticket = your_ticket_str
      .lines()
      .skip(1)
      .next()
      .unwrap()
      .parse::<Ticket>()?;
    let nearby_tickets = nearby_tickets_str
      .lines()
      .skip(1)
      .map(|l| l.parse::<Ticket>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self {
      schema,
      your_ticket,
      nearby_tickets,
    })
  }
}

pub fn solve_part_1(input_root: &str) {
  let system = get_data(input_root);
  let result = part_1::ticket_scanning_error_rate(system);
  println!("(Day 16, Part 1) Ticket scanning error rate = {}", result);
}

pub fn solve_part_2(input_root: &str) {
  let system = get_data(input_root);
  let result = part_2::departure_validation(system);
  println!("(Day 16, Part 2) Departure validation: {}", result);
}

fn get_data(root: &str) -> TicketSystem {
  use crate::common::file_reader as fr;
  let path = format!("{}/day_16.input.txt", root);
  println!("Reading input from '{}'", path);

  fr::parse_input::<TicketSystem>(&path, "||||").remove(0)
}
