use crate::day_exec::DayExecutor;

use std::{collections::HashSet, str::FromStr};

pub struct Day9;

impl DayExecutor for Day9 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Number of unique positions visited by tail: {}",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Number of unique positions visited by tail: {}",
            solve_part2(&input)
        ))
    }
}

fn solve_part1(input: &str) -> usize {
    let instr_list = get_instructions(input);
    let mut rope = Rope::new(2);
    let mut unique_positions_visited_by_tail: HashSet<Position> = HashSet::new();

    for instr in instr_list.into_iter() {
        unique_positions_visited_by_tail.extend(rope.move_head(&instr).into_iter())
    }

    unique_positions_visited_by_tail.len()
}

fn solve_part2(input: &str) -> usize {
    let instr_list = get_instructions(input);
    let mut rope = Rope::new(10);
    let mut unique_positions_visited_by_tail: HashSet<Position> = HashSet::new();

    for instr in instr_list.into_iter() {
        unique_positions_visited_by_tail.extend(rope.move_head(&instr).into_iter())
    }

    unique_positions_visited_by_tail.len()
}

fn get_instructions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| l.trim().parse::<Direction>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse instruction list")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn origin() -> Self {
        Self::new(0, 0)
    }

    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_touching(&self, other: &Self) -> bool {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);

        // (1, 1) -> points touch on diagonal
        // (1,0), (0,1) -> points touch on one of 4 cardinal directions
        // (0,0) -> points overlap
        x_dist <= 1 && y_dist <= 1
    }

    fn step_towards(&self, other: &Self) -> Step {
        Step::new(other.x - self.x, other.y - self.y)
    }

    fn single_step_towards(&self, other: &Self) -> Step {
        let step = self.step_towards(other);
        Step::new(step.x_delta.clamp(-1, 1), step.y_delta.clamp(-1, 1))
    }

    fn step(&self, step: &Step) -> Self {
        Self::new(self.x + step.x_delta, self.y + step.y_delta)
    }

    fn step_mut(&mut self, step: &Step) {
        self.x += step.x_delta;
        self.y += step.y_delta;
    }
}

struct Step {
    x_delta: isize,
    y_delta: isize,
}

impl Step {
    fn stay_inplace() -> Self {
        Self::new(0, 0)
    }

    fn single_right() -> Self {
        Self::new(1, 0)
    }

    fn single_left() -> Self {
        Self::new(-1, 0)
    }

    fn single_up() -> Self {
        Self::new(0, 1)
    }

    fn single_down() -> Self {
        Self::new(0, -1)
    }

    fn new(x_delta: isize, y_delta: isize) -> Self {
        Self { x_delta, y_delta }
    }
}

enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Direction {
    fn step_count(&self) -> usize {
        match self {
            Direction::Up(s) => *s,
            Direction::Down(s) => *s,
            Direction::Left(s) => *s,
            Direction::Right(s) => *s,
        }
    }
}

#[derive(Debug)]
struct DirectionParseError;
impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, step_count_str) = s.split_once(" ").ok_or(DirectionParseError)?;

        let step_count = step_count_str
            .parse::<usize>()
            .map_err(|_| DirectionParseError)?;

        match direction_str {
            "R" => Ok(Direction::Right(step_count)),
            "L" => Ok(Direction::Left(step_count)),
            "U" => Ok(Direction::Up(step_count)),
            "D" => Ok(Direction::Down(step_count)),
            _ => Err(DirectionParseError),
        }
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        Self {
            knots: vec![Position::origin(); knot_count],
        }
    }

    fn move_head(&mut self, dir: &Direction) -> Vec<Position> {
        let directional_step = single_step_in_direction(dir);

        let mut tail_visited_positions: Vec<Position> = Vec::new();
        for _ in 0..dir.step_count() {
            self.head().step_mut(&directional_step);

            let (tail, mut knots) = self.knots.split_last_mut().unwrap();
            tail_visited_positions.push(tail.clone());

            let knots_before_tail_count = knots.len();
            for _ in 1..knots_before_tail_count {
                let (leading_knot, following_knots) = knots.split_first_mut().unwrap();
                let following_knot = following_knots.first_mut().unwrap();

                if !following_knot.is_touching(&leading_knot) {
                    let chasing_step = following_knot.single_step_towards(&leading_knot);
                    following_knot.step_mut(&chasing_step);
                }

                knots = following_knots;
            }

            let second_to_last = knots.last_mut().unwrap();

            if !tail.is_touching(&second_to_last) {
                let tail_chasing_step = tail.single_step_towards(&second_to_last);
                let new_tail = tail.step(&tail_chasing_step);
                tail_visited_positions.push(new_tail);
                tail.step_mut(&tail_chasing_step);
            }
        }

        tail_visited_positions
    }

    fn head(&mut self) -> &mut Position {
        self.knots.first_mut().unwrap()
    }

    fn tail(&mut self) -> &mut Position {
        self.knots.last_mut().unwrap()
    }
}

fn single_step_in_direction(dir: &Direction) -> Step {
    match dir {
        Direction::Up(_) => Step::single_up(),
        Direction::Down(_) => Step::single_down(),
        Direction::Left(_) => Step::single_left(),
        Direction::Right(_) => Step::single_right(),
    }
}
