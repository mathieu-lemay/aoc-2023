use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn main() {
    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let plan = parse_plan(input);

    let p1 = plan.get_lowest_seed_location();
    let p2 = 0;

    (p1, p2)
}

#[derive(Debug, Default, Eq, PartialEq)]
struct PlantingPlan {
    pub seeds: Vec<u32>,
    pub maps: HashMap<Category, Map>,
}

impl PlantingPlan {
    fn get_location_for_seed(&self, seed: u32) -> u32 {
        let mut map = self.maps.get(&Category::Seed).unwrap();
        let mut location = map.get_dst_value(seed);

        while map.dst != Category::Location {
            map = self.maps.get(&map.dst).unwrap();
            location = map.get_dst_value(location);
        }

        location
    }

    fn get_lowest_seed_location(&self) -> u32 {
        self.seeds
            .iter()
            .map(|&s| self.get_location_for_seed(s))
            .min()
            .unwrap()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(format!("Invalid category: {}", value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    src: Category,
    dst: Category,
    mappings: Vec<Mapping>,
}

impl Map {
    fn get_dst_value(&self, src_value: u32) -> u32 {
        self.mappings
            .iter()
            .filter_map(|m| m.get_dst_value(src_value))
            .next()
            .unwrap_or(src_value)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Mapping {
    dst_start: u32,
    src_start: u32,
    length: u32,
}

impl Mapping {
    fn get_dst_value(&self, src_value: u32) -> Option<u32> {
        if src_value < self.src_start {
            return None;
        }

        let distance = src_value - self.src_start;

        if distance < self.length {
            Some(self.dst_start + distance)
        } else {
            None
        }
    }
}

fn parse_plan(input: &[String]) -> PlantingPlan {
    let seeds = input[0][7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps = HashMap::new();

    let mut input_iter = input.iter().skip(2);

    loop {
        let categories = input_iter.next();
        if categories.is_none() {
            break;
        }

        let categories = categories.unwrap().split(' ').next().unwrap();
        let (src, dst): (Category, Category) = categories
            .split("-to-")
            .map(|c| c.try_into().unwrap())
            .collect_tuple()
            .unwrap();

        let mut mappings = Vec::new();

        for e in input_iter.by_ref() {
            if e.is_empty() {
                break;
            }

            let (dst_start, src_start, length) = e
                .split(' ')
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap();

            mappings.push(Mapping {
                dst_start,
                src_start,
                length,
            })
        }

        maps.insert(src.clone(), Map { src, dst, mappings });
    }

    PlantingPlan { seeds, maps }
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day05.txt")
    }

    #[rstest]
    fn test_parse_planting_maps(test_input: Vec<String>) {
        let plan = parse_plan(&test_input);

        let maps = HashMap::from([
            (
                Category::Seed,
                Map {
                    src: Category::Seed,
                    dst: Category::Soil,
                    mappings: vec![
                        Mapping {
                            dst_start: 50,
                            src_start: 98,
                            length: 2,
                        },
                        Mapping {
                            dst_start: 52,
                            src_start: 50,
                            length: 48,
                        },
                    ],
                },
            ),
            (
                Category::Soil,
                Map {
                    src: Category::Soil,
                    dst: Category::Fertilizer,
                    mappings: vec![
                        Mapping {
                            dst_start: 0,
                            src_start: 15,
                            length: 37,
                        },
                        Mapping {
                            dst_start: 37,
                            src_start: 52,
                            length: 2,
                        },
                        Mapping {
                            dst_start: 39,
                            src_start: 0,
                            length: 15,
                        },
                    ],
                },
            ),
            (
                Category::Fertilizer,
                Map {
                    src: Category::Fertilizer,
                    dst: Category::Water,
                    mappings: vec![
                        Mapping {
                            dst_start: 49,
                            src_start: 53,
                            length: 8,
                        },
                        Mapping {
                            dst_start: 0,
                            src_start: 11,
                            length: 42,
                        },
                        Mapping {
                            dst_start: 42,
                            src_start: 0,
                            length: 7,
                        },
                        Mapping {
                            dst_start: 57,
                            src_start: 7,
                            length: 4,
                        },
                    ],
                },
            ),
            (
                Category::Water,
                Map {
                    src: Category::Water,
                    dst: Category::Light,
                    mappings: vec![
                        Mapping {
                            dst_start: 88,
                            src_start: 18,
                            length: 7,
                        },
                        Mapping {
                            dst_start: 18,
                            src_start: 25,
                            length: 70,
                        },
                    ],
                },
            ),
            (
                Category::Light,
                Map {
                    src: Category::Light,
                    dst: Category::Temperature,
                    mappings: vec![
                        Mapping {
                            dst_start: 45,
                            src_start: 77,
                            length: 23,
                        },
                        Mapping {
                            dst_start: 81,
                            src_start: 45,
                            length: 19,
                        },
                        Mapping {
                            dst_start: 68,
                            src_start: 64,
                            length: 13,
                        },
                    ],
                },
            ),
            (
                Category::Temperature,
                Map {
                    src: Category::Temperature,
                    dst: Category::Humidity,
                    mappings: vec![
                        Mapping {
                            dst_start: 0,
                            src_start: 69,
                            length: 1,
                        },
                        Mapping {
                            dst_start: 1,
                            src_start: 0,
                            length: 69,
                        },
                    ],
                },
            ),
            (
                Category::Humidity,
                Map {
                    src: Category::Humidity,
                    dst: Category::Location,
                    mappings: vec![
                        Mapping {
                            dst_start: 60,
                            src_start: 56,
                            length: 37,
                        },
                        Mapping {
                            dst_start: 56,
                            src_start: 93,
                            length: 4,
                        },
                    ],
                },
            ),
        ]);

        let expected = PlantingPlan {
            seeds: vec![79, 14, 55, 13],
            maps,
        };

        assert_eq!(plan, expected);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(50, 52)]
    #[case(97, 99)]
    #[case(98, 50)]
    #[case(99, 51)]
    #[case(100, 100)]
    fn test_map_get_dst_value(test_input: Vec<String>, #[case] input: u32, #[case] expected: u32) {
        let plan = parse_plan(&test_input);
        let map = plan.maps.get(&Category::Seed).unwrap();

        assert_eq!(map.get_dst_value(input), expected);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_get_location_for_seed(
        test_input: Vec<String>,
        #[case] input: u32,
        #[case] expected: u32,
    ) {
        let plan = parse_plan(&test_input);

        assert_eq!(plan.get_location_for_seed(input), expected);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let plan = parse_plan(&test_input);

        assert_eq!(plan.get_lowest_seed_location(), 35);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let plan = parse_plan(&puzzle_input);

        assert_eq!(plan.get_lowest_seed_location(), 484023871);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }
}
