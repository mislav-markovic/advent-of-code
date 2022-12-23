use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use itertools::Itertools;

use crate::day_exec::DayExecutor;

pub struct Day12;
impl DayExecutor for Day12 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        todo!()
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        todo!()
    }
}

fn solve_part1(input: &str) -> u32 {
    let map = get_map_from_input(input);
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

    res.unwrap()
}

fn get_map_from_input(input: &str) -> Map {
    todo!()
}

fn manhattan_distance(p: &Position, q: &Position) -> u32 {
    (p.x.abs_diff(q.x) + p.y.abs_diff(q.y)) as u32
}

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

            let valid_neighbours = (pos.x.saturating_sub(1)
                ..=(pos.x.saturating_add(1)).clamp(0, width - 1))
                .cartesian_product(
                    pos.y.saturating_sub(1)..=(pos.y.saturating_add(1)).clamp(0, height),
                )
                .map(|(x, y)| Position::new(x, y))
                .filter(|pos| self.elevation_at(pos) <= my_elevation.inc(1));

            res.extend(valid_neighbours);
        }

        res
    }

    fn elevation_at(&self, pos: &Position) -> Elevation {
        self.elevations[pos.y][pos.x]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Elevation(u8);

impl Elevation {
    fn inc(&self, by: u8) -> Self {
        Self(self.0.saturating_add(by))
    }
}

struct ElevationParseError;
impl FromStr for Elevation {
    type Err = ElevationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
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
