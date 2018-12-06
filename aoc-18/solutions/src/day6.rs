use crate::input_reader;

type point_t = (isize, isize);

struct BoundingBox {
    top_left: point_t,
    top_right: point_t,
    bottom_left: point_t,
    bottom_right: point_t,
}

impl BoundingBox {
    fn determine_box(points: &Vec<point_t>) -> BoundingBox {
        let top = points.iter().min_by_key(|p| p.0).unwrap().0;
        let left = points.iter().min_by_key(|p| p.1).unwrap().1;
        let bottom = points.iter().max_by_key(|p| p.0).unwrap().0;
        let right = points.iter().max_by_key(|p| p.1).unwrap().1;

        BoundingBox {
            top_left: (top, left),
            top_right: (top, right),
            bottom_left: (bottom, left),
            bottom_right: (bottom, right),
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
