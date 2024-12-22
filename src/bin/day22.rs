fn main() {
    let input = std::fs::read_to_string("input/day22.txt").unwrap();

    let secret_numbers = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("{secret_numbers:?}");
}
