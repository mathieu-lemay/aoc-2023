use std::time::Instant;
use std::{collections::HashSet, fmt::Display};

use aoc_common::{format_duration, get_input, Point};

fn main() {
    let input = get_input("day11.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let space_map = parse_space_map(input);

    let p1 = get_sum_of_minimum_distances(&space_map, 2);
    let p2 = get_sum_of_minimum_distances(&space_map, 1_000_000);

    (p1, p2)
}

type Position = Point<usize>;

#[derive(Debug, PartialEq)]
struct SpaceMap {
    height: usize,
    width: usize,
    galaxies: Vec<Position>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

impl SpaceMap {
    fn get_distance(&self, idx_a: usize, idx_b: usize, expansion_factor: usize) -> usize {
        let ga = self.galaxies[idx_a];
        let gb = self.galaxies[idx_b];

        let x1 = ga.x.min(gb.x);
        let x2 = ga.x.max(gb.x);
        let y1 = ga.y.min(gb.y);
        let y2 = ga.y.max(gb.y);

        let dx = x2 - x1;
        let dy = y2 - y1;

        let exp_x = self
            .empty_rows
            .iter()
            .filter(|&&r| r > x1 && r < x2)
            .count()
            * (expansion_factor - 1);
        let exp_y = self
            .empty_columns
            .iter()
            .filter(|&&r| r > y1 && r < y2)
            .count()
            * (expansion_factor - 1);

        dx + dy + exp_x + exp_y
    }
}

fn parse_space_map(input: &[String]) -> SpaceMap {
    let height = input.len();
    let width = input[0].len();

    let mut galaxies = Vec::new();

    for (x, row) in input.iter().enumerate() {
        for (y, i) in row.chars().enumerate() {
            if i == '#' {
                galaxies.push(Position::new(x, y));
            }
        }
    }

    let occupied_rows = galaxies.iter().map(|g| g.x).collect::<HashSet<usize>>();
    let occupied_columns = galaxies.iter().map(|g| g.y).collect::<HashSet<usize>>();

    let empty_rows = (0..height).filter(|i| !occupied_rows.contains(i)).collect();
    let empty_columns = (0..width)
        .filter(|i| !occupied_columns.contains(i))
        .collect();

    SpaceMap {
        height,
        width,
        galaxies,
        empty_rows,
        empty_columns,
    }
}

fn get_sum_of_minimum_distances(space_map: &SpaceMap, expansion_factor: usize) -> usize {
    let nb_galaxies = space_map.galaxies.len();

    let distances: Vec<usize> = (0..nb_galaxies - 1)
        .flat_map(|a| {
            (a + 1..nb_galaxies).map(move |b| space_map.get_distance(a, b, expansion_factor))
        })
        .collect();

    distances.iter().sum()
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
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day11.txt")
    }

    #[rstest]
    fn test_parse_space_map(test_input: Vec<String>) {
        let space_map = parse_space_map(&test_input);

        let expected_map = SpaceMap {
            height: 10,
            width: 10,
            galaxies: vec![
                Position::new(0, 3),
                Position::new(1, 7),
                Position::new(2, 0),
                Position::new(4, 6),
                Position::new(5, 1),
                Position::new(6, 9),
                Position::new(8, 7),
                Position::new(9, 0),
                Position::new(9, 4),
            ],
            empty_rows: vec![3, 7],
            empty_columns: vec![2, 5, 8],
        };

        assert_eq!(space_map, expected_map);
    }

    #[rstest]
    #[case(4, 8, 9)]
    #[case(0, 6, 15)]
    #[case(2, 5, 17)]
    #[case(7, 8, 5)]
    fn test_get_distance(
        test_input: Vec<String>,
        #[case] x: usize,
        #[case] y: usize,
        #[case] expected: usize,
    ) {
        let space_map = parse_space_map(&test_input);

        assert_eq!(space_map.get_distance(x, y, 2), expected);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let space_map = parse_space_map(&test_input);

        assert_eq!(get_sum_of_minimum_distances(&space_map, 2), 374);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let space_map = parse_space_map(&puzzle_input);

        assert_eq!(get_sum_of_minimum_distances(&space_map, 2), 9623138);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let space_map = parse_space_map(&test_input);

        assert_eq!(get_sum_of_minimum_distances(&space_map, 10), 1030);
        assert_eq!(get_sum_of_minimum_distances(&space_map, 100), 8410);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let space_map = parse_space_map(&puzzle_input);

        assert_eq!(
            get_sum_of_minimum_distances(&space_map, 1_000_000),
            726820169514
        );
    }
}
