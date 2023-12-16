use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    thread,
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
    // Left.
    let result_left = Arc::new(Mutex::new(0));
    let map_left = map.clone();
    let thread_left = thread::spawn({
        let result_left = Arc::clone(&result_left);
        move || {
            for y in 0..map_left.len() {
                let mut visit_status: Vec<Vec<CellVisitStatus>> =
                    vec![vec![CellVisitStatus::new(); map_left[0].len()]; map_left.len()];
                track_beam(
                    &map_left,
                    &mut visit_status,
                    (-1, y as isize),
                    Direction::Right,
                );
                let mut result_left = result_left.lock().unwrap();
                *result_left = (*result_left).max(
                    visit_status
                        .iter()
                        .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                        .sum(),
                );
            }
        }
    });
    // Right.
    let result_right = Arc::new(Mutex::new(0));
    let map_right = map.clone();
    let thread_right = thread::spawn({
        let result_right = Arc::clone(&result_right);
        move || {
            for y in 0..map_right.len() {
                let mut visit_status: Vec<Vec<CellVisitStatus>> =
                    vec![vec![CellVisitStatus::new(); map_right[0].len()]; map_right.len()];
                track_beam(
                    &map_right,
                    &mut visit_status,
                    (map_right[0].len() as isize, y as isize),
                    Direction::Left,
                );
                let mut result_right = result_right.lock().unwrap();
                *result_right = (*result_right).max(
                    visit_status
                        .iter()
                        .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                        .sum(),
                );
            }
        }
    });
    // Top.
    let result_top = Arc::new(Mutex::new(0));
    let map_top = map.clone();
    let thread_top = thread::spawn({
        let result_top = Arc::clone(&result_top);
        move || {
            for x in 0..map_top[0].len() {
                let mut visit_status: Vec<Vec<CellVisitStatus>> =
                    vec![vec![CellVisitStatus::new(); map_top[0].len()]; map_top.len()];
                track_beam(
                    &map_top,
                    &mut visit_status,
                    (x as isize, -1),
                    Direction::Down,
                );
                let mut result_top = result_top.lock().unwrap();
                *result_top = (*result_top).max(
                    visit_status
                        .iter()
                        .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                        .sum(),
                );
            }
        }
    });
    // Bottom.
    let result_bottom = Arc::new(Mutex::new(0));
    let map_bottom = map.clone();
    let thread_bottom = thread::spawn({
        let result_bottom = Arc::clone(&result_bottom);
        move || {
            for x in 0..map_bottom[0].len() {
                let mut visit_status: Vec<Vec<CellVisitStatus>> =
                    vec![vec![CellVisitStatus::new(); map_bottom[0].len()]; map_bottom.len()];
                track_beam(
                    &map_bottom,
                    &mut visit_status,
                    (x as isize, map_bottom.len() as isize),
                    Direction::Up,
                );
                let mut result_bottom = result_bottom.lock().unwrap();
                *result_bottom = (*result_bottom).max(
                    visit_status
                        .iter()
                        .map(|line| line.iter().filter(|cell| cell.was_visited()).count())
                        .sum(),
                );
            }
        }
    });

    thread_left.join().unwrap();
    thread_right.join().unwrap();
    thread_top.join().unwrap();
    thread_bottom.join().unwrap();

    let result_left = *result_left.lock().unwrap();
    let result_right = *result_right.lock().unwrap();
    let result_top = *result_top.lock().unwrap();
    let result_bottom = *result_bottom.lock().unwrap();

    result_left.max(result_right.max(result_top.max(result_bottom)))
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
