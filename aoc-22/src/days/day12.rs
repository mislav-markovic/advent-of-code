use itertools::Itertools;
use std::sync::mpsc::channel;
use std::thread;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use crate::day_exec::DayExecutor;

pub struct Day12;
impl DayExecutor for Day12 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Steps to reach goal: {}", solve_part1(&input)))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Shortest path from any lowest elevation point to goal: {}",
            solve_part2(&input)
        ))
    }
}

fn solve_part1(input: &str) -> u32 {
    let map = get_map_from_input(input);
    get_shortest_path_for_map(&map)
}

fn get_shortest_path_for_map(map: &Map) -> u32 {
    let edge_weight = |_: Position, _: Position| -> u32 { 1 };
    let goal = map.destination_pos.clone();
    let heuristic = |current| manhattan_distance(&current, &goal);
    let get_neighbours = |current: Position| map.get_valid_neighbours(&current);

    let res = a_star(
        map.start_pos.clone(),
        map.destination_pos.clone(),
        get_neighbours,
        heuristic,
        edge_weight,
    );

    res.unwrap_or(u32::MAX)
}

fn solve_part2(input: &str) -> u32 {
    let multi_map = get_multi_map(input);
    let map_count = multi_map.len();

    let (tx, rx) = channel();
    thread::scope(|scope| {
        for map in multi_map {
            let tx_clone = tx.clone();
            scope.spawn(move || tx_clone.send(get_shortest_path_for_map(&map)));
        }
    });

    let mut res_vec: Vec<u32> = Vec::with_capacity(map_count);
    let mut recv_count = 0;

    while recv_count < map_count {
        res_vec.push(rx.recv().unwrap());
        recv_count += 1;
    }

    res_vec.into_iter().min().unwrap()
}

fn get_multi_map(input: &str) -> Vec<Map> {
    let map = get_map_from_input(input);

    let width = map.elevations[0].len();
    let height = map.elevations.len();

    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| Position::new(x, y))
        .filter(|pos| map.elevation_at(pos).0 == 0)
        .map(|new_start| {
            let mut cloned = map.clone();
            cloned.start_pos = new_start;
            cloned
        })
        .collect()
}

fn print_map(map: &Map) {
    let height = map.elevations.len();

    for y in 0..height {
        let width = map.elevations[y].len();
        for x in 0..width {
            let current = Position::new(x, y);

            if current == map.start_pos {
                print!(" S ");
            } else if current == map.destination_pos {
                print!(" E ");
            } else {
                print!(" {} ", map.elevation_at(&current).0);
            }
        }
        println!("\t: {}", width);
    }
}

fn get_map_from_input(input: &str) -> Map {
    input
        .parse::<Map>()
        .expect("Failed to parse input into Map type")
}

fn manhattan_distance(p: &Position, q: &Position) -> u32 {
    (p.x.abs_diff(q.x) + p.y.abs_diff(q.y)) as u32
}

#[derive(Clone)]
struct Map {
    start_pos: Position,
    destination_pos: Position,
    elevations: Vec<Vec<Elevation>>,
}

impl Map {
    fn get_valid_neighbours(&self, pos: &Position) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let width = self.elevations[0].len();
        let height = self.elevations.len();

        if pos.x < width && pos.y < height {
            let my_elevation = self.elevation_at(&pos);

            let neighbour_candidates = [
                (pos.x.saturating_sub(1), pos.y),
                (pos.x.saturating_add(1).clamp(0, width - 1), pos.y),
                (pos.x, pos.y.saturating_sub(1)),
                (pos.x, pos.y.saturating_add(1).clamp(0, height - 1)),
            ];

            neighbour_candidates
                .into_iter()
                .map(|(x, y)| Position::new(x, y))
                .unique()
                .filter(|p| p != pos)
                .filter(|p| self.elevation_at(p) <= my_elevation.inc(1))
                .collect::<Vec<_>>()
        } else {
            Vec::default()
        }
    }

    fn elevation_at(&self, pos: &Position) -> Elevation {
        self.elevations
            .get(pos.y)
            .unwrap()
            .get(pos.x)
            .map(|e| *e)
            .unwrap()
    }
}

