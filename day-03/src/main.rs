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

fn is_part_number_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

fn generate_lookup_part_number(lines: &Vec<String>) -> Vec<Vec<bool>> {
    let mut lookup_part_number: Vec<Vec<bool>> = Vec::new();
    let mut previous_line: Vec<bool> = Vec::new();
    let mut current_line: Vec<bool> = Vec::new();
    let mut next_line: Vec<bool> = Vec::new();
    let mut width = 0;
    for (idx, line) in lines.iter().enumerate() {
        // Get the width of the matrix only once.
        if idx == 0 {
            width = line.chars().count();
            // Initialize.
            current_line = vec![false; width];
            next_line = vec![false; width];
        }
        // If we already did something in the previous run, save the previous line,
        // and change the others to their new tasks.
        // We will only save the line if we are sure that there will nothing happen
        // to this line, so when we are two lines ahead.
        if idx > 1 {
            lookup_part_number.push(previous_line);
        }
        previous_line = current_line;
        current_line = next_line;
        next_line = vec![false; width];
        // Now read the current line and depending on the position of the symbols,
        // mark the surroundings in the lookup.
        for (idx_c, c) in line.chars().enumerate() {
            if is_part_number_symbol(c) {
                previous_line[idx_c] = true;
                current_line[idx_c] = true;
                next_line[idx_c] = true;
                if idx_c > 0 {
                    previous_line[idx_c - 1] = true;
                    current_line[idx_c - 1] = true;
                    next_line[idx_c - 1] = true;
                }
                if idx_c < width - 1 {
                    previous_line[idx_c + 1] = true;
                    current_line[idx_c + 1] = true;
                    next_line[idx_c + 1] = true;
                }
            }
        }
    }
    // At the end, since nothing will happen anymore, save the previous and the current
    // line to the lookup.
    // Normally we should also test wether the number of lines was greater than 1 but
    // we will skip this for now.
    lookup_part_number.push(previous_line);
    lookup_part_number.push(current_line);
    lookup_part_number
}

fn find_gears(lines: &Vec<String>) -> usize {
    let mut lookup_numbers: Vec<Vec<Option<usize>>> = vec![vec![None; lines[0].len()]; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        let mut current_number = String::from("");
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current_number.push(c);
            } else {
                if current_number.len() > 0 {
                    let number: usize = current_number.parse().unwrap();
                    for rev_idx in 1..=current_number.len() {
                        lookup_numbers[row][col - rev_idx] = Some(number);
                    }
                    current_number.clear();
                }
            }
        }
        // Catch the last one.
        if current_number.len() > 0 {
            let number: usize = current_number.parse().unwrap();
            for rev_idx in 1..=current_number.len() {
                lookup_numbers[row][line.len() - rev_idx] = Some(number);
            }
        }
    }
    let mut gears: Vec<Vec<HashSet<usize>>> =
        vec![vec![HashSet::new(); lines[0].len()]; lines.len()];
    let mut result: usize = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '*' {
                // Look around and get the values.
                for look_x in 0..=2 {
                    for look_y in 0..=2 {
                        if let Some(number_row) = lookup_numbers.get(row + look_y - 1) {
                            if let Some(Some(number)) = number_row.get(col + look_x - 1) {
                                gears[row][col].insert(number.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    for line in gears.iter() {
        for hashset in line.iter() {
            if hashset.len() == 2 {
                result = result + hashset.iter().cloned().fold(1, |acc, x| acc * x);
            }
        }
    }
    result
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let lookup_part_number = generate_lookup_part_number(&lines);
    let mut part_numbers: Vec<usize> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let mut current_number = String::from("");
        let mut is_part_number = false;
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current_number.push(c);
                is_part_number = is_part_number || lookup_part_number[row][col];
            } else {
                if current_number.len() > 0 {
                    if is_part_number {
                        part_numbers.push(current_number.parse().unwrap());
                    }
                    current_number.clear();
                    is_part_number = false;
                }
            }
        }
        // Catch the last one.
        if current_number.len() > 0 && is_part_number {
            part_numbers.push(current_number.parse().unwrap());
        }
    }
    part_numbers.iter().sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    find_gears(&lines)
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
    use std::io::Cursor;

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_first_task(reader), 4361);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 467835);
    }

    fn print_lookup(lookup: Vec<Vec<bool>>) {
        lookup.iter().for_each(|line| {
            line.iter()
                .for_each(|value| print!("{}", if *value { '#' } else { '.' }));
            println!("");
        });
    }

    #[test]
    fn test_lookup_generation() {
        let input_str = "....\n.*..\n....\n....";
        let reader = BufReader::new(Cursor::new(input_str));
        let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
        let generated_lookup = generate_lookup_part_number(&lines);
        print_lookup(generated_lookup.clone());
        assert_eq!(
            generated_lookup,
            vec![
                vec![true, true, true, false],
                vec![true, true, true, false],
                vec![true, true, true, false],
                vec![false, false, false, false]
            ]
        );
    }
}
