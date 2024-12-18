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

fn flood_fill(indices: &[UIndex2], grid: &mut Grid<Tile>) -> Result<u32, ()> {
    for idx in indices.iter().cloned() {
        grid[idx] = Tile::Blocked;
    }

    let start = uidx2(0, 0);
    let target = grid.dimension() - uidx2(1, 1);

    let mut open = vec![start];
    let mut next = vec![];

    let mut ret = 0;

    loop {
        while let Some(idx) = open.pop() {
            if idx == target {
                return Ok(ret);
            }
            for d in Direction::ALL {
                let Some(idx) = d.get_neighbor(idx, &grid) else {
                    continue;
                };

                if grid[idx] == Tile::Blocked {
                    continue;
                }

                next.push(idx);
                grid[idx] = Tile::Blocked; // no walk back
            }
        }
        if next.is_empty() {
            // flood fill failed
            return Err(());
        }
        ret += 1;
        std::mem::swap(&mut open, &mut next);
        next.clear();
    }
}

fn part2(indices: &[UIndex2], grid: &mut Grid<Tile>) -> usize {
    (1..=indices.len())
        .collect::<Vec<_>>()
        .partition_point(|i| {
            grid.reset_to_default();
            flood_fill(&indices[..*i], grid).is_ok()
        })
}

fn main() {
    let input = std::fs::read_to_string("input/day18.txt").unwrap();

    let grid_size = uidx2(71, 71);

    let indices = parse(&input);

    let mut grid = Grid::new_with_default(grid_size);

    let part1_limit = 1024;
    let part1_res = flood_fill(&indices[..part1_limit], &mut grid).unwrap();
    println!("part 1 result: {part1_res}");

    let part2_res = part2(&indices, &mut grid);
    let blocking_element = indices[part2_res];
    println!(
        "part 2 result: {},{}",
        blocking_element.x, blocking_element.y
    );
}
