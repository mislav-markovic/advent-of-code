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
        cords[1] = &cords[1][0..cords[1].len() - 1];
        let dim = arr[3].split('x').collect::<Vec<&str>>();

        //println!("{:?}", arr);

        let id = arr[0][1..].parse::<u32>().unwrap();
        let width = dim[0].parse::<u32>().unwrap();
        let height = dim[1].parse::<u32>().unwrap();
        let left_edge_distance = cords[0].parse::<u32>().unwrap();
        let top_edge_distance = cords[1].parse::<u32>().unwrap();

        Rectangle::new(id, width, height, left_edge_distance, top_edge_distance)
    }

    pub fn iter(&self) -> RectangleIter {
        RectangleIter {
            rect: self,
            iter_curr: Some((self.left_edge_distance, self.top_edge_distance)),
            is_first: true,
        }
    }
}

pub struct RectangleIter<'a> {
    rect: &'a Rectangle,
    iter_curr: Option<point_t>,
    is_first: bool,
}

impl<'a> Iterator for RectangleIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            let (end_x, end_y) = (
                self.rect.left_edge_distance + self.rect.width - 1,
                self.rect.top_edge_distance + self.rect.height - 1,
            );

            self.iter_curr = match self.iter_curr {
                Some((x, y)) if x < end_x => Some((x + 1, y)),
                Some((_x, y)) if y < end_y => Some((self.rect.left_edge_distance, y + 1)),
                Some((_x, _y)) => None,
                None => None,
            };
        }

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

    pub fn unoverlaping_claim(&self) -> u32 {
        self.claims.iter().filter(|r| !self.is_overlaping(&r)).take(1).next().unwrap().id
    }

    fn is_overlaping(&self, rect: &Rectangle) -> bool {
        rect.iter().map(|x| *self.area.get(&x).unwrap()).filter(|x| *x > 1).count() > 0
    }
}

fn do_the_job(input_location: &str) -> (u32, u32) {
    let data = input_reader::read_all_lines(input_location);
    let mut fabric = Fabric::new();

    for l in data {
        fabric.add_claim(Rectangle::from_str(&l));
    }

    (fabric.overlap(), fabric.unoverlaping_claim())
}


pub fn day3() {
    let input = String::from("day3");
    let (overlap, not_overlaping) = do_the_job(&input);
    println!("***Day Three***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOverlaping inches: {}", overlap);
    println!("\t**Part Two**");
    println!("\t\tClaim ID: {}", not_overlaping);
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn init_with_str_1() {
        let r = Rectangle::from_str("#1 @ 1,1: 1x1");
        assert_eq!(r.id, 1);
        assert_eq!(r.width, 1);
        assert_eq!(r.height, 1);
        assert_eq!(r.left_edge_distance, 1);
        assert_eq!(r.top_edge_distance, 1);
    }

    #[test]
    fn init_with_str_2() {
        let r = Rectangle::from_str("#1234 @ 12,34: 56x78");
        assert_eq!(r.id, 1234);
        assert_eq!(r.width, 56);
        assert_eq!(r.height, 78);
        assert_eq!(r.left_edge_distance, 12);
        assert_eq!(r.top_edge_distance, 34);
    }

    #[test]
    fn iter_test() {
        let r = Rectangle::from_str("#3 @ 5,5: 2x2");
        let mut it = r.iter();

        assert_eq!(it.next(), Some((5, 5)));
        assert_eq!(it.next(), Some((6, 5)));
        assert_eq!(it.next(), Some((5, 6)));
        assert_eq!(it.next(), Some((6, 6)));

        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn overlap_test_1() {
        let r1 = Rectangle::from_str("#1 @ 1,3: 4x4");
        let r2 = Rectangle::from_str("#2 @ 3,1: 4x4");
        let r3 = Rectangle::from_str("#3 @ 5,5: 2x2");
        let mut f = Fabric::new();
        f.add_claim(r1);
        f.add_claim(r2);
        f.add_claim(r3);

        assert_eq!(f.overlap(), 4);
    }

    #[test]
    fn overlap_test_2() {
        let r1 = Rectangle::from_str("#1 @ 1,3: 4x4");
        let r2 = Rectangle::from_str("#2 @ 3,1: 4x4");
        let r3 = Rectangle::from_str("#3 @ 5,5: 2x2");
        let r4 = Rectangle::from_str("#4 @ 3,3: 2x2");
        let mut f = Fabric::new();
        f.add_claim(r1);
        f.add_claim(r2);
        f.add_claim(r3);
        f.add_claim(r4);

        assert_eq!(f.overlap(), 4);
    }

        #[test]
    fn unoverlap_test_1() {
        let r1 = Rectangle::from_str("#1 @ 1,3: 4x4");
        let r2 = Rectangle::from_str("#2 @ 3,1: 4x4");
        let r3 = Rectangle::from_str("#3 @ 5,5: 2x2");
        let mut f = Fabric::new();
        f.add_claim(r1);
        f.add_claim(r2);
        f.add_claim(r3);

        assert_eq!(f.unoverlaping_claim(), 3);
    }
}
