use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day8;

impl DayExecutor for Day8 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Visible trees in forest: {}", solve_part1(&input)))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new("TODO")
    }
}

fn solve_part1(input: &str) -> usize {
    let forest = input
        .parse::<Forest>()
        .expect("Failed to parse input as forest");
    forest.count_visible()
}

struct Tree {
    height: usize,
    blocked_left: bool,
    blocked_right: bool,
    blocked_up: bool,
    blocked_down: bool,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            blocked_left: false,
            blocked_right: false,
            blocked_up: false,
            blocked_down: false,
        }
    }

    fn is_visible(&self) -> bool {
        !self.blocked_down || !self.blocked_left || !self.blocked_right || !self.blocked_up
    }

    fn block_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.blocked_up = true,
            Direction::Down => self.blocked_down = true,
            Direction::Left => self.blocked_left = true,
            Direction::Right => self.blocked_right = true,
        }
    }
}

struct TreeParseError(String);
impl FromStr for Tree {
    type Err = TreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(TreeParseError(s.to_owned()))
        } else {
            s.parse::<usize>()
                .map(|height| Self::new(height))
                .map_err(|parse_err| TreeParseError(parse_err.to_string()))
        }
    }
}

struct Forest {
    rows: Vec<TreeRow>,
}

impl Forest {
    fn new(mut rows: Vec<TreeRow>) -> Self {
        for row in rows.iter_mut() {
            update_visibility_within_row(row)
        }

        for column in 0..rows.len() {
            update_visibilitiy_within_column(&mut rows, column);
        }

        Self { rows }
    }

    fn count_visible(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.0.iter())
            .filter(|tree| tree.is_visible())
            .count()
    }
}

fn update_visibilitiy_within_column(rows: &mut Vec<TreeRow>, column_idx: usize) {
    for row_idx in 1..rows.len() - 1 {
        let tree_height = rows[row_idx].0[column_idx].height;

        // check up
        let mut block_up = false;
        for potential_blocker in rows[0..row_idx].iter().rev().map(|row| &row.0[column_idx]) {
            if tree_height <= potential_blocker.height {
                block_up = true;
                break;
            }
        }

        if block_up {
            rows[row_idx].0[column_idx].block_direction(Direction::Up);
        }

        // check down
        let mut block_down = false;
        for potential_blocker in rows[row_idx + 1..].iter().map(|row| &row.0[column_idx]) {
            if tree_height <= potential_blocker.height {
                block_down = true;
                break;
            }
        }

        if block_down {
            rows[row_idx].0[column_idx].block_direction(Direction::Down);
        }
    }
}

fn update_visibility_within_row(row: &mut TreeRow) {
    let row = &mut row.0;

    /* exclude first and last item */
    for column_idx in 1..row.len() - 1 {
        // check if something blocks us from left direction
        let (left, right) = row.split_at_mut(column_idx);
        let tree = right.first_mut().unwrap();

        for potential_blocker in left.iter().rev() {
            if tree.height <= potential_blocker.height {
                tree.block_direction(Direction::Left);
                break;
            }
        }

        // check if something blocks us from right
        let (left, right) = row.split_at_mut(column_idx + 1);
        let tree = left.last_mut().unwrap();

        for potential_blocker in right.iter() {
            if tree.height <= potential_blocker.height {
                tree.block_direction(Direction::Right);
                break;
            }
        }
    }
}

#[derive(Debug)]
struct ForestParseError(String);
impl FromStr for Forest {
    type Err = ForestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|l| l.trim())
            .map(<TreeRow as FromStr>::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(|rows| Self::new(rows))
            .map_err(|err| ForestParseError(err.0))
    }
}

struct TreeRow(Vec<Tree>);
struct TreeRowParseError(String);
impl FromStr for TreeRow {
    type Err = TreeRowParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| c.to_string().parse::<Tree>())
            .collect::<Result<Vec<Tree>, _>>()
            .map(|v| TreeRow(v))
            .map_err(|err| TreeRowParseError(format!("Line: {}; Err: {}", s, err.0)))
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
