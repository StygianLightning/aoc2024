use aoc2024::{
    grid::Grid,
    index2::{self, uidx2, Index2, UIndex2},
};

fn parse_grid(input: &str) -> Grid<char> {
    let num_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().len();
    let mut grid = Grid::new_with_default(uidx2(line_len as _, num_lines as _));
    for (line_idx, line) in input.lines().enumerate() {
        for (row_idx, c) in line.char_indices() {
            grid[uidx2(row_idx as _, line_idx as _)] = c;
        }
    }
    grid
}

fn get_neighbor<T>(position: UIndex2, direction: Index2, grid: &Grid<T>) -> Option<UIndex2> {
    let target = position.to_index2() + direction;
    if target.x < 0 || target.y < 0 {
        return None;
    }
    let target = uidx2(target.x as _, target.y as _);
    if target.x < grid.dimension().x && target.y < grid.dimension().y {
        return Some(target);
    } else {
        return None;
    }
}

fn part1(grid: &Grid<char>) -> usize {
    let mut ret = 0;
    let to_find = ['X', 'M', 'A', 'S'];

    for line in 0..grid.dimension().y {
        for x in 0..grid.dimension().x {
            let idx = uidx2(line, x);
            ret += find_targets(idx, grid, &to_find);
        }
    }

    ret
}

fn find_targets(idx: UIndex2, grid: &Grid<char>, to_find: &[char]) -> usize {
    if grid[idx] != to_find[0] {
        return 0;
    }
    let directions = [
        Index2::new(-1, -1),
        Index2::new(-1, 0),
        Index2::new(-1, 1),
        Index2::new(0, -1),
        // no (0, 0)
        Index2::new(0, 1),
        Index2::new(1, -1),
        Index2::new(1, 0),
        Index2::new(1, 1),
    ];

    let mut ret = 0;

    'next_direction: for d in directions {
        let mut idx = idx;
        for i in 0..to_find.len() - 1 {
            if let Some(target) = get_neighbor(idx, d, grid) {
                idx = target;
                if grid[idx] == to_find[i + 1] {
                    continue;
                }
            }
            continue 'next_direction;
        }
        ret += 1;
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();
    let grid = parse_grid(&input);
    let part1_res = part1(&grid);
    println!("part 1: {part1_res}");
}
