use crate::days::*;

struct System {
    moons: Vec<Moon>,
    original: Vec<Moon>,
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        let original = moons.clone();
        Self { moons, original }
    }

    fn parsed(text: &[String]) -> Self {
        Self::new(text.iter().map(|line| Moon::parsed(line)).collect())
    }

    fn time_step(&mut self) {
        use rayon::prelude::*;

        for moon in self.moons.clone() {
            self.apply_moons_gravity_to_others(&moon);
        }

        self.moons.par_iter_mut().for_each(|m| m.apply_velocity());
    }

    fn apply_moons_gravity_to_others(&mut self, moon: &Moon) {
        use rayon::prelude::*;

        self.moons
            .par_iter_mut()
            .filter(|m| m.id != moon.id)
            .for_each(|m| m.apply_gravity(moon));
    }

    fn time_steps_until_initial_position(&mut self) -> u64 {
        let mut counter = 0u64;

        while self.moons.iter().any(|m| m.cycle.is_none()) {
            self.time_step();
            counter += 1;

            for (i, moon) in self.moons.iter_mut().enumerate() {
                if moon.cycle.is_none() {
                    if moon.position == self.original[i].position
                        && moon.velocity == self.original[i].velocity
                    {
                        moon.cycle = Some(counter)
                    }
                }
            }
        }

        self.moons
            .iter()
            .for_each(|x| println!("{}", x.cycle.unwrap()));

        lcm_multiple(
            self.moons
                .iter()
                .map(|x| x.cycle.unwrap())
                .collect::<Vec<_>>()
                .as_ref(),
        )
    }

    fn is_initial_state(&self) -> bool {
        self.original
            .iter()
            .zip(&self.moons)
            .all(|(orig, moon)| orig.position == moon.position && orig.velocity == moon.velocity)
    }

    fn simulate_time(&mut self, time_steps: usize) {
        (0..time_steps).for_each(|_| self.time_step());
    }

    fn total_energy(&self) -> i64 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut result = a;
    let mut modulo = b;

    while modulo > 0 {
        let temp = result % modulo;
        result = modulo;
        modulo = temp;
    }
    result
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_multiple(args: &[u64]) -> u64 {
    use rayon::prelude::*;

    args.par_iter()
        .cloned()
        .reduce(|| 1u64, |acc, x| lcm(acc, x))
}

#[derive(Clone)]
struct Moon {
    id: usize,
    position: Vec3,
    velocity: Vec3,
    cycle: Option<u64>,
}

impl Moon {
    fn new(id: usize, position: Vec3, velocity: Vec3) -> Self {
        Self {
            id,
            position,
            velocity,
            cycle: None,
        }
    }

    fn parsed(line: &str) -> Self {
        use lazy_static::*;
        use regex::Regex;

        lazy_static! {
            static ref X_REG: Regex = Regex::new(r"x=(-?\d+)").unwrap();
            static ref Y_REG: Regex = Regex::new(r"y=(-?\d+)").unwrap();
            static ref Z_REG: Regex = Regex::new(r"z=(-?\d+)").unwrap();
        }

        let vel = Vec3::new_zeroed();
        let x = X_REG.captures_iter(line).next().unwrap()[1]
            .parse::<i64>()
            .expect("Parsed x coordinate");
        let y = Y_REG.captures_iter(line).next().unwrap()[1]
            .parse::<i64>()
            .expect("Parsed y coordinate");
        let z = Z_REG.captures_iter(line).next().unwrap()[1]
            .parse::<i64>()
            .expect("Parsed z coordinate");

        let pos = Vec3::new(x, y, z);

        let id = (x.abs() * 100 + y.abs() * 10 + z.abs()) as usize;

        Self::new(id, pos, vel)
    }

    fn apply_gravity(&mut self, other: &Moon) {
        let my_x_pull = if other.position.x == self.position.x {
            0
        } else if other.position.x > self.position.x {
            1i64
        } else {
            -1i64
        };

        let my_y_pull = if other.position.y == self.position.y {
            0
        } else if other.position.y > self.position.y {
            1i64
        } else {
            -1i64
        };

        let my_z_pull = if other.position.z == self.position.z {
            0
        } else if other.position.z > self.position.z {
            1i64
        } else {
            -1i64
        };

        self.velocity.x += my_x_pull;
        self.velocity.y += my_y_pull;
        self.velocity.z += my_z_pull;
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn new_zeroed() -> Self {
        Self::new(0, 0, 0)
    }

    fn display(&self) -> String {
        format!("(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

pub struct Day12Runner {
    path: String,
    part: Parts,
}

impl Day12Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> u64 {
        let mut system = self.load();
        system.simulate_time(1000);
        system.total_energy() as u64
    }
    fn part2(&self) -> u64 {
        use std::time::Instant;
        let now = Instant::now();
        let mut system = self.load();
        let res = system.time_steps_until_initial_position();
        println!("Time until result {}s", now.elapsed().as_secs());
        res
    }

    fn load(&self) -> System {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n");
        System::parsed(&text.expect("Could not read instructions"))
    }
}

impl Runner for Day12Runner {
    fn run(&self) -> String {
        let result = match self.part {
            Parts::Part1 => self.part1(),
            Parts::Part2 => self.part2(),
        };

        format!("Result: {}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::System;

    #[test]
    fn part1_test1() {
        let input = r"<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut system = System::parsed(&input);
        system.simulate_time(10);

        assert_eq!(179, system.total_energy());
    }

    #[test]
    fn part2_test1() {
        let input = r"<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut system = System::parsed(&input);
        let time_steps = system.time_steps_until_initial_position();

        assert_eq!(2772, time_steps);
    }

    #[test]
    fn part2_test2() {
        let input = r"<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>"
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut system = System::parsed(&input);
        let time_steps = system.time_steps_until_initial_position();

        assert_eq!(4686774924u64, time_steps);
    }
}
