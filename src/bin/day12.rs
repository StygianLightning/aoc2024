use aoc2024::{grid::Grid, index2::uidx2};

fn parse(input: &str) -> Grid<char> {
    let num_lines = input.lines().count();
    let num_per_line = input.lines().next().unwrap().len();
    let mut grid = Grid::new_with_default(uidx2(num_per_line as _, num_lines as _));

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            grid[uidx2(x as _, y as _)] = c;
        }
    }

    grid
}

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let grid = parse(&input);
}
