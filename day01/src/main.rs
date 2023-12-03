use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let numbers = extract_first_and_last_digits(input, false);
    let p1 = get_calibration_value(&numbers);
    let numbers = extract_first_and_last_digits(input, true);
    let p2 = get_calibration_value(&numbers);

    (p1, p2)
}

fn main() {
    let input = get_input("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn extract_first_and_last_digits(input: &[String], include_spelled_out: bool) -> Vec<(u32, u32)> {
    let mut all_digits = Vec::with_capacity(input.len());

    for entry in input {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for (i, c) in entry.chars().enumerate() {
            if c.is_numeric() {
                first = Some(c.to_digit(10).unwrap());
                break;
            }

            if !include_spelled_out {
                continue;
            }

            let substr = &entry[i..];

            if substr.starts_with("one") {
                first = Some(1);
                break;
            } else if substr.starts_with("two") {
                first = Some(2);
                break;
            } else if substr.starts_with("three") {
                first = Some(3);
                break;
            } else if substr.starts_with("four") {
                first = Some(4);
                break;
            } else if substr.starts_with("five") {
                first = Some(5);
                break;
            } else if substr.starts_with("six") {
                first = Some(6);
                break;
            } else if substr.starts_with("seven") {
                first = Some(7);
                break;
            } else if substr.starts_with("eight") {
                first = Some(8);
                break;
            } else if substr.starts_with("nine") {
                first = Some(9);
                break;
            }
        }

        for (i, c) in entry.chars().rev().enumerate() {
            if c.is_numeric() {
                last = Some(c.to_digit(10).unwrap());
                break;
            }

            if !include_spelled_out {
                continue;
            }

            let substr = &entry[entry.len() - i - 1..];

            if substr.starts_with("one") {
                last = Some(1);
                break;
            } else if substr.starts_with("two") {
                last = Some(2);
                break;
            } else if substr.starts_with("three") {
                last = Some(3);
                break;
            } else if substr.starts_with("four") {
                last = Some(4);
                break;
            } else if substr.starts_with("five") {
                last = Some(5);
                break;
            } else if substr.starts_with("six") {
                last = Some(6);
                break;
            } else if substr.starts_with("seven") {
                last = Some(7);
                break;
            } else if substr.starts_with("eight") {
                last = Some(8);
                break;
            } else if substr.starts_with("nine") {
                last = Some(9);
                break;
            }
        }

        let first = first.expect("string has no digit.");
        let last = last.expect("string has no digit.");

        all_digits.push((first, last))
    }

    all_digits
}

fn get_calibration_value(entries: &[(u32, u32)]) -> u32 {
    entries.iter().map(|e| e.0 * 10 + e.1).sum()
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;

    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_test_input(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            ",
        );

        let digits = extract_first_and_last_digits(&input, false);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 142);
    }

    #[test]
    fn test_p2() {
        let input = parse_test_input(
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            ",
        );

        let digits = extract_first_and_last_digits(&input, true);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 281);
    }

    #[test]
    fn test_p1_full_input() {
        let input = get_input("day01.txt");

        let digits = extract_first_and_last_digits(&input, false);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 56049);
    }

    #[test]
    fn test_p2_full_input() {
        let input = get_input("day01.txt");

        let digits = extract_first_and_last_digits(&input, true);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 54530);
    }
}
