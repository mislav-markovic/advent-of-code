use crate::input_reader;

type point_t = (isize, isize);

struct BoundingBox {
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

impl BoundingBox {
    fn determine_box(points: &[point_t]) -> BoundingBox {
        let top = points.iter().min_by_key(|p| p.0).unwrap().0;
        let left = points.iter().min_by_key(|p| p.1).unwrap().1;
        let bottom = points.iter().max_by_key(|p| p.0).unwrap().0;
        let right = points.iter().max_by_key(|p| p.1).unwrap().1;

        BoundingBox {top, right, bottom, left}
    }

    fn contains(&self, p: &point_t) -> bool {
        let (x,y) = *p;
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }

    fn iter(&self) -> BoxIter {
        BoxIter {_box: self, curr_pos: Some((self.left-1, self.top))}
    }
}

struct BoxIter<'a> {
    _box: &'a BoundingBox,
    curr_pos: Option<point_t>,
}

impl<'a> Iterator for BoxIter<'a> {
        type Item = point_t;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_pos {
            None => None,
            Some(p) => {
                self.curr_pos = match p {
                    (x, y) if y >= self._box.bottom => None,
                    (x, y) if x >= self._box.right => Some((self._box.left, y+1)),
                    (x, y) => Some((x+1, y))

                };
                self.curr_pos
            }
        }
    }
}

enum Closest {
    One(point_t),
    Several,
}

fn closest_to(p: point_t, cords: &[point_t]) -> Closest {
    use self::Closest::*;

    let distances = cords.iter().map(|&cp| (cp, manhattan(p, cp))).collect::<Vec<_>>();
    let min = distances.iter().min().unwrap();

    match distances.iter().filter(|(_, dist)| *dist == min.1).count() {
        x if x == 1 => One(min.0),
        _ => Several
    }
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

fn do_the_job(input_location: &str) -> u32 {
    let data = input_reader::read_all_lines(input_location);
    let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>();
    let bound_box = BoundingBox::determine_box(&points);
    let cords_in_box = points.iter().filter(|p| bound_box.contains(p)).map(|p| p.clone()).collect::<Vec<_>>();

    bound_box.iter().filter(|p| !points.contains(p)).map(|p| closest_to(p, &cords_in_box));

    0
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
