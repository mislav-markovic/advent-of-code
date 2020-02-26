use crate::days::*;
use day5::{Intcode, ProgramResult};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    fn from(val: isize) -> Self {
        match val {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!(),
        }
    }

    fn to_num(&self) -> isize {
        match self {
            Self::White => 1,
            Self::Black => 0,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate(&self, rotate: Rotate) -> Self {
        match self {
            Direction::Left => match rotate {
                Rotate::Left => Self::Down,
                Rotate::Right => Self::Up,
            },
            Direction::Right => match rotate {
                Rotate::Left => Self::Up,
                Rotate::Right => Self::Down,
            },
            Direction::Up => match rotate {
                Rotate::Left => Self::Left,
                Rotate::Right => Self::Right,
            },
            Direction::Down => match rotate {
                Rotate::Left => Self::Right,
                Rotate::Right => Self::Left,
            },
        }
    }
}

enum Rotate {
    Left,
    Right,
}

impl Rotate {
    fn from(val: isize) -> Self {
        match val {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!(),
        }
    }
}

struct Robot {
    brain: Intcode,
    map: HashMap<(isize, isize), Color>,
    direction: Direction,
    position: (isize, isize),
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top_left, bottom_right) = map_dimensions(self);

        for h in (bottom_right.1..=top_left.1).rev() {
            for w in top_left.0..=bottom_right.0 {
                let _ = write!(f, "{}", color_to_str(&self.color_at((w, h))));
            }
            let _ = writeln!(f, "");
        }

        Ok(())
    }
}

// returns pair of points, top left corner and bottom right corner ((min_x, max_y), (max_x, min_y))
fn map_dimensions(robot: &Robot) -> ((isize, isize), (isize, isize)) {
    let max_x = *robot.map.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *robot.map.keys().map(|(x, _)| x).min().unwrap();
    let max_y = *robot.map.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *robot.map.keys().map(|(_, y)| y).min().unwrap();

    ((min_x, max_y), (max_x, min_y))
}

fn color_to_str(color: &Color) -> String {
    match color {
        Color::White => "⬜".to_string(),
        Color::Black => "⬛".to_string(),
    }
}

impl Robot {
    fn new(brain: Intcode, starting_panel_color: Color) -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), starting_panel_color);
        Self {
            brain,
            map,
            direction: Direction::Up,
            position: (0, 0),
        }
    }

    // returns number of cells colored at least once
    fn run(&mut self) -> usize {
        let mut counter = 0usize;
        loop {
            match self.brain.run_program() {
                ProgramResult::Halt => {
                    break;
                }
                ProgramResult::OutputPause(Some(val)) => {
                    let is_first_paint = !self.paint(self.position, Color::from(val));
                    if is_first_paint {
                        counter += 1;
                    }
                    if let ProgramResult::OutputPause(Some(rot)) = self.brain.run_program() {
                        self.rotate(Rotate::from(rot));
                        self.move_robot();
                    } else {
                        panic!("Unexpected program result, expected output of rotation direction")
                    }
                }
                ProgramResult::InputPause => {
                    let input = self.color_at(self.position).to_num();
                    self.brain.add_inputs(&[input])
                }
                ProgramResult::OutputPause(None) => panic!("Unexpected output without value"),
            }
        }
        counter
    }

    fn color_at(&self, index: (isize, isize)) -> Color {
        if self.map.contains_key(&index) {
            *self.map.get(&index).unwrap()
        } else {
            Color::Black
        }
    }

    // returns true if position was already colored
    fn paint(&mut self, index: (isize, isize), color: Color) -> bool {
        let result = self.map.contains_key(&index);

        *self.map.entry(index).or_insert(Color::Black) = color;
        result
    }

    fn move_robot(&mut self) {
        self.position = match self.direction {
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
            Direction::Up => (self.position.0, self.position.1 + 1),
            Direction::Down => (self.position.0, self.position.1 - 1),
        }
    }

    fn rotate(&mut self, rotation: Rotate) {
        self.direction = self.direction.rotate(rotation);
    }
}

pub struct Day11Runner {
    path: String,
    part: Parts,
}

impl Day11Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let intcode = self.load();
        let mut robot = Robot::new(intcode, Color::Black);
        robot.run()
    }
    fn part2(&self) -> usize {
        let intcode = self.load();
        let mut robot = Robot::new(intcode, Color::White);
        let res = robot.run();
        println!("{}", robot);
        res
    }

    fn load(&self) -> Intcode {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        Intcode::parsed(&text.expect("Could not read instructions")[0], &[], true)
    }
}

impl Runner for Day11Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
