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

fn find_galaxies<B: BufRead>(reader: B, expansion_number: usize) -> Vec<(usize, usize)> {
    let (mut x, mut y) = (0, 0);
    let mut found_galaxy = false;
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut number_of_columns = 0;
    for byte in reader.bytes().map(Result::unwrap) {
        match byte {
            b'.' => {
                x += 1;
            }
            b'#' => {
                galaxies.push((x, y));
                found_galaxy = true;
                x += 1;
            }
            b'\n' => {
                if y == 0 {
                    number_of_columns = x;
                }
                x = 0;
                y += 1;
                if found_galaxy == false {
                    y += expansion_number - 1;
                } else {
                    found_galaxy = false;
                }
            }
            _ => unreachable!("Unknown character in map."),
        }
    }
    // Now fill in one column at the empty columns.
    let mut lookup_col = vec![false; number_of_columns];
    for galaxy in galaxies.iter() {
        lookup_col[galaxy.0] = true;
    }
    let mut move_col = vec![0; number_of_columns];
    let mut total_move = 0;
    for (idx, col_is_filled) in lookup_col.iter().enumerate() {
        if col_is_filled == &false {
            total_move += expansion_number - 1;
        }
        move_col[idx] = total_move;
    }
    for idx in 0..galaxies.len() {
        galaxies[idx].0 += move_col[galaxies[idx].0];
    }
    galaxies
}

fn calculate_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    (pos1.0.max(pos2.0) - pos1.0.min(pos2.0)) + (pos1.1.max(pos2.1) - pos1.1.min(pos2.1))
}

fn calculate_sum_of_all_distances(galaxies: Vec<(usize, usize)>) -> usize {
    let mut total_distance = 0;
    for (idx, galaxy) in galaxies.iter().enumerate() {
        for (_, other) in galaxies.iter().skip(idx + 1).enumerate() {
            total_distance += calculate_distance(*galaxy, *other);
        }
    }
    total_distance
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
            Task::First => calculate_sum_of_all_distances(find_galaxies(reader, 2)),
            Task::Second => calculate_sum_of_all_distances(find_galaxies(reader, 1_000_000)),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        assert_eq!(calculate_distance((1, 6), (5, 11)), 9);
    }

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(
            calculate_sum_of_all_distances(find_galaxies(reader, 2)),
            374
        );
    }

    #[test]
    fn test_second_task_10_times_larger() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(
            calculate_sum_of_all_distances(find_galaxies(reader, 10)),
            1030
        );
    }

    #[test]
    fn test_second_task_100_times_larger() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(
            calculate_sum_of_all_distances(find_galaxies(reader, 100)),
            8410
        );
    }
}
