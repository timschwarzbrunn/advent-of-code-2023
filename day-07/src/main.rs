use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
enum Task {
    #[default]
    First,
    Second,
}

// Types of hands possible. They need to be ordered from lowest value to highest.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    hand: String,
    cards: [usize; 13],
    bid: usize,
    hand_type: HandType,
    task: Task,
}

// A function used instead of sorting the whole array. We only need the two highest
// numbers to determine which type of hand we have, so we do not need to sort the
// whole array.
fn find_two_largest_numbers(values: &[usize]) -> (usize, usize) {
    let mut first: usize = 0;
    let mut second: usize = 0;
    for value in values {
        if *value > first {
            second = first;
            first = *value;
        } else if *value > second {
            second = *value;
        }
    }
    (first, second)
}

impl Hand {
    fn char_to_card_index(c: u8, task: Task) -> usize {
        // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
        // In task 1, 2 is the lowest, in task 2, joker is the lowest.
        if c.is_ascii_digit() {
            (c - b'2') as usize
                + match task {
                    Task::First => 0,
                    // Reserve index 0 for the joker.
                    Task::Second => 1,
                }
        } else {
            match c {
                b'T' => match task {
                    Task::First => 8,
                    Task::Second => 9,
                },
                b'J' => match task {
                    Task::First => 9,
                    Task::Second => 0,
                },
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => unreachable!(),
            }
        }
    }

    fn determine_hand_type(cards: &[usize; 13], task: Task) -> HandType {
        let (first_largest, second_largest) = match task {
            Task::First => find_two_largest_numbers(cards),
            // Do not include the joker within the second task.
            Task::Second => find_two_largest_numbers(&cards[1..]),
        };
        match (
            first_largest
                + match task {
                    Task::First => 0,
                    Task::Second => cards[0],
                },
            second_largest,
        ) {
            (5, 0) => HandType::FiveOfAKind,
            (4, 1) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn from_line(line: String, task: Task) -> Self {
        // Example line: 32T3K 765
        let mut parts = line.split_ascii_whitespace();
        let hand = parts.next().unwrap().to_string();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        let mut cards = [0; 13];
        for c in hand.bytes() {
            cards[Hand::char_to_card_index(c, task)] += 1;
        }
        let hand_type = Hand::determine_hand_type(&cards, task);
        Self {
            hand,
            cards,
            bid,
            hand_type,
            task,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            // The hand types are equal so we will look at the hand.
            for (c1, c2) in self.hand.bytes().zip(other.hand.bytes()) {
                if c1 == c2 {
                    continue;
                }
                return Hand::char_to_card_index(c1, self.task)
                    .partial_cmp(&Hand::char_to_card_index(c2, other.task));
            }
            None
        } else {
            // The hand types can be simply compared.
            self.hand_type.partial_cmp(&other.hand_type)
        }
    }
}

fn solve_task<B: BufRead>(reader: B, task: Task) -> usize {
    let mut hands: Vec<Hand> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| Hand::from_line(line, task))
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
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

    println!("{:?} task solution: {:?}", task, solve_task(reader, task));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, Task::First), 6440);
    }

    #[test]
    fn test_second_task() {
        let reader = BufReader::new(File::open("./input.test").expect("Input file not found."));
        assert_eq!(solve_task(reader, Task::Second), 5905);
    }
}
