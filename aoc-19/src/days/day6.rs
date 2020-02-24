use crate::days::*;
use std::collections::HashMap;

struct Map {
    objects: HashMap<String, Object>,
}

impl Map {
    fn new() -> Self {
        let objects = HashMap::new();
        Self { objects }
    }

    fn parsed(text: &[String]) -> Self {
        let mut map = Map::new();

        text.iter().for_each(|line| {
            let vec = line.split(")").collect::<Vec<_>>();
            map.add_orbit(vec[0], vec[1]);
        });

        map
    }

    fn add_orbit(&mut self, center: &str, satellite: &str) {
        let center_object = self
            .objects
            .entry(center.to_string())
            .or_insert(Object::new(center.to_string(), None));
        center_object.satellites.push(satellite.to_string());

        let sat = self
            .objects
            .entry(satellite.to_string())
            .or_insert(Object::new(satellite.to_string(), None));
        sat.in_orbit_of = Some(center.to_string());
    }

    fn direct_and_indirect_orbits(&self) -> usize {
        self.objects
            .iter()
            .fold(0, |acc, (k, _)| acc + self.objects_indirect_orbits_count(k))
    }

    fn objects_indirect_orbits_count(&self, object_name: &str) -> usize {
        let mut current_object = self.objects.get(object_name).unwrap();
        let mut counter = 0;
        while let Some(center_name) = &current_object.in_orbit_of {
            counter += 1;
            current_object = self.objects.get(center_name).unwrap();
        }
        counter
    }

    fn orbit_transfers_between_objects(&self, start: &str, end: &str) -> usize {
        let mut visited = Vec::<&str>::new();
        let mut next = Vec::<(&str, usize)>::new();
        let is_end = |object: &str| -> bool { object == end };

        visited.push(start);
        self.objects
            .get(start)
            .unwrap()
            .satellites
            .iter()
            .for_each(|x| next.push((x, 1)));
        next.push((
            self.objects
                .get(start)
                .as_ref()
                .unwrap()
                .in_orbit_of
                .as_ref()
                .unwrap(),
            1,
        ));

        let mut result = None;
        loop {
            let mut next_iter = Vec::<(&str, usize)>::new();
            if next.is_empty() {
                break;
            }
            for obj in next.drain(..) {
                visited.push(obj.0);
                if is_end(obj.0) {
                    if let Some(prev) = result {
                        result = if prev < obj.1 {
                            Some(prev)
                        } else {
                            Some(obj.1)
                        }
                    } else {
                        result = Some(obj.1)
                    }
                } else {
                    self.objects
                        .get(obj.0)
                        .unwrap()
                        .satellites
                        .iter()
                        .for_each(|x| next_iter.push((x, obj.1 + 1)));

                    if let Some(center) = self.objects.get(obj.0).unwrap().in_orbit_of.as_ref() {
                        next_iter.push((center, obj.1 + 1));
                    }
                }
            }
            next = next_iter
                .drain_filter(|x| !visited.contains(&x.0))
                .collect();
        }

        result.unwrap()
    }
}

#[derive(Debug)]
struct Object {
    name: String,
    in_orbit_of: Option<String>,
    satellites: Vec<String>,
}

impl Object {
    fn new(name: String, in_orbit_of: Option<String>) -> Self {
        Self {
            name,
            in_orbit_of,
            satellites: vec![],
        }
    }
}

pub struct Day6Runner {
    path: String,
    part: Parts,
}

impl Day6Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let map = self.load();
        map.direct_and_indirect_orbits()
    }
    fn part2(&self) -> usize {
        let map = self.load();
        let start = map
            .objects
            .get("YOU")
            .unwrap()
            .in_orbit_of
            .as_ref()
            .unwrap();
        let end = map
            .objects
            .get("SAN")
            .unwrap()
            .in_orbit_of
            .as_ref()
            .unwrap();
        map.orbit_transfers_between_objects(start, end)
    }

    fn load(&self) -> Map {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n")
            .expect("Could not read orbits");
        Map::parsed(&text)
    }
}

impl Runner for Day6Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
