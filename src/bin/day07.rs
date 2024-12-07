#[derive(Debug)]
struct Combination {
    total: u32,
    operands: Vec<u32>,
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

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    let combinations = parse_combinations(&input);
    println!("{combinations:#?}");
}
