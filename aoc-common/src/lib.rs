use itertools::Itertools;
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

    reader.trim().parse().unwrap()
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

pub fn format_duration(nanos: u128) -> String {
    let elapsed = nanos as f64 / 1000.0;

    if elapsed > 1000.0 {
        format!("{:.03}ms", elapsed / 1000.0)
    } else {
        format!("{:.03}μs", elapsed)
    }
}

/// Parse a puzzle's input data provided as a multi line string. The input is dedented first, then
/// the first and last lines are removed if they are empty.
/// This is useful for providing test input as a string.
pub fn parse_test_input(input: &str) -> Vec<String> {
    dedent(input)
        .trim()
        .split('\n')
        .map(String::from)
        .collect_vec()
}

/// Parse a puzzle's input data provided as a multi line string. The input is dedented first, then
/// the first and last lines are removed if they are empty.
/// This is useful for providing test input as a string.
pub fn parse_test_input_as_string(input: &str) -> String {
    dedent(input).trim().to_owned()
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_parse_input() {
        let input = "abc
123
foobar";

        let expected = vec!["abc", "123", "foobar"];
        assert_eq!(expected, parse_test_input(input));
    }
    #[rstest]
    fn test_parse_input_dedents_input() {
        let input = "
            abc
            123
            foobar
        ";

        let expected = vec!["abc", "123", "foobar"];

        assert_eq!(expected, parse_test_input(input));
    }

    #[rstest]
    fn test_parse_input_removes_empty_lines_at_start_and_end() {
        let input = "

            abc
            123

            foobar
        ";

        let expected = vec!["abc", "123", "", "foobar"];

        assert_eq!(expected, parse_test_input(input));
    }

    #[rstest]
    fn test_parse_input_as_string() {
        let input = "abc
123
foobar";

        let expected = "abc\n123\nfoobar".to_string();
        assert_eq!(expected, parse_test_input_as_string(input));
    }

    #[rstest]
    fn test_parse_input_as_string_dedents_input() {
        let input = "
            abc
            123
            foobar
        ";

        let expected = "abc\n123\nfoobar".to_string();
        assert_eq!(expected, parse_test_input_as_string(input));
    }

    #[rstest]
    fn test_parse_input_as_string_removes_empty_lines_at_start_and_end() {
        let input = "

            abc
            123

            foobar
        ";

        let expected = "abc\n123\n\nfoobar".to_string();
        assert_eq!(expected, parse_test_input_as_string(input));
    }

    #[rstest]
    #[case(1, "0.001μs")]
    #[case(1000, "1.000μs")]
    #[case(1234, "1.234μs")]
    #[case(123456, "123.456μs")]
    #[case(1234567, "1.235ms")]
    #[case(12345678, "12.346ms")]
    fn test_format_duration(#[case] nanos: u128, #[case] expected: &str) {
        assert_eq!(format_duration(nanos), expected);
    }
}
