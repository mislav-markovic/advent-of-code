use super::Schedule;

pub(super) fn earliest_timestamp(schedule: Schedule) -> u64 {
  let ids_offsets = schedule // (bus_id, time_offset)
    .bus_lines
    .iter()
    .enumerate()
    .filter_map(|(i, bus)| bus.id.map(|id| (id as u64, i as u64)))
    .collect::<Vec<_>>();

  let &(first_id, _) = ids_offsets.first().unwrap();
  let mut timestamp = 0;
  let mut inc = first_id;

  for (id, offset) in ids_offsets.into_iter().skip(1) {
    while (timestamp + offset) % id != 0 {
      timestamp += inc;
    }
    inc *= id;
  }

  timestamp
}

fn is_solution(schedule: &Vec<(u64, u64)>, candidate: &u64) -> bool {
  schedule.iter().all(|(id, i)| (candidate + i) % id == 0)
}

#[cfg(test)]
mod tests {
  use super::super::Bus;
  use super::*;

  #[test]
  fn test_1() {
    let buses = vec![
      Bus { id: Some(17) },
      Bus { id: None },
      Bus { id: Some(13) },
      Bus { id: Some(19) },
    ];

    let sched = Schedule {
      earliest_departure: 0,
      bus_lines: buses,
    };
    let id_offsets = sched
      .bus_lines
      .iter()
      .enumerate()
      .filter_map(|(i, bus)| bus.id.map(|id| (id as u64, i as u64)))
      .collect::<Vec<_>>();
    assert!(is_solution(&id_offsets, &3417));
    let result = super::earliest_timestamp(sched);
    assert_eq!(3417, result);
  }
  #[test]
  fn test_2() {
    let buses = vec![
      Bus { id: Some(67) },
      Bus { id: Some(7) },
      Bus { id: Some(59) },
      Bus { id: Some(61) },
    ];

    let sched = Schedule {
      earliest_departure: 0,
      bus_lines: buses,
    };
    let id_offsets = sched
      .bus_lines
      .iter()
      .enumerate()
      .filter_map(|(i, bus)| bus.id.map(|id| ((id as u64, i as u64))))
      .collect::<Vec<_>>();
    assert!(is_solution(&id_offsets, &754018));
    let result = super::earliest_timestamp(sched);
    assert_eq!(754018, result);
  }
  #[test]
  fn test_3() {
    let buses = vec![
      Bus { id: Some(67) },
      Bus { id: None },
      Bus { id: Some(7) },
      Bus { id: Some(59) },
      Bus { id: Some(61) },
    ];
    let sched = Schedule {
      earliest_departure: 0,
      bus_lines: buses,
    };

    let id_offsets = sched
      .bus_lines
      .iter()
      .enumerate()
      .filter_map(|(i, bus)| bus.id.map(|id| ((id as u64, i as u64))))
      .collect::<Vec<_>>();
    assert!(is_solution(&id_offsets, &779210));
    let result = super::earliest_timestamp(sched);
    assert_eq!(779210, result);
  }
  #[test]
  fn test_4() {
    let buses = vec![
      Bus { id: Some(67) },
      Bus { id: Some(7) },
      Bus { id: None },
      Bus { id: Some(59) },
      Bus { id: Some(61) },
    ];
    let sched = Schedule {
      earliest_departure: 0,
      bus_lines: buses,
    };
    let id_offsets = sched
      .bus_lines
      .iter()
      .enumerate()
      .filter_map(|(i, bus)| bus.id.map(|id| ((id as u64, i as u64))))
      .collect::<Vec<_>>();
    assert!(is_solution(&id_offsets, &1261476));
    let result = super::earliest_timestamp(sched);
    assert_eq!(1261476, result);
  }
  #[test]
  fn test_5() {
    let buses = vec![
      Bus { id: Some(1789) },
      Bus { id: Some(37) },
      Bus { id: Some(47) },
      Bus { id: Some(1889) },
    ];
    let sched = Schedule {
      earliest_departure: 0,
      bus_lines: buses,
    };
    let id_offsets = sched
      .bus_lines
      .iter()
      .enumerate()
      .filter_map(|(i, bus)| bus.id.map(|id| ((id as u64, i as u64))))
      .collect::<Vec<_>>();
    assert!(is_solution(&id_offsets, &1202161486));
    let result = super::earliest_timestamp(sched);
    assert_eq!(1202161486, result);
  }
}
