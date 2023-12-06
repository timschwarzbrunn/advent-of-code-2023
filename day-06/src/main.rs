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

fn get_number_of_winning_cases(time: usize, distance: usize) -> usize {
    let first_term = time as f64 * 0.5;
    let sqrt_term = f64::sqrt(f64::powi(first_term, 2) - distance as f64);
    if sqrt_term.is_nan() {
        // There is no root.
        // This is not necessary for our case but what so ever.
        return 0;
    }
    let mut x1 = first_term - sqrt_term;
    let mut x2 = first_term + sqrt_term;
    // Check if the value is equal to the integer part. If so add / subtract one.
    if x1.trunc() == x1 {
        x1 += 1.0;
    }
    if x2.trunc() == x2 {
        x2 -= 1.0;
    }
    // Only keep the integer parts by ceiling / flooring.
    x2.floor() as usize - x1.ceil() as usize + 1
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut lines = reader.lines();
    let times = lines
        .next()
        .ok_or_else(|| "Cannot find first line (times).")
        .unwrap()
        .unwrap();
    let distances = lines
        .next()
        .ok_or_else(|| "Cannot find second line (distances).")
        .unwrap()
        .unwrap();
    let times = times
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap);
    let distances = distances
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap);
    times
        .zip(distances)
        .map(|(time, distance)| get_number_of_winning_cases(time, distance))
        .fold(1, |mut product, value| {
            product *= value;
            product
        })
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut lines = reader.lines();
    let times = lines
        .next()
        .ok_or_else(|| "Cannot find first line (times).")
        .unwrap()
        .unwrap();
    let distances = lines
        .next()
        .ok_or_else(|| "Cannot find second line (distances).")
        .unwrap()
        .unwrap();
    let time = times
        .bytes()
        .filter(|byte| byte.is_ascii_digit())
        .fold(0_usize, |number, digit| {
            number * 10 + (digit - b'0') as usize
        });
    let distance = distances
        .bytes()
        .filter(|byte| byte.is_ascii_digit())
        .fold(0_usize, |number, digit| {
            number * 10 + (digit - b'0') as usize
        });
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
