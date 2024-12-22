use std::collections::{hash_map::Entry, HashMap};

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(x: u64) -> u64 {
    x % 16777216
}

fn next_number(input: u64) -> u64 {
    let input = prune(mix(input, input * 64));
    let input = prune(mix(input / 32, input));
    prune(mix(input, 2048 * input))
}

fn part1(secret_numbers: &[u64]) -> u64 {
    secret_numbers
        .iter()
        .cloned()
        .map(|mut number| {
            for _ in 0..2000 {
                number = next_number(number);
            }
            number
        })
        .sum()
}

type Sequence = [i32; 4];

fn compute_prices(mut secret_number: u64, num_secrets_generated: usize) -> Vec<u32> {
    std::iter::once((secret_number % 10) as u32)
        .chain(std::iter::repeat_with(move || {
            secret_number = next_number(secret_number);
            (secret_number % 10) as u32
        }))
        .take(num_secrets_generated + 1)
        .collect()
}

fn compute_diffs(numbers: &[u32]) -> Vec<i32> {
    let mut diffs = vec![];
    for i in 1..numbers.len() {
        let diff = numbers[i] as i32 - numbers[i - 1] as i32;
        diffs.push(diff);
    }
    diffs
}

fn sequence_map(secret_number: u64, num_secrets_generated: usize) -> HashMap<Sequence, u32> {
    let mut ret = HashMap::new();

    let numbers = compute_prices(secret_number, num_secrets_generated);
    let diffs = compute_diffs(&numbers);

    for i in 3..diffs.len() {
        let mut sequence: [i32; 4] = [0; 4];

        for j in 0..4 {
            sequence[j] = diffs[i - 3 + j];
        }

        /*
        if sequence.contains(&0) {
            // four consecutive changes are required
            continue;
        }
        */
        if let Entry::Vacant(entry) = ret.entry(sequence) {
            let price = numbers[i + 1]; // the first secret number has no diff with a previous number
            entry.insert(price);
        }
    }

    ret
}

fn merge_maps(mut a: HashMap<Sequence, u32>, b: HashMap<Sequence, u32>) -> HashMap<Sequence, u32> {
    for (k, v) in b.iter() {
        let a_entry = a.entry(*k).or_insert(0);
        *a_entry += v;
    }
    a
}

fn part2(secret_numbers: &[u64], num_secrets_generated: usize) -> u32 {
    let merged_map = secret_numbers
        .iter()
        .map(|secret_number| sequence_map(*secret_number, num_secrets_generated))
        .reduce(merge_maps)
        .unwrap();

    let best = find_best_sequence(&merged_map);
    best.1
}

fn find_best_sequence(map: &HashMap<Sequence, u32>) -> (Sequence, u32) {
    let (a, b) = map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    (*a, *b)
}

fn main() {
    let input = std::fs::read_to_string("input/day22.txt").unwrap();

    let secret_numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1_res = part1(&secret_numbers);
    println!("part 1: {part1_res}");

    let part2_res = part2(&secret_numbers, 2000);
    println!("part 2: {part2_res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_example() {
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        let mut current = 123;
        for i in 0..expected.len() {
            current = next_number(current);
            assert_eq!(current, expected[i]);
        }
    }

    #[test]
    fn test_sequence_prices() {
        let secret_number = 123;
        let prices = compute_prices(secret_number, 9);
        assert_eq!(&prices, &[3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);

        let sequence_prices = sequence_map(123, 10);
        let best = find_best_sequence(&sequence_prices);
        assert_eq!(best, ([-1, -1, 0, 2], 6));
    }

    #[test]
    fn test_part2() {
        let secret_numbers = [1, 2, 3, 2024u64];
        let num_secrets_generated = 2000;
        let sequence_maps = secret_numbers.map(|s| sequence_map(s, num_secrets_generated));
        let desired_sequence = [-2, 1, -1, 3];
        assert_eq!(sequence_maps[0][&desired_sequence], 7);
        assert_eq!(sequence_maps[1][&desired_sequence], 7);
        assert!(sequence_maps[2].get(&desired_sequence).is_none());
        assert_eq!(sequence_maps[3][&desired_sequence], 9);
        let result = part2(&secret_numbers, num_secrets_generated);
        assert_eq!(result, 23);
    }
}
