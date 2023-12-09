use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input};

fn main() {
    let input = get_input("day09.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let oasis = parse_oasis(input);

    let p1 = get_sum_of_next_values(&oasis);
    let p2 = get_sum_of_previous_values(&oasis);

    (p1, p2)
}

#[derive(Debug, PartialEq, Eq)]
struct Sequence {
    values: Vec<i64>,
    decrements: Vec<i64>,
    increments: Vec<i64>,
}

impl Sequence {
    fn new(values: Vec<i64>) -> Self {
        let mut decrements = vec![*values.first().unwrap()];
        let mut increments = vec![*values.last().unwrap()];

        let mut deltas: Vec<i64> = values[..values.len() - 1]
            .iter()
            .zip(&values[1..])
            .map(|(a, b)| b - a)
            .collect();

        loop {
            decrements.insert(0, *deltas.first().unwrap());
            increments.insert(0, *deltas.last().unwrap());

            deltas = deltas[..deltas.len() - 1]
                .iter()
                .zip(&deltas[1..])
                .map(|(a, b)| b - a)
                .collect();

            if deltas.iter().all(|&i| i == 0) {
                decrements.insert(0, 0);
                increments.insert(0, 0);
                break;
            }
        }

        Self {
            values,
            increments,
            decrements,
        }
    }

    fn extrapolate(&self) -> i64 {
        self.increments.iter().fold(0, |acc, v| v + acc)
    }

    fn extrapolate_backwards(&self) -> i64 {
        self.decrements.iter().fold(0, |acc, v| v - acc)
    }
}

fn parse_oasis(input: &[String]) -> Vec<Sequence> {
    input
        .iter()
        .map(|i| {
            let values = i.split(' ').filter_map(|i| i.parse::<i64>().ok()).collect();
            Sequence::new(values)
        })
        .collect()
}

fn get_sum_of_next_values(oasis: &[Sequence]) -> i64 {
    oasis.iter().map(|s| s.extrapolate()).sum()
}

fn get_sum_of_previous_values(oasis: &[Sequence]) -> i64 {
    oasis.iter().map(|s| s.extrapolate_backwards()).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use aoc_common::parse_test_input;

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day09.txt")
    }

    #[rstest]
    fn test_sequence_new() {
        let values = vec![10, 13, 16, 21, 30, 45];
        let seq = Sequence::new(values);

        assert_eq!(
            seq,
            Sequence {
                values: vec![10, 13, 16, 21, 30, 45],
                decrements: vec![0, 2, 0, 3, 10],
                increments: vec![0, 2, 6, 15, 45],
            }
        );
    }

    #[rstest]
    fn test_sequence_extrapolate() {
        let values = vec![10, 13, 16, 21, 30, 45];

        let seq = Sequence::new(values);

        let prediction = seq.extrapolate();

        assert_eq!(prediction, 68);
    }

    #[rstest]
    fn test_sequence_extrapolate_backwards() {
        let values = vec![10, 13, 16, 21, 30, 45];

        let seq = Sequence::new(values);

        let prediction = seq.extrapolate_backwards();

        assert_eq!(prediction, 5);
    }

    #[rstest]
    fn test_parse_oasis(test_input: Vec<String>) {
        let oasis = parse_oasis(&test_input);

        assert_eq!(
            oasis,
            vec![
                Sequence {
                    values: vec![0, 3, 6, 9, 12, 15],
                    decrements: vec![0, 3, 0],
                    increments: vec![0, 3, 15],
                },
                Sequence {
                    values: vec![1, 3, 6, 10, 15, 21],
                    decrements: vec![0, 1, 2, 1],
                    increments: vec![0, 1, 6, 21],
                },
                Sequence {
                    values: vec![10, 13, 16, 21, 30, 45],
                    decrements: vec![0, 2, 0, 3, 10],
                    increments: vec![0, 2, 6, 15, 45],
                },
            ]
        );
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let oasis = parse_oasis(&test_input);
        let res = get_sum_of_next_values(&oasis);

        assert_eq!(res, 114);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let oasis = parse_oasis(&puzzle_input);
        let res = get_sum_of_next_values(&oasis);

        assert_eq!(res, 2043183816);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let oasis = parse_oasis(&test_input);
        let res = get_sum_of_previous_values(&oasis);

        assert_eq!(res, 2);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let oasis = parse_oasis(&puzzle_input);
        let res = get_sum_of_previous_values(&oasis);

        assert_eq!(res, 1118);
    }
}
