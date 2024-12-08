use std::collections::{HashMap, HashSet};

use aoc2024::{
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

fn positions_per_node(grid: &Grid<char>) -> HashMap<char, Vec<UIndex2>> {
    let mut positions_per_node = HashMap::new();

    for x in 0..grid.dimension().x {
        for y in 0..grid.dimension().y {
            let idx = uidx2(x, y);
            let c = grid[idx];
            if c == '.' {
                continue;
            }

            let positions = positions_per_node.entry(c).or_insert(Vec::new());
            positions.push(idx);
        }
    }
    positions_per_node
}

fn part1(grid: &Grid<char>) -> usize {
    let positions_per_node = positions_per_node(grid);

    let mut anti_nodes = HashSet::new();

    for (_, positions) in positions_per_node {
        for (i, pos_a) in positions.iter().enumerate() {
            for pos_b in positions[i + 1..].iter() {
                let pos_a = pos_a.to_index2();
                let pos_b = pos_b.to_index2();

                let diff = pos_b - pos_a;
                let c = pos_b + diff;
                let d = pos_a - diff;

                for pos in [c, d] {
                    if pos.x < 0 || pos.y < 0 {
                        continue;
                    }
                    let pos = uidx2(pos.x as _, pos.y as _);

                    if grid.get(pos).is_some() {
                        anti_nodes.insert(pos);
                    }
                }
            }
        }
    }

    anti_nodes.len()
}

fn main() {
    let input = std::fs::read_to_string("input/day08.txt").unwrap();
    let grid = parse(&input);

    let part1_res = part1(&grid);
    println!("part 1 result : {part1_res}");
}
