#[derive(Debug)]
struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

fn parse(input: &str) -> Input {
    let mut it = input.lines();
    let towels = it
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let designs = it.skip(1).map(|line| line.to_owned()).collect();

    Input { towels, designs }
}

fn main() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();

    let input = parse(&input);
    println!("{input:?}");
}
