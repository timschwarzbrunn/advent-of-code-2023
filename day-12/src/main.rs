use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
enum Task {
    #[default]
    First,
    Second,
}

#[derive(Debug, Copy, Clone)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct ConditionRecord {
    line: Vec<char>,
    spring_conditions: Vec<SpringCondition>,
    damaged_springs_group_size: Vec<usize>,
    position_of_unknown_conditions: Vec<usize>,
}

impl ConditionRecord {
    fn from_line(line: String) -> Self {
        // Example line: ???.### 1,1,3
        let mut spring_conditions: Vec<SpringCondition> = Vec::new();
        let mut damaged_springs_group_size: Vec<usize> = Vec::new();
        let mut position_of_unknown_conditions: Vec<usize> = Vec::new();
        let mut current_group_size = 0;
        for (idx, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    spring_conditions.push(SpringCondition::Operational);
                }
                '#' => {
                    spring_conditions.push(SpringCondition::Damaged);
                }
                '?' => {
                    spring_conditions.push(SpringCondition::Unknown);
                    position_of_unknown_conditions.push(idx);
                }
                c if c.is_ascii_digit() => {
                    current_group_size = current_group_size * 10
                        + str::parse::<usize>(c.to_string().as_str()).unwrap();
                }
                ',' => {
                    damaged_springs_group_size.push(current_group_size);
                    current_group_size = 0;
                }
                _ => {}
            }
        }
        // Catch the last group size.
        damaged_springs_group_size.push(current_group_size);
        Self {
            line: line
                .split_ascii_whitespace()
                .nth(0)
                .unwrap()
                .chars()
                .collect(),
            spring_conditions,
            damaged_springs_group_size,
            position_of_unknown_conditions,
        }
    }

    fn unfold(&mut self) {
        let mut new_position_of_unknown_conditions = Vec::new();
        for i in 0..5 {
            for pos in self.position_of_unknown_conditions.iter() {
                new_position_of_unknown_conditions.push(self.spring_conditions.len() * i + pos + i);
            }
            if i > 0 {
                new_position_of_unknown_conditions.push(self.spring_conditions.len() * i + (i - 1));
            }
        }
        self.position_of_unknown_conditions = new_position_of_unknown_conditions;
        self.spring_conditions = [
            self.spring_conditions.clone(),
            vec![SpringCondition::Unknown],
            self.spring_conditions.clone(),
            vec![SpringCondition::Unknown],
            self.spring_conditions.clone(),
            vec![SpringCondition::Unknown],
            self.spring_conditions.clone(),
            vec![SpringCondition::Unknown],
            self.spring_conditions.clone(),
        ]
        .concat();
        self.line = [
            self.line.clone(),
            vec!['?'],
            self.line.clone(),
            vec!['?'],
            self.line.clone(),
            vec!['?'],
            self.line.clone(),
            vec!['?'],
            self.line.clone(),
        ]
        .concat();
        self.damaged_springs_group_size = [
            self.damaged_springs_group_size.clone(),
            self.damaged_springs_group_size.clone(),
            self.damaged_springs_group_size.clone(),
            self.damaged_springs_group_size.clone(),
            self.damaged_springs_group_size.clone(),
        ]
        .concat();
    }

    fn check_if_conditions_equal_group_sizes(
        spring_conditions: Vec<SpringCondition>,
        expected_group_sizes: &Vec<usize>,
    ) -> bool {
        let mut actual_group_sizes: Vec<usize> = Vec::new();
        let mut current_group_size = 0;
        for spring_condition in spring_conditions.iter() {
            match spring_condition {
                SpringCondition::Operational => {
                    if current_group_size > 0 {
                        actual_group_sizes.push(current_group_size);
                        current_group_size = 0;
                    }
                }
                SpringCondition::Damaged => {
                    current_group_size += 1;
                }
                _ => unreachable!("Unknown type is not allowed in here!"),
            }
        }
        // Catch the last one.
        if current_group_size > 0 {
            actual_group_sizes.push(current_group_size);
        }
        actual_group_sizes == *expected_group_sizes
    }

    fn get_number_of_different_arrangements_brute_force(&self) -> usize {
        let mut result = 0;
        for i in 0..(1 << self.position_of_unknown_conditions.len()) {
            let assigned_spring_conditions: Vec<SpringCondition> =
                (0..self.position_of_unknown_conditions.len())
                    .map(|bit| match (i & (1 << bit)) == 0 {
                        true => SpringCondition::Damaged,
                        false => SpringCondition::Operational,
                    })
                    .collect();
            let mut spring_conditions = self.spring_conditions.clone();
            for (idx, pos) in self.position_of_unknown_conditions.iter().enumerate() {
                spring_conditions[*pos] = assigned_spring_conditions[idx];
            }
            if ConditionRecord::check_if_conditions_equal_group_sizes(
                spring_conditions,
                &self.damaged_springs_group_size,
            ) == true
            {
                result += 1;
            }
        }
        result
    }

    fn get_number_of_different_arrangements_substring(
        substring: &Vec<char>,
        groups_to_fulfill: &Vec<usize>,
        lookup: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    ) -> usize {
        // Check if there are still characters. If not, this is a solution, but only if there are also no more groups to fulfill.
        if substring.is_empty() {
            if groups_to_fulfill.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        match substring[0] {
            '.' => {
                return ConditionRecord::get_number_of_different_arrangements_substring(
                    &substring[1..].to_vec(),
                    groups_to_fulfill,
                    lookup,
                );
            }
            '#' => {
                return ConditionRecord::get_number_of_different_arrangements_found_group_begin(
                    substring,
                    groups_to_fulfill,
                    lookup,
                );
            }
            '?' => {
                return ConditionRecord::get_number_of_different_arrangements_substring(
                    &substring[1..].to_vec(),
                    groups_to_fulfill,
                    lookup,
                ) + ConditionRecord::get_number_of_different_arrangements_found_group_begin(
                    substring,
                    groups_to_fulfill,
                    lookup,
                );
            }
            _ => unreachable!("Unallowed character detected."),
        }
    }

    fn get_number_of_different_arrangements_found_group_begin(
        substring: &Vec<char>,
        groups_to_fulfill: &Vec<usize>,
        lookup: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    ) -> usize {
        // Check if we already got this case. If so, reuse it.
        if let Some(&result) = lookup.get(&(substring.clone(), groups_to_fulfill.clone())) {
            return result;
        }
        // If there is currently a group begin but there is no more a group to fulfill, this is not a solution.
        if groups_to_fulfill.is_empty() {
            return 0;
        }
        // Check if there are enough characters left to fulfill all the needed groups.
        let needed_length_min: usize =
            groups_to_fulfill.iter().sum::<usize>() + groups_to_fulfill.len() - 1;
        if substring.len() < needed_length_min {
            return 0;
        }
        // Now check for the next wanted group length. There should only be '#' or '?', so no '.'.
        let current_group_length = groups_to_fulfill[0] as usize;
        if substring[1..current_group_length].iter().any(|&c| c == '.') {
            return 0;
        }
        // It is also necessary to check if there is another '#' behind our current group. This is not allowed, this would make the group too large.
        if substring.len() == current_group_length {
            if groups_to_fulfill.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        } else if let Some(&next_character) = substring.get(current_group_length) {
            if next_character == '#' {
                return 0;
            }
        }

        // Ok, we can fulfill this (currently, maybe the next groups are not possible but that is not the problem of this call).
        let result = ConditionRecord::get_number_of_different_arrangements_substring(
            &substring[(current_group_length + 1)..].to_vec(),
            &groups_to_fulfill[1..].to_vec(),
            lookup,
        );
        // Save the result.
        lookup.insert((substring.clone(), groups_to_fulfill.clone()), result);
        result
    }

    fn get_number_of_different_arrangements(
        &self,
        lookup: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    ) -> usize {
        return ConditionRecord::get_number_of_different_arrangements_substring(
            &self.line,
            &self.damaged_springs_group_size,
            lookup,
        );
    }
}

