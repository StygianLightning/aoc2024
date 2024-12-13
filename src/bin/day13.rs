use aoc2024::index2::{uidx2, UIndex2};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Configuration {
    a: UIndex2,
    b: UIndex2,
    target: UIndex2,
}

fn parse(input: &str) -> Vec<Configuration> {
    let mut ret = vec![];

    let mut current = Configuration {
        a: UIndex2::zero(),
        b: UIndex2::zero(),
        target: UIndex2::zero(),
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
            current.a = uidx2(x, y);
        } else if line.contains("B") {
            current.b = uidx2(x, y);
        } else if line.contains("Prize") {
            current.target = uidx2(x, y);
            ret.push(current);
        } else {
            panic!("unknown input line {line}");
        }
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    let config = parse(&input);
    println!("{config:#?}");
}
