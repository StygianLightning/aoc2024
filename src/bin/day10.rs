use aoc2024::{grid::Grid, index2::uidx2};

fn parse(input: &str) -> Grid<u32> {
    let num_lines = input.lines().count();
    let num_per_line = input.lines().next().unwrap().len();

    let mut grid = Grid::new_with_default(uidx2(num_per_line as _, num_lines as _));

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.char_indices() {
            let idx = uidx2(x as _, y as _);
            grid[idx] = c.to_digit(10).unwrap();
        }
    }

    grid
}

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();

    let grid = parse(&input);
    for y in 0..grid.dimension().y {
        for x in 0..grid.dimension().x {
            print!("{}", grid[uidx2(x, y)]);
        }
        println!();
    }
}
