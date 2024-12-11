use std::collections::HashMap;

fn blink(nums: &[u32], num_iterations: u32) -> u64 {
    let mut current = HashMap::new();

    for num in nums {
        current.entry(*num as u64).or_insert(1);
    }
    let mut next = HashMap::new();

    for _ in 0..num_iterations {
        for (x, amount) in current.iter() {
            if *x == 0 {
                let successor = 1;
                let successors = next.entry(successor).or_default();
                *successors += *amount;
            } else if x.ilog10() % 2 == 1 {
                let num_digits = x.ilog10() + 1;
                let separator = u64::pow(10, num_digits / 2);
                let left = *x / separator;
                let right = *x % separator;

                let left = next.entry(left).or_default();
                *left += *amount;
                let right = next.entry(right).or_default();
                *right += *amount;
            } else {
                let successor = x * 2024;
                let successors = next.entry(successor).or_default();
                *successors += *amount;
            }
        }
        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    current.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    let nums = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    let part1_res = blink(&nums, 25);
    println!("part 1: {part1_res}");

    let part2_res = blink(&nums, 75);
    println!("part 2: {part2_res}");
}
