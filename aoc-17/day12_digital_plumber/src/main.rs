use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const FILE_PATH: &str = "./input.txt";

struct Node {
    id: u32,
    neighbours: HashSet<u32>
}

impl Node {
    fn new(id: u32) -> Node {
        Node {id: id, neighbours: HashSet::new()}
    }

    fn add_neighbour(&mut self, n_id: u32) {
        self.neighbours.insert(n_id);
    }
}

fn main() {
    let input = read_input();

    let mut map = HashMap::<u32, Node>::new();

    for (id, vec) in input {
        let n = map.entry(id).or_insert(Node::new(id));

        for n_id in vec {
            n.add_neighbour(n_id);
        }
    }

    {
        let mut update_map = HashMap::<u32, Vec<u32>>::new();

        for (id, node) in map.iter() {
            for n_id in node.neighbours.iter() {
                update_map.entry(*n_id).or_insert(Vec::new()).push(*id);
            }
        }

        for (id, node) in map.iter_mut() {
            for n in update_map.get(id).unwrap() {
                node.add_neighbour(*n);
            }
        }
    } 

    let mut all =Vec::<u32>::new();

    for (id, _) in map.iter() {
        all.push(*id);
    }

    let result = bfs(0, &map);
    println!("{}", result.len());

    let mut group_counter = 1;

    all.retain(|a| !result.contains(a));

    while all.len() > 0 {
        let temp = bfs(all[0], &map);
        all.retain(|a| !temp.contains(a));
        group_counter += 1;
    }

    println!("Groups: {}", group_counter);

}

fn bfs(start: u32, map: &HashMap<u32, Node>) -> Vec<u32> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();
    open_set.push_front(start);

    while open_set.len() != 0 {
        let parent = open_set.pop_front().unwrap();

        for child in map.get(&parent).unwrap().neighbours.iter() {
            if closed_set.contains(child) {
                continue;
            }

            if !open_set.contains(child) {
                open_set.push_back(*child);
            }
        }
        closed_set.insert(parent);
    }

    closed_set.iter().map(|a| *a).collect()
}

fn read_input() -> Vec<(u32, Vec<u32>)> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<(u32, Vec<u32>)> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> (u32, Vec<u32>) {
    let temp: Vec<&str> = line.split("<->").collect();
    let mut vec = Vec::new();
    let lhs = temp[0].trim().parse().unwrap();

    for st in temp[1].split(',') {
        vec.push(st.trim().parse().unwrap());
    }
    (lhs, vec)
}