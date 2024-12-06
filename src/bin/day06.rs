use aoc2024::{
    grid::Grid,
    index2::{uidx2, UIndex2},
};

#[derive(Debug, Copy, Clone, Default)]
enum Tile {
    #[default]
    Empty,
    Blocked,
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    position: UIndex2,
}

fn parse(text: &str) -> (Grid<Tile>, Guard) {
    let num_lines = text.lines().count();
    let num_per_line = text.lines().next().unwrap().len();

    let mut grid = Grid::new_with_default(uidx2(num_per_line as _, num_lines as _));

    let mut position = uidx2(0, 0);

    for (line_idx, line) in text.lines().enumerate() {
        for (idx, c) in line.char_indices() {
            let idx = uidx2(line_idx as _, idx as _);

            grid[idx] = match c {
                '#' => Tile::Blocked,
                '.' => Tile::Empty,
                '^' => {
                    position = idx;
                    Tile::Empty
                }
                _ => panic!("unknown tile: {c}"),
            };
        }
    }

    (
        grid,
        Guard {
            direction: Direction::Up,
            position,
        },
    )
}

fn main() {
    let input = std::fs::read_to_string("input/day06.txt").unwrap();
    let (grid, guard) = parse(&input);

    println!("{guard:?}");
}
