#[derive(Debug)]
struct Combination {
    total: u64,
    operands: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
    Concat,
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

fn check_combination(combination: &Combination, base: u64) -> bool {
    let num_operations = combination.operands.len() - 1;
    // brute force all combinations
    let pow = u64::pow(base, num_operations as _);

    for bitset in 0..pow {
        let mut res = combination.operands[0];
        let mut bitset = bitset;
        for i in 0..num_operations {
            let digit = bitset % base;
            bitset = bitset / base;

            let op = match digit {
                0 => Operation::Add,
                1 => Operation::Mul,
                2 => Operation::Concat,
                _ => panic!("unsupported digit {digit}"),
            };

            let next_operand = combination.operands[i + 1];

            res = match op {
                Operation::Add => res + next_operand,
                Operation::Mul => res * next_operand,
                Operation::Concat => {
                    let num_digits = format!("{next_operand}").len() as u32;
                    res * u64::pow(10, num_digits) + next_operand
                }
            };
        }

        if res == combination.total {
            return true;
        }
    }

    false
}

fn compute(combinations: &[Combination], base: u64) -> u64 {
    combinations
        .iter()
        .filter(|c| check_combination(c, base))
        .map(|c| c.total)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    let combinations = parse_combinations(&input);

    let part1_res = compute(&combinations, 2);
    println!("part 1 result: {part1_res}");

    let part2_res = compute(&combinations, 3);
    println!("part 2 result: {part2_res}");
}
