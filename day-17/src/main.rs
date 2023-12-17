use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input<B: BufRead>(reader: B) -> Vec<Vec<usize>> {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.bytes().map(|c| (c - b'0') as usize).collect())
        .collect()
}

fn direction_to_index(direction: Direction) -> usize {
    match direction {
        Direction::Left | Direction::Right => 0,
        Direction::Up | Direction::Down => 1,
    }
}

fn find_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(map: &Vec<Vec<usize>>) -> usize {
    let (width, height) = (map[0].len(), map.len());
    // A distance-map that keeps track of the smallest distances we found.
    let mut distances: Vec<Vec<[usize; 2]>> = vec![vec![[usize::MAX; 2]; width]; height];
    distances[0][0] = [0, 0];
    // We also need to keep track of the next nodes to consider. The next node should
    // always be the node with the smallest distance.
    // We use a binary heap. This is a tree where the root is always the largest number.
    // We want the root to be the smallest number, so we reverse the ordering.
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), Direction::Right, (0, 0)));
    queue.push((Reverse(0), Direction::Down, (0, 0)));
    // While there is something in the queue, calculate new distances.
    // Stop if we arrived at the bottom right corner.
    while let Some((Reverse(distance), direction, (x, y))) = queue.pop() {
        // If we see the bottom right corner we can be sure that this is the fastest
        // way to get there because it was at the top of the binary heap.
        if (x, y) == (width - 1, height - 1) {
            return distance;
        }
        // Check if we already found a better way to this spot.
        if distance > distances[y][x][direction_to_index(direction)] {
            continue;
        }
        // Now we move in the two possible directions as far as possible.
        // We will not move in the same direction we came from because we already
        // moved there as far as we could.
        for new_direction in match direction {
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        } {
            // We move into one direction and add up the cost to get here.
            let mut acc_distance = distance;
            for steps in 1..=MAX_STEPS {
                let (new_x, new_y) = match new_direction {
                    Direction::Left => (x.wrapping_sub(steps), y),
                    Direction::Right => (x + steps, y),
                    Direction::Up => (x, y.wrapping_sub(steps)),
                    Direction::Down => (x, y + steps),
                };
                // Still inside the map?
                if new_x >= width || new_y >= height {
                    break;
                }
                acc_distance += map[new_y][new_x];
                // Have we done enough steps already?
                if steps >= MIN_STEPS {
                    // Check if we are better than the previous one.
                    // Also keep track of the direction we came from.
                    if acc_distance < distances[new_y][new_x][direction_to_index(new_direction)] {
                        // We found a better solution.
                        distances[new_y][new_x][direction_to_index(new_direction)] = acc_distance;
                        queue.push((Reverse(acc_distance), new_direction, (new_x, new_y)));
                    }
                }
            }
        }
    }
    unreachable!("We did not arrive at the bottom right corner!");
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let map = parse_input(reader);
    find_path::<0, 3>(&map)
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let map = parse_input(reader);
    find_path::<4, 10>(&map)
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
        assert_eq!(solve_first_task(reader), 102);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 94);
    }
}
