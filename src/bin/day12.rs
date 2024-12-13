use std::collections::HashSet;

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, UIndex2},
};

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

fn part1(grid: &Grid<char>) -> u32 {
    let mut ret = 0;

    let mut visited = Grid::new_with_default(grid.dimension());
    let mut open = Vec::new();

    for x in 0..grid.dimension().x {
        for y in 0..grid.dimension().y {
            open.push(uidx2(x, y));
        }
    }

    while let Some(idx) = open.pop() {
        if visited[idx] {
            continue;
        }
        ret += floodfill(idx, grid, &mut visited);
    }

    ret
}

fn floodfill(start_idx: UIndex2, grid: &Grid<char>, visited: &mut Grid<bool>) -> u32 {
    let mut num_fences = 0;
    let mut area = 0;
    let letter = grid[start_idx];

    let mut next = Vec::new();
    next.push(start_idx);
    visited[start_idx] = true;

    while let Some(idx) = next.pop() {
        area += 1;
        for direction in Direction::ALL.into_iter() {
            if let Some(neighbor) = direction.get_neighbor(idx, grid) {
                if grid[neighbor] == letter {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        next.push(neighbor);
                    }
                } else {
                    num_fences += 1;
                }
            } else {
                num_fences += 1;
            }
        }
    }

    println!("region {letter} perimeter {num_fences}  area {area}");
    num_fences * area
}

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let grid = parse(&input);

    let part1_res = part1(&grid);
    println!("part 1 result: {part1_res}");
}
