use std::fmt::Display;
use std::time::Instant;

use geo::algorithm::contains::Contains;
use geo::{coord, Coord, LineString, Polygon};
use pathfinding::prelude::strongly_connected_component;

use aoc_common::{format_duration, get_input, Point};

fn main() {
    let input = get_input("day10.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let map = parse_map(input);

    let p1 = get_farthest_from_start(&map);
    let p2 = get_tiles_in_loop(&map);

    (p1, p2)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Start,
    Ground,
    PipeNS,
    PipeEW,
    PipeNE,
    PipeNW,
    PipeSE,
    PipeSW,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Ground,
            '|' => Self::PipeNS,
            '-' => Self::PipeEW,
            'L' => Self::PipeNE,
            'J' => Self::PipeNW,
            'F' => Self::PipeSE,
            '7' => Self::PipeSW,
            _ => panic!("Invalid tile: {}", value),
        }
    }
}

type TileGrid = Vec<Vec<Tile>>;
type Position = Point<usize>;

#[derive(Debug, PartialEq)]
struct TileMap {
    height: usize,
    width: usize,
    start: Position,
    tiles: TileGrid,
}

impl TileMap {
    fn get_successors(&self, pos: &Position) -> Vec<Position> {
        let x = pos.x;
        let y = pos.y;
        let cur = self.tiles[x][y];
        let mut edges = Vec::new();

        if x > 0 {
            let val = self.tiles[x - 1][y];
            if is_walkable(cur, val, Direction::Up) {
                edges.push(Point { x: x - 1, y });
            }
        }
        if x < self.height - 1 {
            let val = self.tiles[x + 1][y];
            if is_walkable(cur, val, Direction::Down) {
                edges.push(Point { x: x + 1, y });
            }
        }
        if y > 0 {
            let val = self.tiles[x][y - 1];
            if is_walkable(cur, val, Direction::Left) {
                edges.push(Point { x, y: y - 1 });
            }
        }
        if y < self.width - 1 {
            let val = self.tiles[x][y + 1];
            if is_walkable(cur, val, Direction::Right) {
                edges.push(Point { x, y: y + 1 });
            }
        }

        if edges.len() > 2 {
            panic!("Too many successors!")
        }

        edges
    }

    fn get_loop(&self, start: &Position) -> Option<Vec<Position>> {
        let loop_ = strongly_connected_component(start, |p| self.get_successors(p));

        if loop_.len() > 1 {
            Some(loop_)
        } else {
            None
        }
    }
}

#[inline]
fn is_walkable(current: Tile, target: Tile, direction: Direction) -> bool {
    if target == Tile::Ground {
        return false;
    }

    match (current, direction, target) {
        (Tile::PipeNS, Direction::Up, Tile::PipeNS) => true,
        (Tile::PipeNS, Direction::Up, Tile::PipeSE) => true,
        (Tile::PipeNS, Direction::Up, Tile::PipeSW) => true,
        (Tile::PipeNS, Direction::Down, Tile::PipeNS) => true,
        (Tile::PipeNS, Direction::Down, Tile::PipeNE) => true,
        (Tile::PipeNS, Direction::Down, Tile::PipeNW) => true,

        (Tile::PipeEW, Direction::Left, Tile::PipeEW) => true,
        (Tile::PipeEW, Direction::Left, Tile::PipeSE) => true,
        (Tile::PipeEW, Direction::Left, Tile::PipeNE) => true,
        (Tile::PipeEW, Direction::Right, Tile::PipeEW) => true,
        (Tile::PipeEW, Direction::Right, Tile::PipeNW) => true,
        (Tile::PipeEW, Direction::Right, Tile::PipeSW) => true,

        (Tile::PipeNE, Direction::Up, Tile::PipeNS) => true,
        (Tile::PipeNE, Direction::Up, Tile::PipeSE) => true,
        (Tile::PipeNE, Direction::Up, Tile::PipeSW) => true,
        (Tile::PipeNE, Direction::Right, Tile::PipeEW) => true,
        (Tile::PipeNE, Direction::Right, Tile::PipeNW) => true,
        (Tile::PipeNE, Direction::Right, Tile::PipeSW) => true,

        (Tile::PipeNW, Direction::Up, Tile::PipeNS) => true,
        (Tile::PipeNW, Direction::Up, Tile::PipeSE) => true,
        (Tile::PipeNW, Direction::Up, Tile::PipeSW) => true,
        (Tile::PipeNW, Direction::Left, Tile::PipeEW) => true,
        (Tile::PipeNW, Direction::Left, Tile::PipeSE) => true,
        (Tile::PipeNW, Direction::Left, Tile::PipeNE) => true,

        (Tile::PipeSE, Direction::Down, Tile::PipeNS) => true,
        (Tile::PipeSE, Direction::Down, Tile::PipeNE) => true,
        (Tile::PipeSE, Direction::Down, Tile::PipeNW) => true,
        (Tile::PipeSE, Direction::Right, Tile::PipeEW) => true,
        (Tile::PipeSE, Direction::Right, Tile::PipeNW) => true,
        (Tile::PipeSE, Direction::Right, Tile::PipeSW) => true,

        (Tile::PipeSW, Direction::Down, Tile::PipeNS) => true,
        (Tile::PipeSW, Direction::Down, Tile::PipeNE) => true,
        (Tile::PipeSW, Direction::Down, Tile::PipeNW) => true,
        (Tile::PipeSW, Direction::Left, Tile::PipeEW) => true,
        (Tile::PipeSW, Direction::Left, Tile::PipeSE) => true,
        (Tile::PipeSW, Direction::Left, Tile::PipeNE) => true,

        _ => false,
    }
}

