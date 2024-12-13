use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc2024::index2::{uidx2, UIndex2};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Configuration {
    a: UIndex2,
    b: UIndex2,
    target: UIndex2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ClawInputs {
    a: u32,
    b: u32,
}

impl ClawInputs {
    fn cost(&self) -> u32 {
        self.a * 3 + self.b
    }
}

impl Ord for ClawInputs {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for ClawInputs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Configuration> {
    let mut ret = vec![];

    let mut current = Configuration {
        a: UIndex2::zero(),
        b: UIndex2::zero(),
        target: UIndex2::zero(),
    };
    let re = Regex::new(r#"X[+=](\d+),\sY[+=](\d+)"#).unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }
        let mut numbers = re.captures_iter(line).map(|c| c.extract());
        let (_, [x, y]) = numbers.next().unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        if line.contains("A") {
            current.a = uidx2(x, y);
        } else if line.contains("B") {
            current.b = uidx2(x, y);
        } else if line.contains("Prize") {
            current.target = uidx2(x, y);
            ret.push(current);
        } else {
            panic!("unknown input line {line}");
        }
    }

    ret
}

fn part1(configs: &[Configuration]) -> u32 {
    configs.iter().map(|c| shortest_path(c)).sum()
}

fn shortest_path(configuration: &Configuration) -> u32 {
    let mut inserted = HashSet::new();
    let start = ClawInputs { a: 0, b: 0 };
    inserted.insert(start);

    let mut heap = BinaryHeap::new();
    // heap is max heap, Reverse to get min cost
    heap.push(Reverse(start));

    const LIMIT: u32 = 100;

    while let Some(Reverse(input)) = heap.pop() {
        let position = input.a * configuration.a + input.b * configuration.b;
        if position == configuration.target {
            return input.cost();
        } else {
            // check all successors
            let successors = [
                ClawInputs {
                    a: input.a + 1,
                    b: input.b,
                },
                ClawInputs {
                    a: input.a,
                    b: input.b + 1,
                },
            ];
            for successor in successors {
                if successor.a > LIMIT || successor.b > LIMIT {
                    continue;
                }
                if !inserted.contains(&successor) {
                    inserted.insert(successor);
                    heap.push(Reverse(successor));
                }
            }
        }
    }

    0
}

fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    let config = parse(&input);

    let part1_res = part1(&config);
    println!("part 1 result: {part1_res}")
}
