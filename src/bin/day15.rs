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
    Wall,
}

#[derive(Debug)]
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
        }
    }
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
                Tile::Wall => print!("#"),
            }
        }
        println!();
    }

    let mut score = 0;

    for y in 0..input.grid.dimension().y {
        for x in 0..input.grid.dimension().x {
            let idx = uidx2(x, y);
            if let Tile::Box = input.grid[idx] {
                score += 100 * y as u64 + x as u64;
            }
        }
    }

    score
}

fn main() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();

    let mut input = parse(&input);
    let part1_res = part1(&mut input);
    println!("part 1 result: {part1_res}");
}