fn parse_map(input: &[String]) -> TileMap {
    let tiles = input
        .iter()
        .map(|i| i.chars().map(Tile::from).collect())
        .collect();

    let start = get_start(&tiles);

    let mut map = TileMap {
        height: input.len(),
        width: input[0].len(),
        start,
        tiles,
    };

    for tile in [
        Tile::PipeNS,
        Tile::PipeEW,
        Tile::PipeNE,
        Tile::PipeNW,
        Tile::PipeSE,
        Tile::PipeSW,
    ] {
        map.tiles[start.x][start.y] = tile;

        if map.get_successors(&start).len() == 2 {
            break;
        }
    }

    map
}

fn get_start(tiles: &TileGrid) -> Position {
    for (x, row) in tiles.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value == Tile::Start {
                return Point { x, y };
            }
        }
    }

    panic!("Start not found");
}

fn get_main_loop(map: &TileMap) -> Vec<Position> {
    let start = &map.start;

    if let Some(loop_) = map.get_loop(start) {
        return loop_;
    }

    panic!("No loop found")
}

fn get_farthest_from_start(map: &TileMap) -> usize {
    get_main_loop(map).len() / 2
}

fn get_tiles_in_loop(map: &TileMap) -> usize {
    let path_loop = get_main_loop(map);

    let ls = LineString::from(
        path_loop
            .iter()
            .map(|p| coord! {x: p.x as f64, y: p.y as f64})
            .collect::<Vec<Coord<f64>>>(),
    );
    let polygon = Polygon::new(ls, vec![]);
    let mut n = 0;

    for (x, row) in map.tiles.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if polygon.contains(&coord!(x: x as f64, y:y as f64)) {
                n += 1;
            }
        }
    }

    n
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
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day10.txt")
    }

    #[rstest]
    fn test_parse_map(test_input: Vec<String>) {
        let map = parse_map(&test_input);

        let expected = TileMap {
            height: 5,
            width: 5,
            start: Position::new(2, 0),
            tiles: vec![
                vec![
                    Tile::Ground,
                    Tile::Ground,
                    Tile::PipeSE,
                    Tile::PipeSW,
                    Tile::Ground,
                ],
                vec![
                    Tile::Ground,
                    Tile::PipeSE,
                    Tile::PipeNW,
                    Tile::PipeNS,
                    Tile::Ground,
                ],
                vec![
                    Tile::PipeSE,
                    Tile::PipeNW,
                    Tile::Ground,
                    Tile::PipeNE,
                    Tile::PipeSW,
                ],
                vec![
                    Tile::PipeNS,
                    Tile::PipeSE,
                    Tile::PipeEW,
                    Tile::PipeEW,
                    Tile::PipeNW,
                ],
                vec![
                    Tile::PipeNE,
                    Tile::PipeNW,
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                ],
            ],
        };

        assert_eq!(map, expected);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let map = parse_map(&test_input);

        let res = get_farthest_from_start(&map);

        assert_eq!(res, 8);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let map = parse_map(&puzzle_input);
        let res = get_farthest_from_start(&map);

        assert_eq!(res, 6867);
    }

    #[rstest]
    fn test_p2() {
        let test_input = parse_test_input(
            "
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ.F7FJ-
            L---JF-JLJ....FJLJJ7
            |F|F-JF---7...L7L|7|
            |FFJF7L7F-JF7..L---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        ",
        );
        let map = parse_map(&test_input);

        assert_eq!(get_tiles_in_loop(&map), 10);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let map = parse_map(&puzzle_input);
        let res = get_tiles_in_loop(&map);

        assert_eq!(res, 595);
    }
}
