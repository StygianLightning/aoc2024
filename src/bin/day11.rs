fn part1(nums: &[u32]) -> u32 {
    // naive, no memoization

    nums.iter()
        .cloned()
        .map(|num| {
            let mut current = vec![num as u64];
            let mut next = vec![];
            for _ in 0..25 {
                for x in current.iter().cloned() {
                    if x == 0 {
                        next.push(1);
                    } else if x.ilog10() % 2 == 1 {
                        let num_digits = x.ilog10() + 1;
                        let separator = u64::pow(10, num_digits / 2);
                        let right = x % separator;
                        let left = x / separator;
                        next.push(left);
                        next.push(right);
                    } else {
                        next.push(x * 2024);
                    }
                }
                std::mem::swap(&mut current, &mut next);
                next.clear();
            }

            current.len() as u32
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    let nums = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    let part1_res = part1(&nums);
    println!("part 1: {part1_res}");
}
