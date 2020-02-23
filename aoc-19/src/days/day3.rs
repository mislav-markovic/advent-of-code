use crate::days::*;
use crate::input_reader::{read_sparated_values_from_input, ParseError, Parser};

enum Direction {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct CircuitBoard {
    wires: Vec<Wire>,
}

impl Parser for CircuitBoard {
    type R = Wire;
    fn parse_line(line: &str) -> Result<Self::R, ParseError> {
        let directions = line
            .split(",")
            .map(|x| direction_from_str(x))
            .collect::<Vec<_>>();
        Ok(Wire::new(directions))
    }
}

fn direction_from_str(string: &str) -> Direction {
    let value = string[1..]
        .parse::<usize>()
        .expect("Could not parse direction value");
    if string.starts_with("R") {
        Direction::Right(value)
    } else if string.starts_with("L") {
        Direction::Left(value)
    } else if string.starts_with("U") {
        Direction::Up(value)
    } else {
        Direction::Down(value)
    }
}

impl CircuitBoard {
    fn new(wires: Vec<Wire>) -> Self {
        Self { wires }
    }

    fn parsed(text: &[&str]) -> Self {
        let result = Self::parse_all(text)
            .into_iter()
            .map(|elem| elem.expect("Could not parse!"))
            .collect();

        Self::new(result)
    }

    fn closest_wire_intersection(&self) -> usize {
        let intersections =
            wire_intersections(self.wires.get(0).unwrap(), self.wires.get(1).unwrap()).unwrap();

        let result = closest_intersection_distance(&Point::new(0, 0), intersections.as_slice());
        result
    }
    fn closest_wire_intersection_by_steps(&self) -> usize {
        let intersections =
            wire_intersections(self.wires.get(0).unwrap(), self.wires.get(1).unwrap()).unwrap();

        let result = closest_intersection_step_distance(
            self.wires.get(0).unwrap().path_in_points().as_slice(),
            self.wires.get(1).unwrap().path_in_points().as_slice(),
            intersections.as_slice(),
        );
        result
    }
}

fn wire_intersections(wire_a: &Wire, wire_b: &Wire) -> Option<Vec<Point>> {
    let wire_a_in_points = wire_a.path_in_points();
    let wire_b_in_points = wire_b.path_in_points();
    let mut result: Vec<Point> = vec![];

    for point in wire_a_in_points {
        if wire_b_in_points.contains(&point) {
            result.push(point);
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn closest_intersection_distance(origin: &Point, intersections: &[Point]) -> usize {
    let mut closest_distance: Option<usize> = None;

    for point in intersections {
        let distance = ((origin.x - point.x).abs() + (origin.y - point.y).abs()) as usize;
        match closest_distance {
            None => closest_distance = Some(distance),
            Some(dist) => {
                if dist > distance {
                    closest_distance = Some(distance)
                }
            }
        }
    }

    closest_distance.unwrap()
}

fn closest_intersection_step_distance(
    wire_a_in_points: &[Point],
    wire_b_in_points: &[Point],
    intersections: &[Point],
) -> usize {
    let mut closest_distance: Option<usize> = None;

    for point in intersections {
        let distance = (wire_a_in_points.iter().position(|x| x == point).unwrap()
            + 1
            + wire_b_in_points.iter().position(|x| x == point).unwrap()
            + 1) as usize;
        match closest_distance {
            None => closest_distance = Some(distance),
            Some(dist) => {
                if dist > distance {
                    closest_distance = Some(distance)
                }
            }
        }
    }

    closest_distance.unwrap()
}

struct Wire {
    path: Vec<Direction>,
}

impl Wire {
    fn new(path: Vec<Direction>) -> Self {
        Self { path }
    }

    // assumes central port is (0, 0)
    fn path_in_points(&self) -> Vec<Point> {
        let mut current_point = Point::new(0, 0);
        let mut result = Vec::<Point>::new();
        for dir in self.path.iter() {
            match dir {
                Direction::Right(value) => {
                    (current_point.x + 1..=current_point.x + (*value as i32))
                        .for_each(|x| result.push(Point::new(x, current_point.y)));
                    current_point.x += *value as i32;
                }
                Direction::Left(value) => {
                    (current_point.x - (*value as i32)..=current_point.x - 1)
                        .for_each(|x| result.push(Point::new(x, current_point.y)));
                    current_point.x -= *value as i32;
                }
                Direction::Up(value) => {
                    (current_point.y + 1..=current_point.y + (*value as i32))
                        .for_each(|y| result.push(Point::new(current_point.x, y)));
                    current_point.y += *value as i32;
                }
                Direction::Down(value) => {
                    (current_point.y - (*value as i32)..=current_point.y - 1)
                        .for_each(|y| result.push(Point::new(current_point.x, y)));
                    current_point.y -= *value as i32;
                }
            };
        }
        result
    }
}

pub struct Day3Runner {
    path: String,
    part: Parts,
}

impl Day3Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let board = self.load();
        board.closest_wire_intersection()
    }
    fn part2(&self) -> usize {
        let board = self.load();
        board.closest_wire_intersection_by_steps()
    }
    fn load(&self) -> CircuitBoard {
        let text = read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        CircuitBoard::parsed(
            text.expect("Could not read wires")
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .as_ref(),
        )
    }
}

impl Runner for Day3Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
