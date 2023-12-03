use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn main() {
    let input = get_input("day12.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(_input: &[String]) -> (impl Display, impl Display) {
    let p1 = 0;
    let p2 = 0;

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input("")
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day12.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }
}
