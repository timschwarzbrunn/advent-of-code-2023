use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

#[derive(Debug, Default, Copy, Clone)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeType {
    Vertical,
    CornerLeftUp,
    CornerRightUp,
    CornerLeftDown,
    CornerRightDown,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
struct TrenchEdge {
    x: isize,
    edge_type: EdgeType,
}

#[derive(Debug)]
struct Trench {
    map: HashMap<isize, Vec<TrenchEdge>>,
}

fn get_edge_type(direction1: Direction, direction2: Direction) -> EdgeType {
    match (direction1, direction2) {
        (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => {
            EdgeType::CornerRightUp
        }
        (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
            EdgeType::CornerLeftUp
        }
        (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right) => {
            EdgeType::CornerRightDown
        }
        (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => {
            EdgeType::CornerLeftDown
        }
        (Direction::Unknown, _) => EdgeType::Unknown,
        _ => unreachable!(),
    }
}

impl FromIterator<(Direction, isize)> for Trench {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Direction, isize)>,
    {
        // Our result we build from the iterator.
        let mut map: HashMap<isize, Vec<TrenchEdge>> = HashMap::new();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut prev_direction = Direction::Unknown;
        // At the end we need to set the first direction.
        let mut first_direction = Direction::Unknown;
        let mut idx_instruction = 0;

        for (direction, steps) in iter {
            // Save the first direction and color for later use.
            if idx_instruction == 0 {
                first_direction = direction;
            }
            idx_instruction += 1;
            // Determine what kind of corner we have here.
            let edge_type = get_edge_type(prev_direction, direction);
            // Set the corner at the current position but only if we know the type.
            if edge_type != EdgeType::Unknown {
                map.entry(y)
                    .or_insert_with(Vec::new)
                    .push(TrenchEdge { x, edge_type });
            }
            // If we are moving up or down we will also need the vertical edges.
            if direction == Direction::Up || direction == Direction::Down {
                for _ in 0..steps - 1 {
                    if direction == Direction::Up {
                        y -= 1;
                    } else {
                        y += 1;
                    }
                    map.entry(y).or_insert_with(Vec::new).push(TrenchEdge {
                        x,
                        edge_type: EdgeType::Vertical,
                    });
                }
                // Last step that is not going to be inserted.
                if direction == Direction::Up {
                    y -= 1;
                } else {
                    y += 1;
                }
            } else if direction == Direction::Left {
                x -= steps;
            } else {
                x += steps;
            }

            // Next one.
            prev_direction = direction;
        }
        // Get the first corner for position (0, 0).
        let edge_type = get_edge_type(prev_direction, first_direction);
        map.entry(0)
            .or_insert_with(Vec::new)
            .push(TrenchEdge { x: 0, edge_type });

        // Sort the map.
        for (_, edges) in map.iter_mut() {
            edges.sort_by(|a, b| a.x.cmp(&b.x));
        }

        Trench { map }
    }
}

impl Trench {
    fn count_cubic_meters(&self) -> isize {
        let mut result = 0;
        for (_, edges) in self.map.iter() {
            // We will not process the first edge since we already know that there is
            // one, so we add one per line for the first edge.
            result += 1;
            let mut number_of_edges = 1;
            for window in edges.windows(2) {
                // We always count the second edge itself.
                result += 1;
                match (window[0].edge_type, window[1].edge_type) {
                    (EdgeType::CornerRightUp, EdgeType::CornerLeftUp)
                    | (EdgeType::CornerRightUp, EdgeType::CornerLeftDown)
                    | (EdgeType::CornerRightDown, EdgeType::CornerLeftUp)
                    | (EdgeType::CornerRightDown, EdgeType::CornerLeftDown) => {
                        // In these cases the points are always inside.
                        result += window[1].x - window[0].x - 1;
                    }
                    _ => {
                        // In these cases we need to check if the number of walls is odd.
                        if number_of_edges % 2 == 1 {
                            result += window[1].x - window[0].x - 1;
                        }
                    }
                };
                // Check if the number of edges changed.
                number_of_edges += match (window[0].edge_type, window[1].edge_type) {
                    // Left Up.
                    (EdgeType::CornerLeftUp, EdgeType::CornerRightUp) => 1,
                    (EdgeType::CornerLeftUp, EdgeType::CornerRightDown) => 1,
                    (EdgeType::CornerLeftUp, EdgeType::Vertical) => 1,
                    // Left Down.
                    (EdgeType::CornerLeftDown, EdgeType::CornerRightUp) => 1,
                    (EdgeType::CornerLeftDown, EdgeType::CornerRightDown) => 1,
                    (EdgeType::CornerLeftDown, EdgeType::Vertical) => 1,
                    // Right Up.
                    (EdgeType::CornerRightUp, EdgeType::CornerLeftUp) => 1,
                    (EdgeType::CornerRightUp, EdgeType::CornerLeftDown) => 0,
                    // Right Down.
                    (EdgeType::CornerRightDown, EdgeType::CornerLeftUp) => 0,
                    (EdgeType::CornerRightDown, EdgeType::CornerLeftDown) => 1,
                    // Vertical.
                    (EdgeType::Vertical, EdgeType::CornerRightUp) => 1,
                    (EdgeType::Vertical, EdgeType::CornerRightDown) => 1,
                    (EdgeType::Vertical, EdgeType::Vertical) => 1,
                    _ => unreachable!(),
                };
            }
        }
        result
    }
}

fn parse_line(line: String, task: Task) -> (Direction, isize) {
    // Example line: R 6 (#70c710)
    let mut parts = line.split_ascii_whitespace();
    match task {
        Task::First => {
            let direction = match parts.next().unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!("Unknown direction found in parse_line."),
            };
            let steps = parts.next().unwrap().parse::<isize>().unwrap();
            return (direction, steps);
        }
        Task::Second => {
            let color_string = parts.nth(2).unwrap();
            let steps = isize::from_str_radix(&color_string[2..7], 16).unwrap();
            // "0 means R, 1 means D, 2 means L, and 3 means U"
            let direction = match &color_string[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };
            return (direction, steps);
        }
    }
}

