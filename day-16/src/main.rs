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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct CellVisitStatus {
    to_left: bool,
    to_right: bool,
    to_top: bool,
    to_bottom: bool,
}

impl CellVisitStatus {
    fn new() -> Self {
        Self {
            to_left: false,
            to_right: false,
            to_top: false,
            to_bottom: false,
        }
    }

    fn was_visited(&self) -> bool {
        self.to_left || self.to_right || self.to_top || self.to_bottom
    }
}

fn track_beam(
    map: &Vec<Vec<char>>,
    visit_status: &mut Vec<Vec<CellVisitStatus>>,
    mut position: (isize, isize),
    mut direction: Direction,
) {
    loop {
        position = match direction {
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
        };
        if position.0 < 0
            || position.0 >= map[0].len() as isize
            || position.1 < 0
            || position.1 >= map.len() as isize
        {
            return;
        }
        let _pos = (position.0 as usize, position.1 as usize);
        if match direction {
            Direction::Left => visit_status[_pos.1][_pos.0].to_left,
            Direction::Right => visit_status[_pos.1][_pos.0].to_right,
            Direction::Up => visit_status[_pos.1][_pos.0].to_top,
            Direction::Down => visit_status[_pos.1][_pos.0].to_bottom,
        } == true
        {
            return;
        } else {
            match direction {
                Direction::Left => visit_status[_pos.1][_pos.0].to_left = true,
                Direction::Right => visit_status[_pos.1][_pos.0].to_right = true,
                Direction::Up => visit_status[_pos.1][_pos.0].to_top = true,
                Direction::Down => visit_status[_pos.1][_pos.0].to_bottom = true,
            }
        }
        match map[_pos.1][_pos.0] {
            '\\' => match direction {
                Direction::Left => direction = Direction::Up,
                Direction::Right => direction = Direction::Down,
                Direction::Up => direction = Direction::Left,
                Direction::Down => direction = Direction::Right,
            },
            '/' => match direction {
                Direction::Left => direction = Direction::Down,
                Direction::Right => direction = Direction::Up,
                Direction::Up => direction = Direction::Right,
                Direction::Down => direction = Direction::Left,
            },
            '-' => {
                if direction == Direction::Up || direction == Direction::Down {
                    track_beam(map, visit_status, position, Direction::Left);
                    track_beam(map, visit_status, position, Direction::Right);
                    return;
                }
            }
            '|' => {
                if direction == Direction::Left || direction == Direction::Right {
                    track_beam(map, visit_status, position, Direction::Up);
                    track_beam(map, visit_status, position, Direction::Down);
                    return;
                }
            }
            _ => {}
        }
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let map: Vec<Vec<char>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();
    let mut visit_status: Vec<Vec<CellVisitStatus>> =
        vec![vec![CellVisitStatus::new(); map[0].len()]; map.len()];
    track_beam(&map, &mut visit_status, (-1, 0), Direction::Right);
    visit_status
        .iter()
        .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
        .sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let map: Vec<Vec<char>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();
    let mut result = 0;
    for y in 0..map.len() {
        // Left.
        let mut visit_status: Vec<Vec<CellVisitStatus>> =
            vec![vec![CellVisitStatus::new(); map[0].len()]; map.len()];
        track_beam(&map, &mut visit_status, (-1, y as isize), Direction::Right);
        result = result.max(
            visit_status
                .iter()
                .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                .sum(),
        );
        // Right.
        let mut visit_status: Vec<Vec<CellVisitStatus>> =
            vec![vec![CellVisitStatus::new(); map[0].len()]; map.len()];
        track_beam(
            &map,
            &mut visit_status,
            (map[0].len() as isize, y as isize),
            Direction::Left,
        );
        result = result.max(
            visit_status
                .iter()
                .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                .sum(),
        );
    }
    for x in 0..map[0].len() {
        // Top.
        let mut visit_status: Vec<Vec<CellVisitStatus>> =
            vec![vec![CellVisitStatus::new(); map[0].len()]; map.len()];
        track_beam(&map, &mut visit_status, (x as isize, -1), Direction::Down);
        result = result.max(
            visit_status
                .iter()
                .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                .sum(),
        );
        // Bottom.
        let mut visit_status: Vec<Vec<CellVisitStatus>> =
            vec![vec![CellVisitStatus::new(); map[0].len()]; map.len()];
        track_beam(
            &map,
            &mut visit_status,
            (x as isize, map.len() as isize),
            Direction::Up,
        );
        result = result.max(
            visit_status
                .iter()
                .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                .sum(),
        );
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
        assert_eq!(solve_first_task(reader), 46);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 51);
    }
}
