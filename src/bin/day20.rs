use std::collections::HashMap;

use aoc2024::{
    direction::Direction,
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

fn shortcut_savings(input: &Input) -> HashMap<u32, u32> {
    let mut distance_from_start = HashMap::new();
    distance_from_start.insert(input.start, 0);

    let mut next = vec![input.start];
    let mut path = vec![];

    while let Some(node) = next.pop() {
        path.push(node);
        for direction in Direction::ALL {
            if let Some(neighbor) = direction.get_neighbor(node, &input.grid) {
                if input.grid[neighbor] == Tile::Empty
                    && !distance_from_start.contains_key(&neighbor)
                {
                    next.push(neighbor);
                    distance_from_start.insert(neighbor, distance_from_start[&node] + 1);
                }
            }
        }
    }

    let mut shortcut_savings = HashMap::new();

    for node in path.iter().cloned() {
        for direction in Direction::ALL {
            if let Some(neighbor) = direction.get_neighbor(node, &input.grid) {
                if input.grid[neighbor] != Tile::Wall {
                    continue;
                }
                let Some(next_in_direction) = direction.get_neighbor(neighbor, &input.grid) else {
                    continue;
                };
                if input.grid[next_in_direction] != Tile::Empty {
                    continue;
                }

                if distance_from_start[&next_in_direction] < distance_from_start[&node] {
                    // going backwards
                    continue;
                }

                let savings =
                    distance_from_start[&next_in_direction] - distance_from_start[&node] - 2;
                let count = shortcut_savings.entry(savings).or_default();
                *count += 1;
            }
        }
    }

    shortcut_savings
}

fn part1(input: &Input) -> u32 {
    let shortcut_savings = shortcut_savings(input);
    shortcut_savings
        .iter()
        .filter_map(|(k, v)| (*k >= 100).then_some(*v))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day20.txt").unwrap();
    let input = parse(&input);
    input.print();

    let part1 = part1(&input);
    println!("part 1: {part1}");
}
