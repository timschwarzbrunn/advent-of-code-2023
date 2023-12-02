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

/// Function to calculate the calibration value.
fn calculate_calibration_value(line: String) -> usize {
    if let Some(first_number) = line.chars().next() {
        let last_number = line.chars().last().unwrap();
        format!("{}{}", first_number, last_number)
            .parse::<usize>()
            .unwrap()
    } else {
        0
    }
}

/// Function needed by the second task to parse written numbers to numbers (eight -> 8).
fn words_to_numbers(line: String) -> String {
    let mut result = String::from("");
    let mut index = 0;
    while index < line.len() {
        let partial_line = &line[index..];
        let first_character = partial_line.chars().next().unwrap();
        if first_character.is_digit(10) == true {
            result.push(first_character);
        } else {
            for word in [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ] {
                if partial_line.starts_with(word) {
                    match word {
                        "one" => result.push('1'),
                        "two" => result.push('2'),
                        "three" => result.push('3'),
                        "four" => result.push('4'),
                        "five" => result.push('5'),
                        "six" => result.push('6'),
                        "seven" => result.push('7'),
                        "eight" => result.push('8'),
                        "nine" => result.push('9'),
                        _ => unreachable!(),
                    };
                    // Move not directly after the word but also watch the last letter
                    // to recognize cases like eightwo (82 instead of 8wo).
                    index = index + word.len() - 2;
                    break;
                }
            }
        }
        index = index + 1;
    }
    result
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .map(calculate_calibration_value)
        .sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| words_to_numbers(line))
        .map(calculate_calibration_value)
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
        assert_eq!(solve_first_task(reader), 142);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input2.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 281);
    }
}
