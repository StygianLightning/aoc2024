use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc2024::index2::{u64idx2, U64Index2};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Configuration {
    a: U64Index2,
    b: U64Index2,
    target: U64Index2,
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
        a: U64Index2::zero(),
        b: U64Index2::zero(),
        target: U64Index2::zero(),
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
            current.a = u64idx2(x, y);
        } else if line.contains("B") {
            current.b = u64idx2(x, y);
        } else if line.contains("Prize") {
            current.target = u64idx2(x, y);
            ret.push(current);
        } else {
            panic!("unknown input line {line}");
        }
    }

    ret
}

fn find_min_cost(configs: &[Configuration]) -> u32 {
    configs.iter().map(|c| shortest_path(c)).sum()
}

fn shortest_path(configuration: &Configuration) -> u32 {
    let mut inserted = HashSet::new();
    let start = ClawInputs { a: 0, b: 0 };
    inserted.insert(start);

    let mut heap = BinaryHeap::new();
    // heap is max heap, Reverse to get min cost
    heap.push(Reverse(start));

    while let Some(Reverse(input)) = heap.pop() {
        let position = input.a as u64 * configuration.a + input.b as u64 * configuration.b;
        if position == configuration.target {
            return input.cost();
        }
        if position.x > configuration.target.x || position.y > configuration.target.y {
            continue;
        }

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
            if !inserted.contains(&successor) {
                inserted.insert(successor);
                heap.push(Reverse(successor));
            }
        }
    }

    0
}

fn part2(configs: &[Configuration]) -> u64 {
    configs
        .iter()
        .map(|c| {
            let num = (c.a.y * c.target.x) as i128 - (c.a.x * c.target.y) as i128;
            let denom = (c.a.y * c.b.x) as i128 - (c.a.x * c.b.y) as i128;
            let amount_b = num / denom;

            let num = c.target.x as i128 - amount_b * c.b.x as i128;
            let denom = c.a.x as i128;
            let amount_a = num / denom;

            if amount_a as u64 * c.a + amount_b as u64 * c.b != c.target {
                0
            } else {
                amount_a as u64 * 3 + amount_b as u64
            }
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    let configs = parse(&input);

    let part1_res = find_min_cost(&configs);
    println!("part 1 result: {part1_res}");

    const OFFSET: u64 = 10000000000000;
    let offset = u64idx2(OFFSET, OFFSET);

    let configs = configs
        .iter()
        .map(|c| Configuration {
            a: c.a,
            b: c.b,
            target: c.target + offset,
        })
        .collect::<Vec<_>>();
    let part2_res = part2(&configs);
    println!("part 2 result: {part2_res}")
}
