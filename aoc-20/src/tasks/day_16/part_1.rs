use super::{Range, TicketSystem};

pub(super) fn ticket_scanning_error_rate(system: TicketSystem) -> usize {
  let valid_ranges = system.schema.fields.values().flatten().collect::<Vec<_>>();
  let values = system
    .nearby_tickets
    .iter()
    .flat_map(|ticket| ticket.field_values.iter())
    .collect::<Vec<_>>();

  values
    .into_iter()
    .filter(|val| !is_value_valid(*val, valid_ranges.as_slice()))
    .sum()
}

fn is_value_valid(value: &usize, ranges: &[&Range]) -> bool {
  ranges.iter().any(|r| r.contains(value))
}
