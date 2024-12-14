use aoc2024::index2::Index2;

#[derive(Debug)]
struct Robot {
    position: Index2,
    velocity: Index2,
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            fn extract(text: &str) -> Index2 {
                let mut split = text.split("=").skip(1).next().unwrap().split(",");
                Index2::new(
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            }

            let position = extract(split.next().unwrap());
            let velocity = extract(split.next().unwrap());
            Robot { position, velocity }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    let robots = parse(&input);
    println!("{robots:?}");
}
