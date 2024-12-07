#[derive(Debug)]
struct Combination {
    total: u64,
    operands: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
}

fn parse_combinations(input: &str) -> Vec<Combination> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(":");
            let total = iter.next().unwrap().parse().unwrap();
            let operands = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Combination { total, operands }
        })
        .collect()
}

fn check_combination(combination: &Combination) -> bool {
    let num_operations = combination.operands.len() - 1;

    // brute force all combinations
    let pow2 = 1 << num_operations;
    for bitset in 0..pow2 {
        let mut res = combination.operands[0];
        for i in 0..num_operations {
            let op = if (bitset & (1 << i)) > 0 {
                Operation::Mul
            } else {
                Operation::Add
            };

            let next_operand = combination.operands[i + 1];

            res = match op {
                Operation::Add => res + next_operand,
                Operation::Mul => res * next_operand,
            };
        }

        if res == combination.total {
            return true;
        }
    }

    false
}

fn part1(combinations: &[Combination]) -> u64 {
    combinations
        .iter()
        .filter(|c| check_combination(c))
        .map(|c| c.total)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    let combinations = parse_combinations(&input);

    let part1_res = part1(&combinations);
    println!("part 1 result: {part1_res}");
}