fn solve_first_task<B: BufRead>(reader: B) -> usize {
    let mut lookup: HashMap<(Vec<char>, Vec<usize>), usize> = HashMap::new();
    reader
        .lines()
        .map(Result::unwrap)
        .map(ConditionRecord::from_line)
        .map(|record| record.get_number_of_different_arrangements(&mut lookup))
        .sum()
}

fn solve_second_task<B: BufRead>(reader: B) -> usize {
    let mut lookup: HashMap<(Vec<char>, Vec<usize>), usize> = HashMap::new();
    reader
        .lines()
        .map(Result::unwrap)
        .map(ConditionRecord::from_line)
        .map(|mut record| {
            record.unfold();
            record.get_number_of_different_arrangements(&mut lookup)
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
        assert_eq!(solve_first_task(reader), 21);
    }

    #[test]
    fn test_number_of_different_arrangements_example_1() {
        let record = ConditionRecord::from_line(String::from("???.### 1,1,3"));
        assert_eq!(record.get_number_of_different_arrangements_brute_force(), 1)
    }

    #[test]
    fn test_number_of_different_arrangements_example_2() {
        let record = ConditionRecord::from_line(String::from(".??..??...?##. 1,1,3"));
        assert_eq!(record.get_number_of_different_arrangements_brute_force(), 4)
    }

    #[test]
    fn test_number_of_different_arrangements_example_3() {
        let record = ConditionRecord::from_line(String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(record.get_number_of_different_arrangements_brute_force(), 1)
    }

    #[test]
    fn test_number_of_different_arrangements_example_4() {
        let record = ConditionRecord::from_line(String::from("????.#...#... 4,1,1"));
        assert_eq!(record.get_number_of_different_arrangements_brute_force(), 1)
    }

    #[test]
    fn test_number_of_different_arrangements_example_5() {
        let record = ConditionRecord::from_line(String::from("????.######..#####. 1,6,5"));
        assert_eq!(record.get_number_of_different_arrangements_brute_force(), 4)
    }

    #[test]
    fn test_number_of_different_arrangements_example_6() {
        let record = ConditionRecord::from_line(String::from("?###???????? 3,2,1"));
        assert_eq!(
            record.get_number_of_different_arrangements_brute_force(),
            10
        )
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_second_task(reader), 525152);
    }

    #[test]
    fn test_unfolding() {
        let mut record = ConditionRecord::from_line(String::from(".# 1"));
        record.unfold();
        assert_eq!(record.position_of_unknown_conditions, vec![2, 5, 8, 11])
    }
}
