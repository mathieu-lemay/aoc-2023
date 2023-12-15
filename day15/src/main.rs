use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string};
use itertools::Itertools;

fn main() {
    let input = get_input_as_string("day15.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let instrs = parse_instructions(input);

    let p1 = get_sum_of_hashes(&instrs);
    let p2 = get_focusing_power(&instrs);

    (p1, p2)
}

struct HolidayHasher {
    value: u16,
}

impl HolidayHasher {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn write(&mut self, data: &str) {
        for c in data.chars() {
            self.value = ((self.value + c as u16) * 17) % 256
        }
    }

    fn finish(&self) -> u8 {
        self.value as u8
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Set(u8),
    Remove,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    raw: String,
    label: String,
    op: Op,
}

impl Instruction {
    fn get_box_id(&self) -> usize {
        let mut hasher = HolidayHasher::new();
        hasher.write(&self.label);

        hasher.finish() as usize
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = HolidayHasher::new();
        hasher.write(&self.raw);

        hasher.finish() as u64
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some(i) = value.find('-') {
            Instruction {
                raw: value.to_string(),
                label: value[..i].to_string(),
                op: Op::Remove,
            }
        } else {
            let (instr, n) = value.split('=').collect_tuple().unwrap();
            Instruction {
                raw: value.to_string(),
                label: instr.to_string(),
                op: Op::Set(n.parse::<u8>().unwrap()),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Lens {
    label: String,
    value: u8,
}

struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: vec![] }
    }

    fn set(&mut self, label: &str, value: u8) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses[idx].value = value;
        } else {
            self.lenses.push(Lens {
                label: label.to_string(),
                value,
            });
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses.remove(idx);
        }
    }

    fn focal_power(&self, multiplier: usize) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(idx, lens)| (idx + 1) * lens.value as usize * multiplier)
            .sum()
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.split(',').map(Instruction::from).collect()
}

fn get_sum_of_hashes(instructions: &[Instruction]) -> u64 {
    instructions.iter().map(Instruction::get_hash).sum::<u64>()
}

fn get_focusing_power(instructions: &[Instruction]) -> usize {
    let mut boxes: Vec<LensBox> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(LensBox::new());
    }

    for instr in instructions {
        let box_idx = instr.get_box_id();

        let box_ = &mut boxes[box_idx];

        match instr.op {
            Op::Set(v) => box_.set(&instr.label, v),
            Op::Remove => box_.remove(&instr.label),
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(idx, box_)| box_.focal_power(idx + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use aoc_common::parse_test_input_as_string;

    use super::*;

    #[fixture]
    fn test_input() -> String {
        parse_test_input_as_string("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    #[fixture]
    fn puzzle_input() -> String {
        get_input_as_string("day15.txt")
    }

    #[rstest]
    fn test_hash() {
        let mut hasher = HolidayHasher::new();
        hasher.write("HASH");

        assert_eq!(hasher.finish(), 52);
    }

    #[rstest]
    fn test_parse_instructions(test_input: String) {
        let instrs = parse_instructions(&test_input);

        assert_eq!(
            instrs,
            vec![
                Instruction {
                    raw: String::from("rn=1"),
                    label: String::from("rn"),
                    op: Op::Set(1)
                },
                Instruction {
                    raw: String::from("cm-"),
                    label: String::from("cm"),
                    op: Op::Remove
                },
                Instruction {
                    raw: String::from("qp=3"),
                    label: String::from("qp"),
                    op: Op::Set(3)
                },
                Instruction {
                    raw: String::from("cm=2"),
                    label: String::from("cm"),
                    op: Op::Set(2)
                },
                Instruction {
                    raw: String::from("qp-"),
                    label: String::from("qp"),
                    op: Op::Remove
                },
                Instruction {
                    raw: String::from("pc=4"),
                    label: String::from("pc"),
                    op: Op::Set(4)
                },
                Instruction {
                    raw: String::from("ot=9"),
                    label: String::from("ot"),
                    op: Op::Set(9)
                },
                Instruction {
                    raw: String::from("ab=5"),
                    label: String::from("ab"),
                    op: Op::Set(5)
                },
                Instruction {
                    raw: String::from("pc-"),
                    label: String::from("pc"),
                    op: Op::Remove
                },
                Instruction {
                    raw: String::from("pc=6"),
                    label: String::from("pc"),
                    op: Op::Set(6)
                },
                Instruction {
                    raw: String::from("ot=7"),
                    label: String::from("ot"),
                    op: Op::Set(7)
                },
            ]
        );
    }

    #[rstest]
    fn test_p1(test_input: String) {
        let instrs = parse_instructions(&test_input);

        let sum = get_sum_of_hashes(&instrs);

        assert_eq!(sum, 1320);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let instrs = parse_instructions(&puzzle_input);

        let sum = get_sum_of_hashes(&instrs);

        assert_eq!(sum, 514025);
    }

    #[rstest]
    fn test_p2(test_input: String) {
        let instrs = parse_instructions(&test_input);

        assert_eq!(get_focusing_power(&instrs), 145);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let instrs = parse_instructions(&puzzle_input);

        assert_eq!(get_focusing_power(&instrs), 244461);
    }
}
