use crate::shared::utils::lines;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node {
    name: String,
    links: Vec<String>,
    is_small: bool,
}

impl Node {
    pub fn new(name: &str) -> Node {
        let name = String::from(name);
        let is_small = name.to_lowercase() == name;
        Node {
            name,
            links: Vec::new(),
            is_small,
        }
    }

    pub fn add_link(self: &mut Self, dest: &str) {
        self.links.push(String::from(dest));
    }
}

fn add_node<'a>(nodes: &mut HashMap<&'a str, Node>, name: &'a str, dest: &str) {
    if let Some(node) = nodes.get_mut(name) {
        if dest != "start" {
            node.add_link(dest);
        }
    } else {
        let mut node = Node::new(name);
        if dest != "start" {
            node.add_link(dest);
        }
        nodes.insert(name, node);
    }
}

fn make_nodes(lines: &Vec<String>) -> HashMap<&str, Node> {
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        if let Some(index) = line.find("-") {
            let left = &line[0..index];
            let right = &line[index + 1..];
            {
                add_node(&mut nodes, &left, &right);
            }
            {
                add_node(&mut nodes, &right, &left);
            }
        } else {
            panic!("Invalid line {}", line);
        }
    }

    nodes
}

fn process(input: &str, visit_twice: bool) {
    let lines = lines(input);
    let nodes = make_nodes(&lines);

    let mut paths = Vec::new();
    paths.push(String::from("start"));
    let mut finished = false;
    while !finished {
        finished = true;

        let mut new_paths: Vec<String> = Vec::new();
        for path in paths {
            let index = path.rfind(|c| c == ',' || c == '|').unwrap_or(0);
            let current = &path[if index == 0 { 0 } else { index + 1 }..];
            if current == "end" {
                new_paths.push(path);
                continue;
            }

            let node_option = nodes.get(current);
            if let Some(node) = node_option {
                for child in &node.links {
                    let mut sep = ",";
                    if *child == child.to_lowercase() && path.find(child).is_some() {
                        if !visit_twice || path.find('|').is_some() {
                            continue;
                        } else {
                            sep = "|";
                        }
                    }

                    let mut new_path = String::from(&path);
                    new_path.push_str(sep);
                    new_path.push_str(child);

                    new_paths.push(new_path);
                    finished = false;
                }
            } else {
                panic!("Failed to find node {}", current);
            }
        }

        paths = new_paths;
    }

    println!("{}", paths.len());
}

pub fn run() {
    let test_input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    println!("Test Input:");
    process(test_input, false);
    process(test_input, true);

    let test_case2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    println!("Test Input 2:");
    process(test_case2, false);
    process(test_case2, true);

    let test_case3 = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    println!("Test Input 3:");
    process(test_case3, false);
    process(test_case3, true);

    let input = fs::read_to_string("data/day12.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input, false);
    process(&input, true);
}
