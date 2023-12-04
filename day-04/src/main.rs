use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

fn extract_numbers(line: String) -> (HashSet<usize>, HashSet<usize>) {
    let numbers = line.split(':').nth(1).unwrap();
    let mut number_parts = numbers.trim().split('|');
    let winning_numbers: HashSet<_> = number_parts
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    let drawn_numbers: HashSet<_> = number_parts
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    (winning_numbers, drawn_numbers)
}

fn get_number_of_wins(line: String) -> usize {
    let (winning_hashset, drawn_hashset) = extract_numbers(line);
    let common_numbers: HashSet<_> = winning_hashset.intersection(&drawn_hashset).collect();
    common_numbers.len()
}

fn calculate_points(number_of_wins: usize) -> usize {
    if number_of_wins == 0 {
        0
    } else {
        i32::pow(2, (number_of_wins - 1) as u32) as usize
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(get_number_of_wins)
        .map(calculate_points)
        .sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let mut result: Vec<usize> = vec![1; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        let number_of_wins = get_number_of_wins(line.clone());
        for scratchcard in 1..=number_of_wins {
            if row + scratchcard < result.len() {
                result[row + scratchcard] = result[row + scratchcard] + result[row];
            }
        }
    }
    result.iter().sum()
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
        assert_eq!(solve_first_task(reader), 13);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 30);
    }
}
