use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

fn get_new_position(
    pos: (usize, usize),
    prev_pos: (usize, usize),
    c: char,
) -> ((usize, usize), bool) {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    match c {
        '|' => (
            match prev_pos.1 < pos.1 {
                true => (pos.0, pos.1 + 1),
                false => (pos.0, pos.1 - 1),
            },
            false,
        ),
        '-' => (
            match prev_pos.0 < pos.0 {
                true => (pos.0 + 1, pos.1),
                false => (pos.0 - 1, pos.1),
            },
            false,
        ),
        'L' => (
            match pos.0 == prev_pos.0 {
                true => (pos.0 + 1, pos.1),
                false => (pos.0, pos.1 - 1),
            },
            false,
        ),
        'J' => (
            match pos.0 == prev_pos.0 {
                true => (pos.0 - 1, pos.1),
                false => (pos.0, pos.1 - 1),
            },
            false,
        ),
        '7' => (
            match pos.0 == prev_pos.0 {
                true => (pos.0 - 1, pos.1),
                false => (pos.0, pos.1 + 1),
            },
            false,
        ),
        'F' => (
            match pos.0 == prev_pos.0 {
                true => (pos.0 + 1, pos.1),
                false => (pos.0, pos.1 + 1),
            },
            false,
        ),
        'S' => (pos, true),
        _ => unreachable!(),
    }
}

fn get_start_pipe(c_north: char, c_east: char, c_south: char, c_west: char) -> char {
    match (
        "|7F".contains(c_north),
        "-7J".contains(c_east),
        "|LJ".contains(c_south),
        "-LF".contains(c_west),
    ) {
        (true, true, false, false) => 'L',
        (true, false, true, false) => '|',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '-',
        (false, false, true, true) => '7',
        _ => unreachable!(),
    }
}

fn get_first_next_position(starting_pos: (usize, usize), c: char) -> (usize, usize) {
    match c {
        '|' => (starting_pos.0, starting_pos.1 + 1),
        '-' => (starting_pos.0 + 1, starting_pos.1),
        'L' => (starting_pos.0 + 1, starting_pos.1),
        'J' => (starting_pos.0 - 1, starting_pos.1),
        '7' => (starting_pos.0 - 1, starting_pos.1),
        'F' => (starting_pos.0 + 1, starting_pos.1),
        _ => unreachable!(),
    }
}

fn get_start_position(lines: &Vec<String>) -> (usize, usize) {
    for (row, line) in lines.iter().enumerate() {
        if let Some(col) = line.find('S') {
            return (col, row);
        }
    }
    unreachable!()
}

fn get_pipe_path(lines: &mut Vec<String>, replace_start_pipe: bool) -> Vec<(usize, usize)> {
    // Find the start position.
    let start_position = get_start_position(lines);
    // Determine in which direction to walk at the start.
    let c_north = match start_position.1 > 0 {
        true => lines
            .get(start_position.1 - 1)
            .unwrap_or(&String::from("."))
            .chars()
            .nth(start_position.0)
            .unwrap_or('.'),
        false => '.',
    };
    let c_east = match start_position.0 < lines.get(0).unwrap().len() - 1 {
        true => lines
            .get(start_position.1)
            .unwrap_or(&String::from("."))
            .chars()
            .nth(start_position.0 + 1)
            .unwrap_or('.'),
        false => '.',
    };
    let c_south = match start_position.1 < lines.len() - 1 {
        true => lines
            .get(start_position.1 + 1)
            .unwrap_or(&String::from("."))
            .chars()
            .nth(start_position.0)
            .unwrap_or('.'),
        false => '.',
    };
    let c_west = match start_position.0 > 0 {
        true => lines
            .get(start_position.1)
            .unwrap_or(&String::from("."))
            .chars()
            .nth(start_position.0 - 1)
            .unwrap_or('.'),
        false => '.',
    };
    let mut prev_pos = start_position;
    let start_pipe = get_start_pipe(c_north, c_east, c_south, c_west);
    let mut pos = get_first_next_position(start_position, start_pipe);
    // Extract all positions that are part of the loop.
    let mut path: Vec<(usize, usize)> = vec![start_position, pos];
    loop {
        let c = lines
            .get(pos.1)
            .unwrap_or(&String::from("."))
            .chars()
            .nth(pos.0)
            .unwrap_or('.');
        let (next_pos, reached_start) = get_new_position(pos, prev_pos, c);
        prev_pos = pos;
        pos = next_pos;
        if reached_start {
            break;
        }
        path.push(pos);
    }
    if replace_start_pipe {
        lines.get_mut(start_position.1).unwrap().replace_range(
            start_position.0..start_position.0 + 1,
            start_pipe.to_string().as_str(),
        );
    }
    path
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let path = get_pipe_path(&mut lines, false);
    path.len() / 2
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let path = get_pipe_path(&mut lines, true);
    let mut result = 0;
    // Ray casting algorithm.
    for (row, line) in lines.iter().enumerate() {
        // Replace the horizontal lines but keep their length.
        let pattern_l7 = Regex::new(r"L-*7").unwrap();
        let pattern_lj = Regex::new(r"L-*J").unwrap();
        let pattern_f7 = Regex::new(r"F-*7").unwrap();
        let pattern_fj = Regex::new(r"F-*J").unwrap();
        let line = pattern_l7.replace_all(&line, |caps: &regex::Captures| {
            let matched_length = caps[0].len();
            "|".to_string() + &".".repeat(matched_length - 1)
        });
        let line = pattern_lj.replace_all(&line, |caps: &regex::Captures| {
            let matched_length = caps[0].len();
            "||".to_string() + &".".repeat(matched_length - 2)
        });
        let line = pattern_f7.replace_all(&line, |caps: &regex::Captures| {
            let matched_length = caps[0].len();
            "||".to_string() + &".".repeat(matched_length - 2)
        });
        let line = pattern_fj.replace_all(&line, |caps: &regex::Captures| {
            let matched_length = caps[0].len();
            "|".to_string() + &".".repeat(matched_length - 1)
        });
        let mut number_of_intersections = 0;
        for (col, c) in line.chars().enumerate() {
            if path.contains(&(col, row)) {
                if c == '|' {
                    number_of_intersections += 1;
                }
            } else if number_of_intersections % 2 == 1 {
                result += 1;
            }
        }
    }
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
    fn test_first_task_example_1_with_ground() {
        let reader = BufReader::new(File::open("./input1_1.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 4);
    }

    #[test]
    fn test_first_task_example_1_with_pipes() {
        let reader = BufReader::new(File::open("./input1_2.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 4);
    }

    #[test]
    fn test_first_task_example_2_with_ground() {
        let reader = BufReader::new(File::open("./input1_3.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 8);
    }

    #[test]
    fn test_first_task_example_2_with_pipes() {
        let reader = BufReader::new(File::open("./input1_4.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 8);
    }

    #[test]
    fn test_second_task_example_1() {
        let reader = BufReader::new(File::open("./input2_1.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 4);
    }

    #[test]
    fn test_second_task_example_2() {
        let reader = BufReader::new(File::open("./input2_2.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 8);
    }

    #[test]
    fn test_second_task_example_3() {
        let reader = BufReader::new(File::open("./input2_3.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 10);
    }

    #[test]
    fn test_second_task_example_4() {
        let reader = BufReader::new(File::open("./input1_1.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 1);
    }
}
