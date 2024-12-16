use std::collections::HashSet;

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, UIndex2},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Box,
    LeftBox,
    RightBox,
    Wall,
}

impl Tile {
    fn is_box(self) -> bool {
        matches!(self, Tile::Box | Tile::LeftBox | Tile::RightBox)
    }
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<Tile>,
    robot: UIndex2,
    movements: Vec<Direction>,
}

fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().take_while(|l| !l.trim().is_empty()).count() as u32;
    let size = uidx2(width, height);

    let mut grid = Grid::new_with_default(size);
    let mut robot = uidx2(0, 0);
    let mut movements = vec![];

    let mut parsing_grid = true;

    for (y, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            parsing_grid = false;
        } else if parsing_grid {
            for (x, c) in line.char_indices() {
                let idx = uidx2(x as u32, y as u32);
                match c {
                    '#' => {
                        grid[idx] = Tile::Wall;
                    }
                    '.' => {
                        grid[idx] = Tile::Empty;
                    }
                    'O' => {
                        grid[idx] = Tile::Box;
                    }
                    '@' => {
                        robot = idx;
                    }
                    _ => panic!("unknown tile {c}"),
                }
            }
        } else {
            for c in line.chars() {
                let direction = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    'v' => Direction::Down,

                    _ => panic!("unknown direction {c}"),
                };
                movements.push(direction);
            }
        }
    }

    Input {
        grid,
        robot,
        movements,
    }
}

fn print_state(input: &Input) {
    for y in 0..input.grid.dimension().y {
        for x in 0..input.grid.dimension().x {
            let idx = uidx2(x, y);
            match input.grid[idx] {
                Tile::Empty => {
                    if idx == input.robot {
                        print!("@");
                    } else {
                        print!(".")
                    }
                }
                Tile::Box => print!("O"),
                Tile::LeftBox => print!("["),
                Tile::RightBox => print!("]"),
                Tile::Wall => print!("#"),
            }
        }
        println!();
    }
}

fn part1(input: &mut Input) -> u64 {
    for direction in &input.movements {
        let target = direction.get_neighbor(input.robot, &input.grid).unwrap();
        match input.grid[target] {
            Tile::Empty => input.robot = target,
            Tile::Box => {
                let mut end = target;
                loop {
                    end = direction.get_neighbor(end, &input.grid).unwrap();
                    if input.grid[end] != Tile::Box {
                        break;
                    }
                }
                if input.grid[end] == Tile::Empty {
                    // move and push all boxes
                    input.grid[target] = Tile::Empty;
                    input.grid[end] = Tile::Box;
                    input.robot = target;
                } else {
                    // blocked by wall, do nothing
                }
            }
            Tile::Wall => {}
            _ => panic!("only single-tile boxes are allowed in part 1"),
        }
    }
    print_state(input);

    score(input)
}

fn score(input: &mut Input) -> u64 {
    let mut score = 0;

    for y in 0..input.grid.dimension().y {
        for x in 0..input.grid.dimension().x {
            let idx = uidx2(x, y);
            if matches!(input.grid[idx], Tile::Box | Tile::LeftBox) {
                score += 100 * y as u64 + x as u64;
            }
        }
    }
    score
}

fn box_neighbor(position: UIndex2, grid: &Grid<Tile>) -> UIndex2 {
    match grid[position] {
        Tile::LeftBox => Direction::Right.get_neighbor(position, grid).unwrap(),
        Tile::RightBox => Direction::Left.get_neighbor(position, grid).unwrap(),
        _ => panic!("only left/right boxes are supported"),
    }
}

fn part2(input: &mut Input) -> u64 {
    for direction in &input.movements {
        let target = direction.get_neighbor(input.robot, &input.grid).unwrap();
        match input.grid[target] {
            Tile::Empty => input.robot = target,
            Tile::Box => panic!("single-tile boxes are not allowed in part 2"),
            Tile::LeftBox | Tile::RightBox => {
                let mut to_push = vec![];
                let mut seen = HashSet::new();

                // TODO can't do this with a stack, need to do breadth first approach
                let mut current = vec![target];
                let mut next = Vec::new();
                seen.insert(target);

                let vertical_push = matches!(direction, Direction::Down | Direction::Up);
                if vertical_push {
                    let neighbor = box_neighbor(target, &input.grid);
                    current.push(neighbor);
                    seen.insert(neighbor);
                }

                let mut push_possible = true;
                loop {
                    for idx in current.iter().cloned() {
                        to_push.push((idx, input.grid[idx]));
                        let Some(target) = direction.get_neighbor(idx, &input.grid) else {
                            push_possible = false;
                            break;
                        };
                        match input.grid[target] {
                            Tile::Empty => {
                                // do nothing
                            }
                            Tile::LeftBox | Tile::RightBox => {
                                // push both sides of the blocking box
                                if !seen.contains(&target) {
                                    next.push(target);
                                    seen.insert(target);
                                }
                                let box_neighbor = box_neighbor(target, &input.grid);
                                if vertical_push && !seen.contains(&box_neighbor) {
                                    next.push(box_neighbor);
                                    seen.insert(box_neighbor);
                                }
                            }
                            Tile::Wall => {
                                push_possible = false;
                                break;
                            }
                            Tile::Box => panic!("no single-tile boxes allowed in part 2"),
                        }
                    }
                    if next.is_empty() {
                        break;
                    }
                    std::mem::swap(&mut current, &mut next);
                    next.clear();
                }

                if push_possible {
                    // push all boxes & reset state in reverse order.
                    for (idx, tile) in to_push.iter().rev().cloned() {
                        let target = direction.get_neighbor(idx, &input.grid).unwrap();
                        input.grid[target] = tile;
                        input.grid[idx] = Tile::Empty;
                    }
                    input.robot = target;
                }
            }
            Tile::Wall => {} // push fails, do nothing
        }
    }

    println!("part 2 final state: ");
    print_state(input);
    score(input)
}

fn part2_input(input: &Input) -> Input {
    let new_dim = uidx2(2 * input.grid.dimension().x, input.grid.dimension().y);
    let mut new_grid = Grid::new_with_default(new_dim);

    for y in 0..input.grid.dimension().y {
        for x in 0..input.grid.dimension().x {
            let idx = uidx2(x, y);
            let output_idx = uidx2(idx.x * 2, idx.y);
            let output_right_idx = output_idx + uidx2(1, 0);
            match input.grid[idx] {
                Tile::Empty => {} // empty is default, do nothing
                Tile::Box => {
                    new_grid[output_idx] = Tile::LeftBox;
                    new_grid[output_right_idx] = Tile::RightBox;
                }
                Tile::Wall => {
                    new_grid[output_idx] = Tile::Wall;
                    new_grid[output_right_idx] = Tile::Wall;
                }
                _ => panic!("no double-width boxes in input"),
            }
        }
    }
    Input {
        grid: new_grid,
        robot: uidx2(2 * input.robot.x, input.robot.y),
        movements: input.movements.clone(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();

    let mut input = parse(&input);
    let mut part2_input = part2_input(&input);

    let part1_res = part1(&mut input);
    println!("part 1 result: {part1_res}");

    print_state(&part2_input);
    let part2_res = part2(&mut part2_input);
    println!("part 2 result: {part2_res}");
}
