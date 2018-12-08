use crate::input_reader;

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

    fn data_sum(&self) -> usize {
        self.data.iter().sum()
    }

    fn node_value(&self) -> usize {
        if self.children.is_empty() {
            self.data_sum()
        } else {
            let mut result = 0;
            for node_ref in self.data.iter() {
                result += self
                    .children
                    .get(*node_ref - 1)
                    .unwrap_or(&Node::new_boxed())
                    .node_value();
            }
            result
        }
    }
}

fn do_the_job(input: &str) -> (usize, usize) {
    let data = input_reader::read_all(input);
    let parsed = data
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (result, _, root) = traverse_data(&parsed);
    (result, root.node_value())
}

//(sum, # of numbers read)
fn traverse_data(data: &[usize]) -> (usize, usize, Box<Node>) {
    let children = data[0];
    let data_count = data[1];
    let mut node = Node::new_boxed();

    if children == 0 {
        node.add_data(
            data.iter()
                .skip(2)
                .take(data_count)
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
        );
        (node.data_sum(), 2 + data_count, node)
    } else {
        let (mut sum, mut skip): (usize, usize) = (0, 2);

        for _ in 0..children {
            let (partial, jump, child) = traverse_data(&data[skip..]);
            skip += jump;
            sum += partial;
            node.add_child(child);
        }

        node.add_data(
            data.iter()
                .skip(skip)
                .take(data_count)
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
        );
        sum += node.data_sum();
        skip += data_count;
        (sum, skip, node)
    }
}

pub fn day8() {
    let input = String::from("day8");
    let (part1, part2) = do_the_job(&input);

    println!("***Day Eight***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tMetadata entries sum: {}", part1);
    println!("\t**Part Two**");
    println!("\t\tRoot node value: {}", part2);
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
