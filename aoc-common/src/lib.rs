use itertools::Itertools;
use std::env;
use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use textwrap::dedent;

pub fn get_input(filename: &str) -> Vec<String> {
    let path = format!("{}/../input/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn get_input_as_string(filename: &str) -> String {
    let path = format!("{}/../input/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let reader = match read_to_string(path) {
        Ok(r) => r,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    reader.parse().unwrap()
}

pub fn get_input_as_int<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Ord + FromStr>(
    filename: &str,
) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    get_input(filename)
        .iter()
        .map(|i| i.parse().unwrap())
        .collect()
}

/// Parse a puzzle's input data provided as a multi line string. The input is dedented first, then
/// the first and last lines are removed if they are empty.
/// This is useful for providing test input as a string.
pub fn parse_input(input: &str) -> Vec<String> {
    dedent(input)
        .trim()
        .split('\n')
        .map(String::from)
        .collect_vec()
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point<T>
where
    T: Clone + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "abc
123
foobar";

        let expected = vec!["abc", "123", "foobar"];
        assert_eq!(expected, parse_input(input));
    }
    #[test]
    fn test_parse_input_dedents_input() {
        let input = "
            abc
            123
            foobar
        ";

        let expected = vec!["abc", "123", "foobar"];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn test_parse_input_removes_empty_lines_at_start_and_end() {
        let input = "

            abc
            123

            foobar
        ";

        let expected = vec!["abc", "123", "", "foobar"];

        assert_eq!(expected, parse_input(input));
    }
}
