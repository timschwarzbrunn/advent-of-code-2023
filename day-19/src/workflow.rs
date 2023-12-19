use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WorkflowResult {
    Accept,
    Reject,
    NextWorkflow(String),
    NoResult,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comparison {
    pub category: char,
    pub comparison_char: char,
    pub comparison_function: fn(&usize, &usize) -> Ordering,
    pub value: usize,
    pub result: WorkflowResult,
}

impl Comparison {
    pub fn from_string(s: &str) -> Self {
        // Example: a<2006:qkq
        // Example: m>2090:A
        let mut category: char = '?';
        let mut comparison_char: char = '?';
        let mut comparison_function: fn(&usize, &usize) -> Ordering = usize::cmp;
        let mut value: usize = 0;
        let mut result_str: String = String::from("");
        for (idx, c) in s.bytes().enumerate() {
            if idx == 0 {
                category = c as char;
            } else if idx == 1 {
                comparison_function = match c {
                    b'<' => |a, b| usize::cmp(b, a),
                    b'>' => usize::cmp,
                    _ => unreachable!(),
                };
                comparison_char = c as char;
            } else if c.is_ascii_digit() {
                value = value * 10 + (c - b'0') as usize;
            } else if c == b':' {
                // Do nothing.
            } else {
                result_str.push(c as char);
            }
        }
        Self {
            category,
            comparison_char,
            comparison_function,
            value,
            result: match result_str.as_str() {
                "A" => WorkflowResult::Accept,
                "R" => WorkflowResult::Reject,
                _ => WorkflowResult::NextWorkflow(result_str),
            },
        }
    }

    pub fn compare(&self, xmas_value: usize) -> WorkflowResult {
        if (self.comparison_function)(&self.value, &xmas_value) == Ordering::Less {
            return self.result.clone();
        } else {
            return WorkflowResult::NoResult;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Workflow {
    pub name: String,
    pub comparisons: Vec<Comparison>,
    pub otherwise: WorkflowResult,
}

impl Workflow {
    pub fn from_line(line: String) -> Self {
        // Example line: px{a<2006:qkq,m>2090:A,rfg}
        let mut parts = line.split('{');
        let name: String = parts.next().unwrap().to_string();
        let comparison_parts: Vec<&str> = parts.next().unwrap().split(',').collect();
        let mut comparisons: Vec<Comparison> = Vec::new();
        for part in comparison_parts.iter().take(comparison_parts.len() - 1) {
            comparisons.push(Comparison::from_string(part));
        }
        let mut otherwise_str = comparison_parts.last().unwrap().to_string();
        otherwise_str.pop();
        Self {
            name,
            comparisons,
            otherwise: match otherwise_str.as_str() {
                "A" => WorkflowResult::Accept,
                "R" => WorkflowResult::Reject,
                _ => WorkflowResult::NextWorkflow(otherwise_str),
            },
        }
    }

    pub fn compare(&self, part: &Part) -> WorkflowResult {
        for comparison in self.comparisons.iter() {
            let result = comparison.compare(match comparison.category {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!(),
            });
            if result != WorkflowResult::NoResult {
                return result;
            }
        }
        self.otherwise.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WorkflowCollection {
    pub workflows: HashMap<String, Workflow>,
}

impl WorkflowCollection {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn check_acceptance(&self, part: &Part) -> bool {
        let mut workflow_name = String::from("in");
        loop {
            match self.workflows.get(&workflow_name) {
                Some(workflow) => match workflow.compare(&part) {
                    WorkflowResult::Accept => return true,
                    WorkflowResult::Reject => return false,
                    WorkflowResult::NextWorkflow(next_workflow_name) => {
                        workflow_name = next_workflow_name;
                    }
                    WorkflowResult::NoResult => panic!(),
                },
                None => {
                    panic!()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    pub fn from_line(line: String) -> Self {
        // Example line: {x=787,m=2655,a=1222,s=2876}
        let mut number: usize = 0;
        let mut xmas: [usize; 4] = [0; 4];
        let mut idx = 0;
        for c in line.bytes().skip(3) {
            if c.is_ascii_digit() {
                number = number * 10 + (c - b'0') as usize;
            } else if number > 0 {
                xmas[idx] = number;
                number = 0;
                idx += 1;
            }
        }
        Self {
            x: xmas[0],
            m: xmas[1],
            a: xmas[2],
            s: xmas[3],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartRanges {
    pub x: (usize, usize),
    pub m: (usize, usize),
    pub a: (usize, usize),
    pub s: (usize, usize),
}

impl PartRanges {
    pub fn is_valid(&self) -> bool {
        self.x.1 >= self.x.0 && self.m.1 >= self.m.0 && self.a.1 >= self.a.0 && self.s.1 >= self.s.0
    }
    pub fn get_score(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_from_line() {
        assert_eq!(
            Part::from_line(String::from("{x=787,m=2655,a=1222,s=2876}")),
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }
        );
    }

    #[test]
    fn test_comparison() {
        let comparison = Comparison::from_string("a<2006:qkq");
        assert_eq!(
            comparison.compare(100),
            WorkflowResult::NextWorkflow(String::from("qkq"))
        );
        let comparison = Comparison::from_string("a<2006:qkq");
        assert_eq!(comparison.compare(5000), WorkflowResult::NoResult);
        let comparison = Comparison::from_string("a>2006:qkq");
        assert_eq!(comparison.compare(100), WorkflowResult::NoResult);
        let comparison = Comparison::from_string("a<2006:A");
        assert_eq!(comparison.compare(100), WorkflowResult::Accept);
        let comparison = Comparison::from_string("a<2006:R");
        assert_eq!(comparison.compare(100), WorkflowResult::Reject);
    }

    #[test]
    fn test_workflow() {
        let workflow = Workflow::from_line(String::from("ex{x>10:one,m<20:two,a>30:R,A}"));
        let part = Part::from_line(String::from("{x=11,m=1,a=1,s=1}"));
        assert_eq!(
            workflow.compare(&part),
            WorkflowResult::NextWorkflow("one".to_string())
        );
        let part = Part::from_line(String::from("{x=10,m=1,a=1,s=1}"));
        assert_eq!(
            workflow.compare(&part),
            WorkflowResult::NextWorkflow("two".to_string())
        );
        let part = Part::from_line(String::from("{x=10,m=100,a=100,s=1}"));
        assert_eq!(workflow.compare(&part), WorkflowResult::Reject);
        let part = Part::from_line(String::from("{x=10,m=100,a=10,s=1}"));
        assert_eq!(workflow.compare(&part), WorkflowResult::Accept);
    }
}
