use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::time::Instant;

use aoc_common::{format_duration, get_input, Point};

fn main() {
    let input = get_input("day16.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let floor = parse_floor(input);

    let p1 = get_energized_tiles(&floor, Beam::default());
    let p2 = get_max_energized_tiles(&floor);

    (p1, p2)
}

type Position = Point<i32>;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorLeft,
    MirrorRight,
    MirrorVertical,
    MirrorHorizontal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '\\' => Tile::MirrorLeft,
            '/' => Tile::MirrorRight,
            '|' => Tile::MirrorVertical,
            '-' => Tile::MirrorHorizontal,
            _ => panic!("Invalid tile"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dx(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            Direction::Left => 0,
            Direction::Right => 0,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn tick(&self, tile: &Tile) -> Vec<Beam> {
        let mut beams = Vec::new();

        match tile {
            Tile::Empty => beams.push(Beam {
                position: Point {
                    x: self.position.x + self.direction.dx(),
                    y: self.position.y + self.direction.dy(),
                },
                direction: self.direction,
            }),
            Tile::MirrorLeft => {
                let direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };

                beams.push(Beam {
                    position: Point {
                        x: self.position.x + direction.dx(),
                        y: self.position.y + direction.dy(),
                    },
                    direction,
                })
            }
            Tile::MirrorRight => {
                let direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                beams.push(Beam {
                    position: Point {
                        x: self.position.x + direction.dx(),
                        y: self.position.y + direction.dy(),
                    },
                    direction,
                })
            }
            Tile::MirrorHorizontal => {
                if self.direction == Direction::Left || self.direction == Direction::Right {
                    beams.push(Beam {
                        position: Point {
                            x: self.position.x + self.direction.dx(),
                            y: self.position.y + self.direction.dy(),
                        },
                        direction: self.direction,
                    })
                } else {
                    for direction in [Direction::Left, Direction::Right] {
                        beams.push(Beam {
                            position: Point {
                                x: self.position.x + direction.dx(),
                                y: self.position.y + direction.dy(),
                            },
                            direction,
                        })
                    }
                }
            }
            Tile::MirrorVertical => {
                if self.direction == Direction::Up || self.direction == Direction::Down {
                    beams.push(Beam {
                        position: Point {
                            x: self.position.x + self.direction.dx(),
                            y: self.position.y + self.direction.dy(),
                        },
                        direction: self.direction,
                    })
                } else {
                    for direction in [Direction::Up, Direction::Down] {
                        beams.push(Beam {
                            position: Point {
                                x: self.position.x + direction.dx(),
                                y: self.position.y + direction.dy(),
                            },
                            direction,
                        })
                    }
                }
            }
        }

        beams
    }
}

impl Default for Beam {
    fn default() -> Self {
        Beam {
            position: Position::new(0, 0),
            direction: Direction::Right,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Floor {
    height: i32,
    width: i32,
    tiles: Vec<Vec<Tile>>,
}

impl Floor {
    fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.height && pos.y < self.width
    }
}

fn parse_floor(input: &[String]) -> Floor {
    let tiles = input
        .iter()
        .map(|row| row.chars().map(Tile::from).collect())
        .collect();

    Floor {
        height: input.len() as i32,
        width: input[0].len() as i32,
        tiles,
    }
}

fn get_energized_tiles(floor: &Floor, starting_beam: Beam) -> usize {
    let mut beams = vec![starting_beam];
    let mut energized = HashSet::new();
    let mut seen_beams = HashSet::new();

    while !beams.is_empty() {
        let mut new_beams = Vec::new();

        for b in beams.iter_mut() {
            energized.insert(b.position);
            seen_beams.insert(b.clone());

            let tile = &floor.tiles[b.position.x as usize][b.position.y as usize];

            for nb in b.tick(tile) {
                if floor.is_within_bounds(&nb.position) && !seen_beams.contains(&nb) {
                    new_beams.push(nb);
                }
            }
        }

        beams = new_beams;
    }

    energized.len()
}

fn get_max_energized_tiles(floor: &Floor) -> usize {
    let mut max = 0;

    for i in 0..floor.width {
        let n = get_energized_tiles(
            floor,
            Beam {
                position: Position::new(0, i),
                direction: Direction::Down,
            },
        );
        if n > max {
            max = n;
        }

        let n = get_energized_tiles(
            floor,
            Beam {
                position: Position::new(floor.height - 1, i),
                direction: Direction::Up,
            },
        );
        if n > max {
            max = n;
        }
    }

    for i in 0..floor.height {
        let n = get_energized_tiles(
            floor,
            Beam {
                position: Position::new(i, 0),
                direction: Direction::Right,
            },
        );
        if n > max {
            max = n;
        }

        let n = get_energized_tiles(
            floor,
            Beam {
                position: Position::new(i, floor.width - 1),
                direction: Direction::Left,
            },
        );
        if n > max {
            max = n;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use aoc_common::parse_test_input;

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day16.txt")
    }

    #[rstest]
    fn test_parse_floor(test_input: Vec<String>) {
        let floor = parse_floor(&test_input);

        assert_eq!(
            floor,
            Floor {
                height: 10,
                width: 10,
                tiles: vec![
                    vec![
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorLeft,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::MirrorHorizontal,
                        Tile::Empty,
                        Tile::MirrorLeft,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::MirrorHorizontal,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorLeft,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorRight,
                        Tile::Empty,
                        Tile::MirrorLeft,
                        Tile::MirrorLeft,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::MirrorHorizontal,
                        Tile::Empty,
                        Tile::MirrorHorizontal,
                        Tile::MirrorRight,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorHorizontal,
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::MirrorLeft,
                    ],
                    vec![
                        Tile::Empty,
                        Tile::Empty,
                        Tile::MirrorRight,
                        Tile::MirrorRight,
                        Tile::Empty,
                        Tile::MirrorVertical,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                        Tile::Empty,
                    ],
                ],
            }
        )
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let floor = parse_floor(&test_input);

        assert_eq!(get_energized_tiles(&floor, Beam::default()), 46);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let floor = parse_floor(&puzzle_input);

        assert_eq!(get_energized_tiles(&floor, Beam::default()), 7111);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let floor = parse_floor(&test_input);

        assert_eq!(get_max_energized_tiles(&floor), 51);
    }

    #[rstest]
    #[ignore]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let floor = parse_floor(&puzzle_input);

        assert_eq!(get_max_energized_tiles(&floor), 7831);
    }
}
