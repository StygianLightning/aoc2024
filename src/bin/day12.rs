use std::collections::{HashMap, HashSet};

use aoc2024::{
    direction::Direction,
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

fn calculate_area_costs(grid: &Grid<char>) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut visited = Grid::new_with_default(grid.dimension());
    let mut open = Vec::new();

    for y in (0..grid.dimension().y).rev() {
        for x in (0..grid.dimension().x).rev() {
            // reverse order to guarantee floods are started top to bottom, left to right
            open.push(uidx2(x, y));
        }
    }

    while let Some(idx) = open.pop() {
        if visited[idx] {
            continue;
        }
        let (a, b) = area_costs(idx, grid, &mut visited);
        part1 += a;
        part2 += b;
    }

    (part1, part2)
}

fn area_costs(start_idx: UIndex2, grid: &Grid<char>, visited: &mut Grid<bool>) -> (u32, u32) {
    let mut num_fences = 0;
    let mut area = 0;
    let letter = grid[start_idx];

    let mut next = Vec::new();
    next.push(start_idx);
    visited[start_idx] = true;

    // pairs of lattice indices representing edges/sides of each region
    let mut edges_per_lattice_index = HashMap::new();

    while let Some(idx) = next.pop() {
        area += 1;
        for direction in Direction::ALL.into_iter() {
            if let Some(neighbor) = direction.get_neighbor(idx, grid) {
                if grid[neighbor] == letter {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        next.push(neighbor);
                    }
                    continue;
                }
            }
            num_fences += 1;

            fn add_edges(
                idx: Index2,
                direction: Direction,
                edges_map: &mut HashMap<Index2, HashSet<Direction>>,
            ) {
                let edges = edges_map.entry(idx).or_default();
                edges.insert(direction);
                let target_idx = idx + direction.to_index2();
                let edges = edges_map.entry(target_idx).or_default();
                edges.insert(direction.invert());
            }
            // add edges
            match direction {
                // each edge has its top left lattice vertex first.
                // moving up or down means the fence separating the two distinct regions will be horizontal
                Direction::Up =>
                // lattice indices: top left = 0,0, bottom right = (width + 1, height + 1)
                // edges: from, to
                // the index of a cell is the same as its top right corner's lattice index
                {
                    add_edges(
                        idx.to_index2(),
                        Direction::Right,
                        &mut edges_per_lattice_index,
                    )
                }
                Direction::Down => {
                    let lower = Direction::Down.offset_index(idx.to_index2());
                    add_edges(lower, Direction::Right, &mut edges_per_lattice_index);
                }
                // moving up or down means the fence separating the two distinct regions will be vertical
                Direction::Left => add_edges(
                    idx.to_index2(),
                    Direction::Down,
                    &mut edges_per_lattice_index,
                ),

                Direction::Right => {
                    let right = Direction::Right.offset_index(idx.to_index2());
                    add_edges(right, Direction::Down, &mut edges_per_lattice_index);
                }
            };
        }
    }

    let mut vertices = edges_per_lattice_index.keys().cloned().collect::<Vec<_>>();
    vertices.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)).reverse());
    let mut visited = HashSet::new();

    let mut num_sides = 0;
    // iterate over all open vertices after sorting by top left to guarantee we catch each connected edge in the right order to not miscount.
    while let Some(start_vertex) = vertices.pop() {
        if visited.contains(&start_vertex) {
            // only check visited vertices when popping them to start a new flood fill since vertices can be visited multiple times in case of an "intersection"
            continue;
        }

        visited.insert(start_vertex);
        let directions = &edges_per_lattice_index[&start_vertex];
        let mut current_direction = directions.iter().next().unwrap().clone();
        let mut current_vertex = start_vertex + current_direction.to_index2();

        loop {
            visited.insert(current_vertex);
            if current_vertex == start_vertex {
                num_sides += 1; // close the loop
                break;
            }
            let directions = &edges_per_lattice_index[&current_vertex];

            if directions.len() == 4 {
                // an intersection: say we're flood filling an area with letter 'A', this looks like
                // AB
                // CA
                // (or mirrored), and both regions of 'A' are connected on the outside.

                // direction always changes here
                num_sides += 1;

                // check the bottom right char to determines which layout the intersection has
                let bottom_right = uidx2(current_vertex.x as _, current_vertex.y as _);
                let bottom_right_char = grid[bottom_right];
                if bottom_right_char == letter {
                    // layout is (assuming letter is 'A')
                    // AB
                    // CA
                    current_direction = match current_direction {
                        Direction::Up => Direction::Left,    // stay inside 'C'
                        Direction::Right => Direction::Down, // stay inside 'C'
                        Direction::Down => Direction::Right, // stay inside 'B'
                        Direction::Left => Direction::Up,    // stay inside 'B'
                    };
                } else {
                    // layout is (assuming letter is 'A')
                    // BA
                    // AC
                    current_direction = match current_direction {
                        Direction::Up => Direction::Right,  // stay inside 'C'
                        Direction::Left => Direction::Down, // stay inside 'C'
                        Direction::Right => Direction::Up,  // stay inside 'B'
                        Direction::Down => Direction::Left, // stay inside 'B'
                    };
                }
            } else {
                for direction in directions.iter().cloned() {
                    if direction != current_direction.invert() {
                        // ignore the reverse edge of the one we took to get here
                        if direction != current_direction {
                            current_direction = direction;
                            num_sides += 1;
                            break;
                        }
                    }
                }
            }

            let next_vertex = current_vertex + current_direction.to_index2();
            current_vertex = next_vertex;
        }
    }

    (num_fences * area, num_sides * area)
}

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let grid = parse(&input);

    let (part1_res, part2_res) = calculate_area_costs(&grid);
    println!("part 1 result: {part1_res}");
    println!("part 2 result: {part2_res}");
}
