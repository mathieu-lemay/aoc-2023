use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input};

fn main() {
    let input = get_input("day13.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let patterns = parse_patterns(input);
    let mirrors = find_mirrors(&patterns);
    let mirrors_with_smudge = find_mirrors_with_smudge(&patterns);

    let p1 = get_summary_value(&mirrors);
    let p2 = get_summary_value(&mirrors_with_smudge);

    (p1, p2)
}

#[derive(Debug, PartialEq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

impl Mirror {
    fn value(&self) -> usize {
        match *self {
            Self::Vertical(i) => i,
            Self::Horizontal(i) => i * 100,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pattern {
    height: usize,
    width: usize,
    rows: Vec<u64>,
    cols: Vec<u64>,
}

fn parse_patterns(input: &[String]) -> Vec<Pattern> {
    input.split(|i| i.is_empty()).map(parse_pattern).collect()
}

fn parse_pattern(input: &[String]) -> Pattern {
    let height = input.len();
    let width = input[0].len();

    let mut rows = vec![0; height];
    let mut cols = vec![0; width];

    for (x, row) in input.iter().enumerate() {
        for (y, item) in row.chars().enumerate() {
            if item != '#' {
                continue;
            }

            rows[x] |= 1 << (width - y - 1);
            cols[y] |= 1 << (height - x - 1);
        }
    }

    Pattern {
        height,
        width,
        rows,
        cols,
    }
}

fn find_mirrors(patterns: &[Pattern]) -> Vec<Mirror> {
    patterns.iter().map(find_mirror).collect()
}

fn find_mirrors_with_smudge(patterns: &[Pattern]) -> Vec<Mirror> {
    patterns.iter().map(find_mirror_with_smudge).collect()
}

fn is_mirrored(values: &[u64]) -> bool {
    let count = values.len();
    if count % 2 != 0 {
        return false;
    }

    (0..count / 2).all(|i| values[i] == values[count - i - 1])
}

fn find_mirror(pattern: &Pattern) -> Mirror {
    let nrows = pattern.rows.len();

    for i in 0..nrows - 1 {
        if is_mirrored(&pattern.rows[i..]) {
            return Mirror::Horizontal((nrows + i) / 2);
        }
        if is_mirrored(&pattern.rows[..nrows - i]) {
            return Mirror::Horizontal((nrows + i) / 2 - i);
        }
    }

    let ncols = pattern.cols.len();

    for i in 0..ncols - 1 {
        if is_mirrored(&pattern.cols[i..]) {
            return Mirror::Vertical((ncols + i) / 2);
        }

        if is_mirrored(&pattern.cols[..ncols - i]) {
            return Mirror::Vertical((ncols + i) / 2 - i);
        }
    }

    panic!("No mirror found")
}

fn is_mirrored_with_one_smudge(values: &[u64]) -> bool {
    let count = values.len();
    if count % 2 != 0 {
        return false;
    }
    let mut total = 0;

    for i in 0..count / 2 {
        total += (values[i] ^ values[count - i - 1]).count_ones();

        if total > 1 {
            return false;
        }
    }

    total == 1
}

fn find_mirror_with_smudge(pattern: &Pattern) -> Mirror {
    let nrows = pattern.rows.len();

    for i in 0..nrows - 1 {
        if is_mirrored_with_one_smudge(&pattern.rows[i..]) {
            return Mirror::Horizontal((nrows + i) / 2);
        }
        if is_mirrored_with_one_smudge(&pattern.rows[..nrows - i]) {
            return Mirror::Horizontal((nrows + i) / 2 - i);
        }
    }

    let ncols = pattern.cols.len();

    for i in 0..ncols - 1 {
        if is_mirrored_with_one_smudge(&pattern.cols[i..]) {
            return Mirror::Vertical((ncols + i) / 2);
        }
        if is_mirrored_with_one_smudge(&pattern.cols[..ncols - i]) {
            return Mirror::Vertical((ncols + i) / 2 - i);
        }
    }

    panic!("No mirror found")
}

fn get_summary_value(mirrors: &[Mirror]) -> usize {
    mirrors.iter().map(|m| m.value()).sum()
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
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day13.txt")
    }

    #[rstest]
    fn test_parse_patterns(test_input: Vec<String>) {
        let patterns = parse_patterns(&test_input);

        assert_eq!(
            patterns,
            vec![
                Pattern {
                    height: 7,
                    width: 9,
                    rows: vec![
                        0b101100110,
                        0b1011010,
                        0b110000001,
                        0b110000001,
                        0b1011010,
                        0b1100110,
                        0b0101011010
                    ],
                    cols: vec![
                        0b1011001, 0b11000, 0b1100111, 0b1000010, 0b100101, 0b100101, 0b1000010,
                        0b1100111, 0b11000
                    ],
                },
                Pattern {
                    height: 7,
                    width: 9,
                    rows: vec![
                        0b100011001,
                        0b100001001,
                        0b1100111,
                        0b111110110,
                        0b111110110,
                        0b1100111,
                        0b100001001
                    ],
                    cols: vec![
                        0b1101101, 0b1100, 0b11110, 0b11110, 0b1001100, 0b1100001, 0b11110,
                        0b11110, 0b1110011
                    ],
                },
            ]
        )
    }

    #[rstest]
    fn test_find_mirrors(test_input: Vec<String>) {
        let patterns = parse_patterns(&test_input);

        let mirrors = find_mirrors(&patterns);

        assert_eq!(mirrors, vec![Mirror::Vertical(5), Mirror::Horizontal(4)]);
    }

    #[rstest]
    #[case(0, false, Mirror::Vertical(5))]
    #[case(1, false, Mirror::Horizontal(4))]
    #[case(0, true, Mirror::Vertical(4))]
    #[case(1, true, Mirror::Horizontal(3))]
    fn test_find_mirror(
        test_input: Vec<String>,
        #[case] pattern_idx: usize,
        #[case] reversed: bool,
        #[case] expected_mirror: Mirror,
    ) {
        let mut pattern = parse_patterns(&test_input)[pattern_idx].clone();

        if reversed {
            pattern = Pattern {
                rows: pattern.rows.iter().rev().copied().collect(),
                cols: pattern.cols.iter().rev().copied().collect(),
                ..pattern
            }
        }

        assert_eq!(find_mirror(&pattern), expected_mirror);
    }

    #[rstest]
    #[case(0, Mirror::Vertical(8))]
    #[case(1, Mirror::Vertical(6))]
    #[case(2, Mirror::Horizontal(8))]
    #[case(3, Mirror::Vertical(1))]
    #[case(4, Mirror::Vertical(2))]
    #[case(5, Mirror::Horizontal(5))]
    #[case(6, Mirror::Vertical(5))]
    #[case(7, Mirror::Vertical(16))]
    #[case(8, Mirror::Horizontal(14))]
    #[case(9, Mirror::Vertical(1))]
    #[case(10, Mirror::Vertical(3))]
    #[case(11, Mirror::Horizontal(2))]
    #[case(12, Mirror::Vertical(4))]
    #[case(13, Mirror::Vertical(12))]
    #[case(14, Mirror::Vertical(1))]
    #[case(15, Mirror::Horizontal(13))]
    #[case(16, Mirror::Horizontal(1))]
    #[case(17, Mirror::Vertical(12))]
    #[case(18, Mirror::Vertical(7))]
    #[case(19, Mirror::Horizontal(6))]
    #[case(20, Mirror::Horizontal(3))]
    #[case(21, Mirror::Horizontal(1))]
    #[case(22, Mirror::Horizontal(11))]
    #[case(23, Mirror::Horizontal(1))]
    #[case(24, Mirror::Horizontal(10))]
    #[case(25, Mirror::Horizontal(4))]
    #[case(26, Mirror::Vertical(2))]
    #[case(27, Mirror::Vertical(1))]
    #[case(28, Mirror::Horizontal(11))]
    #[case(29, Mirror::Vertical(1))]
    #[case(30, Mirror::Horizontal(4))]
    #[case(31, Mirror::Horizontal(1))]
    #[case(32, Mirror::Vertical(10))]
    #[case(33, Mirror::Vertical(4))]
    #[case(34, Mirror::Horizontal(1))]
    #[case(35, Mirror::Horizontal(1))]
    #[case(36, Mirror::Horizontal(4))]
    #[case(37, Mirror::Horizontal(15))]
    #[case(38, Mirror::Horizontal(14))]
    #[case(39, Mirror::Horizontal(11))]
    #[case(40, Mirror::Vertical(1))]
    #[case(41, Mirror::Horizontal(9))]
    #[case(42, Mirror::Horizontal(9))]
    #[case(43, Mirror::Horizontal(3))]
    #[case(44, Mirror::Vertical(2))]
    #[case(45, Mirror::Horizontal(1))]
    #[case(46, Mirror::Horizontal(9))]
    #[case(47, Mirror::Vertical(2))]
    #[case(48, Mirror::Horizontal(9))]
    #[case(49, Mirror::Horizontal(14))]
    #[case(50, Mirror::Vertical(14))]
    #[case(51, Mirror::Horizontal(1))]
    #[case(52, Mirror::Vertical(14))]
    #[case(53, Mirror::Vertical(2))]
    #[case(54, Mirror::Vertical(11))]
    #[case(55, Mirror::Horizontal(1))]
    #[case(56, Mirror::Horizontal(6))]
    #[case(57, Mirror::Vertical(10))]
    #[case(58, Mirror::Vertical(1))]
    #[case(59, Mirror::Vertical(16))]
    #[case(60, Mirror::Vertical(5))]
    #[case(61, Mirror::Horizontal(5))]
    #[case(62, Mirror::Vertical(16))]
    #[case(63, Mirror::Horizontal(2))]
    #[case(64, Mirror::Vertical(2))]
    #[case(65, Mirror::Vertical(8))]
    #[case(66, Mirror::Vertical(14))]
    #[case(67, Mirror::Horizontal(2))]
    #[case(68, Mirror::Horizontal(10))]
    #[case(69, Mirror::Vertical(7))]
    #[case(70, Mirror::Horizontal(7))]
    #[case(71, Mirror::Vertical(5))]
    #[case(72, Mirror::Horizontal(4))]
    #[case(73, Mirror::Horizontal(4))]
    #[case(74, Mirror::Vertical(16))]
    #[case(75, Mirror::Vertical(1))]
    #[case(76, Mirror::Vertical(9))]
    #[case(77, Mirror::Horizontal(9))]
    #[case(78, Mirror::Horizontal(8))]
    #[case(79, Mirror::Vertical(8))]
    #[case(80, Mirror::Vertical(14))]
    #[case(81, Mirror::Vertical(1))]
    #[case(82, Mirror::Horizontal(1))]
    #[case(83, Mirror::Vertical(4))]
    #[case(84, Mirror::Horizontal(13))]
    #[case(85, Mirror::Horizontal(3))]
    #[case(86, Mirror::Horizontal(1))]
    #[case(87, Mirror::Horizontal(2))]
    #[case(88, Mirror::Vertical(4))]
    #[case(89, Mirror::Horizontal(9))]
    #[case(90, Mirror::Horizontal(10))]
    #[case(91, Mirror::Vertical(1))]
    #[case(92, Mirror::Vertical(3))]
    #[case(93, Mirror::Vertical(8))]
    #[case(94, Mirror::Horizontal(4))]
    #[case(95, Mirror::Vertical(7))]
    #[case(96, Mirror::Vertical(5))]
    #[case(97, Mirror::Vertical(11))]
    #[case(98, Mirror::Horizontal(3))]
    #[case(99, Mirror::Horizontal(2))]
    fn test_find_mirror_real_input(
        puzzle_input: Vec<String>,
        #[case] pattern_idx: usize,
        #[case] expected_mirror: Mirror,
    ) {
        let pattern = &parse_patterns(&puzzle_input)[pattern_idx];

        assert_eq!(find_mirror(pattern), expected_mirror);
    }

    #[rstest]
    #[case(0, Mirror::Vertical(4))]
    #[case(1, Mirror::Horizontal(10))]
    #[case(2, Mirror::Horizontal(14))]
    #[case(3, Mirror::Horizontal(5))]
    #[case(4, Mirror::Vertical(10))]
    #[case(5, Mirror::Horizontal(12))]
    #[case(6, Mirror::Horizontal(1))]
    #[case(7, Mirror::Vertical(7))]
    #[case(8, Mirror::Horizontal(5))]
    #[case(9, Mirror::Horizontal(8))]
    #[case(10, Mirror::Vertical(16))]
    #[case(11, Mirror::Vertical(5))]
    #[case(12, Mirror::Horizontal(10))]
    #[case(13, Mirror::Horizontal(5))]
    #[case(14, Mirror::Vertical(7))]
    #[case(15, Mirror::Vertical(3))]
    #[case(16, Mirror::Horizontal(10))]
    #[case(17, Mirror::Vertical(2))]
    #[case(18, Mirror::Horizontal(3))]
    #[case(19, Mirror::Vertical(7))]
    #[case(20, Mirror::Horizontal(8))]
    #[case(21, Mirror::Vertical(2))]
    #[case(22, Mirror::Vertical(12))]
    #[case(23, Mirror::Vertical(3))]
    #[case(24, Mirror::Horizontal(1))]
    #[case(25, Mirror::Horizontal(11))]
    #[case(26, Mirror::Horizontal(2))]
    #[case(27, Mirror::Horizontal(4))]
    #[case(28, Mirror::Vertical(15))]
    #[case(29, Mirror::Horizontal(5))]
    #[case(30, Mirror::Vertical(1))]
    #[case(31, Mirror::Horizontal(8))]
    #[case(32, Mirror::Vertical(14))]
    #[case(33, Mirror::Vertical(13))]
    #[case(34, Mirror::Vertical(1))]
    #[case(35, Mirror::Vertical(7))]
    #[case(36, Mirror::Vertical(3))]
    #[case(37, Mirror::Horizontal(4))]
    #[case(38, Mirror::Horizontal(7))]
    #[case(39, Mirror::Vertical(6))]
    #[case(40, Mirror::Horizontal(12))]
    #[case(41, Mirror::Horizontal(4))]
    #[case(42, Mirror::Vertical(2))]
    #[case(43, Mirror::Horizontal(7))]
    #[case(44, Mirror::Horizontal(9))]
    #[case(45, Mirror::Horizontal(8))]
    #[case(46, Mirror::Horizontal(1))]
    #[case(47, Mirror::Horizontal(6))]
    #[case(48, Mirror::Vertical(1))]
    #[case(49, Mirror::Vertical(13))]
    #[case(50, Mirror::Vertical(7))]
    #[case(51, Mirror::Vertical(6))]
    #[case(52, Mirror::Vertical(5))]
    #[case(53, Mirror::Horizontal(13))]
    #[case(54, Mirror::Vertical(12))]
    #[case(55, Mirror::Horizontal(13))]
    #[case(56, Mirror::Vertical(2))]
    #[case(57, Mirror::Vertical(4))]
    #[case(58, Mirror::Horizontal(6))]
    #[case(59, Mirror::Horizontal(5))]
    #[case(60, Mirror::Vertical(8))]
    #[case(61, Mirror::Horizontal(13))]
    #[case(62, Mirror::Vertical(7))]
    #[case(63, Mirror::Horizontal(7))]
    #[case(64, Mirror::Vertical(7))]
    #[case(65, Mirror::Vertical(1))]
    #[case(66, Mirror::Vertical(5))]
    #[case(67, Mirror::Horizontal(11))]
    #[case(68, Mirror::Horizontal(5))]
    #[case(69, Mirror::Vertical(14))]
    #[case(70, Mirror::Horizontal(2))]
    #[case(71, Mirror::Horizontal(6))]
    #[case(72, Mirror::Vertical(12))]
    #[case(73, Mirror::Vertical(12))]
    #[case(74, Mirror::Vertical(8))]
    #[case(75, Mirror::Horizontal(2))]
    #[case(76, Mirror::Horizontal(3))]
    #[case(77, Mirror::Horizontal(4))]
    #[case(78, Mirror::Horizontal(3))]
    #[case(79, Mirror::Horizontal(4))]
    #[case(80, Mirror::Horizontal(10))]
    #[case(81, Mirror::Vertical(13))]
    #[case(82, Mirror::Horizontal(10))]
    #[case(83, Mirror::Horizontal(10))]
    #[case(84, Mirror::Horizontal(1))]
    #[case(85, Mirror::Horizontal(11))]
    #[case(86, Mirror::Vertical(2))]
    #[case(87, Mirror::Horizontal(11))]
    #[case(88, Mirror::Vertical(14))]
    #[case(89, Mirror::Horizontal(1))]
    #[case(90, Mirror::Horizontal(3))]
    #[case(91, Mirror::Horizontal(4))]
    #[case(92, Mirror::Horizontal(11))]
    #[case(93, Mirror::Vertical(3))]
    #[case(94, Mirror::Vertical(7))]
    #[case(95, Mirror::Horizontal(2))]
    #[case(96, Mirror::Vertical(12))]
    #[case(97, Mirror::Vertical(15))]
    #[case(98, Mirror::Vertical(5))]
    #[case(99, Mirror::Horizontal(13))]
    fn test_find_mirror_with_smudge_real_input(
        puzzle_input: Vec<String>,
        #[case] pattern_idx: usize,
        #[case] expected_mirror: Mirror,
    ) {
        let pattern = &parse_patterns(&puzzle_input)[pattern_idx];

        assert_eq!(find_mirror_with_smudge(pattern), expected_mirror);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let patterns = parse_patterns(&test_input);

        let mirrors = find_mirrors(&patterns);

        assert_eq!(get_summary_value(&mirrors), 405);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let patterns = parse_patterns(&puzzle_input);

        let mirrors = find_mirrors(&patterns);

        assert_eq!(get_summary_value(&mirrors), 30518);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let patterns = parse_patterns(&test_input);

        let mirrors = find_mirrors_with_smudge(&patterns);

        assert_eq!(get_summary_value(&mirrors), 400);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let patterns = parse_patterns(&puzzle_input);

        let mirrors = find_mirrors_with_smudge(&patterns);

        assert_eq!(get_summary_value(&mirrors), 36735);
    }
}
