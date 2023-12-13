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

fn transpose_pattern(lines: &Vec<String>) -> Vec<String> {
    (0..lines[0].len())
        .map(|col_index| {
            (0..lines.len())
                .map(|row_index| lines[row_index].chars().nth(col_index).unwrap())
                .collect()
        })
        .map(|column: Vec<char>| column.into_iter().collect())
        .collect()
}

fn find_number_of_smudges(line1: &String, line2: &String) -> usize {
    line1
        .chars()
        .zip(line2.chars())
        .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
        .sum()
}

fn find_reflection(pattern: &Vec<String>, number_of_allowed_smudges: usize) -> usize {
    for idx in 0..(pattern.len() - 1) {
        let mut number_of_found_smudges = 0;
        // Walk to the outside.
        let mut look_up = idx;
        let mut look_down = idx + 1;
        loop {
            number_of_found_smudges +=
                find_number_of_smudges(&pattern[look_up], &pattern[look_down]);
            if number_of_found_smudges > number_of_allowed_smudges {
                // Too many mismatches found.
                break;
            }
            // Check if we can still compare further.
            if look_up == 0 || look_down == pattern.len() - 1 {
                // No error occured, this is really mirrored.
                // But only if the number of smudges is correct.
                if number_of_found_smudges == number_of_allowed_smudges {
                    return idx + 1;
                } else {
                    break;
                }
            }
            // Next comparison.
            look_up -= 1;
            look_down += 1;
        }
    }
    0
}

fn get_reflection_score(pattern: &Vec<String>, number_of_allowed_smudges: usize) -> usize {
    let mut result = 0;
    // Vertical.
    result += find_reflection(pattern, number_of_allowed_smudges) * 100;
    // Horizontal.
    let pattern_t = transpose_pattern(pattern);
    result += find_reflection(&pattern_t, number_of_allowed_smudges);
    result
}

fn solve_task<B: BufRead>(reader: B, number_of_allowed_smudges: usize) -> usize {
    let mut pattern: Vec<String> = Vec::new();
    let mut result = 0;
    for line in reader.lines().map(Result::unwrap) {
        if line.trim().is_empty() {
            result += get_reflection_score(&pattern, number_of_allowed_smudges);
            pattern.clear();
        } else {
            pattern.push(line.clone());
        }
    }
    // Catch the last one.
    if pattern.is_empty() == false {
        result += get_reflection_score(&pattern, number_of_allowed_smudges);
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
            Task::First => solve_task(reader, 0),
            Task::Second => solve_task(reader, 1),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 0), 405);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, 1), 400);
    }
}
