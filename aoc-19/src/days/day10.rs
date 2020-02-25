use crate::days::*;

#[derive(Debug)]
struct Line {
    slope: f64,
    intercept: f64,
    vertical_x: Option<f64>,
}

impl Line {
    fn new(slope: f64, intercept: f64, vertical_x: Option<f64>) -> Self {
        Self {
            slope,
            intercept,
            vertical_x,
        }
    }

    fn from_points(a: &Point, b: &Point) -> Self {
        let ax = a.x as f64;
        let ay = a.y as f64;
        let bx = b.x as f64;
        let by = b.y as f64;
        let slope = (by - ay) / (bx - ax);
        let intercept = ay - slope * ax;
        let vertical_x = if slope.is_finite() { None } else { Some(ax) };

        Self::new(slope, intercept, vertical_x)
    }

    fn contains(&self, a: &Point) -> bool {
        use float_cmp::*;
        if self.slope.is_finite() {
            (a.y as f64).approx_eq(self.slope * (a.x as f64) + self.intercept, (0.00000003, 2))
        } else {
            (a.x as f64).approx_eq(self.vertical_x.unwrap(), (0.00000003, 2))
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn is_within_bounds(&self, a: &Self, b: &Self) -> bool {
        let lower_bound: Point = {
            let x = if a.x < b.x { a.x } else { b.x };
            let y = if a.y < b.y { a.y } else { b.y };
            Point::new(x, y)
        };
        let upper_bound: Point = {
            let x = if a.x > b.x { a.x } else { b.x };
            let y = if a.y > b.y { a.y } else { b.y };
            Point::new(x, y)
        };

        self.x >= lower_bound.x
            && self.y >= lower_bound.y
            && self.x <= upper_bound.x
            && self.y <= upper_bound.y
    }

    fn angle_from_origin(&self, origin: &Point) -> f64 {
        use float_cmp::*;
        let y_r: f64 = ((self.y as isize - origin.y as isize) * -1isize) as f64;
        let x_r: f64 = (self.x as isize - origin.x as isize) as f64;

        let result = y_r.atan2(x_r) - std::f64::consts::FRAC_PI_2;

        if result.approx_eq(0f64, (0.00000003, 2)) {
            result
        } else if result.is_sign_negative() {
            result.abs()
        } else {
            2f64 * std::f64::consts::PI - result
        }
    }
}

struct Map {
    asteroids: Vec<Point>,
}

impl Map {
    fn new(asteroids: Vec<Point>) -> Self {
        Self { asteroids }
    }

    fn parsed(lines: &[String]) -> Self {
        let mut asteroids: Vec<Point> = vec![];
        for (y, line) in lines.iter().enumerate() {
            for (x, position) in line.chars().enumerate() {
                if position == '#' {
                    asteroids.push(Point::new(x, y))
                }
            }
        }
        Map::new(asteroids)
    }

    fn location_with_most_direct_line_of_sight(&self) -> (Point, usize) {
        let best_location = *self
            .asteroids
            .iter()
            .max_by_key(|p| self.asteroids_detectable_from_point(p))
            .unwrap();
        (
            best_location,
            self.asteroids_detectable_from_point(&best_location),
        )
    }

    fn asteroids_detectable_from_point(&self, origin: &Point) -> usize {
        self.asteroids
            .iter()
            .filter(|&p| p != origin && self.is_detectable(origin, p))
            .count()
    }

    fn is_detectable(&self, origin: &Point, target: &Point) -> bool {
        let line = Line::from_points(origin, target);

        self.asteroid_belt(origin, target)
            .into_iter()
            .filter(|&p| p != origin && p != target)
            .all(|p| !line.contains(p))
    }

    fn asteroid_belt<'a>(&'a self, a: &Point, b: &Point) -> Vec<&'a Point> {
        self.asteroids
            .iter()
            .filter(|p| p.is_within_bounds(a, b))
            .collect()
    }

    fn destroy_n(&mut self, laser_location: &Point, n: usize) -> Point {
        use ordered_float::*;
        let mut destroyed: Vec<Point> = vec![];

        while destroyed.len() < n {
            let mut for_destruction: Vec<Point> = self
                .asteroids
                .iter()
                .filter(|x| *x != laser_location && self.is_detectable(laser_location, x))
                .map(|&x| x)
                .collect();
            self.asteroids.drain_filter(|x| for_destruction.contains(x));

            for_destruction.sort_by(|a, b| {
                OrderedFloat(a.angle_from_origin(laser_location))
                    .cmp(&OrderedFloat(b.angle_from_origin(laser_location)))
            });
            destroyed.append(&mut for_destruction);
        }

        destroyed.into_iter().nth(n - 1).unwrap()
    }
}

pub struct Day10Runner {
    path: String,
    part: Parts,
}

impl Day10Runner {
    pub fn with_input_path(path: &str, part: Parts) -> Self {
        let path = path.to_string();
        Self { path, part }
    }

    fn part1(&self) -> usize {
        let map = self.load();
        let (_, c) = map.location_with_most_direct_line_of_sight();
        c
    }
    fn part2(&self) -> usize {
        let mut map = self.load();
        let (p, _) = map.location_with_most_direct_line_of_sight();
        let last_destroyed = map.destroy_n(&p, 200);
        last_destroyed.x * 100 + last_destroyed.y
    }

