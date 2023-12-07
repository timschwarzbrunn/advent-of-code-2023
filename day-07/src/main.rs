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

impl Hand {
    fn char_to_card_index(c: u8, task: Task) -> usize {
        // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
        if c.is_ascii_digit() {
            match task {
                Task::First => (c - b'2') as usize,
                Task::Second => (c - b'1') as usize,
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
        // Determine the hand type (depending on the task).
        let mut cards_ordered = cards.clone();
        match task {
            Task::First => {
                cards_ordered.sort();
            }
            Task::Second => {
                cards_ordered[1..].sort();
            }
        }
        match (cards_ordered[12] + cards_ordered[0], cards_ordered[11]) {
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
            // Special case.
            // Both have the same hand type. We now need to check the first cards.
            for (c1, c2) in self.hand.bytes().zip(other.hand.bytes()) {
                if c1 == c2 {
                    continue;
                }
                return Hand::char_to_card_index(c1, self.task)
                    .partial_cmp(&Hand::char_to_card_index(c2, other.task));
            }
            None
        } else {
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
