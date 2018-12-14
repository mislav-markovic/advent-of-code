use rayon::prelude::*;
use std::collections::HashMap;

type PointT = (isize, isize); // (x,y)
type DimensionT = (usize, usize); //(width, height)

struct FuelCell {
    position: PointT,
    serial_number: usize
}

impl FuelCell {
    const fn new(position: PointT, serial_number: usize) -> FuelCell{
        FuelCell {position, serial_number}
    }

    const fn power_level(&self) -> isize {
        //(((rackID * Y + input) * rackID) / 100 % 10) - 5
        (((self.position.0 + 10) * self.position.1 + self.serial_number as isize) * (self.position.0 + 10) / 100 % 10) - 5
    }
}

struct Battery {
    dimension: DimensionT,
    fuel_cells: HashMap<PointT, FuelCell>,
    power_levels: HashMap<(PointT, usize), isize> // (position, size) -> power
}

impl Battery {
    fn new_empty(dimension: DimensionT) -> Battery {
        let size = dimension.0*dimension.1;
        Battery {dimension, fuel_cells: HashMap::with_capacity(size), power_levels: HashMap::with_capacity(size)}
    }

    fn new(dimension: DimensionT, serial_number: usize) -> Battery {
        let mut bt = Battery::new_empty(dimension);

        for x in 0..dimension.0 as isize {
            for y in 0..dimension.1 as isize {
                bt.add_fuel_cell((x,y), serial_number);
            }
        }
        bt.get_max_power(1);
        bt.get_max_power(2);
        bt.get_max_power(3);
        bt
    }

    fn add_fuel_cell(&mut self, position: PointT, serial_number: usize) {
        self.fuel_cells.insert(position, FuelCell::new(position, serial_number));
    }

    fn get_max_power(&self, square: usize) -> (PointT, isize) {
        let mut max = isize::min_value();
        let mut max_cell: PointT = (0,0);

        for x in 0..(self.dimension.0 - square) as isize {
            for y in 0..(self.dimension.1 - square) as isize {
                let top_left = (x, y);
                let power = self.square_power(top_left, square);

                if power >= max {
                    max = power;
                    max_cell = top_left;
                }
            }
        }
        (max_cell, max)
    }

    fn square_power(&self, top_left: PointT, size: usize) -> isize {
        let (x, y) = top_left;
        let mut sum = 0isize;
        for i in x..(x + size as isize){
            for j in y..(y + size as isize) {
                sum += self.fuel_cells.get(&(i,j)).unwrap().power_level();
            }
        }
        sum
    }
}

fn part1(input: usize) -> PointT{
    let bt = Battery::new((300, 300), input);
    bt.get_max_power(3).0
}

fn part2(input: usize) -> (PointT, usize) {
    let bt = Battery::new((300, 300), input);
    let v = (1..=300).collect::<Vec<usize>>();
    let ((pos, _power), size) = v.par_iter().map(|size| (bt.get_max_power(*size), size)).max_by_key(|&((_pos, pow), _size)| pow).unwrap();
    (pos, *size)
}

pub fn day11() {
    const INPUT: usize = 8444;

    println!("***Day Eleven***");
    println!("\tInput is {}", INPUT);
    println!("\t**Part One**");
    println!("\t\t{:?}", part1(INPUT));
    println!("\t**Part Two**");    
    println!("\t\t{:?}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::FuelCell;
    #[test]
    fn test_power_level_1() {
        let fc = FuelCell::new((3,5), 8);
        assert_eq!(4, fc.power_level());
    }

        #[test]
    fn test_power_level_2() {
        let fc = FuelCell::new((101,153), 71);
        assert_eq!(4, fc.power_level());
    }

        #[test]
    fn test_power_level_3() {
        let fc = FuelCell::new((217,196), 39);
        assert_eq!(0, fc.power_level());
    }

        #[test]
    fn test_power_level_4() {
        let fc = FuelCell::new((122,79), 57);
        assert_eq!(-5, fc.power_level());
    }
}
