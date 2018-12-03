use crate::input_reader;
use std::collections::HashMap;

type area_t = HashMap<point_t, u32>;
type point_t = (u32, u32);

#[derive(Clone)]
pub struct Rectangle {
    id: u32,
    width: u32,
    height: u32,
    left_edge_distance: u32,
    top_edge_distance: u32,
}

impl Rectangle {
    pub fn new(
        id: u32,
        width: u32,
        height: u32,
        left_edge_distance: u32,
        top_edge_distance: u32,
    ) -> Rectangle {
        Rectangle {
            id,
            width,
            height,
            left_edge_distance,
            top_edge_distance,
        }
    }

    pub fn from_str(input: &str) -> Rectangle {
        let arr = input.split(' ').collect::<Vec<&str>>();
        let mut cords = arr[2].split(',').collect::<Vec<&str>>();
        cords[1] = &cords[1][0..cords[1].len()-1];
        let dim = arr[3].split('x').collect::<Vec<&str>>();


        //println!("{:?}", arr);


        let id = arr[0][1..].parse::<u32>().unwrap();
        let width = dim[0].parse::<u32>().unwrap();
        let height = dim[1].parse::<u32>().unwrap();
        let left_edge_distance = cords[0].parse::<u32>().unwrap();
        let top_edge_distance = cords[1].parse::<u32>().unwrap();

        Rectangle::new(id, width, height, left_edge_distance, top_edge_distance)
    }

    pub fn iter(&self) -> Rectangle_Iter {
        Rectangle_Iter {
            rect: self,
            iter_curr: Some((self.left_edge_distance, self.top_edge_distance)),
        }
    }
}

pub struct Rectangle_Iter<'a> {
    rect: &'a Rectangle,
    iter_curr: Option<point_t>,
}

impl<'a> Iterator for Rectangle_Iter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let (end_x, end_y) = (
            self.rect.left_edge_distance + self.rect.width,
            self.rect.top_edge_distance + self.rect.height,
        );

        self.iter_curr = match self.iter_curr {
            Some((x, y)) if x < end_x => Some((x + 1, y)),
            Some((_x, y)) if y < end_y => Some((self.rect.left_edge_distance, y + 1)),
            Some((_x, _y)) => None,
            None => None,
        };

        self.iter_curr
    }
}

pub struct Fabric {
    area: area_t,
    claims: Vec<Rectangle>,
}

impl Fabric {
    pub fn new() -> Fabric {
        Fabric {
            area: area_t::new(),
            claims: Vec::new(),
        }
    }

    pub fn add_claim(&mut self, claim: Rectangle) {
        claim
            .iter()
            .for_each(|(x, y)| *self.area.entry((x, y)).or_insert(0) += 1);
        self.claims.push(claim);
    }

    pub fn overlap(&self) -> u32 {
        self.area.values().filter(|&&x| x > 1).count() as u32
    }
}

pub fn part1(input_location: &str) -> u32 {
    let data = input_reader::read_all_lines(input_location);
    let mut fabric = Fabric::new();

    for l in data {
        fabric.add_claim(Rectangle::from_str(&l));
    }

    fabric.overlap()
}

pub fn part2(input_location: &str) {
    let data = input_reader::read_all_lines(input_location);
}

pub fn day3() {
    let input = String::from("day3");

    println!("***Day Three***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOverlaping inches: {}", part1(&input));
    //println!("\t**Part Two**");
    //println!("\t\t{}", part2(&input));
}
