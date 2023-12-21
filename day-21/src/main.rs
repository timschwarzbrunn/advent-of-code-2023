use std::{
    collections::{HashMap, HashSet},
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
struct MapLocation {
    pos: (usize, usize),
    map_id: (isize, isize),
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>, // '.' is reachable, '#' is not reachable.
    loc_start: MapLocation,
    width: usize,
    height: usize,
    number_of_obstacles_of_the_start: usize,
}

impl Map {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        let mut map: Vec<Vec<u8>> = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| line.bytes().collect())
            .collect();
        // Find the start position.
        let pos_start = map
            .iter()
            .enumerate()
            .find_map(|(row, line)| {
                line.iter()
                    .position(|&byte| byte == b'S')
                    .map(|col| (col, row))
            })
            .unwrap();
        // Change the value of the start position within the map from b'S' to b'.'.
        map[pos_start.1][pos_start.0] = b'.';
        // Find the number of obstacles that lay in the way of the starting position.
        // This only means vertical and horizontal. The efficient algorithm needs to
        // be able to cast straight lines from the starting position to all four directions.
        let mut number_of_obstacles_of_the_start = 0;
        for (row, line) in map.iter().enumerate() {
            for (col, &b) in line.iter().enumerate() {
                if (row == pos_start.1 || col == pos_start.0) && b == b'#' {
                    number_of_obstacles_of_the_start += 1;
                }
            }
        }
        let width = map[0].len();
        let height = map.len();
        Self {
            map,
            loc_start: MapLocation {
                pos: pos_start,
                map_id: (0, 0),
            },
            width,
            height,
            number_of_obstacles_of_the_start,
        }
    }

    fn print_information(&self, plot_map: bool) {
        println!(
            "Map of the size {}x{} with the start at {:?}",
            self.width, self.height, self.loc_start.pos
        );
        if plot_map {
            for line in self.map.iter() {
                for &b in line.iter() {
                    print!("{}", b as char);
                }
                println!("");
            }
        }
        println!(
            "Number of obstacles in the way of the starting position (horizontal and vertical): {}",
            self.number_of_obstacles_of_the_start
        );
    }

    fn get_neighbors(&self, loc: MapLocation, repeat_map: bool) -> Vec<MapLocation> {
        let mut neighbors: Vec<MapLocation> = Vec::new();
        // Left.
        if loc.pos.0 > 0 {
            // We stay on the same map_id.
            if self.map[loc.pos.1][loc.pos.0 - 1] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0 - 1, loc.pos.1),
                    map_id: loc.map_id,
                });
            }
        } else if repeat_map {
            // Wrap around. We go to the left.
            if self.map[loc.pos.1][self.width - 1] == b'.' {
                neighbors.push(MapLocation {
                    pos: (self.width - 1, loc.pos.1),
                    map_id: (loc.map_id.0 - 1, loc.map_id.1),
                });
            }
        }
        // Right.
        if loc.pos.0 < self.width - 1 {
            // We stay on the same map_id.
            if self.map[loc.pos.1][loc.pos.0 + 1] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0 + 1, loc.pos.1),
                    map_id: loc.map_id,
                });
            }
        } else if repeat_map {
            // Wrap around. We go to the right.
            if self.map[loc.pos.1][0] == b'.' {
                neighbors.push(MapLocation {
                    pos: (0, loc.pos.1),
                    map_id: (loc.map_id.0 + 1, loc.map_id.1),
                });
            }
        }
        // Up.
        if loc.pos.1 > 0 {
            // We stay on the same map_id.
            if self.map[loc.pos.1 - 1][loc.pos.0] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0, loc.pos.1 - 1),
                    map_id: loc.map_id,
                });
            }
        } else if repeat_map {
            // Wrap around. We go to the top.
            if self.map[self.height - 1][loc.pos.0] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0, self.height - 1),
                    map_id: (loc.map_id.0, loc.map_id.1 - 1),
                });
            }
        }
        // Down.
        if loc.pos.1 < self.height - 1 {
            // We stay on the same map_id.
            if self.map[loc.pos.1 + 1][loc.pos.0] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0, loc.pos.1 + 1),
                    map_id: loc.map_id,
                });
            }
        } else if repeat_map {
            // Wrap around. We go to the bottom.
            if self.map[0][loc.pos.0] == b'.' {
                neighbors.push(MapLocation {
                    pos: (loc.pos.0, 0),
                    map_id: (loc.map_id.0, loc.map_id.1 + 1),
                });
            }
        }
        neighbors
    }

    fn get_number_of_possible_positions(&self, number_of_steps: usize, repeat_map: bool) -> usize {
        // If we repeat the map and want to take steps smaller than the 2.5x size of
        // the map, we simply bruteforce. Otherwise we bruteforce until we have 2.5x
        // size of the map and then we extrapolate.
        // Helper variables for this:
        let mut lookup: [usize; 3] = [0; 3];
        let mut max_steps_before_interpolation = ((self.width - 1) / 2) + 2 * self.width;
        // This solution also only works if the number of steps results in full outer maps.
        if number_of_steps < max_steps_before_interpolation {
            // In this case the interpolation cannot be applied but on the other hand would not be faster since we would need at least max_steps_before_interpolation.
        } else if (number_of_steps - ((self.width - 1) / 2)) % self.width != 0 {
            println!("WARNING: These amount of steps can currently only be bruteforced.");
            println!("To use a more efficient algorithm, set the number of steps to (width - 1) + n * width for all n.");
            max_steps_before_interpolation = number_of_steps;
        } else if self.width != self.height {
            println!("WARNING: The efficient algorithm only works for square maps nxn. This map has the size: {}x{}.", self.width, self.height);
            max_steps_before_interpolation = number_of_steps;
        } else if self.number_of_obstacles_of_the_start != 0 {
            println!("WARNING: The efficient algorithm only works if there a no obstacles to the top, right, bottom and left of the start position. It needs to cast straight lines in these four directions.");
            max_steps_before_interpolation = number_of_steps;
        } else if repeat_map == false {
            // The algorithm only makes sense when the map repeats itself.
            max_steps_before_interpolation = number_of_steps;
        }
        let mut possible_locations: HashSet<MapLocation> = HashSet::new();
        // Start at the starting position and then take the number of steps.
        possible_locations.insert(self.loc_start);
        for step_count in 1..=number_of_steps.min(max_steps_before_interpolation) {
            let mut next_locations: Vec<MapLocation> = Vec::new();
            for &loc in possible_locations.iter() {
                next_locations.append(&mut self.get_neighbors(loc, repeat_map));
            }
            possible_locations = next_locations.into_iter().collect();
            // Maybe we need to save this.
            if step_count == (self.width - 1) / 2 {
                lookup[0] = possible_locations.len();
            } else if step_count == ((self.width - 1) / 2) + self.width {
                lookup[1] = possible_locations.len();
            } else if step_count == ((self.width - 1) / 2) + 2 * self.width {
                lookup[2] = possible_locations.len();
            }
        }
        if number_of_steps <= max_steps_before_interpolation {
            return possible_locations.len();
        } else {
            let diff_lookup: [usize; 3] = [lookup[0], lookup[1] - lookup[0], lookup[2] - lookup[1]];
            // We want to interpolate the whole map and not the steps.
            let factor = number_of_steps / self.width;
            // Quadratic polynomial interpolation.
            // f(n) = b0 + b1 * n + (n * (n − 1)) / 2​* (b2 − b1)
            return diff_lookup[0]
                + diff_lookup[1] * factor
                + (factor * (factor - 1) / 2) * (diff_lookup[2] - diff_lookup[1]);
        }
    }
}

fn solve_task<B: BufRead>(reader: B, number_of_steps: usize, repeat_map: bool) -> usize {
    let map = Map::from_reader(reader);
    map.print_information(false);
    map.get_number_of_possible_positions(number_of_steps, repeat_map)
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
            Task::First => solve_task(reader, 64, false),
            Task::Second => solve_task(reader, 26501365, true),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 6, false), 16);
    }

    #[test]
    fn test_second_task_6_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 6, true), 16);
    }

    #[test]
    fn test_second_task_10_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 10, true), 50);
    }

    #[test]
    fn test_second_task_50_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 50, true), 1594);
    }

    #[test]
    fn test_second_task_100_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 100, true), 6536);
    }

    #[test]
    fn test_second_task_500_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 500, true), 167004);
    }

    #[test]
    fn test_second_task_1000_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 1000, true), 668697);
    }

    #[test]
    fn test_second_task_5000_steps() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 5000, true), 16733044);
    }
}
