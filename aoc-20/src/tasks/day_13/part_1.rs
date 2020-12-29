use super::Schedule;

pub(super) fn mul_bus_id_and_wait_time(schedule: Schedule) -> usize {
  let (id, time) = schedule
    .bus_lines
    .iter()
    .filter_map(|bus| {
      bus.id.map(|id| {
        (
          id,
          first_multiplicant_over_target(id, schedule.earliest_departure),
        )
      })
    })
    .min_by_key(|elem| elem.1)
    .unwrap();
  id * (time - schedule.earliest_departure)
}

fn first_multiplicant_over_target(start: usize, target: usize) -> usize {
  let mut temp = start;
  while temp < target {
    temp += start;
  }
  temp
}
