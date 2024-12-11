use std::collections::HashSet;

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, Index2, UIndex2},
};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Blocked,
}
#[derive(Debug, Copy, Clone)]
struct Guard {
    direction: Direction,
    position: UIndex2,
}

impl Guard {
    fn next_position(&self, grid: &Grid<Tile>) -> Option<UIndex2> {
        self.direction.get_neighbor(self.position, grid)
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

    let (mut grid, guard) = parse(&input);

    let part1_res = part1(&grid, guard);

    println!("part 1 result: {part1_res}");

    let part2_res = part2(&mut grid, guard);
    println!("part 2 result: {part2_res}");
}

#[derive(Debug)]
struct WalkResult {
    seen: HashSet<(UIndex2, Direction)>,
    looped: bool,
}

fn walk(grid: &Grid<Tile>, mut guard: Guard) -> WalkResult {
    let mut seen: HashSet<(UIndex2, Direction)> = HashSet::new();

    loop {
        if seen.contains(&(guard.position, guard.direction)) {
            return WalkResult { seen, looped: true };
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
    WalkResult {
        seen,
        looped: false,
    }
}

fn part1(grid: &Grid<Tile>, guard: Guard) -> u32 {
    let WalkResult { seen, .. } = walk(grid, guard);
    // count all seen squares, but ignore multiple visits from different directions
    let all_visited = seen
        .iter()
        .map(|(position, _)| *position)
        .collect::<HashSet<_>>();

    /*
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
    */

    all_visited.len() as _
}

fn part2(grid: &mut Grid<Tile>, guard: Guard) -> u32 {
    let WalkResult { seen, .. } = walk(grid, guard);

    // block each visited position, check for loops, unblock after.
    let mut potential_blocks = seen
        .iter()
        .map(|(position, _)| *position)
        .collect::<HashSet<_>>();
    potential_blocks.remove(&guard.position);

    let mut ret = 0;
    for position in potential_blocks {
        grid[position] = Tile::Blocked;

        let WalkResult { looped, .. } = walk(grid, guard);
        if looped {
            ret += 1;
        }

        grid[position] = Tile::Empty;
    }

    ret
}
