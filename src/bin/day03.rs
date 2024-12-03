use regex::Regex;

fn extract_muls(s: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    re.captures_iter(s)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let muls = extract_muls(input);
    muls.iter().fold(0, |acc, (a, b)| acc + a * b)
}

fn main() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
    let part1_res = part1(&input);
    println!("part 1: {part1_res}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_muls() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let muls = extract_muls(input);
        assert_eq!(muls, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }
}
