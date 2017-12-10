use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

const FILE_PATH: &str = "./input.txt";

struct Tree {
    tree: HashMap<String, Node>,
    root: Option<Node>
}

impl Tree {
    fn new() -> Tree {
        Tree {tree: HashMap::new(), root: None}
    }

    fn add(&mut self, n: Node){
        self.tree.insert(n.name.clone(), n);
    }

    fn add_vec(&mut self, vec: Vec<Node>) {
        for n in vec {
            self.add(n);
        }
    }

    fn build_tree(&mut self){
        let map = self.get_family();

        for (parent, children) in map {
            if children.len() > 0 {
                for child in children {
                    self.tree.get_mut(&child).unwrap().add_parent(&parent);
                }
            }
        }
        self.set_root();
    }

    fn set_root(&mut self) {
        for (_, node) in &self.tree {
            match node.parent {
                None => self.root = Some(node.clone()),
                _ => (),
            }
        }
    }

    fn get_family(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        for (name, node) in &self.tree {
            if node.children.len() > 0 {
                let mut vec = Vec::new();
                for child in &node.children {
                    vec.push(child.clone());
                }
                map.insert(name.clone(), vec);
            }
        }
        map
    }
}

#[derive(Clone)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
    parent: Option<String>,
    total_weight: u32
}

impl Node {
    fn add_parent(&mut self, parent: &String){
        self.parent = Some(parent.clone());
    }

    fn new(name: String, weight: Option<u32>, parent: Option<String>) -> Node {
        let w = match weight {
            Some(n) => n,
            _ => 0
        };
        Node{name: name, weight: w, children: Vec::new(), parent: parent, total_weight: 0}
    }

    fn add_children(&mut self, ch: Vec<String>) {
        for c in ch.iter(){
            self.children.push(c.clone());
        }
    }

    fn total_weight(&self, tree: &Tree) -> u32 {
        let mut sum = 0;
        if self.children.len() > 0 {
            for child in &self.children {
                sum += tree.tree.get(child).unwrap().total_weight(&tree);
            }
        }
        sum + self.weight
    }
}

fn main() {
    let vec = read_input();
    let mut tree = Tree::new();
    tree.add_vec(vec);
    tree.build_tree();

    let mut map: HashMap<String, u32> = HashMap::new();
    for (name, node) in &tree.tree {
        map.insert(name.clone(), node.total_weight(&tree));
    }
    find_imbalance(&map, &tree);
}

fn find_imbalance(weights: &HashMap<String, u32>, tree: &Tree) {
    for (name, node) in &tree.tree {
        if node.children.len() > 1 {
            let test = weights.get(&node.children[0]).unwrap();
            for child in node.children.iter(){
                if test != weights.get(child).unwrap() {
                    println!("test: {}, value: {}", test, weights.get(child).unwrap());
                    println!("test weight: {}, node weight: {}",
                     tree.tree.get(&node.children[0]).unwrap().weight,
                     tree.tree.get(child).unwrap().weight);
                }
            }
        }
    }
}

fn read_input() -> Vec<Node> {
    let f = File::open(PathBuf::from(FILE_PATH)).expect("file not found");
    let mut result: Vec<Node> = Vec::new();
    let reader = BufReader::new(f);

    reader.lines().for_each(|l| result.push(parse(l.unwrap().trim().to_string())));
    result
}

fn parse(line: String) -> Node {
    let temp: Vec<&str> = line.split("->").collect();
    let temp2: Vec<&str> = temp[0].trim().split_whitespace().collect();
    let mut node = Node::new(temp2[0].trim().to_string(), Some(temp2[1][1..temp2[1].len()-1].parse().unwrap()), None);

    if temp.len() == 1 {
        return node;
    } else {
        let mut vec: Vec<String> = Vec::new();
        temp[1].split(',').map(|a| a.trim().to_string()).for_each(|a| vec.push(a));
        node.add_children(vec);
        return node;
    }
}