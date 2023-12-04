use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let cards = parse_cards(input);

    let p1 = get_sum_of_card_values(&cards);
    let p2 = get_number_of_scratch_cards(&cards);

    (p1, p2)
}

fn get_sum_of_card_values(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.value()).sum()
}

fn get_number_of_scratch_cards(cards: &[Card]) -> u32 {
    let mut copies: Vec<u32> = (0..cards.len()).map(|_| 1).collect();

    for c in cards {
        let wins = c.matching_numbers().len() as u32;
        let copies_of_curent = copies[c.id as usize - 1];

        for id in c.id..c.id + wins {
            copies[id as usize] += copies_of_curent;
        }
    }

    copies.iter().sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn matching_numbers(&self) -> HashSet<u32> {
        let numbers: HashSet<u32> = HashSet::from_iter(self.numbers.iter().cloned());
        let winning_numbers: HashSet<u32> =
            HashSet::from_iter(self.winning_numbers.iter().cloned());

        numbers.intersection(&winning_numbers).copied().collect()
    }

    fn value(&self) -> u32 {
        let matching_numbers = self.matching_numbers();

        if matching_numbers.is_empty() {
            return 0;
        }

        2u32.pow(matching_numbers.len() as u32 - 1)
    }
}

fn parse_cards(input: &[String]) -> Vec<Card> {
    input
        .iter()
        .map(|entry| {
            let (title, data) = entry.split(": ").collect_tuple().unwrap();
            let card_id = title[5..].trim().parse::<u32>().unwrap();

            let (raw_winning_numbers, raw_numbers) = data.split('|').collect_tuple().unwrap();

            let winning_numbers = raw_winning_numbers
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect();
            let numbers = raw_numbers
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect();

            Card {
                id: card_id,
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card  6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day04.txt")
    }

    #[rstest]
    fn test_parse_cards(test_input: Vec<String>) {
        let expected = vec![
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                winning_numbers: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                id: 3,
                winning_numbers: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                id: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                id: 5,
                winning_numbers: vec![87, 83, 26, 28, 32],
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                id: 6,
                winning_numbers: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];

        assert_eq!(parse_cards(&test_input), expected);
    }

    #[rstest]
    fn test_get_card_matching_numbers(test_input: Vec<String>) {
        let values: Vec<HashSet<u32>> = parse_cards(&test_input)
            .iter()
            .map(|c| c.matching_numbers())
            .collect();

        let expected = vec![
            HashSet::from_iter(vec![48, 83, 86, 17]),
            HashSet::from_iter(vec![32, 61]),
            HashSet::from_iter(vec![1, 21]),
            HashSet::from_iter(vec![84]),
            HashSet::new(),
            HashSet::new(),
        ];

        assert_eq!(values, expected);
    }

    #[rstest]
    fn test_get_card_value(test_input: Vec<String>) {
        let values: Vec<u32> = parse_cards(&test_input).iter().map(|c| c.value()).collect();

        assert_eq!(values, [8, 2, 2, 1, 0, 0]);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let cards = parse_cards(&test_input);

        let res = get_sum_of_card_values(&cards);

        assert_eq!(res, 13);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let cards = parse_cards(&puzzle_input);

        let res = get_sum_of_card_values(&cards);

        assert_eq!(res, 21088);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let cards = parse_cards(&test_input);

        let res = get_number_of_scratch_cards(&cards);

        assert_eq!(res, 30);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let cards = parse_cards(&puzzle_input);

        let res = get_number_of_scratch_cards(&cards);

        assert_eq!(res, 6874754);
    }
}
