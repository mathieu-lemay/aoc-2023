use itertools::Itertools;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input};

fn main() {
    let input = get_input("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let games = parse_games(input);

    let p1: u32 = get_possible_games(&games, 12, 13, 14).iter().sum();
    let p2: u32 = get_power_of_sets(&games).iter().sum();

    (p1, p2)
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn get_biggest_needed_set(&self) -> GameSet {
        let red = self.sets.iter().map(|s| s.red).max().unwrap();
        let green = self.sets.iter().map(|s| s.green).max().unwrap();
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap();

        GameSet { red, green, blue }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_games(input: &[String]) -> Vec<Game> {
    let mut games = Vec::with_capacity(input.len());

    for entry in input {
        let mut sets = Vec::new();
        let (title, set_entries) = entry.split(": ").collect_tuple().unwrap();
        let game_id = title[5..].parse::<u32>().unwrap();

        for set_entry in set_entries.split("; ") {
            let mut game_set = GameSet::default();
            for block in set_entry.split(", ") {
                let (n, color) = block.split(' ').collect_tuple().unwrap();
                let n = n.parse::<u32>().unwrap();
                match color {
                    "red" => game_set.red = n,
                    "green" => game_set.green = n,
                    "blue" => game_set.blue = n,
                    _ => panic!("Invalid color: {}", color),
                }
            }

            sets.push(game_set);
        }

        games.push(Game { id: game_id, sets })
    }

    games
}

fn get_possible_games(games: &[Game], max_red: u32, max_green: u32, max_blue: u32) -> Vec<u32> {
    games
        .iter()
        .filter(|g| {
            let biggest_needed_set = g.get_biggest_needed_set();

            biggest_needed_set.red <= max_red
                && biggest_needed_set.green <= max_green
                && biggest_needed_set.blue <= max_blue
        })
        .map(|g| g.id)
        .collect()
}

fn get_power_of_sets(games: &[Game]) -> Vec<u32> {
    games
        .iter()
        .map(|g| {
            let biggest_needed_set = g.get_biggest_needed_set();

            biggest_needed_set.red * biggest_needed_set.green * biggest_needed_set.blue
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::*;

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day02.txt")
    }

    #[rstest]
    fn test_parse_games() {
        let input = parse_test_input(
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 42: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            ",
        );

        let games = parse_games(&input);
        assert_eq!(
            games,
            vec![
                Game {
                    id: 1,
                    sets: vec![
                        GameSet {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        GameSet {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        GameSet {
                            red: 0,
                            green: 2,
                            blue: 0
                        }
                    ]
                },
                Game {
                    id: 2,
                    sets: vec![
                        GameSet {
                            red: 0,
                            green: 2,
                            blue: 1
                        },
                        GameSet {
                            red: 1,
                            green: 3,
                            blue: 4
                        },
                        GameSet {
                            red: 0,
                            green: 1,
                            blue: 1
                        }
                    ]
                },
                Game {
                    id: 42,
                    sets: vec![
                        GameSet {
                            red: 20,
                            green: 8,
                            blue: 6
                        },
                        GameSet {
                            red: 4,
                            green: 13,
                            blue: 5
                        },
                        GameSet {
                            red: 1,
                            green: 5,
                            blue: 0
                        }
                    ]
                },
            ]
        );
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let games = parse_games(&test_input);
        let res: u32 = get_possible_games(&games, 12, 13, 14).iter().sum();

        assert_eq!(res, 8);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let games = parse_games(&puzzle_input);
        let res: u32 = get_possible_games(&games, 12, 13, 14).iter().sum();

        assert_eq!(res, 2617);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let games = parse_games(&test_input);
        let res = get_power_of_sets(&games);

        assert_eq!(res, vec![48, 12, 1560, 630, 36]);
        assert_eq!(res.iter().sum::<u32>(), 2286);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let games = parse_games(&puzzle_input);
        let res = get_power_of_sets(&games);

        assert_eq!(res.iter().sum::<u32>(), 59795);
    }
}
