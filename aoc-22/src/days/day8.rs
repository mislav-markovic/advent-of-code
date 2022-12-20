use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day8;

impl DayExecutor for Day8 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Visible trees in forest: {}", solve_part1(&input)))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!("Highest scenic score is: {}", solve_part2(&input)))
    }
}

fn solve_part1(input: &str) -> usize {
    let forest = input
        .parse::<Forest>()
        .expect("Failed to parse input as forest");
    forest.count_visible()
}

fn solve_part2(input: &str) -> usize {
    let forest = input
        .parse::<Forest>()
        .expect("Failed to parse input as forest");
    forest.most_scenic_view()
}

struct ViewingDistance {
    view_left: usize,
    view_right: usize,
    view_up: usize,
    view_down: usize,
}

impl ViewingDistance {
    fn new() -> Self {
        Self {
            view_left: 0,
            view_right: 0,
            view_up: 0,
            view_down: 0,
        }
    }

    fn scenic_score(&self) -> usize {
        self.view_down * self.view_left * self.view_right * self.view_up
    }

    fn extend_by(&mut self, in_direction: Direction, count: usize) {
        match in_direction {
            Direction::Up => self.view_up += count,
            Direction::Down => self.view_down += count,
            Direction::Left => self.view_left += count,
            Direction::Right => self.view_right += count,
        }
    }

    fn exted_view(&mut self, in_direction: Direction) {
        self.extend_by(in_direction, 1);
    }
}

struct Tree {
    height: usize,
    blocked_left: bool,
    blocked_right: bool,
    blocked_up: bool,
    blocked_down: bool,
    view: ViewingDistance,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            blocked_left: false,
            blocked_right: false,
            blocked_up: false,
            blocked_down: false,
            view: ViewingDistance::new(),
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

    fn most_scenic_view(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.0.iter())
            .map(|t| t.view.scenic_score())
            .max()
            .unwrap()
    }
}

fn update_visibilitiy_within_column(rows: &mut Vec<TreeRow>, column_idx: usize) {
    for row_idx in 1..rows.len() - 1 {
        let tree_height = rows[row_idx].0[column_idx].height;

        // check up
        let mut block_up = false;
        let mut trees_seen = 0;
        for potential_blocker in rows[0..row_idx].iter().rev().map(|row| &row.0[column_idx]) {
            trees_seen += 1;
            if tree_height <= potential_blocker.height {
                block_up = true;
                break;
            }
        }

        let tree = &mut rows[row_idx].0[column_idx];
        tree.view.extend_by(Direction::Up, trees_seen);
        if block_up {
            tree.block_direction(Direction::Up);
        }

        // check down
        let mut block_down = false;
        let mut trees_seen = 0;
        for potential_blocker in rows[row_idx + 1..].iter().map(|row| &row.0[column_idx]) {
            trees_seen += 1;
            if tree_height <= potential_blocker.height {
                block_down = true;
                break;
            }
        }

        let tree = &mut rows[row_idx].0[column_idx];
        tree.view.extend_by(Direction::Down, trees_seen);
        if block_down {
            tree.block_direction(Direction::Down);
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
            tree.view.exted_view(Direction::Left);
            if tree.height <= potential_blocker.height {
                tree.block_direction(Direction::Left);
                break;
            }
        }

        // check if something blocks us from right
        let (left, right) = row.split_at_mut(column_idx + 1);
        let tree = left.last_mut().unwrap();

        for potential_blocker in right.iter() {
            tree.view.exted_view(Direction::Right);
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
