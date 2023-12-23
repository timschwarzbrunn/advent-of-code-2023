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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct HikingTrail {
    map: Vec<Vec<char>>,
    pos_start: (usize, usize),
    pos_end: (usize, usize),
}

impl HikingTrail {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        let map: Vec<Vec<char>> = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| line.chars().collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        // The start and end position are currently hard coded since they
        // are always at the same position (in the test and real input).
        let pos_start = (1, 0);
        let pos_end = (width - 2, height - 1);
        Self {
            map,
            pos_start,
            pos_end,
        }
    }

    fn calculate_longest_hike(&self, ignore_slopes: bool) -> usize {
        let mut longest_hike = 0;
        // We walk through the map and remember for each walk the following things:
        // - the current position
        // - the walking direction (to never go back).
        // - the current amount of steps
        // - a vector of nodes already visited (a node is a crossroad).
        // - the last node we visited
        // - the walking direction from the last node
        // - the steps from the last node we visited
        let mut possible_walks: Vec<(
            (usize, usize),
            Direction,
            usize,
            Vec<(usize, usize)>,
            (usize, usize),
            Direction,
            usize,
        )> = Vec::new();
        // Insert the starting position. We will go down.
        possible_walks.push((
            self.pos_start,
            Direction::Down,
            0,
            vec![self.pos_start],
            self.pos_start,
            Direction::Down,
            0,
        ));
        // Also for part two where we ignore the slopes: It will take a while for all
        // possibilites to walk through this maze. We can also create a lookup that saves
        // the last nodes position as well as the walking direction from the last node.
        // Then we will save for this also the next reachable node, so we can use this later.
        // Key: (pos_last_node, walking_direction)
        // Value: (pos_next_node, direction we came from, steps_to_take)
        // Remember to also insert the reverse direction.
        let mut lookup: HashMap<((usize, usize), Direction), ((usize, usize), Direction, usize)> =
            HashMap::new();
        while !possible_walks.is_empty() {
            let (
                mut pos,
                mut direction,
                mut steps,
                mut visited_nodes,
                mut pos_last_node,
                direction_from_last_node,
                mut steps_from_last_node,
            ) = possible_walks.pop().unwrap();
            // Check if we already were here and if we can use the lookup.
            if let Some((pos_next_node, new_direction, steps_to_this_node)) =
                lookup.get(&(pos, direction))
            {
                // We know this one, move directly to the next node.
                pos = *pos_next_node;
                direction = *new_direction;
                steps += steps_to_this_node;
            } else {
                // Take the next step.
                steps += 1;
                steps_from_last_node += 1;
                match direction {
                    Direction::Left => pos.0 -= 1,
                    Direction::Right => pos.0 += 1,
                    Direction::Up => pos.1 -= 1,
                    Direction::Down => pos.1 += 1,
                };
            }
            // If we are at the end position, remember this run.
            if pos == self.pos_end {
                longest_hike = longest_hike.max(steps);
                continue;
            }
            // Check if this is a node. It is a node if there are more than two possible
            // ways to go / to come from.
            if [
                self.map[pos.1][pos.0 - 1] != '#',
                self.map[pos.1][pos.0 + 1] != '#',
                self.map[pos.1 - 1][pos.0] != '#',
                self.map[pos.1 + 1][pos.0] != '#',
            ]
            .iter()
            .filter(|&&x| x == true)
            .count()
                > 2
            {
                // This is a node.
                // Check if we were already here.
                if visited_nodes.contains(&pos) {
                    // Loops are not allowed.
                    continue;
                }
                // Its the first time we visit this node.
                visited_nodes.push(pos);
                // Check if the lookup already contains the walk from the last node
                // to this node. If not, insert it and also the reverse direction.
                if !lookup.contains_key(&(pos_last_node, direction_from_last_node)) {
                    // We do not know this one yet. So save it.
                    lookup.insert(
                        (pos_last_node, direction_from_last_node),
                        (pos, direction, steps_from_last_node),
                    );
                    lookup.insert(
                        (pos, direction.rev()),
                        (
                            pos_last_node,
                            direction_from_last_node.rev(),
                            steps_from_last_node,
                        ),
                    );
                }
                // Nethertheless we are at a new node, so reset the position of the last node as well as the steps.
                pos_last_node = pos;
                steps_from_last_node = 0;
            }
            // Check where we are and where we can go.
            // Look left (if we did not come from there).
            if direction != Direction::Right {
                match self.map[pos.1][pos.0 - 1] {
                    '.' | '<' => {
                        possible_walks.push((
                            pos,
                            Direction::Left,
                            steps,
                            visited_nodes.clone(),
                            pos_last_node,
                            if steps_from_last_node > 0 {
                                direction_from_last_node
                            } else {
                                Direction::Left
                            },
                            steps_from_last_node,
                        ));
                    }
                    '>' => {
                        if ignore_slopes {
                            // Go there.
                            possible_walks.push((
                                pos,
                                Direction::Left,
                                steps,
                                visited_nodes.clone(),
                                pos_last_node,
                                if steps_from_last_node > 0 {
                                    direction_from_last_node
                                } else {
                                    Direction::Left
                                },
                                steps_from_last_node,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            // Look right (if we did not come from there).
            if direction != Direction::Left {
                match self.map[pos.1][pos.0 + 1] {
                    '.' | '>' => {
                        possible_walks.push((
                            pos,
                            Direction::Right,
                            steps,
                            visited_nodes.clone(),
                            pos_last_node,
                            if steps_from_last_node > 0 {
                                direction_from_last_node
                            } else {
                                Direction::Right
                            },
                            steps_from_last_node,
                        ));
                    }
                    '<' => {
                        if ignore_slopes {
                            possible_walks.push((
                                pos,
                                Direction::Right,
                                steps,
                                visited_nodes.clone(),
                                pos_last_node,
                                if steps_from_last_node > 0 {
                                    direction_from_last_node
                                } else {
                                    Direction::Right
                                },
                                steps_from_last_node,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            // Look up (if we did not come from there).
            if direction != Direction::Down {
                match self.map[pos.1 - 1][pos.0] {
                    '.' | '^' => {
                        possible_walks.push((
                            pos,
                            Direction::Up,
                            steps,
                            visited_nodes.clone(),
                            pos_last_node,
                            if steps_from_last_node > 0 {
                                direction_from_last_node
                            } else {
                                Direction::Up
                            },
                            steps_from_last_node,
                        ));
                    }
                    'v' => {
                        if ignore_slopes {
                            possible_walks.push((
                                pos,
                                Direction::Up,
                                steps,
                                visited_nodes.clone(),
                                pos_last_node,
                                if steps_from_last_node > 0 {
                                    direction_from_last_node
                                } else {
                                    Direction::Up
                                },
                                steps_from_last_node,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            // Look down (if we did not come from there).
            if direction != Direction::Up {
                match self.map[pos.1 + 1][pos.0] {
                    '.' | 'v' => {
                        possible_walks.push((
                            pos,
                            Direction::Down,
                            steps,
                            visited_nodes.clone(),
                            pos_last_node,
                            if steps_from_last_node > 0 {
                                direction_from_last_node
                            } else {
                                Direction::Down
                            },
                            steps_from_last_node,
                        ));
                    }
                    '^' => {
                        if ignore_slopes {
                            possible_walks.push((
                                pos,
                                Direction::Down,
                                steps,
                                visited_nodes.clone(),
                                pos_last_node,
                                if steps_from_last_node > 0 {
                                    direction_from_last_node
                                } else {
                                    Direction::Down
                                },
                                steps_from_last_node,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        longest_hike
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let hiking_trail = HikingTrail::from_reader(reader);
    hiking_trail.calculate_longest_hike(false)
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    println!("REMINDER: Check warping to known nodes. Something does not work properly.");
    let hiking_trail = HikingTrail::from_reader(reader);
    hiking_trail.calculate_longest_hike(true)
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
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 94);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 154);
    }
}
