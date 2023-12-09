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

fn solve_first_task_line(line: String) -> isize {
    let initial_values: Vec<isize> = line
        .split_ascii_whitespace()
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .collect();
    let mut all_values: Vec<Vec<isize>> = vec![initial_values];
    // Create the vectors.
    while !all_values.last().unwrap().iter().all(|&value| value == 0) {
        let differences: Vec<isize> = all_values
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        all_values.push(differences);
    }
    // Pass the sum from the bottom to the top.
    all_values
        .iter()
        .map(|values| values.last().unwrap_or(&0))
        .sum()
}

fn solve_first_task<B: BufRead>(reader: B) -> isize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(solve_first_task_line)
        .sum()
}

fn solve_second_task_line(line: String) -> isize {
    let initial_values: Vec<isize> = line
        .split_ascii_whitespace()
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .collect();
    let mut all_values: Vec<Vec<isize>> = vec![initial_values];
    // Create the vectors.
    while !all_values.last().unwrap().iter().all(|&value| value == 0) {
        let differences: Vec<isize> = all_values
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        all_values.push(differences);
    }
    // Pass the sum from the bottom to the top.
    let result = all_values
        .iter()
        .rev()
        .map(|values| values.iter().nth(0).unwrap_or(&0))
        .fold(0_isize, |diff, &value| value - diff);
    result
}

fn solve_second_task<B: BufRead>(reader: B) -> isize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(solve_second_task_line)
        .sum()
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
        assert_eq!(solve_first_task(reader), 114);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 2);
    }
}
