use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let numbers = extract_digits(input, false);
    let p1 = get_calibration_value(&numbers);
    let numbers = extract_digits(input, true);
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

fn extract_digits(input: &[String], include_spelled_out: bool) -> Vec<Vec<u32>> {
    let mut all_digits = Vec::with_capacity(input.len());

    for entry in input {
        let mut digits_of_entry = Vec::new();

        for (i, c) in entry.chars().enumerate() {
            if c.is_numeric() {
                digits_of_entry.push(c.to_digit(10).unwrap());
                continue;
            }

            if !include_spelled_out {
                continue;
            }

            if entry[i..].starts_with("one") {
                digits_of_entry.push(1);
            } else if entry[i..].starts_with("two") {
                digits_of_entry.push(2);
            } else if entry[i..].starts_with("three") {
                digits_of_entry.push(3);
            } else if entry[i..].starts_with("four") {
                digits_of_entry.push(4);
            } else if entry[i..].starts_with("five") {
                digits_of_entry.push(5);
            } else if entry[i..].starts_with("six") {
                digits_of_entry.push(6);
            } else if entry[i..].starts_with("seven") {
                digits_of_entry.push(7);
            } else if entry[i..].starts_with("eight") {
                digits_of_entry.push(8);
            } else if entry[i..].starts_with("nine") {
                digits_of_entry.push(9);
            }
        }

        all_digits.push(digits_of_entry)
    }

    all_digits
}

fn get_calibration_value(entries: &[Vec<u32>]) -> u32 {
    let mut sum: u32 = 0;
    for digits in entries.iter() {
        sum += (*digits.first().unwrap()) * 10;
        sum += *digits.last().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_input;

    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            ",
        );

        let digits = extract_digits(&input, false);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 142);
    }

    #[test]
    fn test_p2() {
        let input = parse_input(
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

        let digits = extract_digits(&input, true);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 281);
    }

    #[test]
    fn test_p1_full_input() {
        let input = get_input("day01.txt");

        let digits = extract_digits(&input, false);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 56049);
    }

    #[test]
    fn test_p2_full_input() {
        let input = get_input("day01.txt");

        let digits = extract_digits(&input, true);
        let res = get_calibration_value(&digits);

        assert_eq!(res, 54530);
    }
}
