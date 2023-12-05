use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod almanac;
use almanac::*;

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let almanac = Almanac::from_reader(reader, SeedMode::Single);
    almanac.get_min_location()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let almanac = Almanac::from_reader(reader, SeedMode::Range);
    almanac.get_min_location()
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
        assert_eq!(solve_first_task(reader), 35);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 46);
    }
}
