use itertools::Itertools;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{get_input, Point};
use regex::Regex;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let board = parse_board(input);

    let p1 = board.get_sum_of_valid_parts();
    let p2 = board.get_sum_of_gear_ratios();

    (p1, p2)
}

fn main() {
    let input = get_input("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[derive(Debug, Eq, PartialEq)]
struct EnginePart {
    value: u32,
    position: (Point<usize>, Point<usize>),
}

impl EnginePart {
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        let y = self.position.0.y;

        (self.position.0.x..=self.position.1.x)
            .any(|x| symbol.position.x.abs_diff(x) <= 1 && symbol.position.y.abs_diff(y) <= 1)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Symbol {
    value: char,
    position: Point<usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct Board {
    parts: Vec<EnginePart>,
    symbols: Vec<Symbol>,
}

impl Board {
    fn get_valid_parts(&self) -> Vec<&EnginePart> {
        let mut valid_parts = Vec::new();

        for p in &self.parts {
            for s in &self.symbols {
                if p.is_adjacent_to(s) {
                    valid_parts.push(p);
                    break;
                }
            }
        }

        valid_parts
    }

    fn get_sum_of_valid_parts(&self) -> u32 {
        self.get_valid_parts().iter().map(|p| p.value).sum()
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        self.symbols
            .iter()
            .filter(|s| s.value == '*')
            .filter_map(|s| {
                let adjacent = self
                    .parts
                    .iter()
                    .filter(|p| p.is_adjacent_to(s))
                    .collect_vec();

                if adjacent.len() == 2 {
                    Some(adjacent.iter().fold(1, |acc, p| acc * p.value))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_sum_of_gear_ratios(&self) -> u32 {
        self.get_gear_ratios().iter().sum()
    }
}

fn parse_board(input: &[String]) -> Board {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    let part_re = Regex::new(r"([0-9]+)").expect("Invalid regex");
    let symbol_re = Regex::new(r"([^0-9.])").expect("Invalid regex");

    for (y, line) in input.iter().enumerate() {
        for caps in part_re.captures_iter(line) {
            let m = caps.get(1).unwrap();
            let value = m.as_str().parse::<u32>().unwrap();
            let start = Point::new(m.start(), y);
            let end = Point::new(m.end() - 1, y);
            parts.push(EnginePart {
                value,
                position: (start, end),
            })
        }
        for caps in symbol_re.captures_iter(line) {
            let m = caps.get(1).unwrap();
            let value = m.as_str().chars().next().unwrap();
            let position = Point::new(m.start(), y);
            symbols.push(Symbol { value, position })
        }
    }

    Board { parts, symbols }
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_input;

    use super::*;

    #[test]
    fn test_parse_board() {
        let input = parse_input(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        let board = parse_board(&input);

        let expected = Board {
            parts: vec![
                EnginePart {
                    value: 467,
                    position: (Point { x: 0, y: 0 }, Point { x: 2, y: 0 }),
                },
                EnginePart {
                    value: 114,
                    position: (Point { x: 5, y: 0 }, Point { x: 7, y: 0 }),
                },
                EnginePart {
                    value: 35,
                    position: (Point { x: 2, y: 2 }, Point { x: 3, y: 2 }),
                },
                EnginePart {
                    value: 633,
                    position: (Point { x: 6, y: 2 }, Point { x: 8, y: 2 }),
                },
                EnginePart {
                    value: 617,
                    position: (Point { x: 0, y: 4 }, Point { x: 2, y: 4 }),
                },
                EnginePart {
                    value: 58,
                    position: (Point { x: 7, y: 5 }, Point { x: 8, y: 5 }),
                },
                EnginePart {
                    value: 592,
                    position: (Point { x: 2, y: 6 }, Point { x: 4, y: 6 }),
                },
                EnginePart {
                    value: 755,
                    position: (Point { x: 6, y: 7 }, Point { x: 8, y: 7 }),
                },
                EnginePart {
                    value: 664,
                    position: (Point { x: 1, y: 9 }, Point { x: 3, y: 9 }),
                },
                EnginePart {
                    value: 598,
                    position: (Point { x: 5, y: 9 }, Point { x: 7, y: 9 }),
                },
            ],
            symbols: vec![
                Symbol {
                    value: '*',
                    position: Point::new(3, 1),
                },
                Symbol {
                    value: '#',
                    position: Point::new(6, 3),
                },
                Symbol {
                    value: '*',
                    position: Point::new(3, 4),
                },
                Symbol {
                    value: '+',
                    position: Point::new(5, 5),
                },
                Symbol {
                    value: '$',
                    position: Point::new(3, 8),
                },
                Symbol {
                    value: '*',
                    position: Point::new(5, 8),
                },
            ],
        };

        assert_eq!(board, expected);
    }

    #[test]
    fn test_get_valid_parts() {
        let input = parse_input(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        let board = parse_board(&input);
        let valid = board.get_valid_parts();

        let expected = vec![
            &EnginePart {
                value: 467,
                position: (Point { x: 0, y: 0 }, Point { x: 2, y: 0 }),
            },
            &EnginePart {
                value: 35,
                position: (Point { x: 2, y: 2 }, Point { x: 3, y: 2 }),
            },
            &EnginePart {
                value: 633,
                position: (Point { x: 6, y: 2 }, Point { x: 8, y: 2 }),
            },
            &EnginePart {
                value: 617,
                position: (Point { x: 0, y: 4 }, Point { x: 2, y: 4 }),
            },
            &EnginePart {
                value: 592,
                position: (Point { x: 2, y: 6 }, Point { x: 4, y: 6 }),
            },
            &EnginePart {
                value: 755,
                position: (Point { x: 6, y: 7 }, Point { x: 8, y: 7 }),
            },
            &EnginePart {
                value: 664,
                position: (Point { x: 1, y: 9 }, Point { x: 3, y: 9 }),
            },
            &EnginePart {
                value: 598,
                position: (Point { x: 5, y: 9 }, Point { x: 7, y: 9 }),
            },
        ];

        assert_eq!(valid, expected);
    }

    #[test]
    fn test_get_gear_ratios() {
        let input = parse_input(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        let board = parse_board(&input);
        assert_eq!(board.get_gear_ratios(), vec![16345, 451490]);
    }

    #[test]
    fn test_p1() {
        let input = parse_input(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        let board = parse_board(&input);
        let res = board.get_sum_of_valid_parts();

        assert_eq!(res, 4361);
    }

    #[test]
    fn test_p1_full_input() {
        let input = get_input("day03.txt");

        let board = parse_board(&input);
        let res = board.get_sum_of_valid_parts();

        assert_eq!(res, 535351);
    }

    #[test]
    fn test_p2() {
        let input = parse_input(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );

        let board = parse_board(&input);
        let res = board.get_sum_of_gear_ratios();

        assert_eq!(res, 467835);
    }

    #[test]
    fn test_p2_full_input() {
        let input = get_input("day03.txt");

        let board = parse_board(&input);
        let res = board.get_sum_of_gear_ratios();

        assert_eq!(res, 87287096);
    }
}
