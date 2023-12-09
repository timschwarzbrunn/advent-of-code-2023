use gcd::Gcd;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from_line(line: String) -> Self {
        // Example line: AAA = (BBB, CCC)
        Node {
            name: line[..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        }
    }
}

impl FromIterator<Node> for HashMap<String, Node> {
    fn from_iter<I: IntoIterator<Item = Node>>(iter: I) -> Self {
        let mut map = HashMap::new();
        for node in iter {
            map.insert(node.name.clone(), node);
        }
        map
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().unwrap();
    let nodes: HashMap<String, Node> = lines
        .skip(1)
        .into_iter()
        .map(Result::unwrap)
        .map(Node::from_line)
        .collect();
    let mut result = 0;
    let mut idx_instruction = 0;
    let mut current_node_name = String::from("AAA");
    while current_node_name != "ZZZ" {
        result += 1;
        if let Some(node) = nodes.get(&current_node_name) {
            current_node_name = match instructions.chars().nth(idx_instruction).unwrap() {
                'L' => node.left.clone(),
                'R' => node.right.clone(),
                _ => unreachable!("Should only be L and R."),
            };
        }
        idx_instruction = (idx_instruction + 1) % instructions.len();
    }
    result
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().unwrap();
    let nodes: HashMap<String, Node> = lines
        .skip(1)
        .into_iter()
        .map(Result::unwrap)
        .map(Node::from_line)
        .collect();
    let mut idx_instruction = 0;
    let mut steps_to_reach_target: Vec<usize> = Vec::new();
    let mut current_node_names: Vec<String> = nodes
        .keys()
        .clone()
        .map(|key| key.to_string())
        .filter(|key| key.ends_with('A'))
        .collect();
    let mut steps = 0;
    loop {
        steps += 1;
        let mut new_node_names = Vec::new();
        for node_name in &current_node_names {
            if let Some(node) = nodes.get(node_name) {
                new_node_names.push(match instructions.chars().nth(idx_instruction).unwrap() {
                    'L' => node.left.clone(),
                    'R' => node.right.clone(),
                    _ => unreachable!("Should only be L and R."),
                });
            }
        }
        current_node_names = new_node_names;
        let mut idx = 0;
        while idx < current_node_names.len() {
            if current_node_names[idx].ends_with('Z') {
                current_node_names.remove(idx);
                steps_to_reach_target.push(steps);
            } else {
                idx += 1;
            }
        }
        if current_node_names.is_empty() {
            break;
        }
        idx_instruction = (idx_instruction + 1) % instructions.len();
    }
    steps_to_reach_target
        .into_iter()
        .reduce(|total, number| number / total.gcd_binary(number) * total)
        .unwrap()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let filename = args.next().unwrap_or_else(|| String::from("./input"));
    let task = args
        .next()
        .map(|arg| match arg.as_str() {
            "first" => Task::First,
            "second" => Task::Second,
            _ => unreachable!(),
        })
        .unwrap_or_default();
    let reader = BufReader::new(File::open(filename).expect("Input file not found."));

    println!(
        "{:?} task solution: {:?}",
        task,
        match task {
            Task::First => solve_first_task(reader),
            Task::Second => solve_second_task(reader),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task_input_1() {
        let reader = BufReader::new(File::open("./input1_1.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 2);
    }

    #[test]
    fn test_first_task_input_2() {
        let reader = BufReader::new(File::open("./input1_2.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 6);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input2.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 6);
    }
}
