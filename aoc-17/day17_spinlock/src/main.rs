
const INPUT: usize = 348;

fn main() {

    println!("Part 1 solution: {}", part1());
    println!("Part 2 solution: {}", part2());
}

fn part2() -> usize {
    let mut curr_pos = 0;
    let mut len = 1;
    let mut after_zero = 0;

    for i in 1..50_000_001 {
        let insert = (curr_pos + INPUT) % len + 1;
        if insert == 1 {
            after_zero = i;
        }
        len += 1;
        curr_pos = insert;
    }
    after_zero
}

fn part1() -> usize {
    let mut vec = Vec::with_capacity(2018);
    let mut curr_pos = 0;

    vec.push(0);

    for i in 1..2018 {
        curr_pos = (curr_pos + INPUT) % vec.len() + 1;
        vec.insert(curr_pos, i);
    }
    vec[curr_pos+1]
}