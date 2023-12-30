use inpt::{inpt, Inpt};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input};
use regex::Regex;

fn main() {
    let input = get_input("day19.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let system = parse_system(input);

    let p1 = get_total_of_accepted_parts(&system);
    let p2 = get_possible_combinations(&system);

    (p1, p2)
}

type Workflows = HashMap<String, Workflow>;

#[derive(Debug, Eq, PartialEq)]
struct System {
    workflows: Workflows,
    parts: Vec<Part>,
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    condition: Option<Condition>,
    action: Action,
}

#[derive(Debug, Eq, PartialEq)]
struct Condition {
    part: String,
    op: Op,
    val: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Lt,
    Gt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Action {
    Accept,
    Reject,
    Process(String),
}

#[derive(Debug, Eq, PartialEq, Inpt)]
#[inpt(regex = r"\{x=([\d]+),m=([\d]+),a=([\d]+),s=([\d]+)\}")]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_system(input: &[String]) -> System {
    let mut idx = 0;
    let mut workflows = HashMap::new();

    loop {
        let entry = input.get(idx).unwrap();
        idx += 1;

        if entry.is_empty() {
            break;
        }

        let workflow = parse_workflow(entry);

        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts = input[idx..]
        .iter()
        .map(|i| inpt::<Part>(i).unwrap())
        .collect();

    System { workflows, parts }
}

fn parse_workflow(entry: &str) -> Workflow {
    let x = entry.find('{').unwrap();
    let name = entry[..x].to_string();
    let mut rules = Vec::new();

    for rule in entry[x + 1..entry.len() - 1].split(',') {
        rules.push(parse_rule(rule));
    }

    Workflow { name, rules }
}

fn parse_rule(val: &str) -> Rule {
    if let Some(i) = val.find(':') {
        let condition = Some(parse_condition(&val[..i]));
        let action = parse_action(&val[i + 1..]);

        Rule { condition, action }
    } else {
        let action = parse_action(val);
        Rule {
            condition: None,
            action,
        }
    }
}

fn parse_condition(val: &str) -> Condition {
    let re = Regex::new(r"([a-zA-Z]+)([<>])([0-9]+)").expect("Invalid regex");

    let caps = re.captures(val).unwrap();

    let part = caps.get(1).unwrap().as_str().to_string();
    let op = match caps.get(2).unwrap().as_str() {
        "<" => Op::Lt,
        ">" => Op::Gt,
        _ => unreachable!(),
    };
    let val = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

    Condition { part, op, val }
}

fn parse_action(val: &str) -> Action {
    match val {
        "A" => Action::Accept,
        "R" => Action::Reject,
        workflow => Action::Process(workflow.to_string()),
    }
}

fn is_accepted(part: &Part, workflows: &Workflows) -> bool {
    let mut workflow = workflows.get("in").unwrap();

    loop {
        let action = get_action(part, workflow);

        match action {
            Action::Accept => return true,
            Action::Reject => return false,
            Action::Process(n) => workflow = workflows.get(&n).unwrap(),
        }
    }
}

fn get_action(part: &Part, workflow: &Workflow) -> Action {
    for rule in &workflow.rules {
        if let Some(c) = &rule.condition {
            let part_value = match c.part.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => unreachable!(),
            };

            match c.op {
                Op::Lt => {
                    if part_value < c.val {
                        return rule.action.clone();
                    }
                }
                Op::Gt => {
                    if part_value > c.val {
                        return rule.action.clone();
                    }
                }
            }
        } else {
            return rule.action.clone();
        }
    }

    Action::Reject
}

fn get_total_of_accepted_parts(system: &System) -> usize {
    system
        .parts
        .iter()
        .filter_map(|p| {
            if is_accepted(p, &system.workflows) {
                Some(p.value())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TheoreticalPart {
    min_x: usize,
    max_x: usize,
    min_m: usize,
    max_m: usize,
    min_a: usize,
    max_a: usize,
    min_s: usize,
    max_s: usize,
}

impl TheoreticalPart {
    fn new() -> Self {
        TheoreticalPart {
            min_x: 0,
            max_x: 4000,
            min_m: 0,
            max_m: 4000,
            min_a: 0,
            max_a: 4000,
            min_s: 0,
            max_s: 4000,
        }
    }

    fn overlaps(&self, other: &Self) -> usize {
        let vals = vec![
            get_overlap_size(self.min_x, self.max_x, other.min_x, other.max_x),
            get_overlap_size(self.min_m, self.max_m, other.min_m, other.max_m),
            get_overlap_size(self.min_a, self.max_a, other.min_a, other.max_a),
            get_overlap_size(self.min_s, self.max_s, other.min_s, other.max_s),
        ];
        println!("{:?}", vals);

        vals.iter().product()
    }
}

fn get_overlap_size(a1: usize, a2: usize, b1: usize, b2: usize) -> usize {
    if a1 <= b2 && b1 <= a2 {
        a2.min(b2) - a1.max(b1)
    } else {
        0
    }
}

fn get_possible_combinations(system: &System) -> usize {
    let part = TheoreticalPart::new();
    let workflow = system.workflows.get("in").unwrap();

    let parts = get_possibles(system, part, workflow);

    for (i, p) in parts.iter().enumerate() {
        println!("{} => {:?}", i, p);
    }

    let mut total = parts
        .iter()
        .map(|p| {
            (p.max_x - p.min_x) * (p.max_m - p.min_m) * (p.max_a - p.min_a) * (p.max_s - p.min_s)
        })
        .sum();

    println!("total: {}", total);

    for (i, p1) in parts[..parts.len() - 1].iter().enumerate() {
        for (j, p2) in parts[i + 1..].iter().enumerate() {
            println!("comparing {} and {}", i, j + i + 1);
            let o = p1.overlaps(p2);
            println!("overlaps : {:20}", o);
            total -= o;
            println!("new total: {:20}", total);
            // if total < 167409079868000 {
            //     panic!("too low");
            // }
        }
    }

    total
}

fn get_possibles(
    system: &System,
    part: TheoreticalPart,
    workflow: &Workflow,
) -> Vec<TheoreticalPart> {
    let mut possibles = Vec::new();

    for rule in &workflow.rules {
        if rule.action == Action::Reject {
            // return vec![];
            // return possibles;
            continue;
        }

        if let Some(c) = &rule.condition {
            let mut p = part.clone();
            match c.part.as_str() {
                "x" => {
                    if c.op == Op::Lt {
                        if p.min_x >= c.val {
                            return vec![];
                        }
                        p.max_x = p.max_x.min(c.val - 1);
                    } else {
                        if p.max_x <= c.val {
                            return vec![];
                        }
                        p.min_x = p.min_x.max(c.val + 1);
                    }
                }
                "m" => {
                    if c.op == Op::Lt {
                        if p.min_m >= c.val {
                            return vec![];
                        }
                        p.max_m = p.max_m.min(c.val - 1);
                    } else {
                        if p.max_m <= c.val {
                            return vec![];
                        }
                        p.min_m = p.min_m.max(c.val + 1);
                    }
                }
                "a" => {
                    if c.op == Op::Lt {
                        if p.min_a >= c.val {
                            return vec![];
                        }
                        p.max_a = p.max_a.min(c.val - 1);
                    } else {
                        if p.max_a <= c.val {
                            return vec![];
                        }
                        p.min_a = p.min_a.max(c.val + 1);
                    }
                }
                "s" => {
                    if c.op == Op::Lt {
                        if p.min_s >= c.val {
                            return vec![];
                        }
                        p.max_s = p.max_s.min(c.val - 1);
                    } else {
                        if p.max_s <= c.val {
                            return vec![];
                        }
                        p.min_s = p.min_s.max(c.val + 1);
                    }
                }
                _ => unreachable!(),
            }

            match &rule.action {
                Action::Accept => {
                    possibles.push(p);
                    return possibles;
                }
                Action::Reject => {}
                Action::Process(n) => {
                    let w = system.workflows.get(n.as_str()).unwrap();
                    let mut others = get_possibles(system, p, w);
                    possibles.append(&mut others);
                    // return possibles;
                }
            }
        } else {
            match &rule.action {
                Action::Accept => {
                    possibles.push(part.clone());
                    return possibles;
                }
                Action::Reject => {}
                Action::Process(n) => {
                    let w = system.workflows.get(n.as_str()).unwrap();
                    let mut others = get_possibles(system, part.clone(), w);
                    possibles.append(&mut others);
                    // return possibles;
                }
            }
        }
    }

    possibles
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
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day19.txt")
    }

    #[rstest]
    fn test_parse_system(test_input: Vec<String>) {
        let system = parse_system(&test_input);
        let expected_workflows = HashMap::from([
            (
                "px".to_string(),
                Workflow {
                    name: "px".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "a".to_string(),
                                op: Op::Lt,
                                val: 2006,
                            }),
                            action: Action::Process("qkq".to_string()),
                        },
                        Rule {
                            condition: Some(Condition {
                                part: "m".to_string(),
                                op: Op::Gt,
                                val: 2090,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Process("rfg".to_string()),
                        },
                    ],
                },
            ),
            (
                "pv".to_string(),
                Workflow {
                    name: "pv".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "a".to_string(),
                                op: Op::Gt,
                                val: 1716,
                            }),
                            action: Action::Reject,
                        },
                        Rule {
                            condition: None,
                            action: Action::Accept,
                        },
                    ],
                },
            ),
            (
                "lnx".to_string(),
                Workflow {
                    name: "lnx".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "m".to_string(),
                                op: Op::Gt,
                                val: 1548,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Accept,
                        },
                    ],
                },
            ),
            (
                "rfg".to_string(),
                Workflow {
                    name: "rfg".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "s".to_string(),
                                op: Op::Lt,
                                val: 537,
                            }),
                            action: Action::Process("gd".to_string()),
                        },
                        Rule {
                            condition: Some(Condition {
                                part: "x".to_string(),
                                op: Op::Gt,
                                val: 2440,
                            }),
                            action: Action::Reject,
                        },
                        Rule {
                            condition: None,
                            action: Action::Accept,
                        },
                    ],
                },
            ),
            (
                "qs".to_string(),
                Workflow {
                    name: "qs".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "s".to_string(),
                                op: Op::Gt,
                                val: 3448,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Process("lnx".to_string()),
                        },
                    ],
                },
            ),
            (
                "qkq".to_string(),
                Workflow {
                    name: "qkq".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "x".to_string(),
                                op: Op::Lt,
                                val: 1416,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Process("crn".to_string()),
                        },
                    ],
                },
            ),
            (
                "crn".to_string(),
                Workflow {
                    name: "crn".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "x".to_string(),
                                op: Op::Gt,
                                val: 2662,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Reject,
                        },
                    ],
                },
            ),
            (
                "in".to_string(),
                Workflow {
                    name: "in".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "s".to_string(),
                                op: Op::Lt,
                                val: 1351,
                            }),
                            action: Action::Process("px".to_string()),
                        },
                        Rule {
                            condition: None,
                            action: Action::Process("qqz".to_string()),
                        },
                    ],
                },
            ),
            (
                "qqz".to_string(),
                Workflow {
                    name: "qqz".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "s".to_string(),
                                op: Op::Gt,
                                val: 2770,
                            }),
                            action: Action::Process("qs".to_string()),
                        },
                        Rule {
                            condition: Some(Condition {
                                part: "m".to_string(),
                                op: Op::Lt,
                                val: 1801,
                            }),
                            action: Action::Process("hdj".to_string()),
                        },
                        Rule {
                            condition: None,
                            action: Action::Reject,
                        },
                    ],
                },
            ),
            (
                "gd".to_string(),
                Workflow {
                    name: "gd".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "a".to_string(),
                                op: Op::Gt,
                                val: 3333,
                            }),
                            action: Action::Reject,
                        },
                        Rule {
                            condition: None,
                            action: Action::Reject,
                        },
                    ],
                },
            ),
            (
                "hdj".to_string(),
                Workflow {
                    name: "hdj".to_string(),
                    rules: vec![
                        Rule {
                            condition: Some(Condition {
                                part: "m".to_string(),
                                op: Op::Gt,
                                val: 838,
                            }),
                            action: Action::Accept,
                        },
                        Rule {
                            condition: None,
                            action: Action::Process("pv".to_string()),
                        },
                    ],
                },
            ),
        ]);
        let expected_parts = vec![
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            },
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496,
            },
            Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244,
            },
            Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291,
            },
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013,
            },
        ];

        assert_eq!(
            system,
            System {
                workflows: expected_workflows,
                parts: expected_parts
            }
        );
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let system = parse_system(&test_input);
        let res = get_total_of_accepted_parts(&system);

        assert_eq!(res, 19114);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let system = parse_system(&puzzle_input);
        let res = get_total_of_accepted_parts(&system);

        assert_eq!(res, 353553);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let system = parse_system(&test_input);
        let res = get_possible_combinations(&system);

        assert_eq!(res, 167409079868000);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let system = parse_system(&puzzle_input);
        let res = get_possible_combinations(&system);

        assert_eq!(res, 167409079868000);
    }
}
