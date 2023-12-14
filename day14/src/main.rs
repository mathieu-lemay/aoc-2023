use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input};

fn main() {
    let input = get_input("day14.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut grid = parse_grid(input);
    grid.tilt_north();
    let p1 = grid.get_load();

    let mut grid = parse_grid(input);
    grid.run_cycles(1_000_000_000);
    let p2 = grid.get_load();

    (p1, p2)
}

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Empty,
    Square,
    Round,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Element::Empty,
            '#' => Element::Square,
            'O' => Element::Round,
            _ => panic!("Invalid value: {}", value),
        }
    }
}

impl Element {
    fn to_char(&self) -> char {
        match *self {
            Element::Empty => '.',
            Element::Square => '#',
            Element::Round => 'O',
        }
    }
}

#[derive(PartialEq, Clone)]
struct Grid {
    height: usize,
    width: usize,
    values: Vec<Vec<Element>>,
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Grid {\n")?;

        for r in &self.values {
            f.write_fmt(format_args!(
                "{}\n",
                r.iter().map(Element::to_char).collect::<String>()
            ))?;
        }

        f.write_str("}\n")
    }
}

impl Grid {
    fn tilt_north(&mut self) {
        for col in 0..self.width {
            let mut ptr = 0;

            for row in 0..self.height {
                match self.values[row][col] {
                    Element::Empty => {}
                    Element::Square => ptr = row + 1,
                    Element::Round => {
                        if ptr != row {
                            self.values[ptr][col] = Element::Round;
                            self.values[row][col] = Element::Empty;
                        }
                        ptr += 1;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.width {
            let mut ptr = self.height - 1;

            for row in (0..self.height).rev() {
                match self.values[row][col] {
                    Element::Empty => {}
                    Element::Square => {
                        if row == 0 {
                            break;
                        }
                        ptr = row - 1;
                    }
                    Element::Round => {
                        if ptr != row {
                            self.values[ptr][col] = Element::Round;
                            self.values[row][col] = Element::Empty;
                        }
                        if row == 0 {
                            break;
                        }
                        ptr -= 1;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in &mut self.values {
            let mut ptr = self.width - 1;

            for col in (0..self.width).rev() {
                match row[col] {
                    Element::Empty => {}
                    Element::Square => {
                        if col == 0 {
                            break;
                        }
                        ptr = col - 1;
                    }
                    Element::Round => {
                        if ptr != col {
                            row[ptr] = Element::Round;
                            row[col] = Element::Empty;
                        }
                        if col == 0 {
                            break;
                        }
                        ptr -= 1;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in &mut self.values {
            let mut ptr = 0;

            for col in 0..self.width {
                match row[col] {
                    Element::Empty => {}
                    Element::Square => ptr = col + 1,
                    Element::Round => {
                        if ptr != col {
                            row[ptr] = Element::Round;
                            row[col] = Element::Empty;
                        }
                        ptr += 1;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn run_cycles(&mut self, cycles: usize) {
        let mut remaining = cycles;
        let mut last_states = VecDeque::with_capacity(128);

        while remaining > 0 {
            remaining -= 1;

            self.cycle();

            if let Some(pos) = last_states.iter().position(|g| g == self) {
                remaining %= pos + 1;

                break;
            }

            if last_states.len() == 128 {
                last_states.pop_back();
            }

            last_states.push_front(self.clone());
        }

        for _ in 0..remaining {
            self.cycle();
        }
    }

    fn get_load(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                row.iter().filter(|&e| e == &Element::Round).count() * (self.height - idx)
            })
            .sum()
    }
}

fn parse_grid(input: &[String]) -> Grid {
    let height = input.len();
    let width = input[0].len();

    let values = input
        .iter()
        .map(|r| r.chars().map(Element::from).collect())
        .collect();

    Grid {
        height,
        width,
        values,
    }
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
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day14.txt")
    }

    #[rstest]
    fn test_parse_grid(test_input: Vec<String>) {
        let grid = parse_grid(&test_input);

        assert_eq!(
            grid,
            Grid {
                height: 10,
                width: 10,
                values: vec![
                    vec![
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty
                    ],
                    vec![
                        Element::Round,
                        Element::Empty,
                        Element::Round,
                        Element::Round,
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Square
                    ],
                    vec![
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Square,
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty
                    ],
                    vec![
                        Element::Round,
                        Element::Round,
                        Element::Empty,
                        Element::Square,
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Round
                    ],
                    vec![
                        Element::Empty,
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Round,
                        Element::Square,
                        Element::Empty
                    ],
                    vec![
                        Element::Round,
                        Element::Empty,
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Round,
                        Element::Empty,
                        Element::Square,
                        Element::Empty,
                        Element::Square
                    ],
                    vec![
                        Element::Empty,
                        Element::Empty,
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Square,
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Round
                    ],
                    vec![
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Round,
                        Element::Empty,
                        Element::Empty
                    ],
                    vec![
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Square,
                        Element::Square,
                        Element::Square,
                        Element::Empty,
                        Element::Empty
                    ],
                    vec![
                        Element::Square,
                        Element::Round,
                        Element::Round,
                        Element::Empty,
                        Element::Empty,
                        Element::Square,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty,
                        Element::Empty
                    ]
                ],
            }
        );
    }

    #[rstest]
    fn test_tilt_north(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);

        grid.tilt_north();

        let expected_grid = parse_grid(&parse_test_input(
            "
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        ",
        ));

        assert_eq!(grid, expected_grid);
    }

    #[rstest]
    fn test_tilt_south(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);

        grid.tilt_south();

        let expected_grid = parse_grid(&parse_test_input(
            "
            .....#....
            ....#....#
            ...O.##...
            ...#......
            O.O....O#O
            O.#..O.#.#
            O....#....
            OO....OO..
            #OO..###..
            #OO.O#...O
        ",
        ));

        assert_eq!(grid, expected_grid);
    }

    #[rstest]
    fn test_tilt_east(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);

        grid.tilt_east();

        let expected_grid = parse_grid(&parse_test_input(
            "
            ....O#....
            .OOO#....#
            .....##...
            .OO#....OO
            ......OO#.
            .O#...O#.#
            ....O#..OO
            .........O
            #....###..
            #..OO#....
        ",
        ));

        assert_eq!(grid, expected_grid);
    }

    #[rstest]
    fn test_tilt_west(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);

        grid.tilt_west();

        let expected_grid = parse_grid(&parse_test_input(
            "
            O....#....
            OOO.#....#
            .....##...
            OO.#OO....
            OO......#.
            O.#O...#.#
            O....#OO..
            O.........
            #....###..
            #OO..#....
        ",
        ));

        assert_eq!(grid, expected_grid);
    }

    #[rstest]
    fn test_get_load(test_input: Vec<String>) {
        let grid = parse_grid(&test_input);

        assert_eq!(grid.get_load(), 104);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);
        grid.tilt_north();

        assert_eq!(grid.get_load(), 136)
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut grid = parse_grid(&puzzle_input);
        grid.tilt_north();

        assert_eq!(grid.get_load(), 110407)
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let mut grid = parse_grid(&test_input);
        grid.run_cycles(1_000_000_000);

        assert_eq!(grid.get_load(), 64)
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let mut grid = parse_grid(&puzzle_input);
        grid.run_cycles(1_000_000_000);

        assert_eq!(grid.get_load(), 87273)
    }
}
