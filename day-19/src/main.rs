use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod workflow;
use workflow::*;

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let mut workflow_collection = WorkflowCollection::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut read_workflow = true;
    for line in lines.iter() {
        if line.is_empty() {
            read_workflow = false;
        } else {
            if read_workflow {
                let workflow = Workflow::from_line(line.clone());
                workflow_collection
                    .workflows
                    .insert(workflow.name.clone(), workflow);
            } else {
                parts.push(Part::from_line(line.clone()));
            }
        }
    }
    parts
        .iter()
        .filter(|part| workflow_collection.check_acceptance(&part) == true)
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let mut workflow_collection = WorkflowCollection::new();
    let mut accepting_ranges: Vec<PartRanges> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            break;
        } else {
            let workflow = Workflow::from_line(line.clone());
            workflow_collection
                .workflows
                .insert(workflow.name.clone(), workflow);
        }
    }
    let mut workflow_todo: Vec<(String, PartRanges)> = vec![(
        String::from("in"),
        PartRanges {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];
    while !workflow_todo.is_empty() {
        let (current_workflow_name, mut current_range) = workflow_todo.pop().unwrap();
        if !current_range.is_valid() {
            // Check if the range is valid (the second range value needs to be larger than or equal the first range value).
            continue;
        }
        let current_workflow = workflow_collection
            .workflows
            .get(&current_workflow_name)
            .unwrap();
        for comparison in current_workflow.comparisons.iter() {
            let mut next_range = current_range.clone();
            match comparison.category {
                'x' => match comparison.comparison_char {
                    '<' => {
                        current_range.x.1 = comparison.value - 1;
                        next_range.x.0 = comparison.value;
                    }
                    '>' => {
                        current_range.x.0 = comparison.value + 1;
                        next_range.x.1 = comparison.value;
                    }
                    _ => unreachable!(),
                },
                'm' => match comparison.comparison_char {
                    '<' => {
                        current_range.m.1 = comparison.value - 1;
                        next_range.m.0 = comparison.value;
                    }
                    '>' => {
                        current_range.m.0 = comparison.value + 1;
                        next_range.m.1 = comparison.value;
                    }
                    _ => unreachable!(),
                },
                'a' => match comparison.comparison_char {
                    '<' => {
                        current_range.a.1 = comparison.value - 1;
                        next_range.a.0 = comparison.value;
                    }
                    '>' => {
                        current_range.a.0 = comparison.value + 1;
                        next_range.a.1 = comparison.value;
                    }
                    _ => unreachable!(),
                },
                's' => match comparison.comparison_char {
                    '<' => {
                        current_range.s.1 = comparison.value - 1;
                        next_range.s.0 = comparison.value;
                    }
                    '>' => {
                        current_range.s.0 = comparison.value + 1;
                        next_range.s.1 = comparison.value;
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
            if comparison.result == WorkflowResult::Accept {
                if current_range.is_valid() {
                    accepting_ranges.push(current_range.clone());
                }
            } else if let WorkflowResult::NextWorkflow(next_workflow) = comparison.result.clone() {
                workflow_todo.push((next_workflow, current_range.clone()));
            }
            current_range = next_range;
        }
        // Catch last one (the else block).
        if current_workflow.otherwise == WorkflowResult::Accept {
            if current_range.is_valid() {
                accepting_ranges.push(current_range.clone());
            }
        } else if let WorkflowResult::NextWorkflow(next_workflow) =
            current_workflow.otherwise.clone()
        {
            workflow_todo.push((next_workflow, current_range.clone()));
        }
    }
    accepting_ranges.iter().map(PartRanges::get_score).sum()
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
        assert_eq!(solve_first_task(reader), 19114);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 167409079868000);
    }
}
