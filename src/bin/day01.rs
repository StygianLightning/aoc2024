use std::collections::HashMap;

fn extract_nums(list: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first = vec![];
    let mut second = vec![];
    for line in list.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse().unwrap();
        first.push(a);
        let b = iter.next().unwrap().parse().unwrap();
        second.push(b);
    }
    (first, second)
}

fn part1(first: &mut Vec<u32>, second: &mut Vec<u32>) -> u32 {
    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(first: &[u32], second: &[u32]) -> u32 {
    let mut score = 0;

    let mut frequency_map = HashMap::new();

    for i in second {
        let freq = frequency_map.entry(*i).or_insert(0);
        *freq += 1;
    }

    first
        .iter()
        .map(|x| x * frequency_map.get(x).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = "input/day01.txt";
    let input = std::fs::read_to_string(input).unwrap();

    let (mut first, mut second) = extract_nums(&input);
    let part1_result = part1(&mut first, &mut second);
    println!("part1 result = {part1_result}");

    let part2_result = part2(&first, &second);
    println!("part2 result = {part2_result}");
}
