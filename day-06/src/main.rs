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

fn parse_input_task1<B: BufRead>(reader: B) -> (Vec<usize>, Vec<usize>) {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let times = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    let distances = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    (times, distances)
}

fn parse_input_task2<B: BufRead>(reader: B) -> (usize, usize) {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let time: String = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();
    let distance: String = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    (
        time.parse::<usize>().unwrap(),
        distance.parse::<usize>().unwrap(),
    )
}

fn get_number_of_winning_cases(time: usize, distance: usize) -> usize {
    let sqrt_term = f64::powi(time as f64 * 0.5, 2) - distance as f64;
    if sqrt_term < 0.0 {
        // There is no root.
        return 0;
    }
    // Find the first root and the second root.
    let mut x1 = time as f64 * 0.5 - f64::sqrt(sqrt_term);
    let mut x2 = time as f64 * 0.5 + f64::sqrt(sqrt_term);
    // Check if the value is equal to the integer part. If so add / subtract one.
    if x1.trunc() == x1 {
        x1 += 1.0;
    }
    if x2.trunc() == x2 {
        x2 -= 1.0;
    }
    // Only keep the integer parts by ceiling / flooring.
    let x1 = x1.ceil() as usize;
    let x2 = x2.floor() as usize;
    x2 - x1 + 1
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let (times, distances) = parse_input_task1(reader);
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| get_number_of_winning_cases(*time, *distance))
        .fold(1, |mut product, value| {
            product *= value;
            product
        })
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let (time, distance) = parse_input_task2(reader);
    get_number_of_winning_cases(time, distance)
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
        assert_eq!(solve_first_task(reader), 288);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 71503);
    }

    #[test]
    fn test_number_of_winning_cases_odd_time() {
        assert_eq!(get_number_of_winning_cases(7, 9), 4);
    }

    #[test]
    fn test_number_of_winning_cases_impossible() {
        assert_eq!(get_number_of_winning_cases(1, 20), 0);
    }

    #[test]
    fn test_number_of_winning_cases_all() {
        assert_eq!(get_number_of_winning_cases(100, 1), 99);
    }
}
