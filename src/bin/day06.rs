use std::collections::{HashMap, HashSet};

use aoc2024::{
    grid::Grid,
    index2::{uidx2, Index2, UIndex2},
};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Blocked,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl Direction {
    fn to_index2(self) -> Index2 {
        match self {
            Direction::Up => Index2 { x: 0, y: -1 },
            Direction::Down => Index2 { x: 0, y: 1 },
            Direction::Left => Index2 { x: -1, y: 0 },
            Direction::Right => Index2 { x: 1, y: 0 },
        }
    }

    fn turn_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Guard {
    fn next_position(&self, grid: &Grid<Tile>) -> Option<UIndex2> {
        let offset = self.direction.to_index2();
        let target = self.position.to_index2() + offset;

        if target.x < 0 || target.y < 0 {
            return None;
        }

        let idx = uidx2(target.x as _, target.y as _);
        if grid.get(idx).is_some() {
            Some(idx)
        } else {
            None
        }
    }
}

fn parse(text: &str) -> (Grid<Tile>, Guard) {
    let num_lines = text.lines().count();
    let num_per_line = text.lines().next().unwrap().len();

    let mut grid = Grid::new_with_default(uidx2(num_per_line as _, num_lines as _));

    let mut position = uidx2(0, 0);

    for (line_idx, line) in text.lines().enumerate() {
        for (idx, c) in line.char_indices() {
            let idx = uidx2(idx as _, line_idx as _);

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

    let part1_res = part1(&grid, guard);

    println!("part 1 result: {part1_res}");
}

fn part1(grid: &Grid<Tile>, mut guard: Guard) -> u32 {
    let mut seen: HashSet<(UIndex2, Direction)> = HashSet::new();

    loop {
        if seen.contains(&(guard.position, guard.direction)) {
            break;
        }
        seen.insert((guard.position, guard.direction));

        let Some(next_position) = guard.next_position(grid) else {
            break;
        };
        if grid[next_position] == Tile::Blocked {
            guard.direction = guard.direction.turn_clockwise();
        } else {
            guard.position = next_position;
        }
    }

    // count all seen squares, but ignore multiple visits from different directions
    let all_visited = seen
        .iter()
        .map(|(position, _)| *position)
        .collect::<HashSet<_>>();

    for y in 0..grid.dimension().y {
        for x in 0..grid.dimension().x {
            let idx = uidx2(x, y);

            if all_visited.contains(&idx) {
                print!("X");
            } else {
                match grid[idx] {
                    Tile::Empty => print!("."),
                    Tile::Blocked => print!("#"),
                }
            }
        }
        println!();
    }
    println!();

    all_visited.len() as _
}
