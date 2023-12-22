use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
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
struct Bricks {
    brick_supports: HashMap<usize, HashSet<usize>>,
    brick_is_supported_by: HashMap<usize, HashSet<usize>>,
    bricks_z_location: BTreeSet<(usize, usize)>,
}

impl Bricks {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        let mut stack: HashMap<(usize, usize), Vec<Option<usize>>> = HashMap::new();
        let mut brick_supports: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut brick_is_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut bricks_z_location: BTreeSet<(usize, usize)> = BTreeSet::new();
        // The test input works directly with this approach but the real input
        // is not as sorted as the test input. So we will first read in the input,
        // sort it by the z value and then apply the algorithm.
        let ordered_bricks: BTreeSet<(usize, ((usize, usize, usize), (usize, usize, usize)))> =
            reader
                .lines()
                .map(Result::unwrap)
                .map(Bricks::parse_line)
                .collect();
        for (brick_id, (_z_min, (pos1, pos2))) in ordered_bricks.iter().enumerate() {
            // Get the height of all the piles for each (x, y) position.
            let mut max_height = 0;
            for x in pos1.0.min(pos2.0)..=pos1.0.max(pos2.0) {
                for y in pos1.1.min(pos2.1)..=pos1.1.max(pos2.1) {
                    let pile = stack.entry((x, y)).or_insert(Vec::new());
                    max_height = max_height.max(pile.len());
                }
            }
            // Now we know the max height of the underlying piles, so we can insert the brick.
            let brick_height = pos1.2.max(pos2.2) - pos1.2.min(pos2.2) + 1;
            // Save the min z value of this brick together with its id.
            bricks_z_location.insert((max_height + 1, brick_id));
            // For this brick we may need to insert some bricks that support this brick.
            let supporter_bricks_of_current_brick = brick_is_supported_by
                .entry(brick_id)
                .or_insert(HashSet::new());
            // Even if this brick is not supporting anything, we need its entry.
            brick_supports.insert(brick_id, HashSet::new());
            for x in pos1.0.min(pos2.0)..=pos1.0.max(pos2.0) {
                for y in pos1.1.min(pos2.1)..=pos1.1.max(pos2.1) {
                    let pile = stack.entry((x, y)).or_insert(Vec::new());
                    // Fill the pile until the desired height is reached with air.
                    if pile.len() < max_height {
                        pile.resize_with(max_height, || None);
                    }
                    // If we have a vertical brick, we will need to insert it multiple times.
                    for _ in 0..brick_height {
                        pile.push(Some(brick_id));
                    }
                    // Check if there is another brick below that supports this brick.
                    if max_height > 0 {
                        if let Some(brick_below_id) = pile.get(max_height - 1).unwrap() {
                            // The brick below supports this brick.
                            // So insert this brick into the list of the brick below.
                            let supported_bricks_of_brick_below = brick_supports
                                .entry(*brick_below_id)
                                .or_insert(HashSet::new());
                            supported_bricks_of_brick_below.insert(brick_id);
                            // This brick is supported by the brick below.
                            // So insert the brick below in this bricks list.
                            supporter_bricks_of_current_brick.insert(*brick_below_id);
                        }
                    }
                }
            }
        }
        Self {
            brick_supports,
            brick_is_supported_by,
            bricks_z_location,
        }
    }

    fn parse_line(line: String) -> (usize, ((usize, usize, usize), (usize, usize, usize))) {
        // Example line: 1,0,1~1,2,1
        // Returns this coordinates as well as the min z value for sorting.
        let (pos1, pos2) = line.split_once('~').unwrap();
        let pos1: Vec<usize> = pos1
            .split(',')
            .into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let pos2: Vec<usize> = pos2
            .split(',')
            .into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let pos1 = (
            pos1.get(0).unwrap().clone(),
            pos1.get(1).unwrap().clone(),
            pos1.get(2).unwrap().clone(),
        );
        let pos2 = (
            pos2.get(0).unwrap().clone(),
            pos2.get(1).unwrap().clone(),
            pos2.get(2).unwrap().clone(),
        );
        // z-min, pos1, pos2.
        (pos1.2.min(pos2.2), (pos1, pos2))
    }

    fn get_number_of_bricks_safe_to_disintegrate(&self) -> usize {
        let mut safe_to_disintegrate_count = 0;
        // The brick_supports hashmap contains for each brick (specified by the brick_id),
        // what bricks lay above it and are supported by this brick.
        for (brick_id, supported_bricks_of_current_brick) in self.brick_supports.iter() {
            // Check for all bricks this brick supports, if it is the only brick
            // that supports the other brick. If so, we can not disintegrate it.
            let mut can_be_disintegrated = true;
            for supported_brick in supported_bricks_of_current_brick.iter() {
                // We know which brick is supported by our current brick.
                // Now check for this supported brick if there is another brick
                // which supports it. If so, we could disintegrate our current brick.
                let brick_is_supported_by =
                    self.brick_is_supported_by.get(supported_brick).unwrap();
                if brick_is_supported_by.len() == 0 {
                    panic!("There should be atleast one brick supporting this brick.");
                } else if brick_is_supported_by.contains(brick_id) == false {
                    panic!("This brick should be supported by our current brick.");
                } else if brick_is_supported_by.len() == 1 {
                    // This brick is only supported by our current brick, so we cannot
                    // disintegrate it.
                    can_be_disintegrated = false;
                }
            }
            if can_be_disintegrated == true {
                safe_to_disintegrate_count += 1;
            }
        }
        safe_to_disintegrate_count
    }

    fn count_chain_reaction(&self, brick_id: usize) -> usize {
        let mut currently_removed_bricks: HashSet<usize> = HashSet::new();
        currently_removed_bricks.insert(brick_id);
        let mut bricks_to_consider: VecDeque<usize> = self
            .brick_supports
            .get(&brick_id)
            .unwrap()
            .clone()
            .into_iter()
            .collect();
        while !bricks_to_consider.is_empty() {
            // Get the next brick from the FIFO stack. Make sure to keep the order.
            // The bricks are inserted with increasing z location, so the bricks
            // eventually holding further bricks are processed first.
            let current_brick = bricks_to_consider.pop_front().unwrap();
            let supporting_bricks_of_current_brick =
                self.brick_is_supported_by.get(&current_brick).unwrap();
            let mut will_fall = true;
            for supporting_brick in supporting_bricks_of_current_brick.iter() {
                if !currently_removed_bricks.contains(supporting_brick) {
                    will_fall = false;
                    break;
                }
            }
            if will_fall == true {
                bricks_to_consider.extend(
                    self.brick_supports
                        .get(&current_brick)
                        .unwrap()
                        .clone()
                        .iter(),
                );
                currently_removed_bricks.insert(current_brick);
            }
        }
        currently_removed_bricks.len() - 1
    }

    fn count_total_chain_reaction(&self) -> usize {
        self.bricks_z_location
            .iter()
            .map(|(_z_min, brick_id)| self.count_chain_reaction(*brick_id))
            .sum()
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let bricks = Bricks::from_reader(reader);
    bricks.get_number_of_bricks_safe_to_disintegrate()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let bricks = Bricks::from_reader(reader);
    bricks.count_total_chain_reaction()
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
    fn test_line_parsing() {
        assert_eq!(
            Bricks::parse_line(String::from("1,1,1~10,10,10")),
            (1, ((1, 1, 1), (10, 10, 10)))
        );
        assert_eq!(
            Bricks::parse_line(String::from("1,1,100~10,10,10")),
            (10, ((1, 1, 100), (10, 10, 10)))
        );
    }

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 5);
    }

    #[test]
    fn test_first_task_own_input() {
        let reader = BufReader::new(File::open("./input_own.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 3);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 7);
    }
}
