use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(_input: &[String]) -> (impl Display, impl Display) {
    (0, 0)
}

fn main() {
    let input = get_input("day25.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}
