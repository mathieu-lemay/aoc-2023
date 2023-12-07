use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn main() {
    let input = get_input("day07.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let hands = parse_hands(input, false);
    let p1 = get_total_winnings(&hands);
    let hands = parse_hands(input, true);
    let p2 = get_total_winnings(&hands);

    (p1, p2)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn get_strength(&self) -> HandStrength {
        let mut counts: Vec<usize> = self
            .cards
            .iter()
            .filter(|&&c| c > 1)
            .sorted()
            .group_by(|&c| c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect();

        if counts.is_empty() {
            return HandStrength::FiveOfAKind;
        }

        let total: usize = counts.iter().sum();
        counts[0] += 5 - total;

        match counts[..] {
            [5] => HandStrength::FiveOfAKind,
            [4, 1] => HandStrength::FourOfAKind,
            [3, 2] => HandStrength::FullHouse,
            [3, 1, 1] => HandStrength::ThreeOfAKind,
            [2, 2, 1] => HandStrength::TwoPairs,
            [2, 1, 1, 1] => HandStrength::OnePair,
            _ => HandStrength::HighCard,
        }
    }
}

fn parse_hands(input: &[String], with_jokers: bool) -> Vec<Hand> {
    input
        .iter()
        .map(|i| {
            let (raw_cards, bid) = i.split(' ').collect_tuple().unwrap();

            let mut cards: [u8; 5] = [0; 5];
            cards
                .iter_mut()
                .set_from(raw_cards.chars().map(|c| get_card_value(c, with_jokers)));

            let bid = bid.parse().unwrap();

            Hand { cards, bid }
        })
        .collect()
}

fn get_card_value(c: char, with_jokers: bool) -> u8 {
    if c.is_ascii_digit() {
        return c.to_digit(10).unwrap() as u8;
    }

    match (c, with_jokers) {
        ('T', _) => 10,
        ('J', false) => 11,
        ('J', true) => 0,
        ('Q', _) => 12,
        ('K', _) => 13,
        ('A', _) => 14,
        _ => panic!("Invalid card: {}", c),
    }
}

fn get_sorted_hands(hands: &[Hand]) -> Vec<&Hand> {
    hands
        .iter()
        .sorted_by(|h1, h2| {
            let s1 = h1.get_strength();
            let s2 = h2.get_strength();

            let ord = s1.cmp(&s2);
            if ord != Ordering::Equal {
                return ord;
            }

            for (c1, c2) in h1.cards.iter().zip(h2.cards) {
                let ord = c1.cmp(&c2);
                if ord != Ordering::Equal {
                    return ord;
                }
            }

            Ordering::Equal
        })
        .collect_vec()
}

fn get_total_winnings(hands: &[Hand]) -> usize {
    get_sorted_hands(hands)
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid as usize * (idx + 1))
        .sum()
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
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day07.txt")
    }

    #[rstest]
    fn test_parse_hands(test_input: Vec<String>) {
        let hands = parse_hands(&test_input, false);

        let expected_hands = vec![
            Hand {
                cards: [3, 2, 10, 3, 13],
                bid: 765,
            },
            Hand {
                cards: [10, 5, 5, 11, 5],
                bid: 684,
            },
            Hand {
                cards: [13, 13, 6, 7, 7],
                bid: 28,
            },
            Hand {
                cards: [13, 10, 11, 11, 10],
                bid: 220,
            },
            Hand {
                cards: [12, 12, 12, 11, 14],
                bid: 483,
            },
        ];

        assert_eq!(hands, expected_hands);
    }

    #[rstest]
    // Without Jokers
    #[case(Hand {cards: [2,2,2,2,2], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [4,4,2,4,4], bid:0}, HandStrength::FourOfAKind)]
    #[case(Hand {cards: [4,2,4,4,4], bid:0}, HandStrength::FourOfAKind)]
    #[case(Hand {cards: [2,3,2,3,2], bid:0}, HandStrength::FullHouse)]
    #[case(Hand {cards: [2,2,3,3,3], bid:0}, HandStrength::FullHouse)]
    #[case(Hand {cards: [2,3,4,2,2], bid:0}, HandStrength::ThreeOfAKind)]
    #[case(Hand {cards: [2,3,4,3,2], bid:0}, HandStrength::TwoPairs)]
    #[case(Hand {cards: [2,3,2,4,5], bid:0}, HandStrength::OnePair)]
    #[case(Hand {cards: [2,3,4,5,5], bid:0}, HandStrength::OnePair)]
    #[case(Hand {cards: [2,3,4,5,6], bid:0}, HandStrength::HighCard)]
    // With Jokers
    #[case(Hand {cards: [2,2,2,2,0], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [2,2,2,0,0], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [2,2,0,0,0], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [2,0,0,0,0], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [0,0,0,0,0], bid:0}, HandStrength::FiveOfAKind)]
    #[case(Hand {cards: [4,4,4,2,0], bid:0}, HandStrength::FourOfAKind)]
    #[case(Hand {cards: [4,4,2,0,0], bid:0}, HandStrength::FourOfAKind)]
    #[case(Hand {cards: [4,2,0,0,0], bid:0}, HandStrength::FourOfAKind)]
    #[case(Hand {cards: [3,3,2,2,0], bid:0}, HandStrength::FullHouse)]
    #[case(Hand {cards: [4,4,3,2,0], bid:0}, HandStrength::ThreeOfAKind)]
    #[case(Hand {cards: [4,3,2,0,0], bid:0}, HandStrength::ThreeOfAKind)]
    #[case(Hand {cards: [5,4,3,2,0], bid:0}, HandStrength::OnePair)]
    fn test_get_strength(#[case] hand: Hand, #[case] expected: HandStrength) {
        assert_eq!(hand.get_strength(), expected);
    }

    #[rstest]
    fn test_get_ranked_hands(test_input: Vec<String>) {
        let hands = parse_hands(&test_input, false);
        let sorted = get_sorted_hands(&hands);

        assert_eq!(
            sorted,
            vec![
                &Hand {
                    cards: [3, 2, 10, 3, 13],
                    bid: 765
                },
                &Hand {
                    cards: [13, 10, 11, 11, 10],
                    bid: 220
                },
                &Hand {
                    cards: [13, 13, 6, 7, 7],
                    bid: 28
                },
                &Hand {
                    cards: [10, 5, 5, 11, 5],
                    bid: 684
                },
                &Hand {
                    cards: [12, 12, 12, 11, 14],
                    bid: 483
                },
            ]
        );
    }

    #[rstest]
    fn test_get_ranked_hands_with_jokers() {
        let hands = vec![
            Hand {
                cards: [0, 0, 0, 0, 2],
                bid: 0,
            },
            Hand {
                cards: [12, 12, 12, 12, 2],
                bid: 0,
            },
            Hand {
                cards: [0, 13, 13, 13, 2],
                bid: 0,
            },
        ];
        let sorted = get_sorted_hands(&hands);

        assert_eq!(
            sorted,
            vec![
                &Hand {
                    cards: [0, 13, 13, 13, 2],
                    bid: 0
                },
                &Hand {
                    cards: [12, 12, 12, 12, 2],
                    bid: 0
                },
                &Hand {
                    cards: [0, 0, 0, 0, 2],
                    bid: 0,
                },
            ]
        );
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let hands = parse_hands(&test_input, false);
        let res = get_total_winnings(&hands);

        assert_eq!(res, 6440);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let hands = parse_hands(&puzzle_input, false);
        let res = get_total_winnings(&hands);

        assert_eq!(res, 248836197);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let hands = parse_hands(&test_input, true);
        let res = get_total_winnings(&hands);

        assert_eq!(res, 5905);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let hands = parse_hands(&puzzle_input, true);
        let res = get_total_winnings(&hands);

        assert_eq!(res, 251195607);
    }
}
