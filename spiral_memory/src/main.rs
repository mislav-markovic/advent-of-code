use std::collections::HashMap;

enum SquareSide {
    Down(i32),
    Up(i32),
    Left(i32),
    Right(i32),
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Enter part 1 input: ");
    let location = read_number();
    println!("Enter part 2 input: ");
    let threshold = read_number();
    let (level, _, _) = find_level(location);
    let zero = Point {x: 0, y : 0};
    let first_after_threshold = find_threshold_value(threshold);

    match find_side(location){
        SquareSide::Down(x) => println!("Part 1 distance: {}", manhattan_distance(zero, Point{x: x, y: -1 * level as i32})),
        SquareSide::Up(x) => println!("Part 1 distance: {}", manhattan_distance(zero, Point{x: x, y: level as i32})),
        SquareSide::Left(y) => println!("Part 1 distance: {}", manhattan_distance(zero, Point{x: -1 * level as i32, y: y})),
        SquareSide::Right(y) => println!("Part 1 distance: {}", manhattan_distance(zero, Point{x: level as i32, y: y})),
    }

    println!("First after threshold is: {}", first_after_threshold);
}

fn find_threshold_value(thrsh: u32) -> u32 {
    let mut memory: HashMap<Point, u32> = HashMap::new();
    memory.insert(Point{x: 0, y: 0}, 1);
    let mut level = 1;
    let mut side = 1;

    loop {
        let (last_max, thrsh_passed) = fill_spiral(&mut memory, side, level, thrsh);
        if thrsh_passed {break last_max;}
        level += 1;
        side += 2
    }
}

fn fill_spiral(mem: &mut HashMap<Point, u32>, prev_side: u32, level: u32, threshold: u32) -> (u32, bool){
    let start_point = Point{x:level as i32, y:-1 * (level-1) as i32};
    let mut sum = 0u32;

    for i in 0..prev_side + 1 {
        let p = Point {x:start_point.x, y: start_point.y + i as i32};
        sum = get_neighbours_sum(&p, &mem);
        if sum > threshold { return (sum, true); }
        mem.insert(p, sum);
    }

    for i in 1..prev_side + 2 {
        let p = Point {x:start_point.x - i as i32, y: start_point.y + prev_side as i32};
        sum = get_neighbours_sum(&p, &mem);
        if sum > threshold { return (sum, true); }
        mem.insert(p, sum);
    }

    for i in 1..prev_side + 2 {
        let p = Point {x:start_point.x - (prev_side+1) as i32, y:start_point.y + prev_side as i32 - i as i32};
        sum = get_neighbours_sum(&p, &mem);
        if sum > threshold { return (sum, true); }
        mem.insert(p, sum);
    }

    for i in 1..prev_side + 2 {
        let p = Point {x:start_point.x - (prev_side + 1) as i32 + i as i32, y: start_point.y - 1};
        sum = get_neighbours_sum(&p, &mem);
        if sum > threshold { return (sum, true); }
        mem.insert(p, sum);
    }
    (sum, false)
}

fn get_neighbours_sum(loc: &Point, mem: &HashMap<Point, u32>) -> u32 {
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            let p = Point{x: loc.x + i, y: loc.y + j};
            sum += mem.get(&p).unwrap_or(&0);
        }
    }
    println!("Sum for point ({}, {}) is {}", loc.x, loc.y, sum);
    sum
}

fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    println!("Point: {}, {}", p2.x, p2.y);
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn read_number() -> u32 {
    use std::io;
    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read line");

    s.trim().parse().unwrap()
}

fn find_side(loc: u32) -> SquareSide {
    let (_, mut high, row_len) = find_level(loc);
    let mut low = high - row_len;

    if loc >= low {
        return SquareSide::Down(loc as i32 - half_point(high, low))
    } 

    high = low;
    low -= row_len;

    if loc >= low {
        return SquareSide::Left(half_point(high, low) - loc as i32)
    } 

    high = low;
    low -= row_len;

    if loc >= low {
        return SquareSide::Up(half_point(high, low) - loc as i32)
    } 

    high = low;
    low -= row_len;

    return SquareSide::Right(loc as i32 - half_point(high, low))
}

fn half_point(h: u32, l: u32) -> i32 {
    println!("Half point: {}", ((h+l) / 2) as i32);
    ((h+l) / 2) as i32
}

fn find_level(loc: u32) -> (u32, u32, u32) {
    let mut max = 1;
    let mut side = 1;

    for i in 1.. {
        max += 2*(side+2) + 2*side;
        side += 2;
        if max >= loc {
            return (i, max, side-1);
        }
    }
    (0, 0, 0)
}