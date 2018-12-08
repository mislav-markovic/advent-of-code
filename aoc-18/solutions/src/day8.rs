use crate::input_reader;

enum State {
    Header,
    Data(usize),
}

struct Node {
    data: Vec<usize>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            data: Vec::new(),
            children: Vec::new(),
        }
    }

    fn new_boxed() -> Box<Node> {
        Box::new(Node::new())
    }

    fn add_child(&mut self, node: Box<Node>) {
        self.children.push(node);
    }

    fn add_data(&mut self, data: &[usize]) {
        self.data.extend_from_slice(data);
    }
}

fn part1(input: &str) -> usize {
    use self::State::*;
    use std::collections::VecDeque;
    let data = input_reader::read_all(input);
    let mut parsed = data
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (result, _) = sum_data(&parsed);
    result
}

//(sum, # of numbers read)
fn sum_data(data: &[usize]) -> (usize, usize) {
    //println!("{:?}", data);
    let children = data[0];
    let data_count = data[1];

    if children == 0 {
        (data.iter().skip(2).take(data_count).sum(), 2 + data_count)
    } else {
        let (mut sum, mut skip): (usize, usize) = (0, 2);

        for _ in 0..children {
            let (partial, jump) = sum_data(&data[skip..]);
            skip += jump;
            sum += partial;
        }
        let mine = data.iter().skip(skip).take(data_count).sum::<usize>();
        sum += mine;
        skip += data_count;
        (sum, skip)
    }
}

fn part2(input: &str) {
    let data = input_reader::read_all(input);
}

pub fn day8() {
    let input = String::from("day8");

    println!("***Day Eight***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tMetadata entries sum: {}", part1(&input));
    println!("\t**Part Two**");
    //println!("\t\tShortest reduction: {}", part2(&input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let a = [1, 2, 3];

        let mut iter = &mut a.iter();
        let mut t = iter.take(2);

        assert_eq!(t.next(), Some(&1));
        assert_eq!(t.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn test2() {
        let a = [1, 2, 3];

        let mut iter = &mut a.iter();
        let mut t = iter.take(2).collect::<Vec<_>>();

        assert_eq!(iter.next(), Some(&3));
    }
}
