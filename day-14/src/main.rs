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
struct Platform {
    pattern: Vec<Vec<char>>,
}

impl Platform {
    fn from_reader<B: BufRead>(reader: B) -> Self {
        Self {
            pattern: reader
                .lines()
                .map(Result::unwrap)
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    fn transpose(&mut self) {
        self.pattern = (0..self.pattern[0].len())
            .map(|col_index| {
                (0..self.pattern.len())
                    .map(|row_index| self.pattern[row_index][col_index])
                    .collect()
            })
            .map(|column: Vec<char>| column.into_iter().collect())
            .collect()
    }

    fn mirror_horizontal(&mut self) {
        self.pattern.reverse();
    }

    fn mirror_vertical(&mut self) {
        for line in self.pattern.iter_mut() {
            line.reverse();
        }
    }

    fn let_line_roll(line: &Vec<char>) -> Vec<char> {
        let mut next_free_position = 0;
        let mut result: Vec<char> = vec!['.'; line.len()];
        for (idx, c) in line.iter().enumerate() {
            match c {
                'O' => {
                    result[next_free_position] = 'O';
                    next_free_position += 1;
                }
                '#' => {
                    result[idx] = '#';
                    next_free_position = idx + 1;
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
        result
    }

    fn let_rocks_roll_to_the_left(&mut self) {
        self.pattern = self
            .pattern
            .iter()
            .map(|line| Platform::let_line_roll(&line))
            .collect();
    }

    fn count_weight_on_northern_support_beams(&self) -> usize {
        let mut result = 0;
        for line in self.pattern.iter() {
            for (idx, &c) in line.iter().enumerate() {
                if c == 'O' {
                    result += line.len() - idx;
                }
            }
        }
        result
    }

    fn print(&self) {
        for line in self.pattern.iter() {
            for c in line.iter() {
                print!("{}", c);
            }
            println!("");
        }
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut platform = Platform::from_reader(reader);
    platform.transpose();
    platform.let_rocks_roll_to_the_left();
    platform.count_weight_on_northern_support_beams()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut platform = Platform::from_reader(reader);
    let mut lookup: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    // Create the east orientation.
    platform.mirror_vertical();
    platform.mirror_horizontal();
    // Starting from the northern orientation, repeat the cycles.
    let total_number_of_cycles = 4 * 1_000_000_000;
    let mut found_inner_cycle = false;
    let mut cycle = 0;
    while cycle < total_number_of_cycles {
        platform.transpose();
        platform.mirror_vertical();
        platform.let_rocks_roll_to_the_left();
        // Check if we already saw this state.
        if found_inner_cycle == false {
            if let Some(&old_cycle) = lookup.get(&platform.pattern) {
                // Nice, we already saw this. We can skip some.
                let cycle_length = cycle - old_cycle;
                let cycle_repeat = (total_number_of_cycles - old_cycle) / cycle_length;
                cycle = old_cycle + cycle_repeat * cycle_length + 1;
                found_inner_cycle = true;
            } else {
                lookup.insert(platform.pattern.clone(), cycle);
                cycle += 1;
            }
        } else {
            cycle += 1;
        }
    }
    // Get it back to an orientation that can calculate the value.
    // North needs to be at the left.
    platform.transpose();
    platform.mirror_vertical();
    platform.count_weight_on_northern_support_beams()
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
        assert_eq!(solve_first_task(reader), 136);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 64);
    }
}
