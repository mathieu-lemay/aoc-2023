use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn main() {
    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut plan = parse_plan(input);

    let p1 = plan.get_lowest_seed_location();
    plan.add_implicit_mappings();
    let p2 = plan.get_lowest_seed_location_from_range();

    (p1, p2)
}

#[derive(Debug, Default, Eq, PartialEq)]
struct PlantingPlan {
    pub seeds: Vec<i64>,
    pub maps: HashMap<Category, ConversionMap>,
}

impl PlantingPlan {
    fn add_implicit_mappings(&mut self) {
        for map in self.maps.values_mut() {
            let mut range_starts: Vec<i64> = vec![0i64, (u32::MAX) as i64];
            range_starts.extend(map.mappings.iter().map(|m| m.src_start));
            range_starts.extend(map.mappings.iter().map(|m| m.src_start + m.length));

            range_starts.sort();

            let new_mappings = range_starts
                .iter()
                .tuple_windows()
                .map(|(&start, end)| {
                    if let Some(m) = map.mappings.iter().find(|m| m.src_start == start) {
                        *m
                    } else {
                        Mapping {
                            src_start: start,
                            dst_start: start,
                            length: end - start,
                        }
                    }
                })
                .collect();

            map.mappings = new_mappings;
        }
    }
}

impl PlantingPlan {
    fn get_conversion_map_by_dst(&self, dst: &Category) -> Option<&ConversionMap> {
        self.maps.values().find(|m| &m.dst == dst)
    }

    fn get_location_for_seed(&self, seed: i64) -> i64 {
        let mut map = self.maps.get(&Category::Seed).unwrap();
        let mut location = map.get_dst_value(seed);

        while map.dst != Category::Location {
            map = self.maps.get(&map.dst).unwrap();
            location = map.get_dst_value(location);
        }

        location
    }

    fn get_lowest_seed_location(&self) -> i64 {
        self.seeds
            .iter()
            .map(|&s| self.get_location_for_seed(s))
            .min()
            .unwrap()
    }

    fn get_lowest_seed_location_from_range(&self) -> i64 {
        let mut conversion_map = self.get_conversion_map_by_dst(&Category::Location).unwrap();
        let mut mappings: Vec<Mapping> = conversion_map
            .mappings
            .iter()
            .sorted_by_key(|m| m.src_start)
            .cloned()
            .collect();

        loop {
            let m = self.get_conversion_map_by_dst(&conversion_map.src);
            if m.is_none() {
                break;
            }

            conversion_map = m.unwrap();
            mappings = conversion_map
                .mappings
                .iter()
                .cartesian_product(&mappings)
                .flat_map(|(m1, m2)| m1.intersection(m2))
                .sorted_by_key(|m| m.src_start)
                .dedup()
                .collect();
        }

        let seed_ranges: Vec<Range> = self
            .seeds
            .chunks(2)
            .map(|c| Range {
                start: c[0],
                end: c[0] + c[1],
            })
            .collect();

        let candidates = mappings
            .iter()
            .map(|m| Range {
                start: m.src_start,
                end: m.src_start + m.length,
            })
            .cartesian_product(seed_ranges)
            .filter_map(|(r1, r2)| r1.intersection(&r2).map(|r| r.start));

        candidates
            .sorted()
            .dedup()
            .map(|s| self.get_location_for_seed(s))
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
struct ConversionMap {
    src: Category,
    dst: Category,
    mappings: Vec<Mapping>,
}

impl ConversionMap {
    fn get_dst_value(&self, src_value: i64) -> i64 {
        self.mappings
            .iter()
            .filter_map(|m| m.get_dst_value(src_value))
            .next()
            .unwrap_or(src_value)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Mapping {
    dst_start: i64,
    src_start: i64,
    length: i64,
}

impl Mapping {
    fn get_dst_value(&self, src_value: i64) -> Option<i64> {
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

    fn intersection(&self, other: &Mapping) -> Vec<Mapping> {
        let self_dst_range = Range {
            start: self.dst_start,
            end: self.dst_start + self.length,
        };
        let other_src_range = Range {
            start: other.src_start,
            end: other.src_start + other.length,
        };

        let range_ixn = self_dst_range.intersection(&other_src_range);
        if range_ixn.is_none() {
            return vec![];
        }

        let range_ixn = range_ixn.unwrap();
        let offset = self.dst_start - self.src_start;

        [
            Mapping {
                src_start: self.src_start,
                dst_start: self.dst_start,
                length: range_ixn.start - self.dst_start,
            },
            Mapping {
                src_start: range_ixn.start - offset,
                dst_start: range_ixn.start,
                length: range_ixn.length(),
            },
            Mapping {
                src_start: range_ixn.end - offset,
                dst_start: range_ixn.end,
                length: self.length - range_ixn.length() - (range_ixn.start - self.dst_start),
            },
        ]
        .into_iter()
        .filter(|&m| m.length > 0)
        .collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn length(&self) -> i64 {
        self.end - self.start
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start < end {
            Some(Range { start, end })
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

        // mappings.sort_by_key(|m| m.dst_start);

        maps.insert(src.clone(), ConversionMap { src, dst, mappings });
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
                ConversionMap {
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
                ConversionMap {
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
                ConversionMap {
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
                ConversionMap {
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
                ConversionMap {
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
                ConversionMap {
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
                ConversionMap {
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
    fn test_map_get_dst_value(test_input: Vec<String>, #[case] input: i64, #[case] expected: i64) {
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
        #[case] input: i64,
        #[case] expected: i64,
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
        let mut plan = parse_plan(&test_input);
        plan.add_implicit_mappings();

        assert_eq!(plan.get_lowest_seed_location_from_range(), 46);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let mut plan = parse_plan(&puzzle_input);
        plan.add_implicit_mappings();

        assert_eq!(plan.get_lowest_seed_location_from_range(), 46294175);
    }
}
