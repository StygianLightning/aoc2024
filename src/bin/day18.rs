use std::collections::HashSet;

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, UIndex2},
};

fn parse(input: &str) -> Vec<UIndex2> {
    input
        .lines()
        .map(|line| {
            let mut it = line.trim().split(",");
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            uidx2(a, b)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Blocked,
}

fn part1(indices: &[UIndex2], grid_size: UIndex2) -> u32 {
    let mut grid = Grid::new_with_default(grid_size);

    for idx in indices.iter().cloned() {
        grid[idx] = Tile::Blocked;
    }

    let start = uidx2(0, 0);
    let target = grid_size - uidx2(1, 1);

    let mut open = vec![start];
    let mut next = vec![];

    // we could also not save the grid and just put all the blocked indices in `visited`. Let's wait for the part 2 twist first.
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut ret = 0;

    loop {
        while let Some(idx) = open.pop() {
            if idx == target {
                return ret;
            }
            for d in Direction::ALL {
                let Some(idx) = d.get_neighbor(idx, &grid) else {
                    continue;
                };

                if visited.contains(&idx) || grid[idx] == Tile::Blocked {
                    continue;
                }

                next.push(idx);
                visited.insert(idx);
            }
        }
        ret += 1;
        std::mem::swap(&mut open, &mut next);
        next.clear();
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day18.txt").unwrap();
    let input = std::fs::read_to_string("input/test_day18.txt").unwrap();

    let grid_size = uidx2(71, 71);
    let grid_size = uidx2(7, 7);

    let indices = parse(&input);

    let part1_limit = 1024;
    let part1_limit = 12;

    let part1_res = part1(&indices[..part1_limit], grid_size);
    println!("part 1 result: {part1_res}");
}
