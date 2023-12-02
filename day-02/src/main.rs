use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_CUBES_RED: usize = 12;
const MAX_CUBES_GREEN: usize = 13;
const MAX_CUBES_BLUE: usize = 14;

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

/// Function that parses strings like "1 blue" to (usize, CubeColor).
fn parse_draw(draw: &str) -> (usize, CubeColor) {
    let mut parts = draw.split_whitespace();
    let number_of_cubes: usize = parts
        .next()
        .expect("Expected a number.")
        .parse()
        .expect("Failed to parse number of cubes.");
    let color_of_cube: CubeColor = match parts.next().expect("Expected color of cube.") {
        "red" => CubeColor::Red,
        "green" => CubeColor::Green,
        "blue" => CubeColor::Blue,
        _ => unreachable!(),
    };
    (number_of_cubes, color_of_cube)
}

/// Function for task 1 that checks if the number of drawed cubes is less or equal than
/// the amount of available cubes of this color.
fn check_amount_of_cubes(draw: &str) -> bool {
    let (number_of_cubes, color_of_cube) = parse_draw(draw);
    number_of_cubes
        <= match color_of_cube {
            CubeColor::Red => MAX_CUBES_RED,
            CubeColor::Green => MAX_CUBES_GREEN,
            CubeColor::Blue => MAX_CUBES_BLUE,
        }
}

/// Function for task 1 that checks if a single reveal of one game is possible.
fn check_if_reveal_is_possible(reveal: &str) -> bool {
    reveal
        .split(',')
        .all(|draw| check_amount_of_cubes(draw.trim()))
}

/// Function for task 1 that checks if a game (a whole line of the input) is possible.
fn check_if_game_is_possible(line: &String) -> bool {
    if let Some(games) = line.split(':').nth(1) {
        return games
            .split(';')
            .all(|game| check_if_reveal_is_possible(game.trim()));
    } else {
        unreachable!()
    }
}

/// Function that solves the first task.
fn solve_first_task<B: BufRead>(reader: B) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .filter(|(_, line)| check_if_game_is_possible(line) == true)
        .map(|(idx, _)| idx + 1)
        .sum()
}

/// Function for task 2 that calculates the power of a cube set.
fn get_power_of_cube_set(line: &String) -> usize {
    let mut number_of_red_cubes = 0;
    let mut number_of_green_cubes = 0;
    let mut number_of_blue_cubes = 0;

    if let Some(games) = line.split(':').nth(1) {
        for game in games.split(';').collect::<Vec<&str>>() {
            for draw in game.split(',').collect::<Vec<&str>>() {
                let (number_of_cubes, color_of_cube) = parse_draw(draw);
                match color_of_cube {
                    CubeColor::Red => {
                        number_of_red_cubes = number_of_red_cubes.max(number_of_cubes)
                    }
                    CubeColor::Green => {
                        number_of_green_cubes = number_of_green_cubes.max(number_of_cubes)
                    }
                    CubeColor::Blue => {
                        number_of_blue_cubes = number_of_blue_cubes.max(number_of_cubes)
                    }
                }
            }
        }
    }

    number_of_red_cubes * number_of_green_cubes * number_of_blue_cubes
}

/// Function that solves the second task.
fn solve_second_task<B: BufRead>(reader: B) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| get_power_of_cube_set(&line))
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
        assert_eq!(solve_first_task(reader), 8);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 2286);
    }
}
