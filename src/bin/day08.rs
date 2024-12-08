use std::collections::{HashMap, HashSet};

use aoc2024::{
    grid::Grid,
    index2::{uidx2, Index2, UIndex2},
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

fn calculate_antinodes(grid: &Grid<char>, only_once: bool) -> usize {
    let positions_per_node = positions_per_node(grid);

    let mut anti_nodes = HashSet::new();

    for (_, positions) in positions_per_node {
        for (i, pos_a) in positions.iter().enumerate() {
            for pos_b in positions[i + 1..].iter() {
                let pos_a = pos_a.to_index2();
                let pos_b = pos_b.to_index2();

                let diff = pos_b - pos_a;
                let c: Box<dyn Iterator<Item = Index2>> = if only_once {
                    // part 1: only two anti-nodes per pair
                    Box::new(std::iter::once(pos_b + diff))
                } else {
                    // part 2: the entire line inside the grid.
                    // in-between points are not necessary (i.e. if diff was (2, 2), we don't need to consider the normalized (1, 1) instead).
                    let mut next = pos_b;
                    Box::new(std::iter::repeat_with(move || {
                        let ret = next;
                        next = next + diff;
                        ret
                    }))
                };
                let d: Box<dyn Iterator<Item = Index2>> = if only_once {
                    Box::new(std::iter::once(pos_a - diff))
                } else {
                    let mut next = pos_a;
                    Box::new(std::iter::repeat_with(move || {
                        let ret = next;
                        next = next - diff;
                        ret
                    }))
                };

                fn check_position(pos: Index2, grid: &Grid<char>) -> bool {
                    if pos.x < 0 || pos.y < 0 {
                        return false;
                    }
                    let pos = uidx2(pos.x as _, pos.y as _);

                    grid.get(pos).is_some()
                }

                for iterator in [c, d] {
                    for pos in iterator {
                        if !check_position(pos, grid) {
                            break;
                        }
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

    let part1_res = calculate_antinodes(&grid, true);
    println!("part 1 result : {part1_res}");

    let part2_res = calculate_antinodes(&grid, false);
    println!("part 2 result : {part2_res}");
}