fn solve_task_raycast<B: BufRead>(reader: B, task: Task) -> usize {
    let trench: Trench = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_line(line, task))
        .collect();
    trench.count_cubic_meters() as usize
}

fn solve_task_shoelace<B: BufRead>(reader: B, task: Task) -> usize {
    let mut area = 0;
    let instructions: Vec<(Direction, isize)> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_line(line, task))
        .collect();
    let (mut x, mut y): (isize, isize) = (0, 0);
    let mut perimeter = 0;
    for (idx, (direction, steps)) in instructions.iter().enumerate() {
        perimeter += steps;
        let (mut x_new, mut y_new) = (x, y);
        match direction {
            Direction::Left => x_new -= steps,
            Direction::Right => x_new += steps,
            Direction::Up => y_new -= steps,
            Direction::Down => y_new += steps,
            _ => unreachable!(),
        }
        if idx < instructions.len() - 1 {
            area += x * y_new - x_new * y;
        }
        x = x_new;
        y = y_new;
    }
    let result = (isize::abs(area) / 2) as usize + (perimeter / 2) as usize + 1;
    result
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
        solve_task_shoelace(reader, task)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task_raycast() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task_raycast(reader, Task::First), 62);
    }

    #[test]
    fn test_first_task_shoelace() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task_shoelace(reader, Task::First), 62);
    }

    #[test]
    fn test_first_task_large_raycast() {
        let reader =
            BufReader::new(File::open("./input_large.test").expect("Input file not found."));
        assert_eq!(solve_task_raycast(reader, Task::First), 39194);
    }

    #[test]
    fn test_first_task_large_shoelace() {
        let reader =
            BufReader::new(File::open("./input_large.test").expect("Input file not found."));
        assert_eq!(solve_task_shoelace(reader, Task::First), 39194);
    }

    #[test]
    fn test_second_task_raycast() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task_raycast(reader, Task::Second), 952408144115);
    }

    #[test]
    fn test_second_task_shoelace() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task_shoelace(reader, Task::Second), 952408144115);
    }
}
