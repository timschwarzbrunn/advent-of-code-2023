use rand::Rng;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

fn calculate_hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.bytes() {
        hash = hash * 100 + (c - b'a') as usize;
    }
    hash
}

#[derive(Debug)]
struct Graph {
    graph: BTreeMap<usize, Vec<usize>>,
}

impl Graph {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        let graph: BTreeMap<usize, Vec<usize>> = reader
            .lines()
            .map(Result::unwrap)
            .map(Graph::get_key_value_pair_from_line)
            .collect();
        Self { graph }
    }

    fn get_key_value_pair_from_line(line: String) -> (usize, Vec<usize>) {
        // Example line: cmg: qnr nvd lhk bvb
        let (part1, part2) = line.split_once(':').unwrap();
        let key = calculate_hash(part1);
        let values: Vec<usize> = part2
            .trim()
            .split_ascii_whitespace()
            .into_iter()
            .map(calculate_hash)
            .collect();
        (key, values)
    }

    fn make_bidirectional(&mut self) {
        // Go through all key value pairs and insert into all values values the key.
        let mut bidirectional_graph = self.graph.clone();
        for (key, values) in self.graph.iter() {
            for value in values {
                let neighbors = bidirectional_graph.entry(*value).or_insert(Vec::new());
                neighbors.push(*key);
            }
        }
        self.graph = bidirectional_graph;
    }

    fn remove_connections(&mut self, connections_to_cut: Vec<(usize, usize)>) {
        for (node_from, node_to) in connections_to_cut.iter() {
            let neighbors = self.graph.get_mut(node_from).unwrap();
            if let Some(idx) = neighbors.iter().position(|&x| x == *node_to) {
                neighbors.remove(idx);
            } else {
                panic!();
            }
            let neighbors = self.graph.get_mut(node_to).unwrap();
            if let Some(idx) = neighbors.iter().position(|&x| x == *node_from) {
                neighbors.remove(idx);
            } else {
                panic!();
            }
        }
    }

    fn count_nodes(&self, node_from: usize) -> usize {
        let mut node_count: HashSet<usize> = HashSet::new();
        let mut next_nodes: VecDeque<usize> = VecDeque::new();
        next_nodes.push_back(node_from);
        while let Some(next_node) = next_nodes.pop_front() {
            if node_count.contains(&next_node) {
                continue;
            }
            node_count.insert(next_node);
            self.graph
                .get(&next_node)
                .unwrap()
                .iter()
                .for_each(|&node| next_nodes.push_back(node));
        }
        node_count.len()
    }

    fn find_shortest_path(&self, node_from: usize, node_to: usize) -> Vec<usize> {
        let mut lookup: VecDeque<Vec<usize>> = VecDeque::new();
        lookup.push_back(vec![node_from]);
        while !lookup.is_empty() {
            let path = lookup.pop_front().unwrap();
            let neighbors = self.graph.get(&path.last().unwrap()).unwrap();
            for neighbor in neighbors.iter() {
                if path.contains(neighbor) {
                    // Found a loop.
                    continue;
                }
                // Create the new path.
                let mut new_path = path.clone();
                new_path.push(*neighbor);
                if *neighbor == node_to {
                    return new_path;
                }
                lookup.push_back(new_path);
            }
        }
        panic!("No path found.");
    }

    fn determine_three_connections_to_cut(&self) -> Vec<(usize, usize)> {
        let mut lookup: HashMap<(usize, usize), usize> = HashMap::new();
        for _ in 0..400 {
            let node_from = rand::thread_rng().gen_range(0..self.graph.len());
            let node_to = rand::thread_rng().gen_range(0..self.graph.len());
            if node_from == node_to {
                continue;
            }
            let node_from = self.graph.keys().nth(node_from).unwrap();
            let node_to = self.graph.keys().nth(node_to).unwrap();
            let path = self.find_shortest_path(*node_from, *node_to);
            for walk in path.windows(2) {
                let walk_count = lookup
                    .entry((walk[0].min(walk[1]), walk[0].max(walk[1])))
                    .or_insert(0);
                *walk_count += 1;
            }
        }

        // Sort them.
        let mut sorted_keys: Vec<_> = lookup.iter().map(|(&k, &v)| (k, v)).collect();
        sorted_keys.sort_by(|a, b| b.1.cmp(&a.1));

        // Take the top three keys.
        let top_three_keys: Vec<(usize, usize)> = sorted_keys
            .into_iter()
            .take(3)
            .map(|(k, _)| (k.0, k.1))
            .collect();

        top_three_keys
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    println!("Create graph from reader.");
    let mut graph = Graph::from_reader(reader);
    println!("Make graph bidirectional.");
    graph.make_bidirectional();
    println!("Determine three connections to cut.");
    let top_three = graph.determine_three_connections_to_cut();
    println!("Cut edges.");
    graph.remove_connections(top_three.clone());
    println!("Count nodes.");
    let node_count_1 = graph.count_nodes(top_three[0].0);
    let node_count_2 = graph.count_nodes(top_three[0].1);
    println!("{}, {}", node_count_1, node_count_2);
    node_count_1 * node_count_2
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    0
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
    fn test_first_task_subfunctions() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        let mut graph = Graph::from_reader(reader);
        graph.make_bidirectional();
        graph.remove_connections(vec![
            (calculate_hash("hfx"), calculate_hash("pzl")),
            (calculate_hash("bvb"), calculate_hash("cmg")),
            (calculate_hash("nvd"), calculate_hash("jqt")),
        ]);
        let node_count_1 = graph.count_nodes(calculate_hash("hfx"));
        let node_count_2 = graph.count_nodes(calculate_hash("pzl"));
        assert_eq!(node_count_1 * node_count_2, 54);
    }

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 54);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 0);
    }
}
