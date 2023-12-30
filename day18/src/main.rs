use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use geo::{coord, Contains, Coord, LineString, Polygon};
use inpt::{inpt, Inpt};
use regex::Regex;

use aoc_common::{format_duration, get_input, Point};

fn main() {
    let input = get_input("day18.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let instructions = parse_instructions(input);
    let plan = get_trench_plan(&instructions);
    let p1 = get_dug_out_size(&plan);

    // let instructions = parse_fixed_instructions(input);
    // let plan = get_trench_plan(&instructions);
    // let p2 = get_dug_out_size(&plan);
    let p2 = 0;

    (p1, p2)
}

type Position = Point<i64>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(format!("Invalid color: {}", s));
        }

        let r = u8::from_str_radix(&s[0..2], 16).unwrap();
        let g = u8::from_str_radix(&s[2..4], 16).unwrap();
        let b = u8::from_str_radix(&s[4..6], 16).unwrap();

        Ok(Self { r, g, b })
    }
}

#[derive(Debug, Eq, PartialEq, Inpt)]
#[inpt(regex = r"([UDLR]) ([\d]+).*")]
struct DigInstruction {
    #[inpt(from_str)]
    direction: Direction,
    length: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TrenchPlan {
    height: usize,
    width: usize,
    blocks: Vec<Position>,
}

impl TrenchPlan {
    fn new(blocks: Vec<Position>) -> Self {
        let height = blocks.iter().map(|b| b.x).max().unwrap() as usize + 1;
        let width = blocks.iter().map(|b| b.y).max().unwrap() as usize + 1;

        Self {
            height,
            width,
            blocks,
        }
    }
}

fn parse_instructions(input: &[String]) -> Vec<DigInstruction> {
    input
        .iter()
        .map(|i| inpt::<DigInstruction>(i).unwrap())
        .collect()
}

#[allow(dead_code)]
fn parse_fixed_instructions(input: &[String]) -> Vec<DigInstruction> {
    let code_re = Regex::new(r"#([0-9a-fA-F]{5})([0-9a-fA-F])").expect("Invalid regex");

    input
        .iter()
        .map(|i| {
            let cap = code_re.captures(i).unwrap();
            let length = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
            let direction = match cap.get(2).unwrap().as_str() {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                val => panic!("Invalid direction: {}", val),
            };

            DigInstruction { direction, length }
        })
        .collect()
}

fn get_trench_plan(instructions: &[DigInstruction]) -> TrenchPlan {
    let mut current = Position::new(0, 0);
    let mut trench_blocks = Vec::new();

    for instr in instructions {
        for _ in 0..instr.length {
            match instr.direction {
                Direction::Up => current.x -= 1,
                Direction::Down => current.x += 1,
                Direction::Left => current.y -= 1,
                Direction::Right => current.y += 1,
            }

            trench_blocks.push(current);
        }
    }

    let min_x = trench_blocks.iter().map(|b| b.x).min().unwrap();
    let min_y = trench_blocks.iter().map(|b| b.y).min().unwrap();

    for b in &mut trench_blocks {
        b.x -= min_x;
        b.y -= min_y;
    }

    TrenchPlan::new(trench_blocks)
}

fn get_dug_out_size(trench_plan: &TrenchPlan) -> usize {
    let ls = LineString::from(
        trench_plan
            .blocks
            .iter()
            .map(|p| coord! {x: p.x as f64, y: p.y as f64})
            .collect::<Vec<Coord<f64>>>(),
    );
    let polygon = Polygon::new(ls, vec![]);

    let inside_count: usize = (0..trench_plan.height)
        .map(|x| {
            (0..trench_plan.width)
                .filter(|&y| polygon.contains(&coord!(x: x as f64, y: y as f64)))
                .count()
        })
        .sum();

    inside_count + trench_plan.blocks.len()
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
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day18.txt")
    }

    #[rstest]
    fn test_parse_fixed_instructions(test_input: Vec<String>) {
        let instructions = parse_fixed_instructions(&test_input);

        assert_eq!(
            instructions,
            vec![
                DigInstruction {
                    direction: Direction::Right,
                    length: 461937,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 56407,
                },
                DigInstruction {
                    direction: Direction::Right,
                    length: 356671,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 863240,
                },
                DigInstruction {
                    direction: Direction::Right,
                    length: 367720,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 266681,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 577262,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 829975,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 112010,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 829975,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 491645,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 686074,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 5411,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 500254
                }
            ]
        );
    }

    #[rstest]
    fn test_parse_instructions(test_input: Vec<String>) {
        let instructions = parse_instructions(&test_input);

        assert_eq!(
            instructions,
            vec![
                DigInstruction {
                    direction: Direction::Right,
                    length: 6,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 5,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Right,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Down,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 5,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 1,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Right,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 3,
                },
                DigInstruction {
                    direction: Direction::Left,
                    length: 2,
                },
                DigInstruction {
                    direction: Direction::Up,
                    length: 2
                }
            ]
        );
    }

    #[rstest]
    fn test_get_trench(test_input: Vec<String>) {
        let instructions = parse_instructions(&test_input);
        let trench_plan = get_trench_plan(&instructions);

        let expected = TrenchPlan {
            width: 7,
            height: 10,
            blocks: vec![
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
                Point { x: 0, y: 4 },
                Point { x: 0, y: 5 },
                Point { x: 0, y: 6 },
                Point { x: 1, y: 6 },
                Point { x: 2, y: 6 },
                Point { x: 3, y: 6 },
                Point { x: 4, y: 6 },
                Point { x: 5, y: 6 },
                Point { x: 5, y: 5 },
                Point { x: 5, y: 4 },
                Point { x: 6, y: 4 },
                Point { x: 7, y: 4 },
                Point { x: 7, y: 5 },
                Point { x: 7, y: 6 },
                Point { x: 8, y: 6 },
                Point { x: 9, y: 6 },
                Point { x: 9, y: 5 },
                Point { x: 9, y: 4 },
                Point { x: 9, y: 3 },
                Point { x: 9, y: 2 },
                Point { x: 9, y: 1 },
                Point { x: 8, y: 1 },
                Point { x: 7, y: 1 },
                Point { x: 7, y: 0 },
                Point { x: 6, y: 0 },
                Point { x: 5, y: 0 },
                Point { x: 5, y: 1 },
                Point { x: 5, y: 2 },
                Point { x: 4, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 0 },
            ],
        };

        assert_eq!(trench_plan, expected);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let instructions = parse_instructions(&test_input);
        let trench_plan = get_trench_plan(&instructions);

        let res = get_dug_out_size(&trench_plan);

        assert_eq!(res, 62);
    }

    #[rstest]
    #[ignore]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let instructions = parse_instructions(&puzzle_input);
        let trench_plan = get_trench_plan(&instructions);

        let res = get_dug_out_size(&trench_plan);

        assert_eq!(res, 52055);
    }

    #[rstest]
    #[ignore]
    fn test_p2(test_input: Vec<String>) {
        let instructions = parse_fixed_instructions(&test_input);
        let trench_plan = get_trench_plan(&instructions);

        let res = get_dug_out_size(&trench_plan);

        assert_eq!(res, 952408144115);
    }

    #[rstest]
    #[ignore]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let instructions = parse_fixed_instructions(&puzzle_input);
        let trench_plan = get_trench_plan(&instructions);

        let res = get_dug_out_size(&trench_plan);

        assert_eq!(res, 952408144115);
    }
}
