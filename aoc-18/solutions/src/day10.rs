use crate::input_reader;
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

type PointT = (isize, isize);
type MomentumT = (isize, isize);

struct Sky {
    stars: Vec<Star>
}

impl Sky {
    fn new() -> Self {
        Sky {stars: Vec::new()}
    }

    fn add_star(&mut self, star: Star) {
        self.stars.push(star);
    }

    fn align(&mut self) {
        self.stars.iter_mut().for_each(|s| s.step());
    }
}

struct Star {
    position: PointT,
    momentum: MomentumT,
}

impl Star {
    fn new(position: PointT, momentum: MomentumT) -> Star {
        Star { position, momentum }
    }

    fn step(&mut self) {
        let (x, y) = self.position;
        let (vel_x, vel_y) = self.momentum;
        self.position = (x + vel_x, y + vel_y);
    }
}

impl FromStr for Star {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<(.+)>.+<(.+)>$").unwrap();
        }

        let data = RE.captures(s).unwrap();
        let pos = data
            .get(1)
            .map_or(Vec::with_capacity(0), |m| m.as_str().split(',').collect::<Vec<_>>());
        let vel = data
            .get(2)
            .map_or(Vec::with_capacity(0), |m| m.as_str().split(',').collect::<Vec<_>>());

        let x = pos[0].parse::<isize>()?;
        let y = pos[1].parse::<isize>()?;
        let v_x = vel[0].parse::<isize>()?;
        let v_y = vel[1].parse::<isize>()?;

        Ok(Star::new((x, y), (v_x, v_y)))
    }
}

fn part1(input: &str) {
    let data = input_reader::read_all_lines(input);

    let mut sky = Sky::new();

    data.iter().for_each(|s| sky.add_star(s.parse::<Star>().unwrap()));
}

fn part2(input: &str) {
    let data = input_reader::read_all(input);
}

pub fn day10() {
    let input = String::from("day10");

    println!("***Day Ten***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    // println!("\t\tWinning elfs score: {}", part1(&input));
    // println!("\t**Part Two**");
    // println!("\t\tWinning elfs score: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