    fn load(&self) -> Map {
        let text = crate::input_reader::read_sparated_values_from_input(self.path.as_ref(), "\r\n")
            .expect("map lines");
        Map::parsed(&text)
    }
}

impl Runner for Day10Runner {
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
    use super::*;

    #[test]
    fn line_test1() {
        let line = Line::from_points(&Point::new(3, 4), &Point::new(1, 0));

        assert_eq!(2f64, line.slope);
        assert_eq!(-2f64, line.intercept);
        assert!(line.contains(&Point::new(2, 2)));
    }

    #[test]
    fn line_contains_test1() {
        let line = Line::from_points(&Point::new(4, 4), &Point::new(4, 2));

        assert!(line.contains(&Point::new(4, 2)));
        assert!(line.contains(&Point::new(4, 3)));
    }

    #[test]
    fn line_contains_test2() {
        let line = Line::from_points(&Point::new(0, 2), &Point::new(4, 2));

        assert!(line.contains(&Point::new(4, 2)));
        assert!(line.contains(&Point::new(3, 2)));
        assert!(line.contains(&Point::new(1, 2)));
    }

    #[test]
    fn line_contains_test3() {
        let line = Line::from_points(&Point::new(3, 4), &Point::new(1, 0));

        assert!(line.contains(&Point::new(3, 4)));
        assert!(line.contains(&Point::new(1, 0)));
        assert!(line.contains(&Point::new(2, 2)));
        assert!(line.contains(&Point::new(4, 6)));
        assert!(line.contains(&Point::new(12, 22)));
    }

    #[test]
    fn detectable_test() {
        let input = r".#..#
  .....
  #####
  ....#
  ...##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let origin = Point::new(3, 4);
        let detectable_target = Point::new(2, 2);
        let undetectable_target = Point::new(1, 0);

        assert!(map.is_detectable(&origin, &detectable_target));
        assert!(!map.is_detectable(&origin, &undetectable_target));

        assert!(!map.is_detectable(&Point::new(4, 4), &Point::new(4, 2)));
        assert!(!map.is_detectable(&Point::new(4, 4), &Point::new(4, 1)));
        assert!(map.is_detectable(&Point::new(4, 4), &Point::new(4, 3)));
    }

    #[test]
    fn asteroids_detectable_from_point_test1() {
        let input = r".#..#
      .....
      #####
      ....#
      ...##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);

        assert_eq!(8, map.asteroids_detectable_from_point(&Point::new(3, 4)));
        assert_eq!(7, map.asteroids_detectable_from_point(&Point::new(4, 4)));
        assert_eq!(7, map.asteroids_detectable_from_point(&Point::new(1, 0)));
        assert_eq!(6, map.asteroids_detectable_from_point(&Point::new(0, 2)));
        assert_eq!(5, map.asteroids_detectable_from_point(&Point::new(4, 2)));
    }

    #[test]
    fn asteroids_detectable_from_point_test2() {
        let input = r"......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);

        assert_eq!(33, map.asteroids_detectable_from_point(&Point::new(5, 8)));
    }

    #[test]
    fn asteroids_detectable_from_point_test3() {
        let input = r".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);

        assert_eq!(
            210,
            map.asteroids_detectable_from_point(&Point::new(11, 13))
        );
    }

    #[test]
    fn part1_test1() {
        let input = r".#..#
      .....
      #####
      ....#
      ...##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();

        assert_eq!(8, c);
        assert_eq!(Point::new(3, 4), p);
    }

    #[test]
    fn part1_test2() {
        let input = r"......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();

        assert_eq!(33, c);
        assert_eq!(Point::new(5, 8), p);
    }

    #[test]
    fn part1_test3() {
        let input = r"#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###."
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();

        assert_eq!(35, c);
        assert_eq!(Point::new(1, 2), p);
    }

    #[test]
    fn part1_test4() {
        let input = r".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#.."
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();

        assert_eq!(41, c);
        assert_eq!(Point::new(6, 3), p);
    }

    #[test]
    fn part1_test5() {
        let input = r".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();

        assert_eq!(210, c);
        assert_eq!(Point::new(11, 13), p);
    }

    #[test]
    fn part2_test1() {
        let input = r".#..##.###...#######
      ##.############..##.
      .#.######.########.#
      .###.#######.####.#.
      #####.##.#.##.###.##
      ..#####..#.#########
      ####################
      #.####....###.#.#.##
      ##.#################
      #####.##.###..####..
      ..######..##.#######
      ####.##.####...##..#
      .#####..#.######.###
      ##...#.##########...
      #.##########.#######
      .####.#.###.###.#.##
      ....##.##.###..#####
      .#.#.###########.###
      #.#.#.#####.####.###
      ###.##.####.##.#..##"
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let mut map = Map::parsed(&input);
        let (p, c) = map.location_with_most_direct_line_of_sight();
        let last_destroyed = map.destroy_n(&p, 200);
        let result = last_destroyed.x * 100 + last_destroyed.y;

        assert_eq!(Point::new(11, 13), p);
        assert_eq!(210, c);
        assert_eq!(Point::new(8, 2), last_destroyed);
        assert_eq!(802, result);
    }
}
