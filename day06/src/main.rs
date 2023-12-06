use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let races = parse_races(input);
    let p1 = get_error_margin(&races);

    let race = parse_race(input);
    let p2 = race.get_number_of_winning_strategies();

    (p1, p2)
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn get_number_of_winning_strategies(&self) -> u64 {
        let a = -1 as f64;
        let b = self.time as f64;
        let c = -1f64 * self.record as f64;

        let x = (((-1f64 * b) + f64::sqrt(b * b - 4f64*a*c)) / (2f64 * a)).floor() as u64 + 1;

        self.time - (x * 2) + 1
    }
}

fn parse_races(input: &[String]) -> Vec<Race> {
    let times: Vec<u64> = input[0][9..]
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();
    let records: Vec<u64> = input[1][9..]
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    times
        .iter()
        .zip_eq(records)
        .map(|(&time, record)| Race { time, record })
        .collect()
}

fn parse_race(input: &[String]) -> Race {
    let time = input[0]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let record = input[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    Race { time, record }
}

fn get_error_margin(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|r| r.get_number_of_winning_strategies())
        .product()
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
            Time:      7  15   30
            Distance:  9  40  200
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day06.txt")
    }

    #[rstest]
    fn test_parse_races(test_input: Vec<String>) {
        let races = parse_races(&test_input);

        let expected = vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ];

        assert_eq!(races, expected);
    }

    #[rstest]
    fn test_parse_race(test_input: Vec<String>) {
        let race = parse_race(&test_input);

        assert_eq!(
            race,
            Race {
                time: 71530,
                record: 940200
            }
        );
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let races = parse_races(&test_input);

        assert_eq!(get_error_margin(&races), 288);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let races = parse_races(&puzzle_input);

        assert_eq!(get_error_margin(&races), 114400);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let race = parse_race(&test_input);

        assert_eq!(race.get_number_of_winning_strategies(), 71503);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let race = parse_race(&puzzle_input);

        assert_eq!(race.get_number_of_winning_strategies(), 21039729);
    }
}
