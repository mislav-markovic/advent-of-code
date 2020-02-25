use crate::days::{Parts, Runner};
use std::fmt;
enum Color {
    Transperent,
    Black,
    White,
}

impl Color {
    fn from_val(val: usize) -> Option<Self> {
        match val {
            2 => Some(Self::Transperent),
            1 => Some(Self::White),
            0 => Some(Self::Black),
            _ => None,
        }
    }

    fn to_display(&self) -> String {
        match self {
            Color::Transperent => " ".to_string(),
            Color::White => "□".to_string(),
            Color::Black => "■".to_string(),
        }
    }
}

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(layers: Vec<Layer>, width: usize, height: usize) -> Self {
        Self {
            layers,
            width,
            height,
        }
    }
    fn parsed(text: &str, width: usize, height: usize) -> Self {
        let mut layers: Vec<Layer> = vec![];
        let mut position = 0usize;

        while position < text.len() {
            let mut layer = Layer::new(vec![]);
            for _ in 0..height {
                layer.push_row(create_row(&text[position..position + width]));
                position += width;
            }
            layers.push(layer);
        }

        Self::new(layers, width, height)
    }

    fn checksum(&self) -> usize {
        let layer = self.layers.iter().min_by_key(|l| l.count_digit(0)).unwrap();
        layer.count_digit(1) * layer.count_digit(2)
    }

    fn pixel_color(&self, x: usize, y: usize) -> Color {
        self.layers
            .iter()
            .find_map(|layer| match Color::from_val(layer.at(x, y)).unwrap() {
                Color::Transperent => None,
                Color::White => Some(Color::White),
                Color::Black => Some(Color::Black),
            })
            .unwrap()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for h in 0..self.height {
            for w in 0..self.width {
                let _ = write!(f, "{}", self.pixel_color(w, h).to_display());
            }
            let _ = writeln!(f, "");
        }

        Ok(())
    }
}

fn create_row(text: &str) -> Vec<usize> {
    text.chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

struct Layer {
    // top down, index 0 is topmost row
    rows: Vec<Vec<usize>>,
}

impl Layer {
    fn new(rows: Vec<Vec<usize>>) -> Self {
        Self { rows }
    }

    fn push_row(&mut self, row: Vec<usize>) {
        self.rows.push(row);
    }

    fn count_digit(&self, digit: usize) -> usize {
        self.rows.iter().flatten().filter(|&&x| x == digit).count()
    }

    fn at(&self, x: usize, y: usize) -> usize {
        self.rows[y][x]
    }
}

pub struct Day8Runner {
    path: String,
    part: Parts,
}

impl Day8Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let img = self.load();
        img.checksum()
    }
    fn part2(&self) -> usize {
        let img = self.load();
        println!("{}", img);
        0
    }

    fn load(&self) -> Image {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        Image::parsed(&text.expect("Could not read instructions")[0], 25, 6)
    }
}

impl Runner for Day8Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}
