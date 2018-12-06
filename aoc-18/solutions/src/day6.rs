use crate::input_reader;

type point_t = (isize, isize);

struct BoundingBox {
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

impl BoundingBox {
    fn determine_box(points: &Vec<point_t>) -> BoundingBox {
        let top = points.iter().min_by_key(|p| p.0).unwrap().0;
        let left = points.iter().min_by_key(|p| p.1).unwrap().1;
        let bottom = points.iter().max_by_key(|p| p.0).unwrap().0;
        let right = points.iter().max_by_key(|p| p.1).unwrap().1;

        BoundingBox {top, right, bottom, left}
    }

    fn iter(&self) -> BoxIter {
        BoxIter {bounding_box: self, curr_pos: Some((self.left, self.top))}
    }
}

struct BoxIter<'a> {
    bounding_box: &'a BoundingBox,
    curr_pos: Option<point_t>,
}

impl<'a> Iterator for BoxIter<'a> {
        type Item = point_t;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_pos {
            None => None,
            Some(p) => {
                let next = match p {
                    (x, _y) if x >= self.right => (s)
                }
            }
        }
    }
}

fn do_the_job(input_location: &str) -> u32 {
    let data = input_reader::read_all_lines(input_location);
    let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>();
    let bound_box = BoundingBox::determine_box(&points);




    0
}

fn manhattan(p1: point_t, p2: point_t) -> isize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x1 - x2).abs() + (y1 - y2).abs()
}

fn point(input: &str) -> point_t {
    let arr = input.split(",").collect::<Vec<_>>();
    (arr[0].parse().unwrap(), arr[1].parse().unwrap())
}

pub fn day6() {
    let input = String::from("day6");
    println!("***Day Three***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tOverlaping inches: {}", do_the_job(&input));
    //println!("\t**Part Two**");
    //println!("\t\tClaim ID: {}", not_overlaping);
}

#[cfg(test)]
mod tests {}
