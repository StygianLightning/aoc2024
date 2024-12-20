use aoc2024::{
    grid::Grid,
    index2::{uidx2, UIndex2},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug)]
struct Input {
    start: UIndex2,
    end: UIndex2,
    grid: Grid<Tile>,
}

impl Input {
    fn print(&self) {
        for y in 0..self.grid.dimension().y {
            for x in 0..self.grid.dimension().x {
                let idx = uidx2(x, y);
                match self.grid[idx] {
                    Tile::Empty => {
                        if idx == self.start {
                            print!("S");
                        } else if idx == self.end {
                            print!("E");
                        } else {
                            print!(".")
                        }
                    }
                    Tile::Wall => print!("#"),
                }
            }
            println!();
        }
    }
}

fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().take_while(|l| !l.trim().is_empty()).count() as u32;
    let size = uidx2(width, height);

    let mut grid = Grid::new_with_default(size);
    let mut start = uidx2(0, 0);
    let mut end = uidx2(0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let idx = uidx2(x as u32, y as u32);
            grid[idx] = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    start = idx;

                    Tile::Empty
                }
                'E' => {
                    end = idx;
                    Tile::Empty
                }
                _ => panic!("unknown tile {c}"),
            };
        }
    }

    Input { grid, start, end }
}

fn main() {
    let input = std::fs::read_to_string("input/day20.txt").unwrap();
    let input = parse(&input);
    input.print();
}