#[derive(Debug)]
struct MapParseErorr;
impl FromStr for Map {
    type Err = MapParseErorr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const START: char = 'S';
        const GOAL: char = 'E';
        const START_MAPPING: char = 'a';
        const GOAL_MAPPING: char = 'z';

        let mut start_pos: Option<Position> = None;
        let mut end_pos: Option<Position> = None;

        let mut elevations: Vec<Vec<Elevation>> = Vec::new();

        for (y, l) in s.trim().lines().enumerate() {
            let mut row: Vec<Elevation> = Vec::with_capacity(l.len());
            for (x, mut c) in l.trim().chars().enumerate() {
                let pos = Position::new(x, y);

                if c == START {
                    start_pos = Some(pos);
                    c = START_MAPPING;
                } else if c == GOAL {
                    end_pos = Some(pos);
                    c = GOAL_MAPPING;
                }

                let elevation: Elevation = c.try_into().map_err(|_| MapParseErorr)?;
                row.push(elevation);
            }

            elevations.push(row);
        }

        if !elevations.iter().map(|e| e.len()).all_equal() {
            return Err(MapParseErorr);
        }

        Ok(Self {
            start_pos: start_pos.unwrap(),
            destination_pos: end_pos.unwrap(),
            elevations,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Elevation(u8);

impl Elevation {
    fn inc(&self, by: u8) -> Self {
        Self(self.0.saturating_add(by))
    }
}

impl TryFrom<char> for Elevation {
    type Error = ElevationParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const LOWEST: u8 = b'a';
        const HEIGHEST: u8 = b'z';

        if let Ok(val_as_byte) = value.try_into() {
            match val_as_byte {
                LOWEST..=HEIGHEST => Ok(Self(val_as_byte - LOWEST)),
                _ => Err(ElevationParseError),
            }
        } else {
            Err(ElevationParseError)
        }
    }
}

impl From<u8> for Elevation {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

struct ElevationParseError;
impl FromStr for Elevation {
    type Err = ElevationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.len() > 1 {
            return Err(ElevationParseError);
        }

        let c = trimmed.chars().next().ok_or(ElevationParseError)?;

        c.try_into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type Score = u32;
struct Node {
    pos: Position,
    score: Score,
}

impl Node {
    fn new(pos: Position) -> Self {
        Self {
            pos,
            score: Score::MAX,
        }
    }

    fn with_score(pos: Position, score: Score) -> Self {
        Self { pos, score }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}

fn a_star(
    start: Position,
    end: Position,
    get_neighbours: impl Fn(Position) -> Vec<Position>,
    heuristic: impl Fn(Position) -> u32,
    edge_weight: impl Fn(Position, Position) -> u32,
) -> Option<u32> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse(Node::with_score(start, (heuristic)(start))));

    let mut goal_score = HashMap::new();
    goal_score.insert(start, 0u32);

    while let Some(Reverse {
        0: Node { pos: current, .. },
    }) = open_set.pop()
    {
        if current == end {
            return Some(goal_score[&current]);
        }

        for neighbour in (get_neighbours)(current) {
            let tentative_score = goal_score[&current] + (edge_weight)(current, neighbour);

            let current_neighbour_score = goal_score.entry(neighbour).or_insert(Score::MAX);
            if tentative_score < *current_neighbour_score {
                *current_neighbour_score = tentative_score;

                if !open_set.iter().any(|n| n.0.pos == neighbour) {
                    let f_score = tentative_score + (heuristic)(neighbour);
                    open_set.push(Reverse(Node::with_score(neighbour, f_score)))
                }
            }
        }
    }

    None
}
