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
            println!("{number}");
            number
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day22.txt").unwrap();

    let secret_numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("{secret_numbers:?}");

    let part1_res = part1(&secret_numbers);
    println!("part 1: {part1_res}");
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
}
