use inpt::{inpt, Inpt};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn main() {
    let input = get_input("day08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let map = parse_network_map(input);

    let p1 = follow_map(&map);
    let p2 = follow_map_parallel(&map);

    (p1, p2)
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
struct NetworkMap {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Inpt)]
#[inpt(regex = r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)")]
struct Node {
    name: String,
    next_left: String,
    next_right: String,
}

fn parse_network_map(input: &[String]) -> NetworkMap {
    NetworkMap {
        directions: input[0].chars().map(Direction::from).collect(),
        nodes: input[2..]
            .iter()
            .map(|n| inpt::<Node>(n).expect("Invalid node entry"))
            .collect(),
    }
}

fn follow_map(map: &NetworkMap) -> u64 {
    let nodes: HashMap<&str, &Node> =
        HashMap::from_iter(map.nodes.iter().map(|n| (n.name.as_str(), n)));

    let start = nodes.get("AAA").expect("Unable to find start node");

    get_steps_to_end(start, &map.directions, &nodes, |n| n.name == "ZZZ")
}

fn follow_map_parallel(map: &NetworkMap) -> u64 {
    let nodes: HashMap<&str, &Node> =
        HashMap::from_iter(map.nodes.iter().map(|n| (n.name.as_str(), n)));

    let has_reached_end = |n: &Node| { n.name.ends_with('Z') };

    map.nodes
        .iter()
        .filter_map(|n| {
            if n.name.ends_with('A') {
                Some(get_steps_to_end(n, &map.directions, &nodes, has_reached_end))
            } else {
                None
            }
        })
        .fold(1, num::integer::lcm)
}

fn get_steps_to_end<F>(
    start_node: &Node,
    directions: &[Direction],
    nodes: &HashMap<&str, &Node>,
    has_reached_end: F,
) -> u64
where
    F: Fn(&Node) -> bool,
{
    let mut current = start_node;

    for (step, dir) in directions.iter().cycle().enumerate() {
        current = nodes
            .get(match dir {
                Direction::Left => current.next_left.as_str(),
                Direction::Right => current.next_right.as_str(),
            })
            .expect("Unable to find next node");

        if has_reached_end(current) {
            return (step + 1) as u64;
        }
    }

    unreachable!("you shouldn't be here");
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input_p1() -> Vec<String> {
        parse_test_input(
            "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ",
        )
    }

    #[fixture]
    fn test_input_p1_alternate() -> Vec<String> {
        parse_test_input(
            "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ",
        )
    }

    #[fixture]
    fn test_input_p2() -> Vec<String> {
        parse_test_input(
            "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day08.txt")
    }

    #[rstest]
    fn test_parse_network_map(test_input_p1: Vec<String>) {
        let map = parse_network_map(&test_input_p1);

        assert_eq!(
            map,
            NetworkMap {
                directions: vec![Direction::Right, Direction::Left],
                nodes: vec![
                    Node {
                        name: "AAA".to_string(),
                        next_left: "BBB".to_string(),
                        next_right: "CCC".to_string(),
                    },
                    Node {
                        name: "BBB".to_string(),
                        next_left: "DDD".to_string(),
                        next_right: "EEE".to_string(),
                    },
                    Node {
                        name: "CCC".to_string(),
                        next_left: "ZZZ".to_string(),
                        next_right: "GGG".to_string(),
                    },
                    Node {
                        name: "DDD".to_string(),
                        next_left: "DDD".to_string(),
                        next_right: "DDD".to_string(),
                    },
                    Node {
                        name: "EEE".to_string(),
                        next_left: "EEE".to_string(),
                        next_right: "EEE".to_string(),
                    },
                    Node {
                        name: "GGG".to_string(),
                        next_left: "GGG".to_string(),
                        next_right: "GGG".to_string(),
                    },
                    Node {
                        name: "ZZZ".to_string(),
                        next_left: "ZZZ".to_string(),
                        next_right: "ZZZ".to_string(),
                    },
                ]
            }
        );
    }

    #[rstest]
    #[case(test_input_p1(), 2)]
    #[case(test_input_p1_alternate(), 6)]
    fn test_p1(#[case] input: Vec<String>, #[case] expected: u64) {
        let map = parse_network_map(&input);
        let steps = follow_map(&map);

        assert_eq!(steps, expected);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let map = parse_network_map(&puzzle_input);
        let steps = follow_map(&map);

        assert_eq!(steps, 16043);
    }

    #[rstest]
    fn test_p2(test_input_p2: Vec<String>) {
        let map = parse_network_map(&test_input_p2);
        let steps = follow_map_parallel(&map);

        assert_eq!(steps, 6);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let map = parse_network_map(&puzzle_input);
        let steps = follow_map_parallel(&map);

        assert_eq!(steps, 15726453850399);
    }
}
