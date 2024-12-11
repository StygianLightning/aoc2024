use std::collections::HashSet;

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, UIndex2},
};

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

fn neighbors(idx: UIndex2, grid: &Grid<u32>) -> impl Iterator<Item = UIndex2> + use<'_> {
    Direction::ALL
        .iter()
        .cloned()
        .filter_map(move |dir| dir.get_neighbor(idx, grid))
}

fn part1(grid: &Grid<u32>) -> u32 {
    let mut ret = 0;

    let mut starts = vec![];
    for y in 0..grid.dimension().y {
        for x in 0..grid.dimension().x {
            let idx = uidx2(x, y);
            if grid[idx] == 0 {
                starts.push(idx);
            }
        }
    }

    for start in starts.into_iter() {
        let mut open = vec![];
        let mut next_open = vec![];
        open.push(start);

        for _ in 0..9 {
            for idx in open.iter().cloned() {
                for neighbor_idx in neighbors(idx, grid) {
                    if grid[neighbor_idx] != grid[idx] + 1 {
                        continue;
                    }
                    next_open.push(neighbor_idx);
                }
            }

            // prepare for next iteration
            std::mem::swap(&mut open, &mut next_open);
            next_open.clear();
        }

        ret += open.into_iter().collect::<HashSet<_>>().len() as u32;
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    let grid = parse(&input);

    let part1_res = part1(&grid);
    println!("part 1 res: {part1_res}");
}
