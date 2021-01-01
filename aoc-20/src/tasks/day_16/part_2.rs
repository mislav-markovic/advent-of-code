use std::collections::{HashMap, HashSet};

use super::{Range, Ticket, TicketSchema, TicketSystem};

pub(super) fn departure_validation(system: TicketSystem) -> usize {
  let valid_nearby_tickets = system
    .nearby_tickets
    .iter()
    .filter(|ticket| is_ticket_valid(ticket, &system.schema))
    .collect::<Vec<_>>();

  let mut column_order: Vec<Vec<usize>> = Vec::new();

  for index in 0..valid_nearby_tickets[0].field_values.len() {
    let mut column: Vec<usize> = Vec::new();

    for ticket in valid_nearby_tickets.iter() {
      column.push(ticket.field_values[index]);
    }
    column_order.push(column);
  }

  let mut field_positions: HashMap<&String, usize> = HashMap::new();
  let mut used_columns: HashSet<usize> = HashSet::new();

  while field_positions.len() < system.schema.fields.len() {
    for (field, constraints) in system.schema.fields.iter() {
      if field_positions.contains_key(field) {
        continue;
      }

      let candidate_positions = column_order
        .iter()
        .enumerate()
        .filter(|(i, _)| !used_columns.contains(i))
        .filter_map(|(i, col)| {
          if is_candidate(&col, &constraints) {
            Some(i)
          } else {
            None
          }
        })
        .collect::<Vec<_>>();

      if candidate_positions.len() == 1 {
        field_positions.insert(field, candidate_positions[0]);
        used_columns.insert(candidate_positions[0]);
      }
    }
  }
  field_positions
    .iter()
    .filter_map(|(k, v)| {
      if k.starts_with("departure") {
        Some(v)
      } else {
        None
      }
    })
    .fold(1usize, |acc, elem| {
      acc * system.your_ticket.field_values[*elem]
    })
}

fn is_value_valid(value: &usize, ranges: &[Range]) -> bool {
  ranges.iter().any(|r| r.contains(value))
}

fn is_ticket_valid(ticket: &Ticket, schema: &TicketSchema) -> bool {
  let valid_ranges = schema
    .fields
    .values()
    .cloned()
    .flatten()
    .collect::<Vec<_>>();

  ticket
    .field_values
    .iter()
    .all(|val| is_value_valid(val, &valid_ranges))
}

fn is_candidate(column: &[usize], field_constraints: &[Range]) -> bool {
  column
    .iter()
    .all(|val| is_value_valid(val, field_constraints))
}
