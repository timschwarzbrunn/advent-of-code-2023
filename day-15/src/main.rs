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

#[derive(Debug)]
enum Operation {
    Undefined,
    Dash,
    Equal,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn calculate_hash(current_hash: usize, c: u8) -> usize {
    ((current_hash + c as usize) * 17) % 256
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut result = 0;
    let mut hash = 0;
    for c in reader.bytes().map(Result::unwrap) {
        match c {
            b',' => {
                result += hash;
                hash = 0;
            }
            b'\n' => {
                // Catch the last one.
                result += hash;
            }
            _ => {
                hash = calculate_hash(hash, c);
            }
        }
    }
    result
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut label: String = String::from("");
    let mut hash = 0;
    let mut operation = Operation::Undefined;
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for c in reader.bytes().map(Result::unwrap) {
        match c {
            b',' => {
                hash = 0;
                label.clear();
                operation = Operation::Undefined;
            }
            b'-' => {
                operation = Operation::Dash;
                // Look into the box of the hash if the label is present.
                for (idx, lens) in boxes[hash].iter_mut().enumerate() {
                    if lens.label == label {
                        boxes[hash].remove(idx);
                        break;
                    }
                }
            }
            b'=' => {
                operation = Operation::Equal;
            }
            b'\n' => {
                // Nothing to do here.
            }
            _ => {
                match operation {
                    Operation::Undefined => {
                        hash = calculate_hash(hash, c);
                        label.push(c as char);
                    }
                    Operation::Equal => {
                        // Insert focal length or replace it.
                        let focal_length = c - b'0';
                        let mut found_lens = false;
                        for lens in boxes[hash].iter_mut() {
                            if lens.label == label {
                                lens.focal_length = focal_length;
                                found_lens = true;
                                break;
                            }
                        }
                        if found_lens == false {
                            boxes[hash].push(Lens {
                                label: label.clone(),
                                focal_length,
                            })
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    // sum((box_id + 1) * (lens_id + 1) * focal_length)
    boxes
        .iter()
        .enumerate()
        .map(|(box_id, current_box)| {
            current_box
                .iter()
                .enumerate()
                .map(|(lens_id, lens)| (box_id + 1) * (lens_id + 1) * lens.focal_length as usize)
                .sum::<usize>()
        })
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
        assert_eq!(solve_first_task(reader), 1320);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 145);
    }
}
